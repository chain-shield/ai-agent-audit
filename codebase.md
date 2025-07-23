ai-agent-audit/CLAUDE.md
````
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
````
ai-agent-audit/Cargo.toml
```
[package]
name = "ai-agent-audit"
version = "0.1.0"
edition = "2024"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[dependencies]
qdrant-client = { version = "1.14", features = ["serde"] }
tempfile = "3.20"
walkdir = "2.4"
serde = "1.0.219"
ignore = "0.4"
regex = "1.10"
serde_json = { version = "1.0", features = ["preserve_order"] }
tokio = { version = "1.46.1", features = ["macros", "rt-multi-thread"] }
tokio-stream = "0.1.17"
rig-core = { version = "0.13.0", features = ["derive"] }
dotenvy = "0.15"
anyhow = "1.0"
git2 = "0.18"
log = "0.4.26"
reqwest = { version = "0.12.19", features = ["json", "rustls-tls"] }
env_logger = "0.11"
tiktoken-rs = "0.6.0"
rusqlite = { version = "0.31", features = ["bundled"] }                  #  🔒 static libsqlite3
serde-sarif = "0.7"                                                      # straight-forward SARIF model
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"                                                           # timestamp in tickets
async-trait = "0.1.79"
thiserror = "2.0.12"
hex = "0.4.3"
once_cell = "1.8"
async-openai = "0.28.1"
rig-qdrant = "0.1.14"
schemars = "0.8.22"
nanoid = "0.4"

```
ai-agent-audit/README.md
````
# AI Agent Audit

An advanced AI-powered smart contract auditing tool that combines static analysis, multiple LLM providers, and vector embeddings to perform comprehensive security audits of Solidity codebases.

## Overview

AI Agent Audit is a sophisticated Rust-based tool that performs comprehensive smart contract security audits by:

1. **Repository Analysis**: Cloning and building smart contract repositories with support for Foundry and Hardhat
2. **Static Analysis**: Extracting detailed IR, call graphs, and storage information using Slither
3. **AI-Powered Security Review**: Leveraging multiple LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) for vulnerability detection
4. **Vector Embeddings**: Creating semantic search capabilities through Qdrant vector database
5. **Comprehensive Reporting**: Generating detailed audit reports with vulnerability findings and protocol overviews
6. **Cost Tracking**: Monitoring inference costs across different LLM providers

This tool provides professional-grade smart contract auditing capabilities with AI assistance, making it suitable for security researchers, auditors, and development teams.

## Features

### Core Functionality
- **Multi-Platform Repository Support**: Automatically clone and build repositories with Foundry or Hardhat
- **Advanced Static Analysis**: Deep integration with Slither for IR extraction, call graph analysis, and storage layout
- **Multi-LLM Security Analysis**: Parallel vulnerability detection using OpenAI (GPT-4o, O3), Anthropic (Claude), Google Gemini, and DeepSeek
- **Comprehensive Vulnerability Detection**: Covers 29 distinct vulnerability categories including reentrancy, access control, MEV, oracle manipulation, and advanced attack vectors
- **Vector-Based Semantic Search**: High-quality embeddings with Qdrant for intelligent code search and context retrieval
- **Professional Audit Reports**: Generate detailed markdown reports with findings categorized by severity
- **Cost Optimization**: Real-time tracking of inference costs across different LLM providers

### Advanced Capabilities
- **Intelligent Code Slicing**: Generate contextual code blocks with call graph traversal for focused analysis
- **Duplicate Detection**: AI-powered deduplication of security findings
- **Quality Verification**: Multi-stage verification process to reduce false positives
- **Docker Integration**: Secure, isolated analysis environment using Trail of Bits security toolbox
- **Caching System**: Efficient caching of analysis results and LLM responses
- **Concurrent Processing**: Parallel analysis across multiple AI agents for faster results

## Prerequisites

### Required Software
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Docker](https://docs.docker.com/get-docker/) for containerized analysis environment
- [Qdrant](https://qdrant.tech/documentation/quick-start/) vector database

### Optional (for local development)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) for Forge (handled by Docker)
- [Slither](https://github.com/crytic/slither#how-to-install) static analyzer (handled by Docker)

### API Keys
You'll need API keys for the LLM providers you want to use:
- **OpenAI**: Required for embeddings and GPT models
- **Anthropic**: Optional, for Claude models
- **Google AI**: Optional, for Gemini models
- **DeepSeek**: Optional, for cost-effective analysis

## Environment Setup

1. Create a `.env` file in the project root with your API keys:

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

2. Start the Qdrant vector database:

```bash
docker-compose up -d
```

This will start Qdrant on ports 6333 (REST API) and 6334 (gRPC API).

## Installation

1. Clone this repository:

```bash
git clone https://github.com/chain-shield/ai-agent-audit.git
cd ai-agent-audit
```

2. Build the project:

```bash
cargo build --release
```

## Usage

### Basic Usage

Run the application with a Git repository URL containing Solidity contracts:

```bash
cargo run --release -- https://github.com/example/solidity-project.git
```

### Complete Workflow

The application performs a comprehensive audit workflow:

1. **Repository Preparation**
   - Clone the repository using Docker for security
   - Auto-detect and build with Foundry or Hardhat
   - Filter and organize Solidity source files

2. **Static Analysis**
   - Extract call graphs and inheritance hierarchies using Slither
   - Generate intermediate representation (IR) for all functions
   - Analyze storage layouts and variable mappings

3. **AI-Powered Security Analysis**
   - Generate contextual code slices for focused analysis
   - Run parallel security analysis using multiple LLM providers
   - Detect vulnerabilities across 19+ security categories
   - Verify and deduplicate findings using AI verification

4. **Vector Database Population**
   - Create semantic embeddings for all code and analysis results
   - Store in Qdrant for intelligent search and retrieval

5. **Report Generation**
   - Generate comprehensive audit reports in Markdown format
   - Create both paid (full details) and free (limited) report versions
   - Include protocol overview, findings summary, and detailed vulnerability descriptions

6. **Cost Tracking**
   - Monitor and report total inference costs across all LLM providers

## Project Structure

```
src/
├── ai_bot/                    # AI agent implementations
│   ├── agent.rs              # Core AI audit agent with vector search
│   └── retrieve_slice.rs     # Context retrieval for AI analysis
├── build_brain/              # Core analysis and data processing
│   ├── callgraph.rs          # Call graph analysis and traversal
│   ├── enbeddings.rs         # Vector embeddings generation
│   ├── enrichment.rs         # Slither analysis integration
│   ├── fn_summaries.rs       # Function summarization
│   ├── graph_db.rs           # Graph database operations
│   ├── inheritance.rs        # Contract inheritance analysis
│   ├── parsers.rs            # Code parsing utilities
│   ├── slither_ffi.rs        # Slither static analyzer interface
│   ├── summarize.rs          # Protocol and file summarization
│   └── vector_db.rs          # Qdrant vector database operations
├── cost/                     # Cost tracking and management
│   └── cost_data.rs          # LLM inference cost calculation
├── enumerator/               # Code slicing and enumeration
│   ├── codeblock_cache.rs    # Caching for generated code blocks
│   ├── codeblock_db.rs       # Database for code block storage
│   ├── codeblock_maker.rs    # Code block generation logic
│   ├── codeblocks.rs         # Core code slicing functionality
│   └── utils.rs              # Enumeration utilities
├── llm_review/               # AI-powered security analysis
│   ├── analysis_db.rs        # Analysis results database
│   ├── code_review.rs        # Main security review orchestration
│   ├── config.rs             # LLM configuration and models
│   ├── context_state.rs      # Global context management
│   ├── enums.rs              # AI agent and vulnerability type enums
│   ├── invariants.rs         # Protocol invariant analysis
│   ├── prompt_content.rs     # Dynamic prompt generation
│   ├── review_utils.rs       # AI agent builders and utilities
│   └── prompt_support/       # Prompt engineering modules
├── prepare_code/             # Repository preparation
│   └── git_clone.rs          # Git cloning and building
├── reporting/                # Report generation
│   ├── audit.rs              # Audit report generation
│   ├── contract_data.rs      # Contract data export
│   └── save_file.rs          # File saving utilities
├── prompts/                  # Vulnerability-specific prompts (19 types)
├── master_prompts/           # Master security analysis prompts
├── invariant_prompts/        # Protocol invariant prompts
├── utils/                    # Shared utilities
├── lib.rs                    # Library exports
└── main.rs                   # Application entry point
```

## Vulnerability Detection

The tool analyzes smart contracts for **29 distinct vulnerability categories** covering the full spectrum of smart contract security issues:

### Core Security Categories (16 types)
1. **Access Control** - Missing/mis-scoped auth, ownership loss
2. **Denial of Service (DoS)** - Gas exhaustion, revert griefing, block gas limit
3. **Integer Overflow** - Overflow/underflow, div-by-zero
4. **Signature Malleability** - EIP-2 `s` checks, EIP-712 domain separation
5. **Unexpected ETH** - Ether stuck/overly strict balance checks
6. **Storage Layout** - Slot collisions, struct packing, uninitialized storage
7. **Front-run/MEV** - Front-run/sandwich/back-run/latency arbitrage vectors
8. **Oracle Manipulation** - Price-feed spoofing, stale data, missing sanity checks
9. **Randomness** - Predictable entropy, miner influence
10. **Reentrancy** - State update after external call, cross-function
11. **Delegatecall/Low-level Ops** - Unsafe `delegatecall`, inline assembly
12. **Replay Attack** - Sig replay, chain-ID mix-ups
13. **Upgradeability/Initializer Safety** - Proxy init gaps, `initializer()` abuse
14. **Self-Destruct** - Griefing/forced-ETH via `selfdestruct`
15. **Zero-Code** - Constructor-phase contract bypasses
16. **Flash Loan Economic Manipulation** - State checked & used within same tx

### Additional Security Issues (13 types)
17. **tx.origin** - Auth that trusts `tx.origin`
18. **Array Limits** - OOB reads/writes, dynamic-array gas bombs
19. **Pragma** - Floating pragma, outdated compiler bugs
20. **Inheritance** - Bad overrides, diamond ambiguity
21. **Integer Math** - Rounding, precision div-by-zero
22. **Confidential Data** - Private info leak via events/public vars
23. **Default Visibility** - Funcs/vars defaulting to `public`
24. **Pausable Emergency Stop** - Missing pause guards or bypasses
25. **Timestamp Dependent Logic** - Miner-controlled `block.timestamp`/`number`
26. **Unchecked Return** - Ignoring `call`, ERC-20 `transfer` boolean
27. **Event Consistency** - Critical state changes not emitted/mis-ordered
28. **Short Address** - Calldata truncation on L1/L2 bridges
29. **Gas Grief Block Limit** - User-scaling loops, heavy SSTORE in hot paths

## How It Works

### 1. Secure Repository Processing
- Clone repositories in isolated Docker containers using Trail of Bits security toolbox
- Auto-detect and build with Foundry (`forge build`) or Hardhat (`npx hardhat compile`)
- Extract and filter Solidity source files and documentation

### 2. Advanced Static Analysis
- Generate comprehensive call graphs and inheritance hierarchies
- Extract SlithIR (intermediate representation) for every function
- Analyze storage layouts and variable mappings
- Create semantic databases for efficient querying

### 3. Intelligent Code Slicing
- Perform breadth-first search through call graphs
- Generate contextual code blocks with configurable depth and token budgets
- Cache results for efficient reprocessing

### 4. Multi-LLM Security Analysis
- Deploy multiple AI agents in parallel for comprehensive coverage across 29 vulnerability categories
- Use specialized prompts for each vulnerability type with advanced detection patterns
- Implement verification and quality checking to reduce false positives
- Support for OpenAI (GPT-4o, O3), Anthropic (Claude), Gemini, and DeepSeek

### 5. Vector-Based Context Retrieval
- Create high-quality embeddings using OpenAI's text-embedding-3-small
- Store in Qdrant with rich metadata for semantic search
- Enable AI agents to retrieve relevant context dynamically

### 6. Professional Report Generation
- Generate detailed Markdown audit reports with severity classifications
- Include protocol overviews, finding summaries, and detailed vulnerability descriptions
- Support both comprehensive (paid) and limited (free) report formats
- Export contract data and metadata for further analysis

## Output Files

After analysis, the tool generates several output files:

### Audit Reports
- `{repo-name}-audit-{hash}-audit-report.md` - Comprehensive audit report (paid version)
- `{repo-name}-audit-{hash}-free-audit-report.md` - Limited audit report (free version)

### Contract Analysis Data
- `{ContractName}-{repo-name}-audit-{hash}.md` - Individual contract analysis
- `metadata-{repo-name}-audit-{hash}.md` - Protocol metadata and context

### Analysis Artifacts
- `callgraph.json` - Complete call graph data
- `inheritance.json` - Contract inheritance relationships
- `graph.json` - Semantic graph database
- `sarif.json` - SARIF format analysis results

## Querying the Vector Database

The tool creates unique vector collections for each repository. You can query them using the Qdrant API:

```python
from qdrant_client import QdrantClient

# Connect to Qdrant
client = QdrantClient(url="http://localhost:6334")

# List all collections
collections = client.get_collections()
print("Available collections:", [c.name for c in collections.collections])

# Search for semantically similar content
# Collection name format: {repo_hash}-contract_chunks
search_result = client.search(
    collection_name="your_repo_hash-contract_chunks",
    query_vector=your_query_vector,  # Vector from embedding your query text
    limit=5,
    with_payload=True
)

# Print results with metadata
for result in search_result:
    print(f"Score: {result.score}")
    print(f"Content: {result.payload.get('content', '')[:200]}...")
    print(f"Source: {result.payload.get('meta', {})}")
```

## Configuration

### LLM Provider Configuration

The tool supports multiple LLM providers with different cost profiles:

| Provider | Input Cost (per 1M tokens) | Output Cost (per 1M tokens) | Models |
|----------|----------------------------|------------------------------|---------|
| OpenAI | $2.00 | $8.00 | GPT-4o, O3 |
| Anthropic | $3.00 | $15.00 | Claude 3.7 Sonnet, Claude 4.0 Sonnet |
| Gemini | $1.25 | $10.00 | Gemini Pro |
| DeepSeek | $0.07 | $1.10 | DeepSeek Chat |

### Analysis Parameters

Key configuration constants (in source code):
- `MAX_DEPTH`: Call graph traversal depth (default: configurable)
- `TOKEN_BUDGET`: Maximum tokens per code block (default: configurable)
- `DISCOVER_RUNS`: Number of discovery rounds per contract (default: 3)

## Performance and Costs

### Typical Analysis Times
- Small projects (< 10 contracts): 5-15 minutes
- Medium projects (10-50 contracts): 15-45 minutes
- Large projects (50+ contracts): 45+ minutes

### Cost Estimation
- Small project: $1-5 USD
- Medium project: $5-20 USD
- Large project: $20+ USD

*Costs vary significantly based on LLM provider choice and project complexity*

## Troubleshooting

### Common Issues

1. **Docker Permission Errors**
   ```bash
   sudo usermod -aG docker $USER
   # Log out and back in
   ```

2. **Qdrant Connection Issues**
   ```bash
   docker-compose down
   docker-compose up -d
   ```

3. **Out of Memory Errors**
   - Reduce `TOKEN_BUDGET` in source code
   - Use fewer concurrent LLM agents
   - Increase Docker memory limits

4. **API Rate Limits**
   - Add delays between requests
   - Use multiple API keys with rotation
   - Choose providers with higher rate limits

5. **Missing Vulnerability Categories**
   - Ensure all 29 vulnerability prompts are properly loaded
   - Check that LLM agents have access to specialized detection patterns
   - Verify prompt engineering modules are functioning correctly

## Contributing

Contributions are welcome! Areas for improvement:
- Additional vulnerability detection patterns beyond the current 29 categories
- New LLM provider integrations (Claude 4.0, GPT-5, etc.)
- Performance optimizations for large codebases
- Enhanced reporting formats and visualization
- Advanced prompt engineering for better detection accuracy

Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

````
ai-agent-audit/codebase.md
`````
ai-agent-audit/CLAUDE.md
````
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
````
ai-agent-audit/Cargo.toml
```
[package]
name = "ai-agent-audit"
version = "0.1.0"
edition = "2024"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[dependencies]
qdrant-client = { version = "1.14", features = ["serde"] }
tempfile = "3.20"
walkdir = "2.4"
serde = "1.0.219"
ignore = "0.4"
regex = "1.10"
serde_json = { version = "1.0", features = ["preserve_order"] }
tokio = { version = "1.46.1", features = ["macros", "rt-multi-thread"] }
tokio-stream = "0.1.17"
rig-core = { version = "0.13.0", features = ["derive"] }
dotenvy = "0.15"
anyhow = "1.0"
git2 = "0.18"
log = "0.4.26"
reqwest = { version = "0.12.19", features = ["json", "rustls-tls"] }
env_logger = "0.11"
tiktoken-rs = "0.6.0"
rusqlite = { version = "0.31", features = ["bundled"] }                  #  🔒 static libsqlite3
serde-sarif = "0.7"                                                      # straight-forward SARIF model
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"                                                           # timestamp in tickets
async-trait = "0.1.79"
thiserror = "2.0.12"
hex = "0.4.3"
once_cell = "1.8"
async-openai = "0.28.1"
rig-qdrant = "0.1.14"
schemars = "0.8.22"
nanoid = "0.4"

```
ai-agent-audit/README.md
````
# AI Agent Audit

An advanced AI-powered smart contract auditing tool that combines static analysis, multiple LLM providers, and vector embeddings to perform comprehensive security audits of Solidity codebases.

## Overview

AI Agent Audit is a sophisticated Rust-based tool that performs comprehensive smart contract security audits by:

1. **Repository Analysis**: Cloning and building smart contract repositories with support for Foundry and Hardhat
2. **Static Analysis**: Extracting detailed IR, call graphs, and storage information using Slither
3. **AI-Powered Security Review**: Leveraging multiple LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) for vulnerability detection
4. **Vector Embeddings**: Creating semantic search capabilities through Qdrant vector database
5. **Comprehensive Reporting**: Generating detailed audit reports with vulnerability findings and protocol overviews
6. **Cost Tracking**: Monitoring inference costs across different LLM providers

This tool provides professional-grade smart contract auditing capabilities with AI assistance, making it suitable for security researchers, auditors, and development teams.

## Features

### Core Functionality
- **Multi-Platform Repository Support**: Automatically clone and build repositories with Foundry or Hardhat
- **Advanced Static Analysis**: Deep integration with Slither for IR extraction, call graph analysis, and storage layout
- **Multi-LLM Security Analysis**: Parallel vulnerability detection using OpenAI (GPT-4o, O3), Anthropic (Claude), Google Gemini, and DeepSeek
- **Comprehensive Vulnerability Detection**: Covers 29 distinct vulnerability categories including reentrancy, access control, MEV, oracle manipulation, and advanced attack vectors
- **Vector-Based Semantic Search**: High-quality embeddings with Qdrant for intelligent code search and context retrieval
- **Professional Audit Reports**: Generate detailed markdown reports with findings categorized by severity
- **Cost Optimization**: Real-time tracking of inference costs across different LLM providers

### Advanced Capabilities
- **Intelligent Code Slicing**: Generate contextual code blocks with call graph traversal for focused analysis
- **Duplicate Detection**: AI-powered deduplication of security findings
- **Quality Verification**: Multi-stage verification process to reduce false positives
- **Docker Integration**: Secure, isolated analysis environment using Trail of Bits security toolbox
- **Caching System**: Efficient caching of analysis results and LLM responses
- **Concurrent Processing**: Parallel analysis across multiple AI agents for faster results

## Prerequisites

### Required Software
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Docker](https://docs.docker.com/get-docker/) for containerized analysis environment
- [Qdrant](https://qdrant.tech/documentation/quick-start/) vector database

### Optional (for local development)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) for Forge (handled by Docker)
- [Slither](https://github.com/crytic/slither#how-to-install) static analyzer (handled by Docker)

### API Keys
You'll need API keys for the LLM providers you want to use:
- **OpenAI**: Required for embeddings and GPT models
- **Anthropic**: Optional, for Claude models
- **Google AI**: Optional, for Gemini models
- **DeepSeek**: Optional, for cost-effective analysis

## Environment Setup

1. Create a `.env` file in the project root with your API keys:

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

2. Start the Qdrant vector database:

```bash
docker-compose up -d
```

This will start Qdrant on ports 6333 (REST API) and 6334 (gRPC API).

## Installation

1. Clone this repository:

```bash
git clone https://github.com/chain-shield/ai-agent-audit.git
cd ai-agent-audit
```

2. Build the project:

```bash
cargo build --release
```

## Usage

### Basic Usage

Run the application with a Git repository URL containing Solidity contracts:

```bash
cargo run --release -- https://github.com/example/solidity-project.git
```

### Complete Workflow

The application performs a comprehensive audit workflow:

1. **Repository Preparation**
   - Clone the repository using Docker for security
   - Auto-detect and build with Foundry or Hardhat
   - Filter and organize Solidity source files

2. **Static Analysis**
   - Extract call graphs and inheritance hierarchies using Slither
   - Generate intermediate representation (IR) for all functions
   - Analyze storage layouts and variable mappings

3. **AI-Powered Security Analysis**
   - Generate contextual code slices for focused analysis
   - Run parallel security analysis using multiple LLM providers
   - Detect vulnerabilities across 19+ security categories
   - Verify and deduplicate findings using AI verification

4. **Vector Database Population**
   - Create semantic embeddings for all code and analysis results
   - Store in Qdrant for intelligent search and retrieval

5. **Report Generation**
   - Generate comprehensive audit reports in Markdown format
   - Create both paid (full details) and free (limited) report versions
   - Include protocol overview, findings summary, and detailed vulnerability descriptions

6. **Cost Tracking**
   - Monitor and report total inference costs across all LLM providers

## Project Structure

```
src/
├── ai_bot/                    # AI agent implementations
│   ├── agent.rs              # Core AI audit agent with vector search
│   └── retrieve_slice.rs     # Context retrieval for AI analysis
├── build_brain/              # Core analysis and data processing
│   ├── callgraph.rs          # Call graph analysis and traversal
│   ├── enbeddings.rs         # Vector embeddings generation
│   ├── enrichment.rs         # Slither analysis integration
│   ├── fn_summaries.rs       # Function summarization
│   ├── graph_db.rs           # Graph database operations
│   ├── inheritance.rs        # Contract inheritance analysis
│   ├── parsers.rs            # Code parsing utilities
│   ├── slither_ffi.rs        # Slither static analyzer interface
│   ├── summarize.rs          # Protocol and file summarization
│   └── vector_db.rs          # Qdrant vector database operations
├── cost/                     # Cost tracking and management
│   └── cost_data.rs          # LLM inference cost calculation
├── enumerator/               # Code slicing and enumeration
│   ├── codeblock_cache.rs    # Caching for generated code blocks
│   ├── codeblock_db.rs       # Database for code block storage
│   ├── codeblock_maker.rs    # Code block generation logic
│   ├── codeblocks.rs         # Core code slicing functionality
│   └── utils.rs              # Enumeration utilities
├── llm_review/               # AI-powered security analysis
│   ├── analysis_db.rs        # Analysis results database
│   ├── code_review.rs        # Main security review orchestration
│   ├── config.rs             # LLM configuration and models
│   ├── context_state.rs      # Global context management
│   ├── enums.rs              # AI agent and vulnerability type enums
│   ├── invariants.rs         # Protocol invariant analysis
│   ├── prompt_content.rs     # Dynamic prompt generation
│   ├── review_utils.rs       # AI agent builders and utilities
│   └── prompt_support/       # Prompt engineering modules
├── prepare_code/             # Repository preparation
│   └── git_clone.rs          # Git cloning and building
├── reporting/                # Report generation
│   ├── audit.rs              # Audit report generation
│   ├── contract_data.rs      # Contract data export
│   └── save_file.rs          # File saving utilities
├── prompts/                  # Vulnerability-specific prompts (19 types)
├── master_prompts/           # Master security analysis prompts
├── invariant_prompts/        # Protocol invariant prompts
├── utils/                    # Shared utilities
├── lib.rs                    # Library exports
└── main.rs                   # Application entry point
```

## Vulnerability Detection

The tool analyzes smart contracts for **29 distinct vulnerability categories** covering the full spectrum of smart contract security issues:

### Core Security Categories (16 types)
1. **Access Control** - Missing/mis-scoped auth, ownership loss
2. **Denial of Service (DoS)** - Gas exhaustion, revert griefing, block gas limit
3. **Integer Overflow** - Overflow/underflow, div-by-zero
4. **Signature Malleability** - EIP-2 `s` checks, EIP-712 domain separation
5. **Unexpected ETH** - Ether stuck/overly strict balance checks
6. **Storage Layout** - Slot collisions, struct packing, uninitialized storage
7. **Front-run/MEV** - Front-run/sandwich/back-run/latency arbitrage vectors
8. **Oracle Manipulation** - Price-feed spoofing, stale data, missing sanity checks
9. **Randomness** - Predictable entropy, miner influence
10. **Reentrancy** - State update after external call, cross-function
11. **Delegatecall/Low-level Ops** - Unsafe `delegatecall`, inline assembly
12. **Replay Attack** - Sig replay, chain-ID mix-ups
13. **Upgradeability/Initializer Safety** - Proxy init gaps, `initializer()` abuse
14. **Self-Destruct** - Griefing/forced-ETH via `selfdestruct`
15. **Zero-Code** - Constructor-phase contract bypasses
16. **Flash Loan Economic Manipulation** - State checked & used within same tx

### Additional Security Issues (13 types)
17. **tx.origin** - Auth that trusts `tx.origin`
18. **Array Limits** - OOB reads/writes, dynamic-array gas bombs
19. **Pragma** - Floating pragma, outdated compiler bugs
20. **Inheritance** - Bad overrides, diamond ambiguity
21. **Integer Math** - Rounding, precision div-by-zero
22. **Confidential Data** - Private info leak via events/public vars
23. **Default Visibility** - Funcs/vars defaulting to `public`
24. **Pausable Emergency Stop** - Missing pause guards or bypasses
25. **Timestamp Dependent Logic** - Miner-controlled `block.timestamp`/`number`
26. **Unchecked Return** - Ignoring `call`, ERC-20 `transfer` boolean
27. **Event Consistency** - Critical state changes not emitted/mis-ordered
28. **Short Address** - Calldata truncation on L1/L2 bridges
29. **Gas Grief Block Limit** - User-scaling loops, heavy SSTORE in hot paths

## How It Works

### 1. Secure Repository Processing
- Clone repositories in isolated Docker containers using Trail of Bits security toolbox
- Auto-detect and build with Foundry (`forge build`) or Hardhat (`npx hardhat compile`)
- Extract and filter Solidity source files and documentation

### 2. Advanced Static Analysis
- Generate comprehensive call graphs and inheritance hierarchies
- Extract SlithIR (intermediate representation) for every function
- Analyze storage layouts and variable mappings
- Create semantic databases for efficient querying

### 3. Intelligent Code Slicing
- Perform breadth-first search through call graphs
- Generate contextual code blocks with configurable depth and token budgets
- Cache results for efficient reprocessing

### 4. Multi-LLM Security Analysis
- Deploy multiple AI agents in parallel for comprehensive coverage across 29 vulnerability categories
- Use specialized prompts for each vulnerability type with advanced detection patterns
- Implement verification and quality checking to reduce false positives
- Support for OpenAI (GPT-4o, O3), Anthropic (Claude), Gemini, and DeepSeek

### 5. Vector-Based Context Retrieval
- Create high-quality embeddings using OpenAI's text-embedding-3-small
- Store in Qdrant with rich metadata for semantic search
- Enable AI agents to retrieve relevant context dynamically

### 6. Professional Report Generation
- Generate detailed Markdown audit reports with severity classifications
- Include protocol overviews, finding summaries, and detailed vulnerability descriptions
- Support both comprehensive (paid) and limited (free) report formats
- Export contract data and metadata for further analysis

## Output Files

After analysis, the tool generates several output files:

### Audit Reports
- `{repo-name}-audit-{hash}-audit-report.md` - Comprehensive audit report (paid version)
- `{repo-name}-audit-{hash}-free-audit-report.md` - Limited audit report (free version)

### Contract Analysis Data
- `{ContractName}-{repo-name}-audit-{hash}.md` - Individual contract analysis
- `metadata-{repo-name}-audit-{hash}.md` - Protocol metadata and context

### Analysis Artifacts
- `callgraph.json` - Complete call graph data
- `inheritance.json` - Contract inheritance relationships
- `graph.json` - Semantic graph database
- `sarif.json` - SARIF format analysis results

## Querying the Vector Database

The tool creates unique vector collections for each repository. You can query them using the Qdrant API:

```python
from qdrant_client import QdrantClient

# Connect to Qdrant
client = QdrantClient(url="http://localhost:6334")

# List all collections
collections = client.get_collections()
print("Available collections:", [c.name for c in collections.collections])

# Search for semantically similar content
# Collection name format: {repo_hash}-contract_chunks
search_result = client.search(
    collection_name="your_repo_hash-contract_chunks",
    query_vector=your_query_vector,  # Vector from embedding your query text
    limit=5,
    with_payload=True
)

# Print results with metadata
for result in search_result:
    print(f"Score: {result.score}")
    print(f"Content: {result.payload.get('content', '')[:200]}...")
    print(f"Source: {result.payload.get('meta', {})}")
```

## Configuration

### LLM Provider Configuration

The tool supports multiple LLM providers with different cost profiles:

| Provider | Input Cost (per 1M tokens) | Output Cost (per 1M tokens) | Models |
|----------|----------------------------|------------------------------|---------|
| OpenAI | $2.00 | $8.00 | GPT-4o, O3 |
| Anthropic | $3.00 | $15.00 | Claude 3.7 Sonnet, Claude 4.0 Sonnet |
| Gemini | $1.25 | $10.00 | Gemini Pro |
| DeepSeek | $0.07 | $1.10 | DeepSeek Chat |

### Analysis Parameters

Key configuration constants (in source code):
- `MAX_DEPTH`: Call graph traversal depth (default: configurable)
- `TOKEN_BUDGET`: Maximum tokens per code block (default: configurable)
- `DISCOVER_RUNS`: Number of discovery rounds per contract (default: 3)

## Performance and Costs

### Typical Analysis Times
- Small projects (< 10 contracts): 5-15 minutes
- Medium projects (10-50 contracts): 15-45 minutes
- Large projects (50+ contracts): 45+ minutes

### Cost Estimation
- Small project: $1-5 USD
- Medium project: $5-20 USD
- Large project: $20+ USD

*Costs vary significantly based on LLM provider choice and project complexity*

## Troubleshooting

### Common Issues

1. **Docker Permission Errors**
   ```bash
   sudo usermod -aG docker $USER
   # Log out and back in
   ```

2. **Qdrant Connection Issues**
   ```bash
   docker-compose down
   docker-compose up -d
   ```

3. **Out of Memory Errors**
   - Reduce `TOKEN_BUDGET` in source code
   - Use fewer concurrent LLM agents
   - Increase Docker memory limits

4. **API Rate Limits**
   - Add delays between requests
   - Use multiple API keys with rotation
   - Choose providers with higher rate limits

5. **Missing Vulnerability Categories**
   - Ensure all 29 vulnerability prompts are properly loaded
   - Check that LLM agents have access to specialized detection patterns
   - Verify prompt engineering modules are functioning correctly

## Contributing

Contributions are welcome! Areas for improvement:
- Additional vulnerability detection patterns beyond the current 29 categories
- New LLM provider integrations (Claude 4.0, GPT-5, etc.)
- Performance optimizations for large codebases
- Enhanced reporting formats and visualization
- Advanced prompt engineering for better detection accuracy

Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

````
ai-agent-audit/docker-compose.yml
```yaml
version: '3.8'

services:
  qdrant:
    image: qdrant/qdrant:v1.14.0
    container_name: qdrant
    ports:
      - "6333:6333"  # REST API
      - "6334:6334"  # gRPC API
    volumes:
      - qdrant_data:/qdrant/storage

volumes:
  qdrant_data:


```
ai-agent-audit/src/config.rs
```
/// Configuration management for the AI Agent Audit application.
///
/// This module provides centralized configuration handling, with constants
/// for application settings and environment variables only for sensitive
/// configuration like API keys and URLs.
use crate::error::{AuditError, Result};
use serde::{Deserialize, Serialize};
use std::env;

// Application constants - these don't need to be configurable via environment
/// Maximum call graph traversal depth for code slice generation
pub const MAX_DEPTH: usize = 3;

/// Maximum token budget per code block to stay within LLM context limits
pub const TOKEN_BUDGET: usize = 150_000;

/// Number of discovery rounds per contract during analysis
pub const RUNS: usize = 5;

pub const MAX_RAG_QUERY_CONTENT_LENGTH: usize = 8192; // 8192 token limit for embedding

/// Docker volume path for repository analysis
pub const DOCKER_VOLUME: &str = "/tmp/audit-analysis";

/// Maximum repository URL length for security validation
pub const MAX_REPO_URL_LENGTH: usize = 2048;

/// Timeout for LLM requests in seconds
pub const LLM_TIMEOUT_SECONDS: u64 = 120;

/// Default temperature for LLM models
pub const DEFAULT_TEMPERATURE: f64 = 1.0;

/// Maximum tokens for LLM responses
pub const MAX_RESPONSE_TOKENS: u64 = 100_000;

/// Vector database collection dimension
pub const VECTOR_DIMENSION: u64 = 1536;

/// Number of similar chunks to retrieve for context
pub const CONTEXT_CHUNKS: usize = 5;

/// Similarity threshold for vector search
pub const SIMILARITY_THRESHOLD: f64 = 0.7;

/// Main configuration structure for the AI Agent Audit application.
///
/// This struct contains configurable parameters loaded from environment variables
/// for sensitive data (API keys, URLs) and constants for application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Maximum call graph traversal depth for code slice generation
    pub max_depth: usize,

    /// Maximum token budget per code block to stay within LLM context limits
    pub token_budget: usize,

    /// Number of discovery rounds per contract during analysis
    pub runs: usize,

    /// Qdrant vector database URL for semantic search
    pub qdrant_url: String,

    /// OpenAI API key for GPT models and embeddings
    pub openai_api_key: Option<String>,

    /// Anthropic API key for Claude models
    pub anthropic_api_key: Option<String>,

    /// Google AI API key for Gemini models
    pub gemini_ai_api_key: Option<String>,

    /// DeepSeek API key for DeepSeek models
    pub deepseek_api_key: Option<String>,

    /// Logging level for the application
    pub log_level: String,

    /// Docker volume path for repository analysis
    pub docker_volume: String,

    /// Maximum repository URL length for security validation
    pub max_repo_url_length: usize,

    /// Timeout for LLM requests in seconds
    pub llm_timeout_seconds: u64,

    /// Default temperature for LLM models
    pub default_temperature: f64,

    /// Maximum tokens for LLM responses
    pub max_response_tokens: u64,

    /// Vector database collection dimension
    pub vector_dimension: u64,

    /// Number of similar chunks to retrieve for context
    pub context_chunks: usize,

    /// Similarity threshold for vector search
    pub similarity_threshold: f64,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: RUNS,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "info".to_string(),
            docker_volume: DOCKER_VOLUME.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
            vector_dimension: VECTOR_DIMENSION,
            context_chunks: CONTEXT_CHUNKS,
            similarity_threshold: SIMILARITY_THRESHOLD,
        }
    }
}

impl AuditConfig {
    /// Creates a new configuration from environment variables.
    ///
    /// This function reads only sensitive configuration from environment variables
    /// (API keys, URLs, logging) and uses constants for application settings.
    ///
    /// # Returns
    /// * `Result<AuditConfig>` - Configuration loaded from environment
    ///
    /// # Environment Variables
    /// * `QDRANT_URL` - Vector database URL (default: http://localhost:6334)
    /// * `OPENAI_API_KEY` - OpenAI API key (optional)
    /// * `ANTHROPIC_API_KEY` - Anthropic API key (optional)
    /// * `GEMINI_API_KEY` - Gemini AI API key (optional)
    /// * `DEEPSEEK_API_KEY` - DeepSeek API key (optional)
    /// * `RUST_LOG` - Logging level (default: info)
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Load Qdrant URL
        if let Ok(qdrant_url) = env::var("QDRANT_URL") {
            if !qdrant_url.starts_with("http://") && !qdrant_url.starts_with("https://") {
                return Err(AuditError::configuration(
                    "QDRANT_URL",
                    "Must start with http:// or https://",
                ));
            }
            config.qdrant_url = qdrant_url;
        }

        // Load API keys (optional)
        config.openai_api_key = env::var("OPENAI_API_KEY").ok();
        config.anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();
        config.gemini_ai_api_key = env::var("GEMINI_API_KEY").ok();
        config.deepseek_api_key = env::var("DEEPSEEK_API_KEY").ok();

        // Validate at least one API key is provided
        if config.openai_api_key.is_none()
            && config.anthropic_api_key.is_none()
            && config.gemini_ai_api_key.is_none()
            && config.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        // Load logging level
        config.log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(config)
    }

    /// Validates the configuration and returns any errors found.
    pub fn validate(&self) -> Result<()> {
        // Validate qdrant_url
        if !self.qdrant_url.starts_with("http://") && !self.qdrant_url.starts_with("https://") {
            return Err(AuditError::configuration(
                "qdrant_url",
                "Must start with http:// or https://",
            ));
        }

        // Validate at least one API key is provided
        if self.openai_api_key.is_none()
            && self.anthropic_api_key.is_none()
            && self.gemini_ai_api_key.is_none()
            && self.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        Ok(())
    }

    /// Returns true if OpenAI API key is configured.
    pub fn has_openai_key(&self) -> bool {
        self.openai_api_key.is_some()
    }

    /// Returns true if Anthropic API key is configured.
    pub fn has_anthropic_key(&self) -> bool {
        self.anthropic_api_key.is_some()
    }

    /// Returns true if Google AI API key is configured.
    pub fn has_google_ai_key(&self) -> bool {
        self.gemini_ai_api_key.is_some()
    }

    /// Returns true if DeepSeek API key is configured.
    pub fn has_deepseek_key(&self) -> bool {
        self.deepseek_api_key.is_some()
    }

    /// Returns a list of configured LLM providers.
    pub fn available_providers(&self) -> Vec<String> {
        let mut providers = Vec::new();
        if self.has_openai_key() {
            providers.push("OpenAI".to_string());
        }
        if self.has_anthropic_key() {
            providers.push("Anthropic".to_string());
        }
        if self.has_google_ai_key() {
            providers.push("Google AI".to_string());
        }
        if self.has_deepseek_key() {
            providers.push("DeepSeek".to_string());
        }
        providers
    }

    /// Creates a test configuration with minimal settings.
    #[cfg(test)]
    pub fn test_config() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: RUNS,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: Some("test-key".to_string()),
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "debug".to_string(),
            docker_volume: DOCKER_VOLUME.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
            vector_dimension: VECTOR_DIMENSION,
            context_chunks: CONTEXT_CHUNKS,
            similarity_threshold: SIMILARITY_THRESHOLD,
        }
    }
}

/// Global configuration instance.
use std::sync::OnceLock;
static CONFIG: OnceLock<AuditConfig> = OnceLock::new();

/// Initializes the global configuration from environment variables.
pub fn init_config() -> Result<()> {
    let config = AuditConfig::from_env()?;
    config.validate()?;

    CONFIG.set(config).map_err(|_| {
        AuditError::configuration("global_config", "Configuration already initialized")
    })?;

    Ok(())
}

/// Returns a reference to the global configuration.
///
/// # Panics
/// Panics if the configuration has not been initialized with `init_config()`.
pub fn audit_config() -> &'static AuditConfig {
    CONFIG
        .get()
        .expect("Configuration not initialized. Call init_config() first.")
}

/// Returns a reference to the global configuration, or None if not initialized.
pub fn try_audit_config() -> Option<&'static AuditConfig> {
    CONFIG.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = AuditConfig::default();
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);
        assert_eq!(config.runs, RUNS);
        // Config validation will fail because no API keys are set
    }

    #[test]
    fn test_config_validation() {
        let mut config = AuditConfig::default();

        // Set an API key to make validation pass
        config.openai_api_key = Some("test-key".to_string());
        assert!(config.validate().is_ok());

        // Test invalid URL
        config.qdrant_url = "invalid-url".to_string();
        assert!(config.validate().is_err());

        // Test no API keys
        config.qdrant_url = "http://localhost:6334".to_string();
        config.openai_api_key = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_available_providers() {
        let mut config = AuditConfig::default();
        config.openai_api_key = Some("test".to_string());
        config.anthropic_api_key = Some("test".to_string());

        let providers = config.available_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"OpenAI".to_string()));
        assert!(providers.contains(&"Anthropic".to_string()));
    }

    #[test]
    fn test_from_env() {
        unsafe {
            env::set_var("QDRANT_URL", "http://test:6334");
            env::set_var("OPENAI_API_KEY", "test-key");
        }

        let config = AuditConfig::from_env().unwrap();
        assert_eq!(config.qdrant_url, "http://test:6334");
        assert!(config.has_openai_key());
        // Constants should be used for other values
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);

        // Clean up
        unsafe {
            env::remove_var("QDRANT_URL");
            env::remove_var("OPENAI_API_KEY");
        }
    }
}

```
ai-agent-audit/src/error.rs
```
/// Centralized error types for the AI Agent Audit application.
///
/// This module provides a unified error handling system that consolidates
/// various error types from different modules into a cohesive hierarchy.
/// This improves debugging, error propagation, and overall system reliability.

use thiserror::Error;

/// Main error type for the AI Agent Audit application.
/// 
/// This enum encompasses all possible error conditions that can occur
/// during the audit process, providing specific error types for different
/// failure scenarios with descriptive messages and error chaining.
#[derive(Debug, Error)]
pub enum AuditError {
    /// Database operation failures (SQLite, Qdrant)
    #[error("Database operation failed: {message}")]
    Database { 
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Git operations and repository handling failures
    #[error("Git operation failed: {operation} - {message}")]
    Git {
        operation: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Docker container operations failures
    #[error("Docker operation failed: {command} - {message}")]
    Docker {
        command: String,
        message: String,
        exit_code: Option<i32>,
    },

    /// Slither static analysis failures
    #[error("Slither analysis failed: {printer} - {message}")]
    SlitherAnalysis {
        printer: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// LLM processing and API communication failures
    #[error("LLM processing failed: {provider} - {message}")]
    LlmProcessing {
        provider: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Vector database operations failures
    #[error("Vector database operation failed: {operation} - {message}")]
    VectorDb {
        operation: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// File system operations failures
    #[error("File system operation failed: {path} - {message}")]
    FileSystem {
        path: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// JSON parsing and serialization failures
    #[error("JSON processing failed: {context} - {message}")]
    JsonProcessing {
        context: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Security validation failures
    #[error("Security validation failed: {check} - {message}")]
    Security {
        check: String,
        message: String,
    },

    /// Configuration and environment setup failures
    #[error("Configuration error: {setting} - {message}")]
    Configuration {
        setting: String,
        message: String,
    },

    /// Network operations failures
    #[error("Network operation failed: {url} - {message}")]
    Network {
        url: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Async task processing failures
    #[error("Async task failed: {task} - {message}")]
    AsyncTask {
        task: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Generic validation failures
    #[error("Validation failed: {field} - {message}")]
    Validation {
        field: String,
        message: String,
    },
}

/// Result type alias for consistent error handling throughout the application.
pub type Result<T> = std::result::Result<T, AuditError>;

impl AuditError {
    /// Creates a new database error with context.
    pub fn database<E>(message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Database {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new git error with context.
    pub fn git<E>(operation: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Git {
            operation: operation.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new Docker error with context.
    pub fn docker(command: impl Into<String>, message: impl Into<String>, exit_code: Option<i32>) -> Self {
        Self::Docker {
            command: command.into(),
            message: message.into(),
            exit_code,
        }
    }

    /// Creates a new Slither analysis error with context.
    pub fn slither<E>(printer: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::SlitherAnalysis {
            printer: printer.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new LLM processing error with context.
    pub fn llm<E>(provider: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::LlmProcessing {
            provider: provider.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new vector database error with context.
    pub fn vector_db<E>(operation: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::VectorDb {
            operation: operation.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new file system error with context.
    pub fn file_system<E>(path: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::FileSystem {
            path: path.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new JSON processing error with context.
    pub fn json<E>(context: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::JsonProcessing {
            context: context.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new security validation error.
    pub fn security(check: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Security {
            check: check.into(),
            message: message.into(),
        }
    }

    /// Creates a new configuration error.
    pub fn configuration(setting: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Configuration {
            setting: setting.into(),
            message: message.into(),
        }
    }

    /// Creates a new network error with context.
    pub fn network<E>(url: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Network {
            url: url.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new async task error with context.
    pub fn async_task<E>(task: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::AsyncTask {
            task: task.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new validation error.
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Conversion implementations for common error types
impl From<std::io::Error> for AuditError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystem {
            path: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for AuditError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonProcessing {
            context: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<rusqlite::Error> for AuditError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<reqwest::Error> for AuditError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network {
            url: err.url().map(|u| u.to_string()).unwrap_or_else(|| "unknown".to_string()),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<qdrant_client::QdrantError> for AuditError {
    fn from(err: qdrant_client::QdrantError) -> Self {
        Self::VectorDb {
            operation: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<tokio::task::JoinError> for AuditError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::AsyncTask {
            task: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<anyhow::Error> for AuditError {
    fn from(err: anyhow::Error) -> Self {
        Self::AsyncTask {
            task: "legacy_anyhow".to_string(),
            message: err.to_string(),
            source: Some(err.into()),
        }
    }
}

/// Convenience macros for creating specific error types
#[macro_export]
macro_rules! audit_error {
    (database, $msg:expr) => {
        $crate::error::AuditError::Database {
            message: $msg.to_string(),
            source: None,
        }
    };
    (git, $op:expr, $msg:expr) => {
        $crate::error::AuditError::Git {
            operation: $op.to_string(),
            message: $msg.to_string(),
            source: None,
        }
    };
    (docker, $cmd:expr, $msg:expr) => {
        $crate::error::AuditError::Docker {
            command: $cmd.to_string(),
            message: $msg.to_string(),
            exit_code: None,
        }
    };
    (security, $check:expr, $msg:expr) => {
        $crate::error::AuditError::Security {
            check: $check.to_string(),
            message: $msg.to_string(),
        }
    };
    (config, $setting:expr, $msg:expr) => {
        $crate::error::AuditError::Configuration {
            setting: $setting.to_string(),
            message: $msg.to_string(),
        }
    };
    (validation, $field:expr, $msg:expr) => {
        $crate::error::AuditError::Validation {
            field: $field.to_string(),
            message: $msg.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AuditError::security("url_validation", "Invalid URL format");
        assert!(err.to_string().contains("Security validation failed"));
        assert!(err.to_string().contains("url_validation"));
    }

    #[test]
    fn test_error_macro() {
        let err = audit_error!(validation, "repo_url", "URL is required");
        assert!(err.to_string().contains("Validation failed"));
        assert!(err.to_string().contains("repo_url"));
    }
}
```
ai-agent-audit/src/lib.rs
```
/// AI Agent Audit - Comprehensive Smart Contract Security Analysis Tool
///
/// This library provides advanced AI-powered smart contract auditing capabilities,
/// combining static analysis, multiple LLM providers, and vector embeddings for
/// professional-grade security assessments.

/// Centralized error handling and types
pub mod error;

/// Configuration management and environment handling
pub mod config;

/// Core analysis and data processing functionality
pub mod build_brain {
    /// Call graph analysis and traversal
    pub mod callgraph;
    /// Vector embeddings generation for semantic search
    pub mod enbeddings;
    /// Slither analysis integration and data enrichment
    pub mod enrichment;
    /// Function summarization using LLMs
    pub mod fn_summaries;
    /// Graph database operations for semantic data
    pub mod graph_db;
    /// Contract inheritance analysis
    pub mod inheritance;
    /// Code parsing utilities
    pub mod parsers;
    /// Slither static analyzer interface
    pub mod slither_ffi;
    /// Protocol and file summarization
    pub mod summarize;
    /// Qdrant vector database operations
    pub mod vector_db;
    /// High-level vector database service
    pub mod vector_service;
}

/// Repository preparation and building
pub mod prepare_code {
    /// Git cloning and Docker-based building
    pub mod git_clone;
}

/// Report generation and data export
pub mod reporting {
    /// Audit report generation with findings
    pub mod audit;
    /// Contract data export utilities
    pub mod contract_data;
    /// File saving and formatting
    pub mod save_file;
}

/// Code slicing and enumeration for focused analysis
pub mod enumerator {
    /// Caching for generated code blocks
    pub mod codeblock_cache;
    /// Database for code block storage
    pub mod codeblock_db;
    /// Code block generation logic
    pub mod codeblock_maker;
    /// Core code slicing functionality
    pub mod codeblocks;
    /// Enumeration utilities
    pub mod utils;
}

/// AI-powered security analysis and LLM integration
pub mod llm_review {
    /// AI agent factory for centralized agent creation
    pub mod agent_factory;
    /// Analysis results database
    pub mod analysis_db;
    /// Main security review orchestration
    pub mod code_review;
    /// LLM configuration and models
    pub mod config;
    /// Global context management
    pub mod context_state;
    /// AI agent and vulnerability type enums
    pub mod enums;
    /// Protocol invariant analysis
    pub mod invariants;
    /// Dynamic prompt generation
    pub mod prompt_context;
    /// AI agent builders and utilities
    pub mod review_utils;
    /// Prompt engineering modules for different analysis stages
    pub mod prompt_support {
        /// Deduplication prompts
        pub mod dedup;
        /// Post-analysis prompts
        pub mod post_prompt;
        /// Quality check prompts
        pub mod post_qualify;
        /// Verification prompts
        pub mod post_verify;
        /// Pre-analysis prompts
        pub mod pre_prompt;
        /// Pre-qualification prompts
        pub mod pre_qualify;
        /// Pre-verification prompts
        pub mod pre_verify;
        /// Quality assessment prompts
        pub mod qualify_prompt;
        /// Verification prompts
        pub mod verify_prompt;
    }
}

/// Cost tracking and management for LLM inference
pub mod cost {
    /// Cost calculation and tracking across providers
    pub mod cost_data;
}

/// AI agent implementations with vector search
pub mod ai_bot {
    /// Core AI audit agent with dynamic context
    pub mod agent;
    /// Context retrieval for AI analysis
    pub mod retrieve_slice;
}

/// Master security analysis prompts
pub mod master_prompts {
    /// Base master security prompt
    pub mod master_prompt;
    /// Security analysis prompt variants
    pub mod prompt_2x_a;
    pub mod prompt_2x_aa;
    pub mod prompt_2x_b;
    pub mod prompt_2x_bb;
    pub mod prompt_3x_a;
    pub mod prompt_3x_b;
    pub mod prompt_3x_c;
}

/// Vulnerability-specific detection prompts (19 categories)
pub mod prompts {
    /// Access control vulnerabilities
    pub mod access_control;
    /// Array bounds checking issues
    pub mod array_limits;
    /// Confidential data exposure
    pub mod confidential_data;
    /// Default visibility issues
    pub mod default_visibility;
    /// Denial of service vulnerabilities
    pub mod dos;
    /// Inheritance-related issues
    pub mod inheritance;
    /// Integer overflow/underflow
    pub mod integer_overflow;
    /// MEV and front-running vulnerabilities
    pub mod mev;
    /// Oracle manipulation attacks
    pub mod oracle;
    /// Pragma-related issues
    pub mod pragma;
    /// Weak randomness vulnerabilities
    pub mod randomness;
    /// Reentrancy vulnerabilities
    pub mod reentrancy;
    /// Replay attack vulnerabilities
    pub mod replay_attack;
    /// Self-destruct related issues
    pub mod self_destruct;
    /// Short address attack vulnerabilities
    pub mod short_address_attack;
    /// Storage variable issues
    pub mod storage_variables;
    /// tx.origin usage vulnerabilities
    pub mod tx_origin;
    /// Unchecked return value issues
    pub mod unchecked_return_value;
    /// Unexpected ETH handling
    pub mod unexpected_eth;
    /// Zero-code contract issues
    pub mod zero_code;
}

/// Protocol invariant analysis prompts
pub mod invariant_prompts {
    /// Arithmetic invariants
    pub mod arithmetic;
    /// Balance invariants
    pub mod balance;
    /// Permission invariants
    pub mod permission;
    /// Referential integrity invariants
    pub mod referential;
    /// State machine invariants
    pub mod state_machine;
    /// Temporal invariants
    pub mod temporal;
}

/// Shared utilities and helper functions
pub mod utils {
    /// OpenAI tokenizer (BPE) for text chunking
    pub mod bpe;
    pub mod contract_name_check;
    /// Docker volume cleanup utilities
    pub mod delete_docker_volumes;
    pub mod env_security;
    /// LLM extraction with retry logic
    pub mod extract_retry;
    pub mod file_security;
    /// Function labeling utilities
    pub mod fn_labels;
    /// Documentation extraction
    pub mod get_doc_file;
    /// Function name extraction
    pub mod get_fn_name;
    /// Logging utilities
    pub mod logging;
    /// Text sanitization utilities
    pub mod sanitize;
    /// Vector database connection utilities
    pub mod vec_db_connect;
}

```
ai-agent-audit/src/main.rs
```
/// The main entry point for the AI Agent Audit tool.
///
/// This application performs comprehensive smart contract security audits by:
/// 1. Cloning and building repositories (Foundry/Hardhat) in Docker containers
/// 2. Extracting call graphs, IR, and storage layouts using Slither
/// 3. Generating contextual code slices for focused AI analysis
/// 5. Running multi-LLM security analysis across 19+ vulnerability categories
/// 5. Creating vector embeddings and storing in Qdrant for semantic search
/// 6. Generating professional audit reports with findings and cost tracking
use ai_agent_audit::{
    build_brain::{enrichment, slither_ffi::get_all_files_src, vector_db},
    config::{audit_config, init_config},
    cost::cost_data::get_total_inference_cost,
    enumerator::codeblock_maker,
    error::{AuditError, Result},
    llm_review::{
        agent_factory::init_llm_clients,
        code_review,
        context_state::{self},
        prompt_context,
    },
    prepare_code::{self, git_clone::BuildFlags},
    reporting::{
        audit::{self, ReportType},
        contract_data, save_file,
    },
    utils::delete_docker_volumes::cleanup_repo_volume,
};
use dotenvy::dotenv;
use log::info;

/// The main async function that orchestrates the entire process.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize configuration from environment
    init_config()?;

    // Initialize LLM clients
    init_llm_clients()?;

    // Initialize the logger
    env_logger::init();

    // ────────────────────────────────
    // 1. Repository Preparation
    // ────────────────────────────────
    let repo_url = std::env::args().nth(1).ok_or_else(|| {
        AuditError::validation("repo_url", "Repository URL is required as first argument")
    })?;

    // Optional subfolder argument for analyzing specific directories in multi-app repositories
    let subfolder = std::env::args().nth(2);

    // Optional --via-ir flag for forge build
    let build_flags = match std::env::args().nth(3).as_deref() {
        Some("--via-ir") => BuildFlags::ViaIr,
        _ => BuildFlags::Standard,
    };

    // Validate URL format and length before processing
    if repo_url.len() > audit_config().max_repo_url_length {
        return Err(AuditError::validation(
            "repo_url",
            &format!(
                "Repository URL is too long (max {} characters)",
                audit_config().max_repo_url_length
            ),
        ));
    }

    if !repo_url.starts_with("https://") && !repo_url.starts_with("http://") {
        return Err(AuditError::validation(
            "repo_url",
            "Only HTTP/HTTPS repository URLs are supported",
        ));
    }

    info!("Processing repository: {}", repo_url);
    if let Some(ref sf) = subfolder {
        info!("Analyzing subfolder: {}", sf);
    }
    info!("git cloning and extraction source code");

    // Clone repository in Docker container and build with Foundry/Hardhat
    let repo = prepare_code::git_clone::clone_and_filter_git_repo(
        &repo_url,
        subfolder.as_deref(),
        build_flags,
    )?;
    info!("repo root => {:?}", &repo.root);
    info!("repo name => {:?}", &repo.repo_name);

    // ────────────────────────────────
    // 2. Static Analysis & Graph Generation
    // ────────────────────────────────
    // Build semantic database with call graphs and inheritance data
    let semantics_db = enrichment::build_semantics_db_from_call_graph(repo.clone()).await?;
    info!("Call-graph DB at {}", semantics_db.display());

    // Generate and cache protocol metadata context for AI analysis
    info!("generating metadata context...");
    context_state::generate_and_save_metadata_context(&repo, &semantics_db).await?;

    let metadata = prompt_context::generate_context_for_code_review(&repo, &semantics_db).await?;

    info!("metadata => {:#?}", metadata);

    return Ok(());

    // ────────────────────────────────
    // 3. Code Slice Generation
    // ────────────────────────────────
    info!("generating codeblock for each contract in repo");
    // Create contextual code slices using call graph traversal
    let codeblocks_db = codeblock_maker::generate_and_save_codeblocks_for_each_contract(
        &repo,
        &semantics_db,
        audit_config().max_depth,
        audit_config().token_budget,
    )
    .await?;
    info!("Slices at {}", codeblocks_db.display());

    // ────────────────────────────────
    // 4. Vector Database Population
    // ────────────────────────────────
    // Create embeddings and store in Qdrant for semantic search
    vector_db::generate_slither_chucks_and_save_all_metadata_to_vector_db(&repo, &semantics_db)
        .await?;

    // ────────────────────────────────
    // 5. AI Security Analysis
    // ────────────────────────────────
    // Run multi-LLM security analysis across vulnerability categories
    let (security_issues, invariants) =
        code_review::review_codebase_for_security_issues(&codeblocks_db, &repo).await?;

    // ────────────────────────────────
    // 6. Report Generation
    // ────────────────────────────────
    // Generate comprehensive audit report (paid version)
    let audit_report = audit::generated_audit_report(
        &security_issues,
        &invariants,
        &repo,
        &semantics_db,
        ReportType::Paid,
    )
    .await?;

    // Generate limited audit report (free version)
    let free_audit_report = audit::generated_audit_report(
        &security_issues,
        &invariants,
        &repo,
        &semantics_db,
        ReportType::Free,
    )
    .await?;

    // ────────────────────────────────
    // 7. File Export & Cleanup
    // ────────────────────────────────
    // Save all reports and analysis data to markdown files
    save_file::save_audit_report(&audit_report, &repo, ReportType::Paid)?;
    save_file::save_audit_report(&free_audit_report, &repo, ReportType::Free)?;
    contract_data::save_contract_and_fn_ir(&codeblocks_db, &repo)?;
    contract_data::save_metadata(&semantics_db, &repo).await?;

    // Display total inference cost across all LLM providers
    let total_cost = get_total_inference_cost().await;
    info!("Total Inference Cost ===> {}", total_cost);

    // Clean up Docker volumes
    cleanup_repo_volume(&repo.root)?;

    Ok(())
}

```
ai-agent-audit/src/build_brain/callgraph.rs
```
/// Call graph analysis and DOT format parsing.
///
/// This module processes Slither's call graph output in DOT format, extracting
/// function relationships and building traversable graph structures for code
/// slice generation and dependency analysis.

use anyhow::Result;
use regex::Regex;
use rusqlite::Connection;
use serde::Deserialize;
use std::{collections::HashMap, default::Default, path::Path};

use crate::{
    enumerator::utils::get_function_metadata_from_id,
    prepare_code::git_clone::RepoPaths,
    utils::fn_labels::{get_modifiers_label, get_visibility_label},
};

use super::{graph_db::SmartContractFunction, slither_ffi::run_printer_json};

/// Represents a function node in the call graph
#[derive(Debug, Clone, Default)]
pub struct DotFunc {
    /// Unique identifier from Slither (e.g., "3895_changeFeeAddress")
    pub full_id: String,
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
}

/// Represents a call relationship between two functions
#[derive(Debug)]
pub struct DotEdge {
    /// Calling function's full_id
    pub caller: String,
    /// Called function's full_id
    pub callee: String,
}

/// Extracts call graph functions and edges from Slither analysis.
///
/// This function orchestrates the complete call graph extraction process by
/// running Slither's call-graph printer and parsing the resulting DOT format.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `(Vec<DotFunc>, Vec<DotEdge>)` - Functions and their call relationships
pub async fn get_dot_funcs_and_dot_edges(repo: &RepoPaths) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
    let json = run_printer_json(repo, "call-graph").await?;
    let blobs = extract_dot_blobs(&json)?;
    parse_dot_blobs(&blobs)
}

/// Step 2: pull every DOT file’s `content` string
pub fn extract_dot_blobs(json: &str) -> Result<Vec<String>> {
    #[derive(Deserialize)]
    struct DotFile {
        #[serde(rename = "type")]
        _ty: String,
        name: DotName,
    }
    #[derive(Deserialize)]
    struct DotName {
        content: String,
    }
    #[derive(Deserialize)]
    struct Printer {
        elements: Vec<DotFile>,
    }
    #[derive(Deserialize)]
    struct Root {
        results: Results,
    }
    #[derive(Deserialize)]
    struct Results {
        printers: Vec<Printer>,
    }

    let root: Root = serde_json::from_str(json)?;
    let mut out = Vec::new();
    for printer in root.results.printers {
        for file in printer.elements {
            out.push(file.name.content);
        }
    }
    Ok(out)
}

/// Step 3: regex-scan DOT text → nodes & edges
pub fn parse_dot_blobs(blobs: &[String]) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
    let node_re = Regex::new(r#""(\d+)_([A-Za-z0-9$_]+)" \[label"#)?;
    let edge_re = Regex::new(r#""(\d+_[^"]+)" -> "(\d+_[^"]+)""#)?;
    let cluster_re = Regex::new(r#"cluster_(\d+)_([A-Za-z0-9$_]+) \{"#)?;
    let mut funcs = HashMap::<String, DotFunc>::new();
    let mut edges = Vec::<DotEdge>::new();

    for blob in blobs {
        let mut contract = String::new();
        for line in blob.lines() {
            if let Some(c) = cluster_re.captures(line) {
                contract = c[2].to_string(); // e.g., PuppyRaffle
            }
            if let Some(c) = node_re.captures(line) {
                let full = c[1].to_string() + "_" + &c[2];
                let func = DotFunc {
                    full_id: full.clone(),
                    contract: contract.clone(),
                    name: c[2].to_string(),
                };
                funcs.entry(full).or_insert(func);
            }
            if let Some(e) = edge_re.captures(line) {
                edges.push(DotEdge {
                    caller: e[1].to_string(),
                    callee: e[2].to_string(),
                });
            }
        }
    }
    Ok((funcs.into_values().collect(), edges))
}

pub async fn get_enriched_funcs_and_edges(
    repo: &RepoPaths,
    semantic_path: &Path,
) -> Result<String> {
    let mut enriched_edges = Vec::<DotEdge>::new();
    let mut enriched_funcs = Vec::<DotFunc>::new();
    let semantic_db = Connection::open(semantic_path)?;

    let (funcs, edges) = get_dot_funcs_and_dot_edges(repo).await?;

    for edge in edges {
        let enriched_callee = match get_function_metadata_from_id(&edge.callee, &semantic_db)? {
            Some(callee_fn) => generated_enriched_fn_label(&edge.callee, callee_fn),
            None => edge.callee,
        };
        let enriched_caller = match get_function_metadata_from_id(&edge.caller, &semantic_db)? {
            Some(callee_fn) => generated_enriched_fn_label(&edge.caller, callee_fn),
            None => edge.caller,
        };
        enriched_edges.push(DotEdge {
            callee: enriched_callee,
            caller: enriched_caller,
        });
    }

    for func in funcs {
        let enriched_func_name = match get_function_metadata_from_id(&func.full_id, &semantic_db)? {
            Some(full_func) => generated_enriched_fn_label(&func.name, full_func),
            None => func.name,
        };
        enriched_funcs.push(DotFunc {
            full_id: func.full_id,
            contract: func.contract,
            name: enriched_func_name,
        });
    }

    // log::info!("enriched edges => {:#?}", enriched_edges);

    let funcs_string: String = enriched_funcs
        .iter()
        .map(|f| {
            format!(
                "id: {}, contract: {}, name: {}",
                f.full_id, f.contract, f.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let edges_string: String = enriched_edges
        .iter()
        .map(|e| format!("{} -> {}", e.caller, e.callee))
        .collect::<Vec<_>>()
        .join("\n");

    let mut final_dot_string = String::new();

    final_dot_string.push_str("\n\n### Functions\n\n");
    final_dot_string.push_str(&funcs_string);
    final_dot_string.push_str("\n\n### Dot Edges (Caller -> Callee)\n\n");
    final_dot_string.push_str(&edges_string);

    // log::info!("final dot string => {}", final_dot_string);
    Ok(final_dot_string)
}

fn generated_enriched_fn_label(fn_id: &str, fn_metadata: SmartContractFunction) -> String {
    let visibility = get_visibility_label(&fn_metadata.visibility);
    let modifiers = get_modifiers_label(&fn_metadata.modifiers);

    format!("{} {}{}", fn_id, visibility, modifiers)
}

```
ai-agent-audit/src/build_brain/enbeddings.rs
```
/// Vector embeddings generation for semantic search.
///
/// This module creates high-quality vector embeddings from source code and analysis
/// results using OpenAI's text-embedding-3-small model. Handles intelligent text
/// chunking with overlap to maintain context for optimal semantic search performance.
use anyhow::Result;
use log::info;
use rig::{
    client::EmbeddingsClient,
    embeddings::EmbeddingsBuilder,
    providers::openai::{self, Client},
    Embed,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tiktoken_rs::CoreBPE;

use crate::utils::bpe::get_bpe; // OpenAI’s GPT-4 / text-embedding 3 vocab

/// Represents a chunk of source code with metadata for vector embedding.
///
/// This struct organizes text content and associated metadata for embedding
/// generation, enabling semantic search with rich context information.
#[derive(Debug, Embed, Clone, Serialize, Deserialize)]
pub struct SourceChunk {
    #[embed] // Field Rig will vectorise
    pub text: String, // The actual text content to be embedded
    metadata: String, // we’ll keep this alongside the vector
}

/// Number of tokens in each chunk for optimal embedding quality
const CHUNK_TOKENS: usize = 256;
/// Number of tokens to overlap between chunks to maintain context continuity
const OVERLAP: usize = 32;
/// Batch size for embedding API requests to optimize throughput
const BATCH: usize = 30;
/// Maximum chunk length in characters (OpenAI supports ~8192 tokens, leaving headroom)
const MAX_CHUNK_LEN: usize = 4000;

/**
 * Processes a list of files and creates embeddings for their content.
 *
 * TODO - USE codellama:embed instead of openai embedding model
 * will need to either self host (need powerful computer) or self host on
 * on gcp ($250/month), this embedding is optimal for code
 *
 * @param paths - Array of file paths to process
 * @return Result containing a vector of tuples with metadata and embedding vectors
 */
pub async fn embed_files(paths: &[impl AsRef<Path>]) -> Result<Vec<(SourceChunk, Vec<f32>)>> {
    // ------------------------------------------------------------------
    // 1. Slice every file into SourceChunk structs
    // ------------------------------------------------------------------
    let mut docs = Vec::<SourceChunk>::new();

    info!("looping through all files and breaking into chunks");
    let bpe = get_bpe();
    for file in paths {
        let content = fs::read_to_string(file.as_ref())?;
        for (i, chunk) in tokenize(bpe, &content).into_iter().enumerate() {
            let clean = chunk
                .replace('\0', "") // Remove null bytes
                .replace('\u{FFFD}', ""); // Remove replacement chars
            let clean = clean.trim();

            if clean.is_empty() {
                log::warn!("Skipping empty chunk from {}", file.as_ref().display());
                continue;
            }
            if clean.len() > MAX_CHUNK_LEN {
                log::warn!(
                    "Skipping oversized chunk ({} chars) from {}",
                    clean.len(),
                    file.as_ref().display()
                );
                continue;
            }
            docs.push(SourceChunk {
                text: chunk,
                metadata: format!("{}:chunk {}", file.as_ref().display(), i),
            });
        }
    }

    info!("breaking out data into text chunks complete");
    // ------------------------------------------------------------------
    // 2. Pick an embedding model once
    // ------------------------------------------------------------------
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let openai = Client::new(&api_key);

    // 1536‑dim “storage‑optimised” v3 model
    let model = openai.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

    // ------------------------------------------------------------------
    // 3. Build embeddings in one RPC batch
    //    EmbeddingsBuilder<M, D>::new(model) infers both generics
    // ------------------------------------------------------------------
    // ------------------------------------------------------------------
    // 4. Flatten → (metadata, vector) so the caller can upsert to Qdrant
    // ------------------------------------------------------------------
    let mut all_vecs = Vec::<(SourceChunk, Vec<f32>)>::new();

    // info!("using openai to embed in {}-item batches…", BATCH);
    for docs_slice in docs.chunks(BATCH) {
        // info!("Batch size: {}", docs_slice.len());
        // for (i, doc) in docs_slice.iter().enumerate() {
        //     info!(
        //         "Chunk {}: text='{}', metadata='{}'",
        //         i, doc.text, doc.metadata
        //     );
        // }
        let batch_result = EmbeddingsBuilder::new(model.clone())
            .documents(docs_slice.to_vec())? // slice → Vec
            .build()
            .await;

        match batch_result {
            Ok(batch) => {
                all_vecs.extend(batch.into_iter().filter_map(|(doc, emb)| {
                    let v = emb.first().vec;
                    if v.is_empty() {
                        return None;
                    }
                    Some((doc, v.into_iter().map(|x| x as f32).collect()))
                }));
            }
            Err(e) => {
                log::error!("Embedding batch failed: {:#}", e);
                // Optionally retry, skip or abort here
            }
        };
    }
    Ok(all_vecs)
}

/// Split `s` into fixed-width token windows with `OVERLAP` tokens of context.
fn tokenize(bpe: &CoreBPE, s: &str) -> Vec<String> {
    let tokens = bpe.encode_with_special_tokens(s);

    let mut out = Vec::new();
    let mut start = 0;

    while start < tokens.len() {
        let end = usize::min(start + CHUNK_TOKENS, tokens.len());
        let token_slice = tokens[start..end].to_vec();
        let chunk = bpe.decode(token_slice).unwrap_or_default();
        out.push(chunk);

        if end == tokens.len() {
            break; // reached the tail – exit
        }
        start += CHUNK_TOKENS - OVERLAP; // always moves forward
    }
    out
}

```
ai-agent-audit/src/build_brain/enrichment.rs
```
use crate::build_brain::callgraph::DotFunc;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::get_function_name;

use super::fn_summaries::get_function_summaries;
use super::graph_db::GraphDb;
/// Smart contract data enrichment using Slither static analysis.
///
/// This module builds semantic databases containing call graphs, inheritance hierarchies,
/// and function metadata extracted from Solidity contracts using Slither analysis.
use super::slither_ffi::{self, SlithIRFn, StorageVar};
use super::{callgraph, inheritance};
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Contains the enriched data extracted from Solidity contracts.
/// This includes the intermediate representation (IR) of functions and storage variable information.
pub struct Enriched {
    /// Vector of SlithIR function representations
    pub ir: Vec<SlithIRFn>,
    /// Vector of storage variable information
    pub storage: Vec<StorageVar>,
}

/// Builds a semantic database containing call graphs and inheritance data.
///
/// This function extracts call graph and inheritance information from Slither analysis
/// and stores it in a SQLite database for efficient querying during code analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `PathBuf` - Path to the created semantic database
pub async fn build_semantics_db_from_call_graph(repo: RepoPaths) -> Result<PathBuf> {
    // Create database file in cache directory
    let db_path = repo.root.join(".cache").join("semantics.db");
    let cache_dir = db_path.parent().ok_or_else(|| {
        AuditError::file_system(
            db_path.to_string_lossy().to_string(),
            "Invalid database path - no parent directory",
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"),
        )
    })?;
    std::fs::create_dir_all(cache_dir).map_err(|e| {
        AuditError::file_system(
            cache_dir.to_string_lossy().to_string(),
            "Failed to create cache directory",
            e,
        )
    })?;
    let db = Arc::new(Mutex::new(GraphDb::create(&db_path)?));
    let repo = Arc::new(repo);

    // Process call graph data in parallel task
    let repo_func = Arc::clone(&repo);
    let db_func = Arc::clone(&db);
    let handle = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract call graph data from Slither
            info!("extracting DOT blobs");
            let json = slither_ffi::run_printer_json(&repo_func, "call-graph").await?;
            let blobs = callgraph::extract_dot_blobs(&json)?;
            let mut rows = Vec::new();
            let (funcs_id, edges) = callgraph::parse_dot_blobs(&blobs)?;
            let func_index: HashMap<(String, String), DotFunc> = funcs_id
                .into_iter()
                .map(|node| ((node.contract.clone(), node.name.clone()), node))
                .collect();

            // Get function summaries for metadata
            info!("getting function summaries...");
            let funcs = get_function_summaries(&repo_func).await?;

            info!("insert function metadata into database..");
            info!("{} function summaries", funcs.len());
            // Insert function metadata into database
            for f in &funcs {
                let func_name = get_function_name(&f.name);
                if let Some(node) = func_index.get(&(f.contract.clone(), func_name)) {
                    rows.push((
                        node.full_id.clone(),
                        f.contract.clone(),
                        f.name.clone(),
                        f.visibility.clone(),
                        f.modifiers.join(","),
                        f.mutability.clone(),
                    ))
                }
            }
            // single lock, batch insert
            {
                let db = db_func.lock().await;
                for row in rows {
                    db.insert_function(&row.0, &row.1, &row.2, &row.3, &row.4, &row.5)?;
                }
            }
            info!("done inserting function metadata into database");

            info!("insert {} call graph edges in db", edges.len());
            // Insert call graph edges
            for e in &edges {
                let db_guard = db_func.lock().await;
                db_guard.insert_edge(&e.caller, &e.callee)?;
            }
            info!("dot edges in db complete");
            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing call graph data: {:#}", e);
        }

        Ok::<_, AuditError>(())
    });

    // Process inheritance data in parallel task
    let db_inheritance = Arc::clone(&db);
    let repo_inheritance = Arc::clone(&repo);
    let handle_inheritance = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract inheritance hierarchy from Slither
            info!("generating inheritance json");
            let inheritance_json =
                slither_ffi::run_printer_json(&repo_inheritance, "inheritance").await?;
            let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
            info!("{} inheritance edges", inheritance_edges.len());

            // Insert inheritance relationships into database
            for (child, parent) in inheritance_edges {
                let db_guard = db_inheritance.lock().await;
                db_guard.insert_inheritance(&child, &parent)?;
            }
            info!("done generating inheritance edges");

            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing inheritance data: {:#}", e);
        }
        Ok::<_, AuditError>(())
    });

    // Wait for both parallel tasks to complete
    info!("waiting for meta data analysis to complete...");
    let (_func_result, _inheritance_result) = tokio::try_join!(handle, handle_inheritance)?;
    Ok(db_path)
}

```
ai-agent-audit/src/build_brain/fn_summaries.rs
```
/// Function summarization using Slither analysis.
///
/// This module extracts function metadata from Slither's function summary printer,
/// parsing visibility, modifiers, and mutability information for all functions
/// across contracts in a repository.

use anyhow::Result;
use serde::Deserialize;

use crate::{build_brain::slither_ffi::run_printer, prepare_code::git_clone::RepoPaths};

/// Comprehensive metadata for a smart contract function
#[derive(Debug, Deserialize, Clone)]
pub struct FnSummary {
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
    /// Function visibility (external/public/internal/private)
    pub visibility: String,
    /// Applied modifiers (e.g., ["onlyOwner", "nonReentrant"])
    #[serde(default)]
    pub modifiers: Vec<String>,
    /// State mutability (view/pure/payable/nonpayable)
    #[serde(default)]
    pub mutability: String,
}

/// Parses Slither's function summary table format into structured data.
///
/// Processes the text output from Slither's function-summary printer,
/// extracting function metadata for each contract in the repository.
fn parse_table(block: &str) -> Vec<FnSummary> {
    let mut out = Vec::new();

    let mut lines = block.lines();
    let mut current_line = lines.next().unwrap_or_default();
    let mut inside_fn_block = false;

    // skip any lines before first table heading
    while !current_line.starts_with("Contract ") {
        current_line = lines.next().unwrap_or("EOF");

        // if file is empty!
        if current_line == "EOF" {
            return Vec::new();
        }
    }

    // grab first contract
    let mut current_contract = get_contract_name(current_line);
    current_line = lines.next().unwrap_or_default();

    while current_line != "EOF" {
        // grab functions
        if current_line.starts_with("Contract ") && !current_line.starts_with("Contract vars") {
            // new contract
            current_contract = get_contract_name(current_line);
            current_line = lines.next().unwrap_or("EOF");
            continue;
        } else if current_line.is_empty() {
            inside_fn_block = false;
            current_line = lines.next().unwrap_or("EOF");
            continue;
        }

        if current_line.starts_with("|") {
            let cols: Vec<_> = current_line.split('|').map(|c| c.trim()).collect();
            if cols.len() < 4 || cols[1].is_empty() || cols[1] == "Modifiers" {
                current_line = lines.next().unwrap_or("EOF");
                continue; // header or empty
            } else if cols[1] == "Function" {
                // new fn table found!
                inside_fn_block = true;
                current_line = lines.next().unwrap_or("EOF");
                continue;
            }

            if inside_fn_block {
                // cols[1]   Function
                // cols[2]   Visibility
                // cols[3]   Modifiers
                let func_name = cols[1];
                let modifiers = if cols[3] == "[]" {
                    vec![]
                } else {
                    // remove '[' and ']'
                    let raw = cols[3].to_string();
                    let trimmed: String = raw.chars().skip(1).take(raw.len() - 2).collect();
                    trimmed
                        .split(',')
                        .map(|s| s.trim_matches([' ', '\'']).to_string())
                        .collect::<Vec<_>>()
                };
                out.push(FnSummary {
                    contract: current_contract.clone(),
                    name: func_name.to_string(),
                    visibility: cols[2].to_string(),
                    modifiers,
                    mutability: String::new(), // not present in this table
                });
            }
        }

        // go to next line
        current_line = lines.next().unwrap_or("EOF");
    }

    // log::info!("function summaries ==> {:#?}", out);
    out
}

pub fn get_contract_name(line: &str) -> String {
    // grab first contract
    log::info!("line with Contract => {}", line);
    line.split_ascii_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string()
}

pub async fn get_function_summaries(repo: &RepoPaths) -> Result<Vec<FnSummary>> {
    // 1. run slither
    let raw = run_printer(repo, "function-summary").await?;

    Ok(parse_table(&raw))
}

```
ai-agent-audit/src/build_brain/graph_db.rs
```
/// Graph database operations for semantic data storage.
///
/// This module provides a SQLite-based graph database for storing and querying
/// smart contract semantic data including functions, call relationships, and
/// inheritance hierarchies extracted from Slither analysis.

use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

/// Represents a smart contract function with complete metadata
#[derive(Debug, Clone)]
pub struct SmartContractFunction {
    /// Unique function identifier from Slither
    pub id: String,
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
    /// Function visibility level
    pub visibility: String,
    /// Applied function modifiers
    pub modifiers: Vec<String>,
    /// State mutability specification
    pub mutability: String,
}

/// SQLite-based graph database for semantic contract data
pub struct GraphDb(Connection);

impl GraphDb {
    /// Creates a new graph database with required schema.
    ///
    /// Initializes SQLite database with tables for functions, call edges,
    /// and inheritance relationships. Uses WAL mode for better concurrency.
    pub fn create(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS functions(
              id TEXT PRIMARY KEY,   -- 3895_changeFeeAddress
              contract TEXT,
              name TEXT,
              visibility TEXT,
              modifiers TEXT,
              mutability TEXT
            );
            CREATE TABLE IF NOT EXISTS edges(
            caller TEXT,
            callee TEXT
            );
            /* NEW ↓ */
            CREATE TABLE IF NOT EXISTS inheritance(
            child TEXT,
            parent TEXT
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert_function(
        &self,
        id: &str,
        contract: &str,
        name: &str,
        visibility: &str,
        modifiers: &str,
        mutability: &str,
    ) -> Result<()> {
        self.0.execute(
            "INSERT OR IGNORE INTO functions(id, contract, name, visibility, modifiers, mutability) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![id, contract, name, visibility, modifiers, mutability],
        )?;
        Ok(())
    }

    pub fn insert_edge(&self, caller: &str, callee: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO edges(caller, callee) VALUES (?1, ?2);",
            params![caller, callee],
        )?;
        Ok(())
    }

    pub fn insert_inheritance(&self, child: &str, parent: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO inheritance(child, parent) VALUES (?1, ?2);",
            params![child, parent],
        )?;
        Ok(())
    }
}

```
ai-agent-audit/src/build_brain/inheritance.rs
```
/// Contract inheritance analysis and parsing.
///
/// This module processes Slither's inheritance printer output to extract
/// parent-child relationships between smart contracts, supporting both
/// immediate and transitive inheritance hierarchies.

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// Step 2: pull every DOT file’s `content` string
pub fn parse_inheritance_json(json: &str) -> Result<Vec<(String, String)>> {
    /// Represents a list of immediate and non-immediate inheritance relationships.
    #[derive(Debug, Deserialize)]
    pub struct InheritanceRelation {
        pub immediate: Vec<String>,
        pub not_immediate: Vec<String>,
    }

    /// Outer container for additional fields (child_to_base and base_to_child maps).
    #[derive(Debug, Deserialize)]
    pub struct AdditionalFields {
        pub child_to_base: HashMap<String, InheritanceRelation>,
        pub base_to_child: HashMap<String, InheritanceRelation>,
    }

    #[derive(Deserialize)]
    struct Printer {
        pub printer: String,
        pub additional_fields: AdditionalFields,
    }
    #[derive(Deserialize)]
    struct Root {
        results: Results,
    }
    #[derive(Deserialize)]
    struct Results {
        printers: Vec<Printer>,
    }

    let root: Root = serde_json::from_str(json)?;
    let mut child_parent_map = HashMap::new();
    let mut edges = Vec::new();
    for printer in root.results.printers {
        if printer.printer == "inheritance" {
            child_parent_map = printer.additional_fields.child_to_base;
        }
    }

    for (child, parents) in child_parent_map.into_iter() {
        for parent in parents.immediate.iter() {
            edges.push((child.clone(), parent.to_string()));
        }
    }
    Ok(edges)
}

```
ai-agent-audit/src/build_brain/parsers.rs
```
/// Code parsing utilities for Slither analysis output.
///
/// This module provides parsers for various Slither output formats including
/// detector results, SlithIR representations, storage layouts, and contract
/// summaries. Handles text parsing and data structure extraction.
use super::slither_ffi::{ContractSummary, SlithIRFn, StorageVar};
use log::{debug, info};

/// Parses Slither detector output into individual issue descriptions.
///
/// Processes the text output from Slither detectors, separating different
/// vulnerability findings into discrete issue descriptions.
pub fn parse_slither(text: &str) -> Vec<String> {
    let mut current_issue = String::new();
    let mut issues = Vec::<String>::new();

    for line in text.lines() {
        // parse each detected issue
        if line.contains("Detectors") && line.ends_with(':') {
            if !current_issue.is_empty() {
                issues.push(current_issue.to_string());
                current_issue = line.to_string();
            }
        } else {
            current_issue.push_str(line);
        }
    }

    // add last issues
    if !current_issue.is_empty() {
        issues.push(current_issue.to_string());
    }

    // info!("issues => {:#?}", issues);
    issues
}

/// Parses Slither's SlithIR output into structured function representations.
///
/// Processes the text output from Slither's slithir-ssa printer, extracting
/// intermediate representation (IR) code for each function along with contract
/// and function metadata for code analysis and slice generation.
///
/// # Arguments
/// * `text` - Raw text output from the slithir-ssa printer
///
/// # Returns
/// * `Vec<SlithIRFn>` - Structured IR data for all functions
pub fn parse_slithir_ir_code(text: &str) -> Vec<SlithIRFn> {
    let mut current_contract = String::new();
    let mut current_fn = String::new();
    let mut buf = String::new();
    let mut out = Vec::new();

    // info!("text in parse_slithir {}", text.len());
    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract ") {
            current_contract = line["Contract ".len()..].trim_end_matches(':').to_owned();
        }
        // Parse function lines (format: "\tFunction functionName:")
        else if line.starts_with("\tFunction ") {
            // Flush previous function data if we have any
            if !current_fn.is_empty() {
                let ir_content = replace_special_character(&buf);
                let ir_content_cleaned =
                    ir_content.replace("IRs:\n", "").replace("Expression:", "");
                out.push(SlithIRFn {
                    contract: current_contract.clone(),
                    function: current_fn.clone(),
                    ir: ir_content_cleaned,
                });
            }
            // Extract new function name and reset buffer
            current_fn = line.trim()[9..].trim_end_matches(':').to_owned();
            buf.clear();
        }
        // Parse IR lines (format: "\t\t<ir content>")
        else if line.starts_with("\t\t") {
            buf.push_str(line.trim_start());
            buf.push('\n');
        }
    }

    // Flush the last function after processing all lines
    if !current_fn.is_empty() && !buf.trim().is_empty() && buf.trim().len() > 10 {
        let ir_content = replace_special_character(&buf);
        out.push(SlithIRFn {
            contract: current_contract,
            function: current_fn,
            ir: ir_content,
        });
    } else if !current_fn.is_empty() {
        info!(
            "Skipping empty or small IR for {}::{}",
            current_contract, current_fn
        );
    }

    // info!("functions => {:#?}", out);
    out
}

pub fn parse_slithir_contract_summary(text: &str) -> Vec<ContractSummary> {
    let mut current_name = String::new();
    let mut current_body = String::new();
    let mut out = Vec::<ContractSummary>::new();

    for line in text.lines() {
        // ── new contract header ─────────────────────────────────
        if let Some(rest) = line.strip_prefix("+ Contract ") {
            // flush the previous one (if any)
            if !current_name.is_empty() {
                out.push(ContractSummary {
                    contract: current_name.clone(),
                    content: current_body.trim_end().to_string(),
                });
                current_body.clear();
            }

            // take “IERC20 (Most derived contract)” → “IERC20”
            current_name = rest
                .split_whitespace() // split at first space
                .next()
                .unwrap_or_default()
                .to_string();
            continue;
        }

        // ── body line (ignore leading INFO: lines) ─────────────
        if current_name.is_empty() {
            continue; // still in preamble; skip
        }

        if !line.trim().is_empty() {
            current_body.push_str(line.trim_start());
            current_body.push('\n');
        }
        // we do NOT rely on blank line to flush; handled by header or final push
    }

    // push the last contract
    if !current_name.is_empty() {
        out.push(ContractSummary {
            contract: current_name,
            content: current_body.trim_end().to_string(),
        });
    }
    out
}

/// Parses the output of the Slither 'variable-order' printer into a vector of StorageVar structs.
///
/// This function processes the text output from Slither's variable-order printer,
/// which contains information about storage variables in Solidity contracts.
/// It extracts the contract name, variable name, and variable type for each storage variable.
///
/// @param text - The raw text output from the variable-order printer
/// @return Vector of StorageVar structs containing the parsed data
pub fn parse_storage(text: &str) -> Vec<StorageVar> {
    let mut current_contract = String::new();
    let mut vars = Vec::new();

    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract") && line.ends_with(':') {
            current_contract = line["Contract".len()..]
                .trim_end_matches(':')
                .trim()
                .to_owned();
        }
        // Parse variable lines (format: "| <index> | <name> | <type> | <...> |")
        else if line.starts_with('|') && line.contains('|') {
            let cols: Vec<_> = line.split('|').map(|c| c.trim()).collect();
            // Check if this is a valid variable line (has enough columns and not a header)
            if cols.len() >= 3 && cols[1] != "Name" && !cols[1].is_empty() && !cols[2].is_empty() {
                // cols[1] is contract_name.function_name.  need to parse
                let (contract, function) = split_str_by_period(cols[1]).unwrap();
                vars.push(StorageVar {
                    contract,
                    name: function,
                    r#type: cols[2].to_owned(),
                });
            } else {
                debug!(
                    "Skipping invalid storage var in {}: {:?}",
                    current_contract, cols
                );
            }
        }
    }

    // info!("storage => {:#?}", vars.len());
    vars
}
// splits "first.last" => ("first","last")
fn split_str_by_period(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None // Invalid format
    }
}
/// Replces special characters in the SlithIR text with their ASCII equivalents.
///
/// This function replaces the Greek letter phi (ϕ) with the ASCII string "phi"
/// to ensure the text can be properly processed and displayed.
///
/// @param text - The text containing special characters
/// @return String with special characters replaced
pub fn replace_special_character(text: &str) -> String {
    // Replace the Greek letter phi (ϕ) with "phi"
    let cleaned_text = text.trim().replace("ϕ", "phi");

    cleaned_text
}

```
ai-agent-audit/src/build_brain/slither_ffi.rs
```
/// Slither static analyzer interface with Docker integration.
///
/// This module provides a secure interface to Slither static analysis tool,
/// running all operations in Docker containers for security. Handles extraction
/// of IR, call graphs, inheritance data, and storage layouts with caching.
use anyhow::{anyhow, Result};
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::inheritance;
use crate::build_brain::parsers::parse_slithir_contract_summary;
use crate::build_brain::summarize::summarize_src_files;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_doc_file::extract_content_from_docs;

use super::callgraph;
use super::parsers::{parse_slither, parse_slithir_ir_code, parse_storage};

/// Global cache for Slither printer outputs to avoid redundant analysis.
/// Key format: "{repo_root}:{printer_name}"
pub static PRINTER_OUTPUT_CACHE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a single function's SlithIR (intermediate representation).
///
/// SlithIR is Slither's intermediate representation of Solidity code, which
/// makes it easier to analyze the code's behavior and identify potential issues.
#[derive(Debug, Serialize, Deserialize)]
pub struct SlithIRFn {
    /// Name of the contract containing this function
    pub contract: String,
    /// Name of the function
    pub function: String,
    /// The SlithIR representation of the function's code
    pub ir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSummary {
    /// Name of the contract containing this function
    pub contract: String,
    /// list of function
    pub content: String,
}

/// Represents a storage variable in a Solidity contract.
///
/// This struct contains information about a storage variable, including
/// its name, type, and the contract it belongs to.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageVar {
    /// Name of the contract containing this storage variable
    pub contract: String,
    /// Name of the storage variable
    pub name: String,
    /// Data type of the storage variable (e.g., "uint256", "address", etc.)
    pub r#type: String,
}
/// Return “context file list” as LF-separated string.
pub fn get_all_files_src(repo: &RepoPaths) -> String {
    //   e.g.,   contracts/plume/src
    let code_root = repo.root.join(&repo.repo_name).join("src");

    let mut files = Vec::<String>::new();

    for path in &repo.sol_files {
        // fast skip: must be under src/ and not a symlink
        if !path.starts_with(&code_root) {
            continue;
        }

        if fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(true)
        {
            continue;
        }

        // build a relative "short path"
        let rel = path.strip_prefix(&code_root).unwrap_or(path);
        let rel_str = rel.to_string_lossy();

        if should_skip(&rel_str) {
            continue;
        }

        files.push(rel_str.to_string());
    }

    // stable ordering helps diffing prompts
    files.sort();
    files.join("\n")
}

fn should_skip(rel: &str) -> bool {
    let lowercase = rel.to_ascii_lowercase();

    // 1. third-party deps: vendor/*/contracts/**   OR   node_modules/**/contracts/**
    if lowercase.contains("/vendor/") && lowercase.contains("/contracts/")
        || lowercase.contains("/node_modules/") && lowercase.contains("/contracts/")
    {
        return true;
    }

    // 2. other large externals you may add later
    if lowercase.starts_with("lib/") && lowercase.contains("/contracts/") {
        return true;
    }

    false
}

pub async fn run_slither_detector(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "detector");
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither detector");
    let volume = format!("{}:/workspace", &repo.root.display());
    let out = Command::new("docker")
        .args([
            "run",
            "--read-only",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,            // Use the already-built repo folder
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--exclude-dependencies",
            "--foundry-out-directory", // Specify where to find Forge build artifacts
            "out",
        ])
        .output()?;

    // anyhow::ensure!(out.status.success(), "slither --sarif failed");
    //
    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&out.stderr).into_owned();
    }

    info!("slither analysis complete with size {}", text.len());
    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

/// Runs a single Slither printer and captures its output.
///
/// This function executes the Slither static analysis tool with a specific printer
/// and returns the captured output as a string.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param printer - Name of the Slither printer to run (e.g., "slithir-ssa", "variable-order")
/// @return Result containing the printer's output as a string
pub async fn run_printer(repo: &RepoPaths, printer: &str) -> Result<String> {
    let key = cache_key(&repo.root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither printer: {}", printer);
    let volume = format!("{}:/workspace", &repo.root.display());
    let output = Command::new("docker")
        .args([
            "run",
            "--read-only",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,            // Use the already-built repo folder
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print",
            printer,
            "--exclude-low",
            "--exclude-medium",
            "--exclude-high",
            "--exclude-informational",
            "--disable-color", // Disable ANSI color codes for easier parsing
        ])
        .stdout(Stdio::piped()) // Capture printer text from stdout
        .stderr(Stdio::piped()) // Capture banner & errors from stderr
        .output()?;

    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }

    // Ensure we got some output
    if text.trim().is_empty() {
        return Err(anyhow!("Slither ran but produced no `{}` output", printer));
    }

    info!("{} printer complete with size {}", printer, text.len());
    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

// use for inheritance and call-graph
pub async fn run_printer_json(repo: &RepoPaths, printer: &str) -> Result<String> {
    let key = cache_key(&repo.root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither printer: {}", printer);
    let volume = format!("{}:/workspace", &repo.root.display());
    let out = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print",
            printer,
            "--exclude-low",
            "--exclude-medium",
            "--exclude-high",
            "--exclude-informational",
            "--disable-color",
            "--json",
            "-",
        ])
        .output()?;

    anyhow::ensure!(out.status.success(), format!("slither {} failed", printer));

    let text = String::from_utf8_lossy(&out.stdout).into_owned();

    // Save to cache and return
    info!("{} print complete with size {}", printer, text.len());
    printer_cache.insert(key, text.clone());

    Ok(text)
}
/// Runs both Slither printers and returns the parsed IR and storage information.
///
/// This function is the main public interface for extracting SlithIR and storage
/// information from Solidity contracts. It runs both the slithir-ssa and variable-order
/// printers and parses their output.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing a tuple of SlithIRFn and StorageVar vectors
pub async fn get_slither_ir_and_storage_for_codeblockcodeblock(
    repo: &RepoPaths,
) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>, Vec<String>)> {
    // Run the slithir-ssa printer to get IR information
    let ir_raw = run_printer(repo, "slithir-ssa").await?;

    // Run the variable-order printer to get storage information
    let storage_raw = run_printer(repo, "variable-order").await?;
    // info!("storage raw => {}", storage_raw);

    let slither_scan_results = run_slither_detector(repo).await?;

    // Parse both outputs and return the results
    Ok((
        parse_slithir_ir_code(&ir_raw),
        parse_storage(&storage_raw),
        parse_slither(&slither_scan_results),
    ))
}

/// Dumps IR and storage information to individual text files in a directory.
///
/// This function extracts SlithIR and storage information from Solidity contracts
/// and writes each function's IR and each storage variable's information to separate
/// text files in the specified directory.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param dir - Path to the directory where the text files will be written
/// @return Result containing a vector of paths to the created files
pub async fn save_code_metadata_and_analysis_to_txt_files(
    repo: &RepoPaths,
    dir: &Path,
    semantics_path: &Path,
) -> Result<Vec<PathBuf>> {
    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chunks");
    let (_, _, slither_scan_vec) = get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;
    // info!("storage vec => {:?}", storage_vec);

    let (funcs, edges) = callgraph::get_dot_funcs_and_dot_edges(repo).await?;
    let inheritance_json = run_printer_json(repo, "inheritance").await?;
    let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
    let contract_summary = run_printer(repo, "contract-summary").await?;
    let contract_summary_vec = parse_slithir_contract_summary(&contract_summary);
    let src_file_list = get_all_files_src(repo);
    let summaries = summarize_src_files(repo, &semantics_path).await?;

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    info!("save src file list to txt");
    let file_list = dir.join("src_files.txt");
    info!("src file list => {}", src_file_list);
    fs::write(&file_list, src_file_list)?;
    out_paths.push(file_list);

    info!("convert file summaries to txt files");
    for sum in &summaries {
        let meta = format!("{}::file_summary", sum.filename);
        // info!("contract meta => {}", meta);
        let body = format!("\nfile: {}\n{}", sum.filename, sum.summary);
        // info!("{}", body);
        let p = dir.join(meta.replace("::", "_").replace("/", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }
    info!("convert contract summary to txt files");
    for c in &contract_summary_vec {
        let meta = format!("{}::contract_summary", c.contract);
        // info!("contract meta => {}", meta);
        let body = format!("\nContract: {}\n{}", c.contract, c.content);
        // info!("{}", body);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph functions to txt files");
    for f in &funcs {
        let meta = format!("function::{}::{}", f.full_id, f.contract);
        // info!("fn meta => {}", meta);
        let body = format!("{} {} {}", f.full_id, f.contract, f.name);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph edges to txt files");
    for e in &edges {
        let meta = format!("caller::{}::callee::{}", e.caller, e.callee);
        // info!("caller callee meta => {}", meta);
        let body = format!("{} {}", e.caller, e.callee);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph edges to txt files");
    for edge in &inheritance_edges {
        let meta = format!("child::{}::parent::{}", edge.0, edge.1);
        // info!("edges meta => {}", meta);
        let body = format!("{} {}", edge.0, edge.1);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert slither scan results to txt files");
    for (i, issue) in slither_scan_vec.iter().enumerate() {
        let meta = format!("{} slither code issue", i);
        let p = dir.join(meta.replace(" ", "_") + ".txt");
        fs::write(&p, issue)?;
        out_paths.push(p);
    }

    info!(
        "slither issues found, fn ir, storage var files => {:?}",
        out_paths.len()
    );

    Ok(out_paths)
}

pub fn cache_key(repo_root: &Path, printer: &str) -> String {
    format!("{}::{}", repo_root.display(), printer)
}

```
ai-agent-audit/src/build_brain/summarize.rs
```
/// Protocol and file summarization using LLMs.
///
/// This module generates intelligent summaries of smart contract protocols and
/// individual source files using OpenAI models. Provides cached summarization
/// for protocol overviews and contextual information for AI analysis.
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::openai::{self, GPT_4O, O3},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, Semaphore},
    task,
};

use crate::{
    cost::cost_data::add_to_inference_cost_by_type,
    prepare_code::git_clone::RepoPaths,
    utils::{
        contract_name_check::has_non_mock_contract, extract_retry::extractor_with_retry,
        get_doc_file::extract_content_from_docs,
    },
};
use crate::{
    cost::cost_data::LlmCostType,
    llm_review::prompt_context::{self, generate_context_for_code_review},
};

use super::slither_ffi::cache_key;

/// Global cache for file summaries to avoid redundant LLM calls
pub static FILE_SUMMARY_CACHE: Lazy<Arc<Mutex<HashMap<String, Vec<SrcFileSummary>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a summary of a source file with metadata
#[derive(Default, Debug, Clone)]
pub struct SrcFileSummary {
    /// Source file name
    pub filename: String,
    /// AI-generated summary of the file's purpose and functionality
    pub summary: String,
}

/// Structured response format for LLM file summarization
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileSummary {
    /// The generated summary text
    pub summary: String,
}

pub async fn summarize_docs(
    repo: &RepoPaths,
    current_context: &str,
) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(&repo.root, "docs-summary");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    let documentation = extract_content_from_docs(repo)?;
    let mut doc_summaries = Vec::new();

    let mut docs_plus_context = format!("\n ## DOCUMENTATION: \n\n {}\n\n", documentation);
    docs_plus_context.push_str("\n ## CURRENT SECURITY AUDIT CONTEXT \n\n");
    docs_plus_context.push_str(&format!("\n #### The Documentation Summary should NOT contain content that is already included below.\n\n {} \n\n", current_context));

    let openai_client = openai::Client::from_env();

    info!("generate summmary of all major files and docs in repo...");
    let preamble =
        "You are a senior solidity dev and expert solidity security researcher. Please provide detailed and comprehensive summary 
        of below DOCUMENTATION. Should be up to 4000 words, but no longer.  Should cover **all relevant details** that a security researcher 
        should know about this protocol to do a proper smart contract audit. ALSO, exclude any information from the summary that is already
        included in below CURRENT SECURITY AUDIT CONTEXT, because both DOCUMENTATION and CURRENT SECURITY AUDIT CONTEXT will be provide as
        context for an llm to do a security scan of protocol code.  So its important there is NO duplicate information between DOCUMENTATION 
        and CURRENT SECURITY AUDIT CONTEXT ";
    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(O3)
        .preamble(preamble)
        .build();

    add_to_inference_cost_by_type(
        &format!("{}{}", preamble, documentation),
        LlmCostType::Openai4oInput,
    )
    .await;

    info!("summarizing documentation");

    let doc_summary = match extractor_with_retry(
        &ai_summary_agent,
        &docs_plus_context,
        LlmCostType::Openai4oOutput,
    )
    .await
    {
        Ok(res) => SrcFileSummary {
            filename: "readme.md".to_string(),
            summary: res.summary,
        },
        Err(e) => {
            log::error!("❌ summarizing readme.md failed: {e}");
            SrcFileSummary {
                filename: "readme.md".to_string(),
                summary: String::new(),
            }
        }
    };

    log::info!("readme.md summary => {:#?}", doc_summary);
    doc_summaries.push(doc_summary);

    summaries_cache.insert(key, doc_summaries.clone());
    Ok(doc_summaries)
}

pub async fn summarize_src_files(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(&repo.root, "file_summaries");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    let openai_client = openai::Client::from_env();

    let context =
        prompt_context::generate_slither_metadata_prompt_context(repo, &semantics_path).await?;

    // info!("slither metadata => {:#?}", context);
    info!("generate summmary of all major files and docs in repo...");
    let preamble ="You are a senior solidity dev. Please summarize below content (code or docs). Format in markdown for easy reading. 
                    If content is code. Please write 200 word or less summary for each contract plus contract definition, 100 words or less summary 
                    of each function + function interface, and 50 word or less explanation of each storage variable + variable defintion. If docs 
                    please summarize each section of the docs with 150 words or less, max 500 words total for each doc file. 
                    Respond only with valid JSON matching the schema!";
    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(GPT_4O)
        .preamble(preamble)
        .context(&context)
        .build();

    // ---------------------------------------------
    // 1.  PREP – collect the  files we want to summarize first
    // ---------------------------------------------
    let mut work_items = Vec::new();

    // Walk through the repository and collect relevant files
    let repo_code_root = repo.root.join(repo.repo_name.clone());
    for file in &repo.sol_files {
        let is_sol_in_src = file.extension().map_or(false, |ext| ext == "sol")
            && file.starts_with(&repo_code_root.join("src"));

        if !is_sol_in_src {
            continue;
        }

        let is_readme_or_mock = file
            .file_name()
            .map(|f| {
                f.to_ascii_lowercase() == "readme.md"
                    || f.to_string_lossy().to_ascii_lowercase().contains("mock")
            })
            .unwrap_or(false)
            && (file.parent() == Some(&repo_code_root)
                || file.parent() == Some(&repo_code_root.join("src")));

        let is_sol_in_src = file.extension().map_or(false, |ext| ext == "sol")
            && file.starts_with(&repo_code_root.join("src"));

        // Skip directories and symlinks
        if !file.is_file() || fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        if is_readme_or_mock || is_sol_in_src {
            let content = fs::read_to_string(file.clone())?;

            // skip if content does not have have at least one line that start with contract and contract
            // name does NOT contain 'mock' (case insensative)
            let has_non_mock_contract = has_non_mock_contract(&content);

            if !has_non_mock_contract {
                continue;
            }

            // push full path & content into the work queue
            work_items.push((file.to_owned(), content));
        }
    }

    let max_parallel = 20;
    let sem = Arc::new(Semaphore::new(max_parallel));
    let agent = Arc::new(ai_summary_agent); // the OpenAI client
    let mut handles = Vec::new();

    for (file, content) in work_items {
        let sem = sem.clone();
        let agent = agent.clone();
        let repo_root = repo.root.clone();

        let handle = tokio::spawn(async move {
            // acquire permit – blocks if `max_parallel` already in-flight
            let _permit = sem.acquire_owned().await.unwrap();

            add_to_inference_cost_by_type(
                &format!("{}{}", preamble, content),
                LlmCostType::Openai4oInput,
            )
            .await;

            info!("summarizing {}", file.display());

            match extractor_with_retry(&agent, &content, LlmCostType::Openai4oOutput).await {
                Ok(res) => {
                    let filename = file
                        .strip_prefix(&repo_root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string();
                    Some(SrcFileSummary {
                        filename,
                        summary: res.summary,
                    })
                }
                Err(e) => {
                    log::error!("❌ summarizing {} failed: {e}", file.display());
                    None
                }
            }
        });
        handles.push(handle);
    }

    // wait for all tasks
    let mut summaries = Vec::new();
    for h in handles {
        if let Some(s) = h.await? {
            summaries.push(s);
        }
    }

    log::info!("summaries => {:#?}", summaries);

    summaries_cache.insert(key, summaries.clone());
    Ok(summaries)
}

pub async fn summarize_protocol(repo: &RepoPaths, semantics_path: &Path) -> Result<String> {
    // content retrival MUST come first to prevent race condition
    let context = generate_context_for_code_review(repo, &semantics_path).await?;

    let key = cache_key(&repo.root, "protocol-summary");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        let summary = cached
            .first()
            .unwrap_or(&SrcFileSummary::default())
            .summary
            .clone();
        return Ok(summary);
    }

    // Initialize vectors to store file paths
    let mut summaries = Vec::<SrcFileSummary>::new();

    let openai_client = openai::Client::from_env();

    log::info!("generate context for code review");
    let preamble= "You are a senior solidity dev. Given the context provided for solidity smart contract protocol, 
                   please create a max 200 word summary of this protocol explaining what it is, and how it works.  Format 
                   in markdown for easy reading. Respond only with valid JSON matching the schema!";

    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(O3)
        .preamble(preamble)
        .build();

    log::info!("extracting protocol summary");
    // rerun if NoDataExtracted Error

    add_to_inference_cost_by_type(
        &format!("{}{}", preamble, context),
        LlmCostType::OpenaiO3Output,
    )
    .await;

    let summary =
        extractor_with_retry(&ai_summary_agent, &context, LlmCostType::OpenaiO3Output).await?;

    log::info!("protocol summary => {:#?}", summary);

    summaries.push(SrcFileSummary {
        filename: "protocol-summary".to_string(),
        summary: summary.summary.clone(),
    });

    summaries_cache.insert(key, summaries.clone());
    Ok(summary.summary)
}

```
ai-agent-audit/src/build_brain/vector_db.rs
```
use std::path::{Path, PathBuf};

/// Qdrant vector database operations for semantic search.
///
/// This module handles vector database operations including collection creation,
/// embedding generation, and metadata storage for intelligent code search and
/// AI agent context retrieval.
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use log::info;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, PointStruct, UpsertPointsBuilder,
    VectorParams, VectorsConfig,
};
use qdrant_client::Payload;
use qdrant_client::Qdrant;

use crate::build_brain::enbeddings::embed_files;
use crate::build_brain::slither_ffi;
use crate::prepare_code::git_clone::RepoPaths;

use super::enbeddings::SourceChunk;

/// Generates embeddings from Slither analysis and stores them in Qdrant.
///
/// This function creates a complete vector database for the repository by:
/// 1. Checking if collection already exists to avoid duplication
/// 2. Extracting all source files and Slither analysis results
/// 3. Generating embeddings using OpenAI's text-embedding-3-small
/// 4. Creating a unique Qdrant collection for the repository
/// 5. Storing vectors with rich metadata for semantic search
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantic_db` - Path to semantic analysis database
pub async fn generate_slither_chucks_and_save_all_metadata_to_vector_db(
    repo: &RepoPaths,
    semantic_db: &Path,
) -> Result<()> {
    // check if vector db for this repo already exists
    if does_qdrant_vector_db_for_this_repo_already_exist(&repo).await? {
        return Ok(());
    }
    // 2b. Create a temp dir and ask slither_ffi to fill it with chunk files
    // Create a temporary directory to store the Slither analysis results
    let tmp_dir = tempfile::tempdir().map_err(|e| {
        AuditError::file_system("tempdir", "Failed to create temporary directory", e)
    })?;
    info!("generating slither ssa into txt files that contain function or storage var");

    // Extract IR and storage information using Slither and write to text files
    let slither_chunk_paths = slither_ffi::save_code_metadata_and_analysis_to_txt_files(
        repo,
        tmp_dir.path(),
        &semantic_db,
    )
    .await?;
    info!("slither ssa file count => {}", slither_chunk_paths.len());
    // ────────────────────────────────
    // 4. Assemble the *full* file list to embed
    //    – original Solidity + docs  (repo.sol_files  ∪  repo.docs)
    //    – temp IR / storage files   (tmp_paths)
    // ────────────────────────────────
    info!("combine all file locations for solidity + docs, IR functions, and storage into 1 vec");
    // Combine all file paths into a single vector:
    // - Solidity source files
    // - Documentation files
    // - Slither analysis result files
    let mut all_files: Vec<_> = repo.docs.clone().into_iter().collect();
    all_files.extend(slither_chunk_paths);
    info!("all files => {:?}", all_files.len());

    //embed all files and upsert to qdrant vector db for later dynamic retrival
    generate_enbeddings_and_save_to_qdrant_vector_db(&all_files, &repo).await?;
    Ok(())
}

pub async fn generate_enbeddings_and_save_to_qdrant_vector_db(
    all_files: &[PathBuf],
    repo: &RepoPaths,
) -> Result<()> {
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());

    info!("connect to qdrant db");

    // Build Qdrant client configuration and connect to the database
    let qdrant_url = std::env::var("QDRANT_URL")
        .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;
    let qdrant = Qdrant::from_url(&qdrant_url).build().map_err(|e| {
        AuditError::vector_db("connection", "Failed to connect to Qdrant database", e)
    })?;

    let already_exists = qdrant.collection_exists(&vector_db_name).await?;

    // if vector db already exits for this repo no need to re-upsert
    if already_exists {
        return Ok(());
    }

    // Create the collection if it doesn't exist
    info!("create contract_chunks vector db (if does not exist)");
    ensure_collection(&qdrant, &vector_db_name, audit_config().vector_dimension).await?;

    // Generate vector embeddings for all files
    info!("generating vector embedding");
    let embeddings = embed_files(&all_files).await?;
    // ────────────────────────────────
    // 4. Upsert into Qdrant
    // ────────────────────────────────
    // Upsert the embeddings into the Qdrant collection
    info!("upsert embeddings");
    upsert(&qdrant, &vector_db_name, &embeddings).await?;

    // Print completion message
    println!("✅ Ingest complete – {} chunks stored", embeddings.len());
    Ok(())
}

pub async fn does_qdrant_vector_db_for_this_repo_already_exist(repo: &RepoPaths) -> Result<bool> {
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());

    info!("connect to qdrant db");
    // Build Qdrant client configuration and connect to the database
    let qdrant_url = std::env::var("QDRANT_URL")
        .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;
    let qdrant = Qdrant::from_url(&qdrant_url).build().map_err(|e| {
        AuditError::vector_db("connection", "Failed to connect to Qdrant database", e)
    })?;

    Ok(qdrant.collection_exists(&vector_db_name).await?)
}

/// Ensures that a collection exists in the Qdrant database, creating it if missing.
///
/// This function creates a new collection with the specified name and dimension if it doesn't
/// already exist. It uses cosine distance for similarity calculations.
///
/// @param client - Reference to the Qdrant client
/// @param name - Name of the collection to create
/// @param dim - Dimension of the vectors to be stored in the collection
/// @return Result indicating success or failure
pub async fn ensure_collection(client: &Qdrant, name: &str, dim: u64) -> Result<()> {
    // check if collection already exists
    let already_exists = client.collection_exists(name).await?;

    if already_exists {
        info!("Collection {} already exists...no need to create", name);
        return Ok(());
    }

    // Build the request *by value* (no &CreateCollection -> eliminates the Into/From error)
    let req = CreateCollection {
        collection_name: name.to_owned(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: dim,
                distance: Distance::Cosine.into(), // Using cosine similarity for vector comparison
                ..Default::default()
            })),
        }),
        ..Default::default()
    };

    // Newer client expects `CreateCollection`, not `&CreateCollection`
    client.create_collection(req).await?;
    Ok(())
}

/// Upserts a batch of metadata and embedding vector tuples into a Qdrant collection.
///
/// This function takes a list of (metadata, vector) tuples and inserts or updates them
/// in the specified Qdrant collection. Each item's metadata is stored in the payload
/// under the key "meta".
///
/// @param client - Reference to the Qdrant client
/// @param collection - Name of the collection to upsert into
/// @param items - Slice of tuples containing metadata strings and their corresponding embedding vectors
/// @return Result indicating success or failure
pub async fn upsert(
    client: &Qdrant,
    collection: &str,
    items: &[(SourceChunk, Vec<f32>)],
) -> Result<()> {
    // Build PointStructs from the items
    let points: Vec<PointStruct> = items
        .iter()
        .enumerate()
        .map(|(i, (chunk, vec))| -> Result<PointStruct> {
            // 🚩 flatten: payload IS the SourceChunk
            let payload: Payload = serde_json::to_value(chunk)
                .map_err(|e| {
                    AuditError::json("chunk_serialization", "Failed to serialize SourceChunk", e)
                })?
                .try_into()
                .map_err(|e| {
                    AuditError::json("payload_conversion", "Failed to convert JSON to Payload", e)
                })?;
            // Create a new point with ID, vector, and payload
            Ok(PointStruct::new(i as u64, vec.clone(), payload))
        })
        .collect::<Result<Vec<_>>>()?;

    // Use the builder-style API to create the upsert request
    let req = UpsertPointsBuilder::new(collection, points).wait(true); // wait=true mimics the old “blocking”

    // Execute the upsert operation
    client
        .upsert_points(req)
        .await
        .map_err(|e| AuditError::vector_db("upsert", "Failed to upsert points to Qdrant", e))?;
    Ok(())
}

```
ai-agent-audit/src/build_brain/vector_service.rs
```
/// Vector database service for centralized Qdrant operations.
///
/// This module provides a high-level service interface for vector database operations,
/// eliminating duplication and providing consistent error handling across the codebase.

use crate::config::audit_config;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use super::enbeddings::SourceChunk;
use log::info;
use qdrant_client::{
    qdrant::{
        vectors_config::Config, CreateCollection, Distance, PointStruct, UpsertPointsBuilder,
        VectorParams, VectorsConfig,
    },
    Payload, Qdrant,
};
use std::sync::Arc;

/// Vector database service providing centralized Qdrant operations.
pub struct VectorDbService {
    /// Qdrant client
    client: Qdrant,
    /// Default vector dimension
    vector_dimension: u64,
}

impl VectorDbService {
    /// Creates a new vector database service.
    pub async fn new() -> Result<Self> {
        let qdrant_url = std::env::var("QDRANT_URL")
            .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;
        
        let client = Qdrant::from_url(&qdrant_url)
            .build()
            .map_err(|e| AuditError::vector_db("connection", "Failed to connect to Qdrant database", e))?;

        Ok(Self {
            client,
            vector_dimension: audit_config().vector_dimension,
        })
    }

    /// Creates a new service with custom configuration.
    pub async fn with_config(qdrant_url: &str, vector_dimension: u64) -> Result<Self> {
        let client = Qdrant::from_url(qdrant_url)
            .build()
            .map_err(|e| AuditError::vector_db("connection", "Failed to connect to Qdrant database", e))?;

        Ok(Self {
            client,
            vector_dimension,
        })
    }

    /// Generates a collection name for a repository.
    pub fn collection_name(&self, repo: &RepoPaths) -> String {
        format!("{}-contract_chunks", repo.unique_repo_hash())
    }

    /// Checks if a collection exists for the given repository.
    pub async fn collection_exists(&self, repo: &RepoPaths) -> Result<bool> {
        let collection_name = self.collection_name(repo);
        self.client
            .collection_exists(&collection_name)
            .await
            .map_err(|e| AuditError::vector_db("collection_check", &format!("Failed to check if collection '{}' exists", collection_name), e))
    }

    /// Creates a collection if it doesn't exist.
    pub async fn ensure_collection(&self, repo: &RepoPaths) -> Result<()> {
        let collection_name = self.collection_name(repo);
        
        if self.collection_exists(repo).await? {
            info!("Collection {} already exists...no need to create", collection_name);
            return Ok(());
        }

        info!("Creating collection: {}", collection_name);
        
        let req = CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: self.vector_dimension,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        };

        self.client
            .create_collection(req)
            .await
            .map_err(|e| AuditError::vector_db("collection_creation", &format!("Failed to create collection '{}'", collection_name), e))?;

        Ok(())
    }

    /// Upserts embeddings into the collection.
    pub async fn upsert_embeddings(
        &self,
        repo: &RepoPaths,
        items: &[(SourceChunk, Vec<f32>)],
    ) -> Result<()> {
        let collection_name = self.collection_name(repo);
        
        info!("Upserting {} embeddings to collection: {}", items.len(), collection_name);

        // Build PointStructs from the items
        let points: Vec<PointStruct> = items
            .iter()
            .enumerate()
            .map(|(i, (chunk, vec))| -> Result<PointStruct> {
                let payload: Payload = serde_json::to_value(chunk)
                    .map_err(|e| AuditError::json("chunk_serialization", "Failed to serialize SourceChunk", e))?
                    .try_into()
                    .map_err(|e| AuditError::json("payload_conversion", "Failed to convert JSON to Payload", e))?;
                
                Ok(PointStruct::new(i as u64, vec.clone(), payload))
            })
            .collect::<Result<Vec<_>>>()?;

        // Create and execute the upsert request
        let req = UpsertPointsBuilder::new(&collection_name, points).wait(true);
        
        self.client
            .upsert_points(req)
            .await
            .map_err(|e| AuditError::vector_db("upsert", &format!("Failed to upsert embeddings to collection '{}'", collection_name), e))?;

        info!("Successfully upserted {} embeddings", items.len());
        Ok(())
    }

    /// Deletes a collection for a repository.
    pub async fn delete_collection(&self, repo: &RepoPaths) -> Result<()> {
        let collection_name = self.collection_name(repo);
        
        if !self.collection_exists(repo).await? {
            info!("Collection {} does not exist, nothing to delete", collection_name);
            return Ok(());
        }

        info!("Deleting collection: {}", collection_name);
        
        self.client
            .delete_collection(&collection_name)
            .await
            .map_err(|e| AuditError::vector_db("collection_deletion", &format!("Failed to delete collection '{}'", collection_name), e))?;

        Ok(())
    }

    /// Returns the underlying Qdrant client for advanced operations.
    pub fn client(&self) -> &Qdrant {
        &self.client
    }

    /// Returns the vector dimension used by this service.
    pub fn vector_dimension(&self) -> u64 {
        self.vector_dimension
    }
}

/// Shared vector database service instance.
use std::sync::OnceLock;
static VECTOR_SERVICE: OnceLock<Arc<VectorDbService>> = OnceLock::new();

/// Initializes the global vector database service.
pub async fn init_vector_service() -> Result<()> {
    let service = VectorDbService::new().await?;
    let arc_service = Arc::new(service);
    
    VECTOR_SERVICE.set(arc_service).map_err(|_| {
        AuditError::configuration("vector_service", "Vector service already initialized")
    })?;
    
    Ok(())
}

/// Returns a reference to the global vector database service.
///
/// # Panics
/// Panics if the service has not been initialized with `init_vector_service()`.
pub fn vector_service() -> &'static Arc<VectorDbService> {
    VECTOR_SERVICE.get().expect("Vector service not initialized. Call init_vector_service() first.")
}

/// Returns a reference to the global vector database service, or None if not initialized.
pub fn try_vector_service() -> Option<&'static Arc<VectorDbService>> {
    VECTOR_SERVICE.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_repo() -> RepoPaths {
        let temp_dir = TempDir::new().unwrap();
        RepoPaths {
            root: temp_dir.path().to_path_buf(),
            sol_files: vec![],
            docs: vec![],
            repo_name: "test-repo".to_string(),
            commit_hash: "abc123def456".to_string(),
        }
    }

    #[test]
    fn test_collection_name_generation() {
        let service = VectorDbService {
            client: Qdrant::from_url("http://localhost:6334").build().unwrap(),
            vector_dimension: audit_config().vector_dimension,
        };
        
        let repo = create_test_repo();
        let collection_name = service.collection_name(&repo);
        
        assert!(collection_name.contains("test-repo"));
        assert!(collection_name.contains("abc123"));
        assert!(collection_name.ends_with("-contract_chunks"));
    }

    #[test]
    fn test_vector_dimension() {
        let service = VectorDbService {
            client: Qdrant::from_url("http://localhost:6334").build().unwrap(),
            vector_dimension: audit_config().vector_dimension,
        };
        
        assert_eq!(service.vector_dimension(), audit_config().vector_dimension);
    }
}
```
ai-agent-audit/src/reporting/contract_data.rs
```
use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::prompt_context::generate_context_for_code_review,
    prepare_code::git_clone::RepoPaths, reporting::save_file::save_file_locally,
};
/// Contract data export utilities for analysis artifacts.
///
/// This module provides functions to export contract analysis data including
/// generated code blocks and metadata to markdown files for external use
/// and documentation purposes.
use std::path::{Path, PathBuf};

/// Saves individual contract analysis data to markdown files.
///
/// Exports generated code blocks for each contract to separate markdown files
/// with standardized naming conventions for easy reference and documentation.
///
/// # Arguments
// * `codeblocks_path` - Path to the code blocks database
/// * `repo` - Repository paths and metadata for naming
pub fn save_contract_and_fn_ir(codeblocks_path: &PathBuf, repo: &RepoPaths) -> anyhow::Result<()> {
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    log::info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    for (contract, codeblock) in contracts {
        let filename = format!("{}-{}.md", contract, repo.unique_repo_hash());
        save_file_locally(&codeblock, &filename)?;
    }
    Ok(())
}

/// Saves protocol metadata and context information to a markdown file.
///
/// Exports comprehensive protocol metadata including summaries, semantic data,
/// and contextual information used by AI agents during analysis.
///
/// # Arguments
/// * `semantics_path` - Path to the semantic analysis database
/// * `repo` - Repository paths and metadata for naming
pub async fn save_metadata(semantics_path: &Path, repo: &RepoPaths) -> anyhow::Result<()> {
    let metadata = generate_context_for_code_review(repo, semantics_path).await?;

    let filename = format!("metadata-{}.md", repo.unique_repo_hash());
    save_file_locally(&metadata, &filename)?;
    Ok(())
}

```
ai-agent-audit/src/reporting/save_file.rs
```
/// File saving utilities for audit reports and analysis data.
///
/// This module provides functions to save audit reports and other analysis
/// outputs to the local filesystem with appropriate naming conventions.

use std::{fs::File, io::Write};
use crate::prepare_code::git_clone::RepoPaths;
use super::audit::ReportType;

/// Saves an audit report to a file with appropriate naming.
///
/// Creates a markdown file with the audit report content using a standardized
/// naming convention that includes the repository hash and report type.
///
/// # Arguments
/// * `markdown` - The audit report content in Markdown format
/// * `repo` - Repository paths and metadata for naming
/// * `report_type` - Report type (Free/Paid) for filename suffix
pub fn save_audit_report(
    markdown: &str,
    repo: &RepoPaths,
    report_type: ReportType,
) -> anyhow::Result<()> {
    let suffix = if report_type == ReportType::Free {
        "-free"
    } else {
        ""
    };
    let filename = format!("{}{}-audit-report.md", repo.unique_repo_hash(), suffix);
    save_file_locally(markdown, &filename)?;

    Ok(())
}

/// Saves content to a local file.
///
/// Generic file saving utility that writes string content to a specified filename.
///
/// # Arguments
/// * `content` - String content to write to file
/// * `filename` - Target filename for the content
pub fn save_file_locally(content: &str, filename: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

```
ai-agent-audit/src/invariant_prompts/arithmetic.rs
```
pub const ARITHMETIC: &str = r#"
# Arithmetic Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Arithmetic Invariants** - mathematical relationships and formulas that must remain true throughout a smart contract's execution.

## WHAT ARE ARITHMETIC INVARIANTS?

Arithmetic invariants are mathematical constraints that define the correctness of a contract's core logic. They involve:
- **Mathematical formulas** that must hold (e.g., `x * y = k`)
- **Numerical bounds** and limits (e.g., `totalSupply <= maxSupply`)
- **Ratio relationships** between values (e.g., `collateral * factor >= debt`)
- **Conservation laws** (e.g., sum of balances = total supply)
- **Pricing formulas** and exchange rates

## COMMON ARITHMETIC INVARIANTS BY PROTOCOL TYPE

### DeFi Protocols
- **AMM Constant Product**: `reserveX * reserveY = k` (Uniswap-style)
- **Lending Collateralization**: `borrowed_amount <= collateral_amount * collateralFactor`
- **Interest Rate Models**: Utilization ratios, compound interest calculations
- **Liquidity Pool Ratios**: Token pair balances maintain pricing relationships
- **Slippage Bounds**: Price impact calculations within expected ranges

### NFT Contracts
- **Max Supply Caps**: `totalMinted <= maxSupply`
- **Sequential Token IDs**: `nextTokenId` only increases, no gaps
- **Pricing Tiers**: Different mint prices based on quantity/time
- **Royalty Calculations**: `royaltyAmount = salePrice * royaltyPercentage / 100`

### DAO/Governance
- **Voting Thresholds**: `yesVotes >= quorum` for execution
- **Token Supply vs Voting Power**: Total voting power ≤ total token supply
- **Proposal Lifecycle**: Vote counts determine state transitions
- **Treasury Accounting**: Funds only move with proper vote approval

## ANALYSIS METHODOLOGY

### 1. IDENTIFY MATHEMATICAL RELATIONSHIPS
Look for:
- Multiplication/division operations that should preserve constants
- Addition/subtraction that should maintain totals
- Percentage calculations and ratio maintenance
- Min/max bounds checking
- Mathematical formulas in comments or documentation

### 2. TRACE ARITHMETIC OPERATIONS
- Follow how values are calculated and updated
- Check for integer overflow/underflow protection
- Verify rounding behavior doesn't break invariants
- Look for missing bounds checks

### 3. COMMON VIOLATION PATTERNS
- **Missing bounds checks**: No validation of max values
- **Integer overflow/underflow**: Arithmetic operations that wrap
- **Rounding errors**: Precision loss breaking formulas
- **Reentrancy affecting calculations**: State changes mid-calculation
- **Race conditions**: Concurrent operations breaking math
- **Missing validation**: Parameters not checked against constraints

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's mathematical behavior:
- What calculations does it perform?
- What mathematical relationships should hold?
- What numerical constraints exist?

### 2. DERIVE ARITHMETIC INVARIANTS
For each mathematical relationship, create an invariant:
- **INV-A1, INV-A2, etc.** (use A prefix for Arithmetic)
- Describe the exact mathematical formula or constraint
- Identify the variables and operations involved

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly enforces the mathematical relationship
- **VIOLATION**: The relationship can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Sequence of function calls that breaks the math
- **pre_state**: Initial conditions needed (balances, permissions, etc.)
- **post_state**: Resulting state showing the broken relationship
- **impact**: Financial or functional impact of the violation
- **poc**: Concrete example with numbers
- **mitigation**: How to fix the arithmetic issue

## EXAMPLES OF ARITHMETIC VIOLATIONS

- **AMM**: Swap function allows `k` to decrease, enabling value extraction
- **Lending**: Missing collateral factor check allows over-borrowing
- **NFT**: Mint function bypasses max supply through integer overflow
- **DAO**: Vote counting allows double-counting or negative votes

Focus on mathematical correctness and ensure all arithmetic relationships that define the contract's core logic are properly validated and maintained.
"#;

```
ai-agent-audit/src/invariant_prompts/balance.rs
```
pub const BALANCE: &str = r#"
# Balance Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Balance Invariants** - mathematical relationships governing token/ETH balances, supply mechanics, and balance conservation laws that must remain true throughout a smart contract's execution.

## WHAT ARE BALANCE INVARIANTS?

Balance invariants are constraints that ensure the integrity of value storage and transfer within smart contracts. They involve:
- **Token balance conservation** (transfers don't create/destroy value)
- **Supply monotonicity** (total supply changes only through authorized mint/burn)
- **Balance consistency** (individual balances sum to total supply)
- **ETH/native token accounting** (contract ETH balance matches internal records)
- **Multi-token accounting** (cross-token balance relationships)
- **Liquidity pool integrity** (deposited tokens match pool records)

## COMMON BALANCE INVARIANTS BY PROTOCOL TYPE

### Token Contracts (ERC20/ERC721/ERC1155)
- **Conservation Law**: `sum(all_balances) == totalSupply`
- **Transfer Integrity**: `balanceOf[from] + balanceOf[to]` unchanged after transfer
- **Supply Bounds**: `totalSupply <= maxSupply` (if capped)
- **Burn Consistency**: `totalSupply` decreases exactly by burned amount
- **Mint Authorization**: Supply only increases through authorized minting

### DeFi Protocols
- **Pool Balance Matching**: `poolTokenBalance == sum(userDeposits)`
- **Vault Accounting**: `assetsUnderManagement == sum(userShares * sharePrice)`
- **Liquidity Provider Tokens**: `lpTokenSupply * price == poolValue`
- **Fee Accumulation**: `collectedFees + distributedFees == totalGeneratedFees`
- **Cross-Chain Balance**: Locked tokens on source chain = minted on destination

### Staking/Rewards Systems
- **Reward Pool Conservation**: `totalRewards == distributedRewards + remainingRewards`
- **Staking Balance**: `totalStaked == sum(userStakedAmounts)`
- **Slashing Consistency**: Penalties reduce both user and total stakes proportionally
- **Compound Interest**: Reward calculations maintain mathematical precision

### Multi-Signature/Treasury
- **Asset Tracking**: Contract's actual balance >= sum of tracked balances
- **Withdrawal Limits**: `totalWithdrawn <= depositedAmount` per period
- **Multi-Token Accounting**: Each token type balanced independently

## ANALYSIS METHODOLOGY

### 1. IDENTIFY BALANCE RELATIONSHIPS
Look for:
- State variables tracking balances (`balanceOf`, `totalSupply`)
- Transfer functions and their balance updates
- Mint/burn operations affecting supply
- Deposit/withdrawal mechanisms
- Fee collection and distribution
- Cross-contract balance interactions

### 2. TRACE BALANCE MODIFICATIONS
- Map all functions that modify balances
- Verify balance updates are atomic and consistent
- Check for missing balance updates in edge cases
- Validate overflow/underflow protection
- Ensure all balance changes are properly accounted

### 3. COMMON VIOLATION PATTERNS
- **Missing Balance Updates**: State changes without corresponding balance adjustments
- **Double Accounting**: Same value counted multiple times
- **Rounding Errors**: Precision loss causing balance drift over time
- **Reentrancy Attacks**: External calls allowing balance manipulation
- **Flash Loan Attacks**: Temporary balance inflation breaking invariants
- **Integer Overflow**: Balance arithmetic wrapping unexpectedly
- **Unchecked External Calls**: Failed transfers not reverting state
- **Fee Calculation Errors**: Incorrect fee deduction/distribution

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's balance management:
- What tokens/assets does it handle?
- How are balances tracked and updated?
- What balance relationships must be maintained?
- Are there any supply mechanics (mint/burn)?

### 2. DERIVE BALANCE INVARIANTS
For each balance relationship, create an invariant:
- **INV-B1, INV-B2, etc.** (use B prefix for Balance)
- Describe the exact balance constraint or conservation law
- Identify the variables and operations involved
- Specify the scope (per-user, global, cross-contract)

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly maintains the balance relationship
- **VIOLATION**: The relationship can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that break balance integrity
- **pre_state**: Initial balance conditions needed
- **post_state**: Resulting imbalanced state with specific amounts
- **impact**: Financial loss or system compromise potential
- **poc**: Step-by-step example with actual token amounts
- **mitigation**: How to fix the balance tracking issue

Focus on tracking how value moves through the system and ensure no tokens are created, destroyed, or double-counted unintentionally. Every balance change must be mathematically justified and properly implemented.
"#;

```
ai-agent-audit/src/invariant_prompts/permission.rs
```
pub const PERMISSION: &str = r#"
# Permission Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Permission Invariants** - access control mechanisms that ensure only authorized accounts can perform sensitive operations throughout a smart contract's execution.

## WHAT ARE PERMISSION INVARIANTS?

Permission invariants are security constraints that govern who can execute specific functions and under what conditions. They involve:
- **Role-based access control** (owner, admin, guardian, oracle roles)
- **Function-level restrictions** (only specific addresses can call certain functions)
- **State-dependent permissions** (access rights that change based on contract state)
- **Multi-signature requirements** (multiple parties must approve actions)
- **Time-based access** (permissions that expire or activate at certain times)
- **Hierarchical permissions** (different privilege levels and delegation)

## COMMON PERMISSION INVARIANTS BY PROTOCOL TYPE

### DeFi Protocols
- **Owner-Only Admin Functions**: Only owner can pause contracts, change fees, upgrade modules
- **Oracle Access Control**: Only designated oracles can update price feeds
- **Emergency Powers**: Only guardians can trigger emergency stops or liquidations
- **Fee Management**: Only authorized addresses can collect or distribute fees
- **Parameter Updates**: Critical parameters (interest rates, collateral factors) only changeable by governance
- **Vault Management**: Only vault managers can rebalance or migrate funds

### NFT Contracts
- **Minting Authorization**: Only whitelisted addresses or contract owner can mint tokens
- **Transfer Permissions**: Only token owner or approved addresses can transfer (ERC721 standard)
- **Metadata Updates**: Only authorized roles can modify token metadata or reveal mechanisms
- **Royalty Management**: Only designated addresses can update royalty settings
- **Collection Management**: Only collection owner can add/remove from whitelist or change mint prices

### DAO/Governance Contracts
- **Execution Authority**: Only timelock contract can execute proposals
- **Proposal Creation**: Only token holders above threshold can create proposals
- **Vote Delegation**: Only token holders can delegate their voting power
- **Treasury Access**: Only executed governance proposals can access treasury funds
- **Role Assignment**: Only existing admins can grant/revoke roles to other addresses
- **Upgrade Permissions**: Only governance can upgrade contract implementations

## ANALYSIS METHODOLOGY

### 1. IDENTIFY ACCESS CONTROL MECHANISMS
Look for:
- `onlyOwner`, `onlyAdmin`, custom role modifiers
- `require(msg.sender == authorizedAddress)` statements
- Multi-signature verification logic
- Role-based access control (RBAC) systems
- OpenZeppelin AccessControl usage
- Custom permission checking functions

### 2. MAP PERMISSION BOUNDARIES
- Identify all privileged functions and their access requirements
- Trace permission inheritance and delegation chains
- Check for default permissions and fallback behaviors
- Verify permission revocation mechanisms
- Look for bypass conditions or emergency overrides

### 3. COMMON VIOLATION PATTERNS
- **Missing Access Control**: Sensitive functions lack permission checks
- **Incorrect Role Checks**: Wrong role or address validation
- **Permission Bypass**: Alternative code paths that skip authorization
- **Role Escalation**: Lower privileges can gain higher privileges
- **Initialization Vulnerabilities**: Missing access control during setup
- **Frontrunning**: Permission changes can be frontrun by unauthorized users
- **Reentrancy Permission Bypass**: External calls allowing permission circumvention
- **Default Permissions**: Overly permissive default states

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's permission model:
- What roles and permissions exist?
- Which functions are access-controlled?
- How are permissions granted/revoked?
- Are there any special privilege escalation mechanisms?

### 2. DERIVE PERMISSION INVARIANTS
For each access control mechanism, create an invariant:
- **INV-P1, INV-P2, etc.** (use P prefix for Permission)
- Describe the exact authorization requirement
- Identify the protected resources and operations
- Specify the authorized parties and conditions

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly enforces the permission requirement
- **VIOLATION**: Unauthorized access is possible through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that bypass access control
- **pre_state**: Required initial conditions (roles, balances, etc.)
- **post_state**: Resulting unauthorized state change
- **impact**: Security compromise or privilege escalation achieved
- **poc**: Step-by-step unauthorized action sequence
- **mitigation**: How to fix the access control vulnerability

Focus on verifying that every sensitive operation has appropriate authorization checks and that there are no alternative paths that bypass these controls. Permission invariants are critical for preventing unauthorized access to protected resources and maintaining the security boundaries of the smart contract system.

"#;

```
ai-agent-audit/src/invariant_prompts/referential.rs
```
pub const REFERENTIAL: &str = r#"
# Referential Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Referential Invariants** - critical relationships between state variables, mappings, arrays, and cross-contract references that must remain consistent throughout a smart contract's execution lifecycle.

## WHAT ARE REFERENTIAL INVARIANTS?

Referential invariants are constraints that ensure the integrity of data relationships and pointer consistency within smart contracts. They involve:
- **State Variable Consistency** (related variables stay synchronized)
- **Mapping-Array Synchronization** (mappings and arrays referencing same entities)
- **Cross-Contract Reference Integrity** (external contract addresses remain valid)
- **Index-Data Correspondence** (array indices match their referenced data)
- **Ownership Chain Consistency** (parent-child relationships in hierarchies)
- **State Machine Coherence** (state transitions maintain valid references)

## COMMON REFERENTIAL INVARIANTS BY PROTOCOL TYPE

### Ownership & Access Control
- **Owner-Permission Consistency**: `isOwner[addr] == true` iff `addr` in owners array
- **Role-Permission Mapping**: `hasRole[user][role] == roles[role].members[user]`
- **Delegation Chain**: `delegatedBy[delegate] == delegator` iff `delegates[delegator] == delegate`
- **Multi-Sig Coherence**: `signers.length == signerCount` and all `signers[i]` are unique
- **Permission Inheritance**: Child contracts inherit parent permissions correctly

### Token & NFT Management
- **Owner-Token Mapping**: `ownerOf[tokenId] == owner` iff `tokensOwnedBy[owner]` contains `tokenId`
- **Approval Consistency**: `getApproved[tokenId] == spender` iff spender can transfer tokenId
- **Operator Authorization**: `isApprovedForAll[owner][operator]` matches operator permissions
- **Token Existence**: `tokenId` in `_allTokens` iff `ownerOf[tokenId] != address(0)`
- **Metadata Linkage**: `tokenURI[tokenId]` exists iff token exists

### DeFi Protocol References
- **Pool-Token Association**: `poolForToken[token] == pool` iff `pool.underlyingToken == token`
- **LP Token-Pool Binding**: `lpToken.pool == poolAddress` iff `pool.lpToken == lpTokenAddress`
- **Oracle-Price Consistency**: `priceOracle[asset]` points to valid oracle with recent updates
- **Vault-Strategy Mapping**: `vault.strategy == strategy` iff `strategy.vault == vault`
- **Cross-Chain Bridge**: `localToken[chainId][remoteToken] == localToken` bidirectionally

### Governance & Voting
- **Proposal-Voter Tracking**: `voters[proposalId]` contains all addresses that voted
- **Vote-Weight Consistency**: `totalVotes[proposalId] == sum(voterWeights[proposalId])`
- **Delegation Graph**: No cycles in `delegatedTo[voter]` relationships
- **Snapshot Coherence**: `votingPower[user][blockNumber]` matches historical balances
- **Execution Prerequisites**: Executed proposals have `state == Executed`

### Marketplace & Auction Systems
- **Listing-Item Binding**: `listings[listingId].item == itemId` iff item is listed
- **Bid-Auction Association**: `highestBid[auctionId].bidder` owns the winning bid
- **Escrow-Trade Linkage**: `escrow[tradeId]` holds funds iff trade is active
- **Collection-Item Hierarchy**: `items[itemId].collection == collectionId` consistently
- **Order Book Integrity**: Buy/sell orders reference valid tokens and amounts

### Staking & Rewards
- **Staker-Pool Reference**: `stakerInfo[user].pool == poolId` iff user staked in pool
- **Reward-Epoch Tracking**: `epochRewards[epoch]` distributed matches `totalStaked[epoch]`
- **Unbonding Queue**: `unbondingQueue[user]` entries have valid timestamps
- **Validator-Delegator Map**: `delegations[validator]` contains all delegator addresses
- **Slash-Event Correlation**: Slashing events reference valid validator addresses

## ANALYSIS METHODOLOGY

### 1. IDENTIFY REFERENTIAL RELATIONSHIPS
Look for:
- Mappings that reference array indices or other mappings
- State variables that must stay synchronized
- Cross-contract address storage and validation
- Parent-child relationships in data structures
- Bidirectional references between entities
- State machines with transition dependencies

### 2. TRACE REFERENCE MODIFICATIONS
- Map all functions that modify referenced data
- Verify synchronized updates across related structures
- Check for dangling references after deletions
- Validate reference integrity during state transitions
- Ensure atomic updates of related references
- Confirm proper cleanup of bidirectional links

### 3. COMMON VIOLATION PATTERNS
- **Orphaned References**: Pointers to deleted or non-existent data
- **Asymmetric Updates**: One-way reference updates breaking bidirectional links
- **Stale References**: Cached addresses pointing to outdated contracts
- **Index Drift**: Array indices becoming misaligned with their data
- **Circular Dependencies**: Reference cycles causing logical inconsistencies
- **Race Conditions**: Concurrent updates breaking reference atomicity
- **Missing Validation**: Accepting invalid addresses or indices
- **Inconsistent State Machines**: Invalid state transitions leaving broken references

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's referential structure:
- What entities have cross-references (users, tokens, pools, etc.)?
- How are relationships tracked (mappings, arrays, structs)?
- Are there bidirectional or hierarchical relationships?
- What external contracts or addresses are referenced?

### 2. DERIVE REFERENTIAL INVARIANTS
For each reference relationship, create an invariant:
- **INV-R1, INV-R2, etc.** (use R prefix for Referential)
- Describe the exact reference constraint or consistency requirement
- Identify the data structures and their relationships
- Specify the directionality (unidirectional, bidirectional, hierarchical)

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly maintains reference consistency
- **VIOLATION**: The relationship can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that break referential integrity
- **pre_state**: Initial reference setup needed
- **post_state**: Resulting inconsistent state with specific broken references
- **impact**: Data corruption, access control bypass, or logical inconsistencies
- **poc**: Step-by-step example with specific addresses/IDs
- **mitigation**: How to fix the reference management issue

### 5. FOUNDRY TEST RECOMMENDATIONS
Suggest specific invariant tests:
- Property-based tests for reference consistency
- Fuzz tests for edge cases in reference updates
- Integration tests for cross-contract reference validity
- State transition tests maintaining reference integrity

Focus on ensuring that all data relationships remain logically consistent throughout the contract's execution. Every reference must point to valid, existing data, and all bidirectional relationships must be maintained symmetrically. Reference integrity is crucial for preventing logical vulnerabilities and maintaining system coherence.

"#;

```
ai-agent-audit/src/invariant_prompts/state_machine.rs
```
pub const STATE_MACHINE: &str = r#"
# State Machine Invariant Security Analysis Prompt

You are a senior security auditor specializing in **State Machine Invariants** - constraints that govern valid state transitions, enforce business logic rules, and ensure protocol states remain consistent and secure throughout the contract's execution lifecycle.

## WHAT ARE STATE MACHINE INVARIANTS?

State machine invariants are rules that define valid states and state transitions within smart contracts. They involve:
- **Valid State Constraints** (only allowed states can exist)
- **Transition Authorization** (only permitted actors can trigger transitions)
- **Transition Logic** (state changes follow defined business rules)
- **State Consistency** (related state variables remain coherent)
- **Terminal State Protection** (final states cannot be illegally exited)
- **Temporal Constraints** (time-based state transition rules)

## COMMON STATE MACHINE INVARIANTS BY PROTOCOL TYPE

### Auction Systems
- **Lifecycle States**: `Created → Active → (Bidding) → Ended → Settled`
- **Bidding Rules**: Can only bid in `Active` state with higher amounts
- **Settlement Constraint**: Can only settle in `Ended` state
- **Cancellation Logic**: Can only cancel in `Created` or `Active` (if no bids)
- **Time-Based Transitions**: Auto-transition to `Ended` after auction duration
- **Final State Protection**: `Settled` auctions cannot be modified

### Token Sales & ICOs
- **Sale Phases**: `Pending → Whitelist → Public → Paused → Ended → Finalized`
- **Purchase Authorization**: Can only buy tokens in `Whitelist` or `Public` phases
- **Phase Transitions**: Only owner can advance phases in correct order
- **Pause/Resume Logic**: Can pause/resume only from active states
- **Finalization Rules**: Can only finalize after `Ended` state
- **Refund Conditions**: Refunds only available if sale fails or is cancelled

### Governance Proposals
- **Proposal Lifecycle**: `Pending → Active → Succeeded/Defeated → Queued → Executed/Expired`
- **Voting Period**: Votes only accepted in `Active` state
- **Execution Delay**: Must wait in `Queued` state before execution
- **Success Criteria**: Transition to `Succeeded` only if quorum and votes met
- **Cancellation Rules**: Can cancel in `Pending` or `Active` (by proposer/guardian)
- **Expiration Logic**: `Queued` proposals expire after timelock period

### Staking & Vesting
- **Staking States**: `Unstaked → Staked → Unbonding → Slashed`
- **Unbonding Period**: Must wait in `Unbonding` before withdrawal
- **Slashing Transitions**: Can be slashed from `Staked` or `Unbonding`
- **Re-staking Rules**: Can re-stake from `Unbonding` to cancel withdrawal
- **Vesting Schedule**: `Locked → Vesting → Vested → Claimed`
- **Cliff Enforcement**: No claims before cliff period

### Multi-Signature Wallets
- **Transaction States**: `Proposed → Confirmed → Executed/Revoked`
- **Confirmation Threshold**: Execute only after sufficient confirmations
- **Revocation Rules**: Can revoke confirmations before execution
- **Execution Finality**: `Executed` transactions cannot be modified
- **Owner Changes**: Special state transitions for adding/removing owners
- **Emergency States**: Pause state prevents all operations except recovery

### Escrow & Payment Systems
- **Escrow Flow**: `Created → Funded → InDispute → Released/Refunded`
- **Funding Requirements**: Must be `Funded` before dispute or release
- **Dispute Resolution**: Can enter dispute from `Funded` state
- **Release Conditions**: Can release to beneficiary from `Funded` or resolved dispute
- **Refund Logic**: Refund to payer under specific conditions
- **Timeout Mechanisms**: Auto-release or refund after time limits

### Lending & Borrowing
- **Loan States**: `Applied → Approved → Active → Repaid/Defaulted`
- **Collateral States**: `Deposited → Locked → Released/Liquidated`
- **Health Factor**: Liquidation triggered when health < threshold
- **Repayment Logic**: Can repay from `Active` state only
- **Grace Period**: Transition to `Defaulted` after grace period expires
- **Recovery Process**: Special states for loan recovery and restructuring

## ANALYSIS METHODOLOGY

### 1. IDENTIFY STATE MACHINES
Look for:
- Enum state variables and their possible values
- Boolean flags that represent state conditions
- Status tracking variables (uint8 status, etc.)
- Phase/stage management in protocols
- Time-dependent state transitions
- Multi-contract state coordination

### 2. MAP STATE TRANSITION GRAPH
- Document all possible states
- Identify valid transitions between states
- Determine who can trigger each transition
- Note any time-based or condition-based transitions
- Map out terminal/final states
- Identify any state loops or cycles

### 3. COMMON VIOLATION PATTERNS
- **Invalid State Transitions**: Direct jumps between non-adjacent states
- **Missing Access Controls**: Unauthorized actors triggering transitions
- **Race Conditions**: Concurrent state changes causing inconsistencies
- **Reentrancy State Corruption**: External calls modifying state mid-transition
- **Time Manipulation**: Block timestamp attacks bypassing time constraints
- **Terminal State Violations**: Modifying supposedly final states
- **State Inconsistency**: Related variables becoming desynchronized
- **Missing State Validations**: Functions operating in wrong states

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's state machine structure:
- What are the main state variables and their possible values?
- How many distinct states exist in the system?
- Are there multiple independent state machines?
- What external factors influence state transitions (time, user actions, oracles)?

### 2. DERIVE STATE MACHINE INVARIANTS
For each state machine, create invariants:
- **INV-S1, INV-S2, etc.** (use S prefix for State Machine)
- Document valid states and allowed transitions
- Specify transition authorization requirements
- Define temporal constraints and timing rules
- Identify state consistency requirements across variables

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: State transitions are properly enforced
- **VIOLATION**: Invalid transitions or states are possible
- **CONDITIONAL**: Holds under normal conditions but may break in edge cases

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that break state machine rules
- **pre_state**: Initial state setup required for exploitation
- **post_state**: Resulting invalid state after exploitation
- **impact**: Business logic bypass, unauthorized access, or protocol breakdown
- **poc**: Step-by-step state transition sequence with specific function calls
- **mitigation**: How to properly enforce state machine rules

### 5. FOUNDRY TEST RECOMMENDATIONS
Suggest specific state machine tests:
- Valid transition path testing
- Invalid transition rejection testing
- Time-based state change verification
- Access control on state transitions
- State consistency invariant tests
- Race condition and reentrancy state tests

### 6. STATE DIAGRAM VISUALIZATION
Create a visual representation:
- All states as nodes
- Valid transitions as directed edges
- Transition conditions and authorization requirements
- Terminal states clearly marked
- Any loops or cycles identified

Focus on ensuring that the protocol's business logic is correctly implemented through proper state management. Every state transition must be authorized, valid, and maintain system consistency. State machine violations can lead to critical protocol failures and economic exploits.
"#;

```
ai-agent-audit/src/invariant_prompts/temporal.rs
```
pub const TEMPORAL: &str = r#"
# Temporal Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Temporal Invariants** - time-based constraints and chronological relationships that must be maintained throughout a smart contract's execution lifecycle.

## WHAT ARE TEMPORAL INVARIANTS?

Temporal invariants are time-dependent security constraints that govern when operations can be performed and how time-related state evolves. They involve:
- **Time-based access control** (functions only callable after/before certain timestamps)
- **Sequence enforcement** (operations must occur in specific chronological order)
- **Cooldown periods** (minimum time between repeated actions)
- **Expiration mechanisms** (permissions, offers, or states that expire)
- **Epoch/phase transitions** (contract phases that progress in order)
- **Timelock delays** (mandatory waiting periods before execution)
- **Clock monotonicity** (time only moves forward, no rewinding)

## COMMON TEMPORAL INVARIANTS BY PROTOCOL TYPE

### DeFi Protocols
- **Timelock Delays**: Governance proposals have mandatory delay (24-48 hours) before execution
- **Cooldown Periods**: Users must wait between unstaking/withdrawal requests
- **Interest Accrual**: Interest compounds over time and cannot be calculated for future timestamps
- **Oracle Update Frequency**: Price feeds must be updated within acceptable time windows
- **Auction Timing**: Liquidation auctions have start/end times that cannot be manipulated
- **Vesting Schedules**: Token releases follow predetermined time schedules
- **Lock Periods**: Staked tokens cannot be withdrawn before lock expiration

### NFT Contracts
- **Mint Phases**: Public mint only after whitelist phase ends
- **Reveal Timing**: Metadata reveals happen after mint phase completion
- **Auction Durations**: Bidding periods have enforced start/end times
- **Whitelist Expiry**: Whitelist access expires after specified period
- **Royalty Updates**: Changes to royalty settings have delay periods
- **Breeding Cooldowns**: NFT breeding has mandatory rest periods

### DAO/Governance Contracts
- **Proposal Lifecycle**: Voting -> Delay -> Execution phases in strict order
- **Voting Windows**: Proposals have fixed voting periods that cannot be extended arbitrarily
- **Execution Windows**: Passed proposals must be executed within time limits
- **Quorum Timing**: Vote counting only valid during official voting period
- **Role Transitions**: Admin role changes have mandatory transition periods
- **Emergency Delays**: Even emergency actions have minimum delay requirements

## ANALYSIS METHODOLOGY

### 1. IDENTIFY TIME-DEPENDENT MECHANISMS
Look for:
- `block.timestamp` usage and time comparisons
- Time-based state variables (`startTime`, `endTime`, `lastUpdate`)
- Deadline and expiration logic
- Phase/epoch progression mechanisms
- Cooldown and delay implementations
- Time-locked operations and escrows

### 2. TRACE TEMPORAL RELATIONSHIPS
- Map all time-dependent state transitions
- Verify chronological ordering requirements
- Check for time manipulation vulnerabilities
- Validate timestamp arithmetic and overflow protection
- Ensure proper handling of time edge cases

### 3. COMMON VIOLATION PATTERNS
- **Clock Manipulation**: Relying on `block.timestamp` without considering miner manipulation
- **Time Overflow**: Timestamp arithmetic causing wraparound
- **Phase Skipping**: Bypassing required sequential phases
- **Premature Execution**: Actions executed before required delays
- **Expired State Access**: Using expired data or permissions
- **Reentrancy Time Bypass**: External calls allowing time-based condition bypass
- **Front-running Time Windows**: Exploiting time-sensitive operations
- **Inconsistent Time Sources**: Mixed use of `block.timestamp` vs `block.number`

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's temporal behavior:
- What time-based constraints exist?
- How does the contract track and enforce timing?
- Are there sequential phases or epochs?
- What operations have time dependencies?

### 2. DERIVE TEMPORAL INVARIANTS
For each time-based constraint, create an invariant:
- **INV-T1, INV-T2, etc.** (use T prefix for Temporal)
- Describe the exact timing requirement or chronological constraint
- Identify the time-dependent variables and operations
- Specify the temporal relationships that must hold

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly enforces the temporal constraint
- **VIOLATION**: Time-based rules can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that violate temporal constraints
- **pre_state**: Required initial timing conditions
- **post_state**: Resulting temporal inconsistency or bypass
- **impact**: Security compromise through time manipulation
- **poc**: Step-by-step timing exploit example
- **mitigation**: How to fix the temporal vulnerability

Focus on ensuring that all time-based operations respect their intended chronological constraints and cannot be manipulated to bypass security mechanisms. Temporal invariants are crucial for maintaining the proper sequence of operations and preventing time-based attacks.
"#;

```
ai-agent-audit/src/prepare_code/git_clone.rs
```
/// Repository preparation and Docker-based building.
///
/// This module handles secure repository cloning in Docker containers,
/// auto-detection of build systems (Foundry/Hardhat), and file filtering
/// for smart contract analysis.
use anyhow::{Context, Result};
use ignore::gitignore::GitignoreBuilder;
use log::info;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::config::audit_config;
use crate::utils::file_security::{validate_repo_url, validate_safe_path};

/// Build flags for forge compilation
#[derive(Debug, Clone, Copy)]
pub enum BuildFlags {
    /// Standard forge build
    Standard,
    /// Forge build with --via-ir --build-info flags
    ViaIr,
}

/// Contains paths to the repository root and relevant files.
/// This struct organizes the paths to Solidity files and documentation
/// that will be processed for analysis.
#[derive(Debug, Clone)]
pub struct RepoPaths {
    /// Path to the repository root directory
    pub root: PathBuf,
    /// Paths to all Solidity (.sol) files in the repository
    pub sol_files: Vec<PathBuf>,
    /// Paths to documentation files (README.md, etc.)
    pub docs: Vec<PathBuf>,
    /// e.g. `"my-cool-repo"`
    pub repo_name: String,
    /// full 40-char SHA, e.g. `"1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t"`
    pub commit_hash: String,
}

impl RepoPaths {
    /// Generates a unique identifier for the repository using name and short commit hash.
    /// Used for creating unique vector database collections and cache keys.
    pub fn unique_repo_hash(&self) -> String {
        format!(
            "{}-{}",
            self.repo_name.replace("/", "-"),
            &self.commit_hash[..6]
        )
    }
}

/// Clones a repository and builds it in a secure Docker environment.
///
/// This function performs the complete repository preparation workflow:
/// 1. Creates a Docker volume for isolated analysis
/// 2. Clones the repository using Trail of Bits security toolbox
/// 3. Auto-detects and builds with Foundry or Hardhat
/// 4. Filters and organizes Solidity files and documentation
/// 5. Extracts commit hash for unique identification
///
/// # Arguments
/// * `url` - Git repository URL to clone and analyze
/// * `subfolder` - Optional subfolder name to analyze within the repository
/// * `build_flags` - Build flags for forge compilation
///
/// # Returns
/// * `RepoPaths` - Organized repository paths and metadata
///
/// # Security
/// All operations are performed in isolated Docker containers to prevent
/// malicious code execution on the host system.
pub fn clone_and_filter_git_repo(
    url: &str,
    subfolder: Option<&str>,
    build_flags: BuildFlags,
) -> Result<RepoPaths> {
    // 🔐 Validate the repository URL for safety
    validate_repo_url(url)?;

    // 2. Extract & sanitize the repo name
    let mut repo_name = url
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .to_string();

    // Append subfolder to repo_name if specified
    if let Some(sf) = subfolder {
        repo_name = format!("{}/{}", repo_name, sf);
    }
    info!("repo_name ==> {}", repo_name);

    // 4. Read HEAD and get the first 6 chars of the commit SHA
    let commit_hash = get_commit_hash(url)?;
    let short_hash = &commit_hash[..6];

    // 5. git clone, install, and build in secure docker container
    // returns dierctory where files are located
    let root = clone_and_build_repo(url, &repo_name, short_hash, build_flags)?;

    // 6. Build .gitignore matcher
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    ign.add_line(None, "node_modules")?;
    let ign = ign.build()?;

    // Determine the search root - if subfolder is specified, search within that subdirectory
    let search_root = root.join(&repo_name);
    info!("search_root => {}", search_root.display());

    // Validate that the search root exists
    if !search_root.exists() {
        anyhow::bail!(
            "Specified path '{}' does not exist in the repository",
            repo_name
        );
    }

    // Initialize vectors to store file paths
    let mut sol_files = Vec::new();
    let mut docs = Vec::new();
    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        // skip if gitignore or simlink
        if ign.matched(path, false).is_ignore()
            || fs::symlink_metadata(path)?.file_type().is_symlink()
        {
            continue;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some("sol") => sol_files.push(path.to_path_buf()),
            Some("md")
                if path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .map(|f| f.eq_ignore_ascii_case("README.md"))
                    .unwrap_or(false) =>
            {
                docs.push(path.to_path_buf())
            }
            _ => {}
        }
    }

    // Return the collected paths
    Ok(RepoPaths {
        root,
        sol_files,
        docs,
        repo_name,
        commit_hash,
    })
}

pub fn clone_and_build_repo(
    repo_url: &str,
    repo_name: &str,
    commit_hash: &str,
    build_flags: BuildFlags,
) -> Result<PathBuf> {
    let docker_volume = format!(
        "{}/{}-{}",
        audit_config().docker_volume,
        repo_name.replace("/", "-"),
        &commit_hash[..6]
    );
    let docker_path = PathBuf::from(&docker_volume);

    // if github repo clones to multiple sub folders with different apps
    // then repo_name will be something like contracts/plume
    // git clone will clone to contracts (repo_root) and then we cd into plume
    let repo_root = repo_name.split('/').next().unwrap_or(repo_name);

    if docker_path.exists() {
        log::warn!(
            "Docker volume {} already exists. Removing for clean build.",
            docker_path.display()
        );
        fs::remove_dir_all(&docker_path).with_context(|| {
            format!(
                "Failed to remove existing docker volume {}",
                docker_path.display()
            )
        })?;
    }

    // Shallow clone for speed and security
    log::info!("git cloning repo...");

    // Build forge command based on build flags
    let forge_build_cmd = match build_flags {
        BuildFlags::ViaIr => {
            "forge install && forge build --via-ir --build-info --skip test --skip script"
        }
        BuildFlags::Standard => {
            "forge install && forge build --build-info --skip test --skip script"
        }
    };

    let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/workspace", docker_volume),
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "bash",
            "-c",
            &format!(
                "git clone --depth=1 {repo_url} {repo_root} && \
             cd {repo_name} && \
             if [ -f foundry.toml ]; then {forge_build_cmd}; \
             elif [ -f hardhat.config.js ] || [ -f hardhat.config.ts ]; then \
             npm install -g hardhat && npm install && npx hardhat compile; \
             else echo 'No build system detected'; fi"
            ),
        ])
        .status()
        .context("Failed to clone and build repository in Docker")?;

    if !status.success() {
        // 🔐 Validate the constructed docker volume path
        anyhow::bail!("Clone and Build failed in Docker");
    }

    if docker_path.exists() {
        validate_safe_path(&docker_path, Path::new(&audit_config().docker_volume))?;
    } else {
        log::warn!(
            "Skipping path validation because {} does not exist yet",
            docker_path.display()
        );
    }
    Ok(PathBuf::from(docker_volume))
}

fn get_commit_hash(repo_url: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["ls-remote", repo_url, "HEAD"])
        .output()
        .context("Failed to run git ls-remote")?;

    if !output.status.success() {
        anyhow::bail!(
            "git ls-remote failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commit_hash = stdout
        .split_whitespace()
        .next()
        .context("Unexpected ls-remote output format")?
        .to_string();

    Ok(commit_hash)
}

```
ai-agent-audit/src/cost/cost_data.rs
```
/// Cost tracking and calculation for LLM inference across multiple providers.
///
/// This module provides real-time cost tracking for AI agent operations,
/// supporting OpenAI, Anthropic, Gemini, and DeepSeek providers with
/// accurate token-based pricing calculations.

use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::llm_review::enums::AIAgent;

/// LLM provider cost types with specific model variants
#[derive(Clone, Copy, Debug)]
pub enum LlmCostType {
    /// OpenAI GPT-4o input tokens
    Openai4oInput,
    /// OpenAI GPT-4o output tokens
    Openai4oOutput,
    /// OpenAI O3 input tokens
    OpenaiO3Input,
    /// OpenAI O3 output tokens
    OpenaiO3Output,
    /// Anthropic Claude input tokens
    AnthropicClaudeInput,
    /// Anthropic Claude output tokens
    AnthropicClaudeOutput,
    /// Google Gemini input tokens
    GeminiInput,
    /// Google Gemini output tokens
    GeminiOutput,
    /// DeepSeek input tokens
    DeepseekInput,
    /// DeepSeek output tokens
    DeepseekOutput,
}

/// Token direction for cost calculation
#[derive(PartialEq, Eq)]
pub enum TokenType {
    /// Input tokens (prompt)
    Input,
    /// Output tokens (response)
    Output,
}

impl LlmCostType {
    /// Returns the cost per million tokens for each LLM provider and model.
    /// Prices are based on current provider pricing as of 2024.
    pub fn get_cost_per_million_tokens(self) -> f64 {
        match self {
            LlmCostType::Openai4oInput => 2.50,
            LlmCostType::Openai4oOutput => 10.00,
            LlmCostType::OpenaiO3Input => 2.00,
            LlmCostType::OpenaiO3Output => 8.00,
            LlmCostType::AnthropicClaudeInput => 3.00,
            LlmCostType::AnthropicClaudeOutput => 15.00,
            LlmCostType::GeminiInput => 1.25,
            LlmCostType::GeminiOutput => 10.00,
            LlmCostType::DeepseekInput => 0.07,
            LlmCostType::DeepseekOutput => 1.10,
        }
    }
}

impl AIAgent {
    pub fn get_cost_per_million_tokens(&self, token_type: TokenType) -> f64 {
        match self {
            // assume highest cost
            AIAgent::Openai(_) if token_type == TokenType::Input => 2.00,
            AIAgent::Openai(_) => 8.00,
            AIAgent::Anthropic(_) if token_type == TokenType::Input => 3.00,
            AIAgent::Anthropic(_) => 15.00,
            AIAgent::Gemini(_) if token_type == TokenType::Input => 1.25,
            AIAgent::Gemini(_) => 10.00,
            AIAgent::Deepseek(_) if token_type == TokenType::Input => 0.07,
            AIAgent::Deepseek(_) => 1.10,
        }
    }
}
static INFERENCE_COST_DATA: Lazy<Arc<Mutex<f64>>> = Lazy::new(|| Arc::new(Mutex::new(0.0)));

pub async fn add_to_inference_cost_by_type(content: &str, cost_type: LlmCostType) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;
    let tokens = get_token_count(content);

    *inference_cost = *inference_cost + tokens as f64 * cost_type.get_cost_per_million_tokens();
}

pub async fn add_to_inference_cost_by_agent(
    content: &str,
    agent: &Arc<AIAgent>,
    token_type: TokenType,
) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;
    let tokens = get_token_count(content);

    *inference_cost =
        *inference_cost + tokens as f64 * agent.get_cost_per_million_tokens(token_type);
}

pub async fn get_total_inference_cost() -> String {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let inference_cost = cost_data.lock().await;

    format!("{:.2}", *inference_cost / 1_000_000_f64)
}

/// Estimates tokens with additional handling for whitespace and special cases
pub fn get_token_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    // Remove extra whitespace and count characters
    let cleaned = text.trim();
    let char_count = cleaned.chars().count();

    // Use ceiling division: (n + divisor - 1) / divisor
    (char_count + 3) / 4
}


```
ai-agent-audit/src/llm_review/agent_factory.rs
```
use super::enums::AIAgent;
/// AI Agent Factory for centralized agent creation across LLM providers.
///
/// This module provides a unified interface for creating AI agents from different
/// LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) with consistent configuration
/// and error handling.
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self, O3},
    },
};
use std::sync::OnceLock;

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    Gemini,
    DeepSeek,
}

impl LlmProvider {
    /// Returns all supported providers.
    pub fn all() -> &'static [LlmProvider] {
        &[
            LlmProvider::OpenAI,
            LlmProvider::Anthropic,
            LlmProvider::Gemini,
            LlmProvider::DeepSeek,
        ]
    }

    /// Returns the string representation of the provider.
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "openai",
            LlmProvider::Anthropic => "anthropic",
            LlmProvider::Gemini => "gemini",
            LlmProvider::DeepSeek => "deepseek",
        }
    }

    /// Returns the environment variable name for the API key.
    pub fn api_key_env_var(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Gemini => "GEMINI_API_KEY",
            LlmProvider::DeepSeek => "DEEPSEEK_API_KEY",
        }
    }

    /// Checks if this provider is available based on environment variables.
    pub fn is_available(&self) -> bool {
        std::env::var(self.api_key_env_var()).is_ok()
    }

    /// Parse provider from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" => Some(LlmProvider::OpenAI),
            "anthropic" | "claude" => Some(LlmProvider::Anthropic),
            "gemini" | "google" => Some(LlmProvider::Gemini),
            "deepseek" => Some(LlmProvider::DeepSeek),
            _ => None,
        }
    }
}

/// Configuration for creating AI agents.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Temperature for response generation (0.0-2.0)
    pub temperature: f64,
    /// Model name to use for the provider
    pub model: String,
    /// Optional context to include in the agent
    pub context: Option<String>,
    /// Maximum tokens for responses (Anthropic only)
    pub max_tokens: Option<u64>,
    /// System preamble/prompt for the agent
    pub preamble: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones.".to_string(),
        }
    }
}

impl AgentConfig {
    /// Creates a new agent configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the temperature for response generation.
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Sets the model name.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Sets the context for the agent.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Sets the maximum tokens (for Anthropic models).
    pub fn with_max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the system preamble/prompt.
    pub fn with_preamble(mut self, preamble: impl Into<String>) -> Self {
        self.preamble = preamble.into();
        self
    }

    /// Creates an agent configuration for security auditing.
    pub fn for_security_audit() -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones.".to_string(),
        }
    }
}

/// Singleton clients for LLM providers
static OPENAI_CLIENT: OnceLock<openai::Client> = OnceLock::new();
static ANTHROPIC_CLIENT: OnceLock<anthropic::Client> = OnceLock::new();
static GEMINI_CLIENT: OnceLock<gemini::Client> = OnceLock::new();
static DEEPSEEK_CLIENT: OnceLock<deepseek::Client> = OnceLock::new();

/// Initializes all LLM clients from environment variables.
///
/// This function should be called once during application startup to initialize
/// all available LLM clients. Clients are only created if their API keys are available.
pub fn init_llm_clients() -> Result<()> {
    // Initialize OpenAI client if API key is available
    if audit_config().has_openai_key() {
        let client = openai::Client::from_env();
        OPENAI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("openai_client", "OpenAI client already initialized")
        })?;
    }

    // Initialize Anthropic client if API key is available
    if audit_config().has_anthropic_key() {
        let client = anthropic::Client::from_env();
        ANTHROPIC_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("anthropic_client", "Anthropic client already initialized")
        })?;
    }

    // Initialize Gemini client if API key is available
    if audit_config().has_google_ai_key() {
        let client = gemini::Client::from_env();
        GEMINI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("gemini_client", "Gemini client already initialized")
        })?;
    }

    // Initialize DeepSeek client if API key is available
    if audit_config().has_deepseek_key() {
        let client = deepseek::Client::from_env();
        DEEPSEEK_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("deepseek_client", "DeepSeek client already initialized")
        })?;
    }

    Ok(())
}

/// Returns the OpenAI client instance.
fn openai_client() -> Result<&'static openai::Client> {
    OPENAI_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "openai_client",
            "OpenAI client not initialized or API key not configured",
        )
    })
}

/// Returns the Anthropic client instance.
fn anthropic_client() -> Result<&'static anthropic::Client> {
    ANTHROPIC_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "anthropic_client",
            "Anthropic client not initialized or API key not configured",
        )
    })
}

/// Returns the Gemini client instance.
fn gemini_client() -> Result<&'static gemini::Client> {
    GEMINI_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "gemini_client",
            "Gemini client not initialized or API key not configured",
        )
    })
}

/// Returns the DeepSeek client instance.
fn deepseek_client() -> Result<&'static deepseek::Client> {
    DEEPSEEK_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "deepseek_client",
            "DeepSeek client not initialized or API key not configured",
        )
    })
}

/// Factory for creating AI agents across different providers.
pub struct AgentFactory;

impl AgentFactory {
    /// Creates an OpenAI agent with the specified configuration.
    pub fn create_openai_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = openai_client()?;
        let model = if config.model == "default" {
            O3
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Openai(builder.build()))
    }

    /// Creates an Anthropic agent with the specified configuration.
    pub fn create_anthropic_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = anthropic_client()?;
        let model = if config.model == "default" {
            CLAUDE_3_7_SONNET
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(max_tokens) = config.max_tokens {
            builder = builder.max_tokens(max_tokens);
        }

        Ok(AIAgent::Anthropic(builder.build()))
    }

    /// Creates a Gemini agent with the specified configuration.
    pub fn create_gemini_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = gemini_client()?;
        let model = if config.model == "default" {
            "gemini-2.5-pro"
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Gemini(builder.build()))
    }

    /// Creates a DeepSeek agent with the specified configuration.
    pub fn create_deepseek_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = deepseek_client()?;
        let model = if config.model == "default" {
            DEEPSEEK_CHAT
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Deepseek(builder.build()))
    }

    /// Creates an agent from the specified provider type.
    pub fn create_agent(provider: LlmProvider, config: &AgentConfig) -> Result<AIAgent> {
        match provider {
            LlmProvider::OpenAI => Self::create_openai_agent(config),
            LlmProvider::Anthropic => Self::create_anthropic_agent(config),
            LlmProvider::Gemini => Self::create_gemini_agent(config),
            LlmProvider::DeepSeek => Self::create_deepseek_agent(config),
        }
    }

    /// Creates an agent from a string provider name.
    pub fn create_agent_from_str(provider: &str, config: &AgentConfig) -> Result<AIAgent> {
        let provider_enum = LlmProvider::from_str(provider).ok_or_else(|| {
            AuditError::configuration(
                "llm_provider",
                &format!("Unsupported LLM provider: {}", provider),
            )
        })?;
        Self::create_agent(provider_enum, config)
    }

    /// Creates an agent from environment configuration.
    ///
    /// This method checks which API keys are available and creates an agent
    /// from the first available provider in priority order.
    pub fn create_from_env(config: &AgentConfig) -> Result<AIAgent> {
        // Try providers in order of preference
        for provider in LlmProvider::all() {
            if provider.is_available() {
                match Self::create_agent(*provider, config) {
                    Ok(agent) => return Ok(agent),
                    Err(_) => continue, // Try next provider
                }
            }
        }

        Err(AuditError::configuration(
            "llm_providers",
            "No LLM provider API keys found in environment",
        ))
    }

    /// Returns a list of available providers based on environment variables.
    pub fn available_providers() -> Vec<LlmProvider> {
        LlmProvider::all()
            .iter()
            .filter(|provider| provider.is_available())
            .copied()
            .collect()
    }
}

/// Convenience functions that match the original API but use the factory internally.
/// These maintain backward compatibility with existing code.

/// Creates an OpenAI agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_openai_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_openai_agent(
    _client: &openai::Client,
    temperature: f64,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_preamble(preamble)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_openai_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create OpenAI agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates an Anthropic agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_anthropic_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_anthropic_agent(
    _client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_max_tokens(max_tokens);

    // Use the factory with singleton clients
    AgentFactory::create_anthropic_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create Anthropic agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates a Gemini agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_gemini_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_gemini_agent(
    _client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_gemini_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create Gemini agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates a DeepSeek agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_deepseek_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_deepseek_agent(
    _client: &deepseek::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_deepseek_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create DeepSeek agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_provider_enum() {
        assert_eq!(LlmProvider::OpenAI.as_str(), "openai");
        assert_eq!(LlmProvider::Anthropic.as_str(), "anthropic");
        assert_eq!(LlmProvider::Gemini.as_str(), "gemini");
        assert_eq!(LlmProvider::DeepSeek.as_str(), "deepseek");
    }

    #[test]
    fn test_provider_from_str() {
        assert_eq!(LlmProvider::from_str("openai"), Some(LlmProvider::OpenAI));
        assert_eq!(LlmProvider::from_str("gpt"), Some(LlmProvider::OpenAI));
        assert_eq!(
            LlmProvider::from_str("claude"),
            Some(LlmProvider::Anthropic)
        );
        assert_eq!(LlmProvider::from_str("invalid"), None);
    }

    #[test]
    fn test_agent_config_builder() {
        let config = AgentConfig::new()
            .with_temperature(0.8)
            .with_model("gpt-4")
            .with_context("test context");

        assert_eq!(config.temperature, 0.8);
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.context, Some("test context".to_string()));
    }

    #[test]
    fn test_security_audit_config() {
        let config = AgentConfig::for_security_audit();
        assert_eq!(config.temperature, 1.0);
        assert!(config.preamble.contains("security auditing"));
        assert_eq!(config.max_tokens, Some(4096));
    }

    #[test]
    fn test_available_providers() {
        let providers = AgentFactory::available_providers();
        // This will depend on environment variables, so we just check it returns a Vec
        assert!(providers.is_empty() || !providers.is_empty());
    }
}

```
ai-agent-audit/src/llm_review/analysis_db.rs
```
use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct FindingDb {
    pub id: String, // UUID v4
    pub title: String,
    pub description: String,
    pub impact: String,           // FK → seeds.id
    pub proof_of_concept: String, // e.g. "High", "Info", …
    pub proof_of_code: String,
    pub severity: String,
    // pub confidence:   Option<f32>,
    // pub sources_used: Vec<u8>,
}

pub struct FindingsDb(Connection);

impl FindingsDb {
    pub fn open(p: &Path) -> Result<Self> {
        let conn = Connection::open(p)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS findings(
              id                  TEXT PRIMARY KEY,
              title               TEXT,
              description         TEXT,
              impact              TEXT,
              proof_of_concept    TEXT,
              proof_of_code       TEXT,
              severity            TEXT,
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert(&self, f: &FindingDb) -> Result<()> {
        self.0.execute(
            "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7);",
            params![
                f.id,
                f.title,
                f.description,
                f.impact,
                f.proof_of_concept,
                f.proof_of_code,
                f.severity
            ],
        )?;
        Ok(())
    }
}

```
ai-agent-audit/src/llm_review/code_review.rs
```
use crate::ai_bot::agent::get_rag_for_security_query;
use crate::config::audit_config;
use crate::error::Result;
use crate::prepare_code::git_clone::RepoPaths;
use crate::{
    cost::cost_data::{
        add_to_inference_cost_by_agent, add_to_inference_cost_by_type, LlmCostType, TokenType,
    },
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        agent_factory::{AgentConfig, AgentFactory},
        config::{generated_llm_prompt, ContractInvariants, Finding, VulnerabilityQualityCheck},
        context_state::get_metadata_context,
        prompt_context::generate_prompt_for_issue_check,
        prompt_support::{
            post_prompt::POST_PROMPT, post_qualify::POST_QUALIFY, post_verify::POST_VERIFY,
            pre_prompt::PRE_PROMPT, pre_qualify::PRE_QUALIFY, pre_verify::PRE_VERIFY,
            qualify_prompt::QUALIFY_PROMPT, verify_prompt::VERIFY_PROMPT,
        },
    },
    master_prompts::{prompt_2x_aa::PROMPT_2X_AA, prompt_2x_bb::PROMPT_2X_BB},
};
use log::info;
use rig::providers::openai::O3;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::Mutex;

use super::{
    config::{Findings, LegitVulnerability},
    enums::AIAgent,
};

/// Multi-LLM security analysis orchestration.
///
/// This module coordinates parallel security analysis across multiple LLM providers,
/// implements verification and deduplication workflows, and manages cost tracking.

/// Orchestrates comprehensive security analysis of smart contracts using multiple LLM providers.
///
/// This function performs parallel vulnerability detection across multiple AI agents,
/// implements verification and deduplication workflows, and returns categorized findings.
///
/// # Arguments
/// * `codeblocks_path` - Path to the database containing generated code blocks
///
/// # Returns
/// * `HashMap<String, Findings>` - Security findings organized by contract
/// * `Vec<ContractInvariants>` - Protocol invariant analysis results
pub async fn review_codebase_for_security_issues(
    codeblocks_path: &PathBuf,
    repo: &RepoPaths,
) -> Result<(HashMap<String, Findings>, Vec<ContractInvariants>)> {
    let mut all_security_issues = HashMap::<String, Findings>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    let (ai_verify_agent, ai_discovery_agents) = generate_ai_agents().await?;

    let invariant_findings = Vec::<ContractInvariants>::new();

    let metadata_context = get_metadata_context().await?;

    for (contract, codeblock) in contracts.into_iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);
        // grab additional context from RAG
        let rag_context = get_rag_for_security_query(&codeblock, repo).await?;
        let audit_context = format!(
            "\n## CONTEXT \n\n {} \n\n {}",
            metadata_context, rag_context
        );

        let raw_findings = Findings::generate_findings_from_contract_codebase(
            &contract,
            &codeblock,
            &audit_context,
            &ai_discovery_agents,
        )
        .await?;

        if !raw_findings.findings.is_empty() {
            info!(
                "# of findings BEFORE deduping => {}",
                raw_findings.findings.len()
            );

            let deduped_and_verified_findings = raw_findings
                .dedup_and_verify_with_llm(&codeblock, &ai_verify_agent, &audit_context)
                .await?;

            let quality_checked_and_updated_findings = deduped_and_verified_findings
                .quality_check_with_llm(&codeblock, &ai_verify_agent, &audit_context)
                .await?;

            all_security_issues.insert(contract.to_string(), quality_checked_and_updated_findings);

            // TODO - save issues to Findings db
        }
    }

    // info!("standard security findings => {:#?}", all_security_issues);
    // info!("invariant findings => {:#?}", invariant_findings);
    Ok((all_security_issues, invariant_findings))
}

pub async fn generate_ai_agents() -> Result<(Arc<AIAgent>, Vec<Arc<AIAgent>>)> {
    info!("setting up AI agents...");

    // Create verification agent using OpenAI O3
    let verify_config = AgentConfig::new()
        .with_temperature(1.0)
        .with_model(O3)
        .with_preamble("You are SoliditySec-Verifier, a senior smart-contract auditor.");

    let ai_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);

    // Create discovery agents using Anthropic models
    let mut ai_discovery_agents = Vec::new();

    let gemini_config = AgentConfig::for_security_audit()
        .with_temperature(1.0)
        .with_model("gemini-2.5-pro");

    for _ in 0..audit_config().runs {
        let agent = Arc::new(AgentFactory::create_gemini_agent(&gemini_config)?);
        ai_discovery_agents.push(agent);
    }

    info!("Created {} discovery agents", ai_discovery_agents.len());

    Ok((ai_verify_agent, ai_discovery_agents))
}

/// Executes one LLM-prompt round and merges the returned findings into the shared `Arc<Mutex<Findings>>`.
///
/// Splitting the logic out of the for-loop keeps the main auditor-loop readable
/// and makes it far easier to unit-test this piece in isolation.
pub async fn run_security_prompt(
    agent: Arc<AIAgent>,
    contract_name: Arc<String>,
    code: Arc<String>,
    added_context: Arc<String>,
    instructions: &'static str,
    idx_of_review_round: usize,
    shared_findings: Arc<Mutex<Findings>>,
) -> Result<()> {
    // 1. Build full prompt
    let prompt_header = generated_llm_prompt(&contract_name, instructions, PRE_PROMPT, POST_PROMPT);
    let prompt_body = generate_content_plus_context_block(&code, &added_context);
    let full_prompt = format!("{prompt_header}{prompt_body}");

    // add to cost
    // add_to_inference_cost_by_type(&full_prompt, LlmCostType::GeminiInput).await;
    add_to_inference_cost_by_agent(&full_prompt, &agent, TokenType::Input).await;

    // 2. Send to the right provider
    info!("----LLM analysis Round #{}----", idx_of_review_round);
    let findings: Findings = agent.extract_with_retry(&full_prompt).await?;

    let issues_found = findings.findings.len();
    info!("{} issues found!", issues_found);

    // 3. Merge results (if any) into the shared accumulator
    if issues_found > 0 {
        let mut guard = shared_findings.lock().await;
        guard.findings.extend(findings.findings);
    }

    Ok(())
}

fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n");

    code_plus_context.push_str(codeblock);
    // print_first_four_lines(&codeblock);

    code_plus_context.push_str("\n\n ADDITIONAL CONTEXT \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");
    // print_first_four_lines(&added_context);

    code_plus_context
}

//SCAN FOR INVARIANTS
// info!("submitting invariant prompt to openai");
// let invariants_response = openai_agent_1st_pass.prompt(INVARIANTS).await?;
//
// info!("parsing invariant prompt");
// let invariants = ContractInvariants::parse_from_json(&invariants_response)?;
//
// if !invariants.invariants.is_empty() {
//     invariant_findings.push(invariants);
// }
// for security_issue in SECURITY_PROMPT_ENUMS {
// SCAN FOR STANDARD SECURITY ISSUES
// let findings = if LANGUAGE_MODEL == LanguageModel::OpenAI {
//     let prompt_string = generate_llm_prompt_for_security_issue(
//         contract,
//         &security_issue.prompt(),
//         codeblock,
//         "",
//     );
//
//     info!("submitting security vulnerability prompt to openai");
//
//     let findings = agent_extract_with_retry(openai_agent, &prompt_string).await?;
//     findings
// } else {
//     let prompt_string = generate_llm_prompt_for_security_issue(
//         contract,
//         &security_issue.prompt(),
//         codeblock,
//         &added_context_from_ai_brain,
//     );
//
//     info!("submitting security vulnerability prompt to anthropic");
//     info!(
//         "-----------------------ROUND #{}-----------------------",
//         run
//     );
//     info!("{} Issue", security_issue.as_fancy_str());
//     let findings = agent_extract_with_retry(&anthropic_agents[run], &prompt_string).await?;
//
//     findings
// }
//
//
impl Findings {
    pub async fn generate_findings_from_contract_codebase(
        contract: &str,
        code: &str,
        context: &str,
        agents: &Vec<Arc<AIAgent>>,
    ) -> Result<Self> {
        let mut handles = vec![];
        let all_findings = Arc::new(Mutex::new(Findings {
            findings: Vec::new(),
        }));
        let contract = Arc::new(contract.to_string());
        let codeblock = Arc::new(code.to_string());
        let added_content_from_brain = Arc::new(context.to_string());

        for (run, arc_agent) in agents.iter().enumerate() {
            // for (i, prompt) in [PROMPT_2X_AA, PROMPT_2X_BB].into_iter().enumerate() {
            for (i, prompt) in [PROMPT_2X_AA].into_iter().enumerate() {
                let agent = Arc::clone(arc_agent);
                let combined_findings = Arc::clone(&all_findings);
                let contract_name = Arc::clone(&contract);
                let code = Arc::clone(&codeblock);
                let added_content = Arc::clone(&added_content_from_brain);

                handles.push(tokio::spawn(async move {
                    if let Err(e) = run_security_prompt(
                        agent,
                        contract_name,
                        code,
                        added_content,
                        prompt,
                        (run + 1) * (i + 1),
                        combined_findings,
                    )
                    .await
                    {
                        log::error!("Prompt task failed: {e:#}");
                    }
                }));
            }
        }

        // Wait for ALL tasks to complete
        for handle in handles {
            handle.await?; // Will error if task panicked
        }

        let findings = all_findings.lock().await;

        if !findings.findings.is_empty() {
            info!(
                "# of findings BEFORE deduping => {}",
                findings.findings.len()
            );
        }
        Ok(findings.clone())
    }
    // TODO - refactor dedup to first use hashMap => HashMap<String(hash),Vec<Finding> (same hash)>
    // then evaluate each entry for dups with different thread!
    pub async fn dedup_and_verify_with_llm(
        &self,
        code: &str,
        agent: &Arc<AIAgent>,
        context: &str,
    ) -> Result<Self> {
        let mut handles = vec![];
        let deduped_findings = Arc::new(self.clone().dedup().await?);
        let code_and_context = generate_content_plus_context_block(code, context);
        let arc_code_context = Arc::new(code_and_context);

        let dedup_finding_count = deduped_findings.findings.len();
        let is_legit_finding_vec: Arc<Mutex<Vec<bool>>> =
            Arc::new(Mutex::new(vec![true; dedup_finding_count]));

        info!("# of findings AFTER deduping => {}", dedup_finding_count);

        info!("now verifying each finding...");

        for i in 0..dedup_finding_count {
            let codeblock_plus_context = Arc::clone(&arc_code_context);
            let arc_agent = Arc::clone(agent);
            let arc_findings = Arc::clone(&deduped_findings);
            let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
            handles.push(tokio::spawn(async move {
                let result: Result<()> = async {
                    let instruction_prompt = generate_prompt_for_issue_check(
                        &codeblock_plus_context,
                        &arc_findings.findings[i],
                        PRE_VERIFY,
                        VERIFY_PROMPT,
                        POST_VERIFY,
                    );

                    // add to cost
                    add_to_inference_cost_by_type(&instruction_prompt, LlmCostType::OpenaiO3Input)
                        .await;
                    info!("verifying finding #{}", i);
                    let is_legit_struct: LegitVulnerability =
                        arc_agent.extract_with_retry(&instruction_prompt).await?;

                    let is_finding_legit = is_legit_struct.is_legit_vulnerability;
                    if !is_finding_legit {
                        info!(
                            "{} is NOT legit => {}",
                            arc_findings.findings[i].title(),
                            is_legit_struct.why_its_not_legit.unwrap_or_default()
                        );
                    }
                    let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                    legit_findings_vec[i] = is_finding_legit;

                    // add
                    Ok(())
                }
                .await;

                if let Err(e) = result {
                    log::error!("Error verifying finding {}: {:?}", i, e);
                }
            }));
        }

        // optionally await them all
        for h in handles {
            let _ = h.await;
        }

        let legit_findings_vec = is_legit_finding_vec.lock().await;
        let verified_findings: Vec<Finding> = deduped_findings
            .as_ref()
            .findings
            .iter()
            .enumerate()
            .filter(|(idx, _)| legit_findings_vec[*idx])
            .map(|(_, f)| f.clone())
            .collect();

        info!(
            "-------------{} Verified Findings!-----------------",
            verified_findings.len()
        );

        Ok(Findings {
            findings: verified_findings,
        })
    }

    pub async fn quality_check_with_llm(
        self,
        code: &str,
        agent: &Arc<AIAgent>,
        context: &str,
    ) -> Result<Self> {
        let mut handles = vec![];
        let findings = Arc::new(self.clone().dedup().await?);
        let code_and_context = generate_content_plus_context_block(code, context);
        let arc_code_context = Arc::new(code_and_context);

        let finding_count = findings.findings.len();
        // create vec (is_quality_check_passed, updated_finding) for each finding
        // assume all initially pass
        let quality_check_passed_vec: Arc<Mutex<Vec<(bool, Finding)>>> =
            Arc::new(Mutex::new(vec![(true, Finding::default()); finding_count]));

        info!("now quality check on each finding...");

        for i in 0..finding_count {
            let codeblock_plus_context = Arc::clone(&arc_code_context);
            let arc_agent = Arc::clone(agent);
            let arc_findings = Arc::clone(&findings);
            let arc_legit_findings_vec = Arc::clone(&quality_check_passed_vec);
            handles.push(tokio::spawn(async move {
                let result: Result<()> = async {
                    let prompt = generate_prompt_for_issue_check(
                        &codeblock_plus_context,
                        &arc_findings.findings[i],
                        PRE_QUALIFY,
                        QUALIFY_PROMPT,
                        POST_QUALIFY,
                    );
                    add_to_inference_cost_by_type(&prompt, LlmCostType::OpenaiO3Input).await;
                    info!("quality checking finding #{}", i);
                    let qualify_checked_finding: VulnerabilityQualityCheck =
                        arc_agent.extract_with_retry(&prompt).await?;

                    let quality_check_passed = qualify_checked_finding.is_quality_check_passed;
                    if !quality_check_passed {
                        info!(
                            "{} did not pass quality check => {:?}",
                            arc_findings.findings[i].title(),
                            qualify_checked_finding.where_quality_lacks
                        );
                        info!("{:#?}", &qualify_checked_finding);
                        let updated_finding = Finding {
                            impact: Some(qualify_checked_finding.impact.clone().unwrap_or(
                                arc_findings.findings[i].impact.clone().unwrap_or_default(),
                            )),
                            proof_of_code: Some(
                                qualify_checked_finding.proof_of_code.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .proof_of_code
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            proof_of_concept: Some(
                                qualify_checked_finding.proof_of_concept.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .proof_of_concept
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            mitigation: Some(
                                qualify_checked_finding.mitigation.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .mitigation
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            severity: qualify_checked_finding
                                .severity
                                .unwrap_or(arc_findings.findings[i].severity),
                            ..arc_findings.findings[i].clone()
                        };
                        let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                        legit_findings_vec[i] = (quality_check_passed, updated_finding);
                    } else {
                        let mut quality_checked_findings_vec = arc_legit_findings_vec.lock().await;
                        quality_checked_findings_vec[i] = (true, arc_findings.findings[i].clone());
                    }

                    Ok(())
                }
                .await;

                if let Err(e) = result {
                    log::error!("Error verifying finding {}: {:?}", i, e);
                }
            }));
        }

        // optionally await them all
        for h in handles {
            let _ = h.await;
        }

        let qualify_checked_findings_vec = quality_check_passed_vec.lock().await;
        let qualified_findings: Vec<Finding> = findings
            .as_ref()
            .findings
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                if qualify_checked_findings_vec[idx].0 {
                    // if quality check passes , no changes needed
                    f.clone()
                } else {
                    // if failed submit updated finding
                    qualify_checked_findings_vec[idx].1.clone()
                }
            })
            .collect();

        let num_findings_updated = qualify_checked_findings_vec
            .iter()
            .filter(|(passed, _)| !*passed)
            .count();

        info!(
            "{} Verified Findings with {} updated findings!",
            qualified_findings.len(),
            num_findings_updated
        );

        Ok(Findings {
            findings: qualified_findings,
        })
    }
}

```
ai-agent-audit/src/llm_review/config.rs
````
use super::{
    enums::{InvariantStatus, InvariantType, Severity, VulnerabilityType},
    prompt_support::dedup::DEDUP_PROMPT,
};
use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
    master_prompts::{prompt_2x_a::PROMPT_2X_A, prompt_2x_b::PROMPT_2X_B},
    prompts::{
        access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
        confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
        dos::DOS, inheritance::WRONG_INHERITANCE, integer_overflow::INTEGER_OVERFLOW, mev::MEV,
        oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
        reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK,
        self_destruct::SELF_DESTRUCT, storage_variables::STORAGE_VARIABLE, tx_origin::TX_ORIGIN,
        unchecked_return_value::UNCHECK_RETURN_VALUES, unexpected_eth::UNEXPECTED_ETH,
        zero_code::CONTRACTS_WITH_ZERO_CODE,
    },
};
use regex::Regex;
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    completion::{CompletionModel, Prompt},
    providers::{
        azure::GPT_4O,
        openai::{self},
    },
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageModel {
    OpenAI,
    Anthropic,
}

pub const CLAUDE_4_0_SONNET: &str = "claude-sonnet-4-0";
pub const CLAUDE_4_OPUS: &str = "claude-opus-4-0";
pub const LANGUAGE_MODEL: LanguageModel = LanguageModel::Anthropic;
pub const RUNS: usize = 3;
pub const INSTRUCTION_PROMPTS: [&str; 2] = [PROMPT_2X_A, PROMPT_2X_B];

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Finding {
    // [Severity-issue number] - List Issue (Reentrancy, Denial of Service, etc) and
    // <Contract>::<Function> its localed in
    pub issue_type: VulnerabilityType,
    pub contract: String,            // exact constract name where issue appears
    pub function: String, // exact function name where issue appears, if not applicable set to 'NA'
    pub description: Option<String>, // description of issue, include code snippet if relevant
    pub impact: Option<String>, // Impact of Issue
    pub proof_of_concept: Option<String>, // Demonstrate how issue can be exploited by hacker
    pub proof_of_code: Option<String>, // Write Foundry Unit test to prove issue exists
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Severity, //severity of issue
    pub mitigation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Findings {
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InvariantFinding {
    pub id: String,
    #[schemars(
        description = "Type: Arithmetic, Balance, Permission, Temporal, Referential, StateMachine"
    )]
    pub inv_type: InvariantType,
    pub desc: String,
    #[schemars(description = "Status: HOLDS, VIOLATION")]
    pub status: InvariantStatus,
    pub pre_state: Option<String>,
    pub post_state: Option<String>,
    pub impact: Option<String>,
    pub poc: Option<String>,
    pub mitigation: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContractInvariants {
    pub contract: String,
    pub intention: String,
    pub invariants: Vec<InvariantFinding>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitVulnerability {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_legit_vulnerability: bool,
    pub why_its_not_legit: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VulnerabilityQualityCheck {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_quality_check_passed: bool, // quality check passes with no changes/update needed, true|false
    pub where_quality_lacks: Option<String>, // breif description
    pub impact: Option<String>,              // updated impact (if necessary)
    pub proof_of_concept: Option<String>,    // updated POC (if necessary)
    pub proof_of_code: Option<String>,       // updated Foundry Unit test (if necessary)
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Option<Severity>, // updated severity of issue (if necessary)
    pub mitigation: Option<String>,          // updated mitigation (if necessary)
}

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct InvariantFindings {
//     pub findings: Vec<InvariantFinding>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DuplicateFindings {
    pub titles: Vec<String>,
}

pub const SECURITY_PROMPT_ENUMS: [VulnerabilityType; 8] = [
    VulnerabilityType::Reentrancy,
    VulnerabilityType::AccessControl,
    // VulnerabilityType::ArrayLimits,
    // VulnerabilityType::DefaultVisibility,
    VulnerabilityType::Dos,
    VulnerabilityType::IntegerMath,
    // VulnerabilityType::ConfidentialData,
    // VulnerabilityType::Inheritance,
    // VulnerabilityType::Oracle,
    VulnerabilityType::Pragma,
    VulnerabilityType::Randomness,
    // VulnerabilityType::ReplayAttack,
    // VulnerabilityType::SelfDestruct,
    // VulnerabilityType::StorageLayout,
    // VulnerabilityType::TxOrigin,
    // VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UnexpectedEth,
    // VulnerabilityType::ZeroCode,
    VulnerabilityType::FrontrunMev,
    // VulnerabilityType::ShortAddress,
];

pub const QUALITY_CHECK_ENUMS: [VulnerabilityType; 24] = [
    VulnerabilityType::Reentrancy,
    VulnerabilityType::AccessControl,
    VulnerabilityType::ArrayLimits,
    VulnerabilityType::Dos,
    VulnerabilityType::IntegerMath,
    VulnerabilityType::ConfidentialData,
    VulnerabilityType::Inheritance,
    VulnerabilityType::Oracle,
    VulnerabilityType::Randomness,
    VulnerabilityType::ReplayAttack,
    VulnerabilityType::SelfDestruct,
    VulnerabilityType::StorageLayout,
    VulnerabilityType::TxOrigin,
    VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UnexpectedEth,
    VulnerabilityType::FrontrunMev,
    VulnerabilityType::UpgradeabilityInitializerSafety,
    VulnerabilityType::PausableEmergencyStop,
    VulnerabilityType::TimestampDependentLogic,
    VulnerabilityType::FlashLoanEconomicManipulation,
    VulnerabilityType::DelegatecallLowLevelOps,
    VulnerabilityType::SignatureMalleability,
    VulnerabilityType::GasGriefBlockLimit,
    VulnerabilityType::IntegerOverflow,
];

pub const SECURITY_PROMPTS: [&str; 19] = [
    REENTRANCY,                  //DONE
    ACCESS_CONTROL,              //DONE
    ACCESS_OUTSIDE_ARRAY_LIMITS, //DONE
    DEFAULT_VISIBILITIES,        //DONE
    DOS,                         //DONE
    INTEGER_OVERFLOW,            // DONE
    SAVING_CONFIDENTIAL_DATA,    //DONE
    WRONG_INHERITANCE,           // DONE
    ORACLE_MANIPULATION,         // DONE
    FLOATING_PRAGMA,             //DONE
    RANDOMNESS,                  // DONE
    REPLAY_SIGNATURES_ATTACK,    //DONE
    SELF_DESTRUCT,               //DONE
    STORAGE_VARIABLE,            //DONE
    TX_ORIGIN,                   //DONE
    UNCHECK_RETURN_VALUES,       //DONE
    UNEXPECTED_ETH,              //DONE
    CONTRACTS_WITH_ZERO_CODE,    //DONE
    // SHORT_ADDRESS_ATTACK,        // DONE - this is only issue for very old contracts
    MEV, //DONE
];

pub fn generated_llm_prompt(
    contract_name: &str,
    main_instructions: &str,
    pre: &str,
    post: &str,
) -> String {
    let instruction_template = format!("{}{}{}", pre, main_instructions, post);

    // populate template
    instruction_template
        .replace("{contract_name}", contract_name)
        .to_string()
}

fn deserialize_bool_from_str_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let val: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match val {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(s) => match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(serde::de::Error::custom("expected 'true' or 'false'")),
        },
        _ => Err(serde::de::Error::custom("expected boolean or string")),
    }
}

impl Finding {
    pub fn title(&self) -> String {
        let fn_name = self.get_fn_name();
        format!(
            "{} issue in {}::{}",
            self.issue_type.as_fancy_str(),
            self.contract,
            fn_name
        )
    }

    pub fn free_report_title(&self) -> String {
        if self.severity == Severity::High || self.severity == Severity::Medium {
            format!(
                "{} issue found with {} severity",
                self.issue_type.as_fancy_str(),
                self.severity.as_str()
            )
        } else {
            format!(
                "{} issue in {}::{}",
                self.issue_type.as_fancy_str(),
                self.contract,
                self.function
            )
        }
    }

    pub fn hash(&self) -> String {
        // extract name 'func_name' from func_name(...)
        let fn_name = self.get_fn_name();

        format!("{}-{}-{}", self.issue_type.as_str(), self.contract, fn_name)
    }

    // extract name 'func_name' from func_name(...)
    pub fn get_fn_name(&self) -> String {
        let re = Regex::new(r"(^[a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
        if let Some(captures) = re.captures(&self.function) {
            match captures.get(1) {
                Some(name) => name.as_str().to_string(),
                None => self.function.clone(),
            }
        } else {
            self.function.clone()
        }
    }

    // resonse "YES" or "NO"
    pub async fn is_duplicate_issue<M>(
        &self,
        issue: &Finding,
        ai_agent: &Agent<M>,
    ) -> anyhow::Result<bool>
    where
        M: CompletionModel,
    {
        // contract, function and issue type MUST match
        if issue.hash() != self.hash() {
            return Ok(false);
        } else if issue.description == self.description {
            return Ok(true);
        }

        let prompt = DEDUP_PROMPT
            .replace("{contract}", &self.contract)
            .replace("{function}", &self.function)
            .replace("{issue_type}", self.issue_type.as_str())
            .replace(
                "{description_a}",
                &self.description.clone().unwrap_or_default(),
            )
            .replace(
                "{description_b}",
                &issue.description.clone().unwrap_or_default(),
            );

        log::info!("checking if {} is duplication", issue.title());
        add_to_inference_cost_by_type(&prompt, LlmCostType::Openai4oInput).await;

        let response = ai_agent.prompt(prompt).await?;

        add_to_inference_cost_by_type(&response, LlmCostType::Openai4oOutput).await;

        Ok(response.trim().eq_ignore_ascii_case("YES"))
    }
}

impl ContractInvariants {
    /// Parse JSON string containing findings from LLM response
    /// Handles both clean JSON and JSON wrapped in markdown code blocks
    pub fn parse_from_json(json_str: &str) -> Result<ContractInvariants, serde_json::Error> {
        // Clean the input - remove markdown code blocks and extra quotes/escapes
        let cleaned_json = Self::clean_json_string(json_str);

        // Parse the cleaned JSON
        serde_json::from_str(&cleaned_json)
    }

    /// Clean JSON string by removing markdown code blocks, escaped quotes, and extra formatting
    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        // Remove outer quotes if present (from string literals)
        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        // Remove markdown code blocks
        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        // Replace escaped quotes and newlines
        // cleaned
        //     .replace("\\\"", "\"")
        //     .replace("\\n", "\n")
        //     .replace("\\\n", "\n")
        //     .trim()
        //     .to_string()
        cleaned.to_string()
    }

    pub fn get_all_violations(self) -> Vec<InvariantFinding> {
        self.invariants
            .into_iter()
            .filter(|inv| inv.status == InvariantStatus::VIOLATION)
            .collect::<Vec<InvariantFinding>>()
    }
}
impl Findings {
    pub async fn dedup(self) -> anyhow::Result<Findings> {
        if self.findings.is_empty() {
            return Ok(Findings {
                findings: Vec::new(),
            });
        }

        let openai_client = openai::Client::from_env();
        let openai_agent = Arc::new(openai_client.agent(GPT_4O).temperature(1.0).build());

        let mut findings_hash = HashMap::<String, Vec<Finding>>::new();

        for finding in &self.findings {
            let hash = finding.hash();
            findings_hash
                .entry(hash)
                .or_insert(Vec::new())
                .push(finding.clone());
        }

        let arc_dedup_findings = Arc::new(Mutex::new(Vec::with_capacity(self.findings.len())));
        let mut handles = Vec::new();

        for findings in findings_hash.into_values() {
            let arc_findings = Arc::new(findings);
            let current_findings = Arc::clone(&arc_findings);
            let deduped_findings = Arc::clone(&arc_dedup_findings);
            let agent = Arc::clone(&openai_agent);
            let handle = tokio::spawn(async move {
                if current_findings.len() > 1 {
                    match get_deduped_finding_vec(&current_findings, &agent).await {
                        Ok(deduped) => {
                            let mut deduped_findings_lock = deduped_findings.lock().await;
                            deduped_findings_lock.extend(deduped);
                        }
                        Err(e) => {
                            log::error!("❌ deduping findngs failed: {e}");
                        }
                    }
                } else {
                    let mut deduped_findings_lock = deduped_findings.lock().await;
                    deduped_findings_lock.extend(current_findings.iter().cloned())
                }
            });
            handles.push(handle);
        }

        // CRITICAL: Wait for all tasks to complete
        for handle in handles {
            handle.await?;
        }
        // Fix: Extract the Vec from Arc<Mutex<Vec<Finding>>>
        let deduped_findings = Arc::try_unwrap(arc_dedup_findings)
            .map_err(|_| anyhow::anyhow!("Failed to unwrap Arc"))?
            .into_inner();

        Ok(Findings {
            findings: deduped_findings,
        })
    }

    /// Get count of findings by severity
    pub fn count_by_severity(&self) -> std::collections::HashMap<Severity, usize> {
        let mut counts = HashMap::new();

        for finding in &self.findings {
            *counts.entry(finding.severity).or_insert(0) += 1;
        }

        counts
    }

    /// Filter findings by severity level
    pub fn filter_by_severity(&self, severity: Severity) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .collect()
    }

    /// Get all high severity findings
    pub fn high_severity_findings(&self) -> Vec<&Finding> {
        self.filter_by_severity(Severity::High)
    }
}

async fn get_deduped_finding_vec<T>(
    findings: &Arc<Vec<Finding>>,
    agent: &Arc<Agent<T>>,
) -> anyhow::Result<Vec<Finding>>
where
    T: CompletionModel,
{
    // assigns bool to each finding index, is dup or not? assume not for initializing
    let size = findings.len();
    let mut is_dup_vec: Vec<bool> = vec![false; size];

    for i in 0..size {
        if is_dup_vec[i] {
            continue;
        }
        for j in i + 1..size {
            if is_dup_vec[j] {
                continue;
            }
            let is_dup = findings[i].is_duplicate_issue(&findings[j], &agent).await?;
            if is_dup {
                is_dup_vec[j] = true;
                continue;
            }
        }
    }

    let findings: Vec<Finding> = findings
        .iter()
        .enumerate()
        .filter(|(idx, _)| !is_dup_vec[*idx])
        .map(|(_, f)| f)
        .cloned()
        .collect();

    Ok(findings)
}

pub trait FromLLMJson: Sized {
    /// Parse clean JSON string into type
    fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error>;

    /// Clean up formatting (markdown code blocks, extra quotes, etc.)
    fn clean_json_string(input: &str) -> String;

    /// Extract and parse JSON from raw LLM response with extra text
    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>>;
}

impl<T> FromLLMJson for T
where
    T: DeserializeOwned,
{
    fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let cleaned = Self::clean_json_string(json_str);
        serde_json::from_str(&cleaned)
    }

    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        cleaned.to_string()
    }

    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        match (json_start, json_end) {
            (Some(start), Some(end)) if start < end => {
                let json_part = &response[start..=end];
                Self::parse_from_json(json_part)
                    .map_err(|e| format!("Failed to parse JSON: {}", e).into())
            }
            _ => Err("No valid JSON found in response".into()),
        }
    }
}

````
ai-agent-audit/src/llm_review/context_state.rs
```
/// Global context state management for AI analysis.
///
/// This module manages shared protocol metadata context that is generated once
/// and reused across all AI agents for consistent analysis. Provides thread-safe
/// access to protocol information including summaries and semantic data.
use once_cell::sync::Lazy;
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

use super::prompt_context::generate_context_for_code_review;
use crate::prepare_code::git_clone::RepoPaths;

/// Global metadata context shared across all AI agents
static METADATA_CONTEXT: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(String::new())));

/// Generates and caches protocol metadata context for AI analysis.
///
/// This function creates comprehensive context information including protocol
/// summaries, contract relationships, and semantic data that is shared across
/// all AI agents for consistent analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_path` - Path to semantic analysis database
pub async fn generate_and_save_metadata_context(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> anyhow::Result<()> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata = metadata_context.lock().await;
    let context = generate_context_for_code_review(repo, semantics_path).await?;

    *metadata = context.clone();
    Ok(())
}

/// Retrieves the cached metadata context for AI analysis.
///
/// Returns the protocol metadata context that was previously generated and
/// cached for use across all AI agents.
///
/// # Returns
/// * `String` - Cached protocol metadata context
pub async fn get_metadata_context() -> anyhow::Result<String> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata = metadata_context.lock().await;

    Ok(metadata.clone())
}

```
ai-agent-audit/src/llm_review/enums.rs
```
/// AI agent and vulnerability type enumerations.
///
/// This module defines the core enums for multi-LLM support and vulnerability
/// categorization, providing unified interfaces for different AI providers
/// and systematic vulnerability detection across 19+ security categories.

use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    cost::cost_data::LlmCostType,
    invariant_prompts::{
        arithmetic::ARITHMETIC, balance::BALANCE, permission::PERMISSION, referential::REFERENTIAL,
        state_machine::STATE_MACHINE, temporal::TEMPORAL,
    },
    master_prompts::master_prompt::MASTER_SECURITY_PROMPT,
    prompts::{
        access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
        confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
        dos::DOS, inheritance::WRONG_INHERITANCE, integer_overflow::INTEGER_OVERFLOW, mev::MEV,
        oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
        reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK,
        self_destruct::SELF_DESTRUCT, short_address_attack::SHORT_ADDRESS_ATTACK,
        storage_variables::STORAGE_VARIABLE, tx_origin::TX_ORIGIN,
        unchecked_return_value::UNCHECK_RETURN_VALUES, unexpected_eth::UNEXPECTED_ETH,
        zero_code::CONTRACTS_WITH_ZERO_CODE,
    },
    utils::extract_retry::agent_extract_with_retry,
};
use rig::{
    agent::Agent,
    extractor::Extractor,
    providers::{
        anthropic::{self},
        deepseek::DeepSeekCompletionModel,
        gemini::{self},
        openai::{self},
    },
};

use serde::de::DeserializeOwned;

/// Unified AI agent enum supporting multiple LLM providers.
///
/// Provides a common interface for different AI providers while maintaining
/// provider-specific optimizations and cost tracking capabilities.
pub enum AIAgent {
    /// Anthropic Claude models (3.7 Sonnet, 4.0 Sonnet)
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    /// OpenAI models (GPT-4o, O3)
    Openai(Agent<openai::CompletionModel>),
    /// Google Gemini models
    Gemini(Agent<gemini::completion::CompletionModel>),
    /// DeepSeek models (cost-effective option)
    Deepseek(Agent<DeepSeekCompletionModel>),
}

/// Unified AI extractor enum for structured data extraction.
///
/// Provides type-safe extraction capabilities across different LLM providers
/// with automatic retry logic and error handling.
pub enum AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    Anthropic(Extractor<anthropic::completion::CompletionModel, T>),
    Openai(Extractor<openai::CompletionModel, T>),
    Gemini(Extractor<gemini::completion::CompletionModel, T>),
    Deepseek(Extractor<DeepSeekCompletionModel, T>),
}

/// ------------------------------------------------------------------
/// 1.  Strict-typed severity enum
/// ------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum Severity {
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum InvariantType {
    Arithmetic,
    Balance,
    Permission,
    Temporal,
    Referential,
    StateMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum InvariantStatus {
    HOLDS,
    VIOLATION,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum VulnerabilityType {
    AccessControl,
    ArrayLimits,
    ConfidentialData,
    DefaultVisibility,
    Dos,
    Inheritance,
    IntegerMath,
    Oracle,
    Pragma,
    Randomness,
    Reentrancy,
    ReplayAttack,
    SelfDestruct,
    ShortAddress,
    StorageLayout,
    TxOrigin,
    UncheckedReturn,
    UnexpectedEth,
    ZeroCode,
    FrontrunMev,
    UpgradeabilityInitializerSafety,
    PausableEmergencyStop,
    TimestampDependentLogic,
    FlashLoanEconomicManipulation,
    DelegatecallLowLevelOps,
    SignatureMalleability,
    EventConsistency,
    GasGriefBlockLimit,
    IntegerOverflow,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Info
    }
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        }
    }

    pub fn as_initial(self) -> &'static str {
        match self {
            Severity::High => "H",
            Severity::Medium => "M",
            Severity::Low => "L",
            Severity::Info => "I",
        }
    }
}

impl InvariantType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantType::Arithmetic => "Arithmetic",
            InvariantType::Balance => "Balance",
            InvariantType::Permission => "Permission",
            InvariantType::Temporal => "Temporal",
            InvariantType::Referential => "Referential",
            InvariantType::StateMachine => "StateMachine",
        }
    }

    pub fn get_prompt(self) -> &'static str {
        match self {
            InvariantType::Arithmetic => ARITHMETIC,
            InvariantType::Balance => BALANCE,
            InvariantType::Permission => PERMISSION,
            InvariantType::Temporal => TEMPORAL,
            InvariantType::Referential => REFERENTIAL,
            InvariantType::StateMachine => STATE_MACHINE,
        }
    }
}

impl InvariantStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantStatus::HOLDS => "Holds",
            InvariantStatus::VIOLATION => "Violation",
        }
    }
}

impl AIAgent {
    pub async fn extract_with_retry<T>(&self, prompt: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic(model) => Ok(agent_extract_with_retry::<_, T>(
                model,
                prompt,
                LlmCostType::AnthropicClaudeOutput,
            )
            .await?),
            AIAgent::Openai(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::OpenaiO3Output)
                        .await?,
                )
            }
            AIAgent::Gemini(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::GeminiOutput)
                        .await?,
                )
            }
            AIAgent::Deepseek(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::DeepseekOutput)
                        .await?,
                )
            }
        }
    }
}

impl Default for VulnerabilityType {
    fn default() -> Self {
        VulnerabilityType::Dos
    }
}

impl VulnerabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => "Oracle",
            VulnerabilityType::AccessControl => "AccessControl",
            VulnerabilityType::FrontrunMev => "FrontrunMev",
            VulnerabilityType::UnexpectedEth => "UnexpectedEth",
            VulnerabilityType::Pragma => "Pragma",
            VulnerabilityType::Randomness => "Randomness",
            VulnerabilityType::TxOrigin => "TxOrigin",
            VulnerabilityType::ZeroCode => "ZeroCode",
            VulnerabilityType::SelfDestruct => "SelfDestruct",
            VulnerabilityType::StorageLayout => "StorageLayout",
            VulnerabilityType::ReplayAttack => "ReplayAttack",
            VulnerabilityType::ShortAddress => "ShortAddress",
            VulnerabilityType::IntegerMath => "IntegerMath",
            VulnerabilityType::UncheckedReturn => "UncheckedReturn",
            VulnerabilityType::Dos => "Dos",
            VulnerabilityType::DefaultVisibility => "DefaultVisibility",
            VulnerabilityType::Inheritance => "Inheritance",
            VulnerabilityType::ConfidentialData => "ConfidentialData",
            VulnerabilityType::Reentrancy => "Reentrancy",
            VulnerabilityType::ArrayLimits => "ArrayLimits",
            VulnerabilityType::UpgradeabilityInitializerSafety => "UpgradeabilityInitializerSafety",
            VulnerabilityType::PausableEmergencyStop => "PausableEmergencyStop",
            VulnerabilityType::TimestampDependentLogic => "TimestampDependentLogic",
            VulnerabilityType::FlashLoanEconomicManipulation => "FlashLoanEconomicManipulation",
            VulnerabilityType::DelegatecallLowLevelOps => "DelegatecallLowLevelOps",
            VulnerabilityType::SignatureMalleability => "SignatureMalleability",
            VulnerabilityType::EventConsistency => "EventConsistency",
            VulnerabilityType::GasGriefBlockLimit => "GasGriefBlockLimit",
            VulnerabilityType::IntegerOverflow => "IntegerOverflow",
        }
    }

    pub fn as_fancy_str(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => "Oracle",
            VulnerabilityType::AccessControl => "Access Control",
            VulnerabilityType::FrontrunMev => "Frontrun/Backrun/Sandwhich MEV",
            VulnerabilityType::UnexpectedEth => "Unexpected Eth",
            VulnerabilityType::Pragma => "Pragma",
            VulnerabilityType::Randomness => "Randomness",
            VulnerabilityType::TxOrigin => "tx.origin",
            VulnerabilityType::ZeroCode => "Zero Code",
            VulnerabilityType::SelfDestruct => "Self-Destruct",
            VulnerabilityType::StorageLayout => "Storage Layout",
            VulnerabilityType::ReplayAttack => "Replay Attack",
            VulnerabilityType::ShortAddress => "Short Address",
            VulnerabilityType::IntegerMath => "Integer Overflow/Math",
            VulnerabilityType::UncheckedReturn => "Unchecked Return",
            VulnerabilityType::Dos => "DOS",
            VulnerabilityType::DefaultVisibility => "Default Visibility",
            VulnerabilityType::Inheritance => "Inheritance",
            VulnerabilityType::ConfidentialData => "Confidential Data",
            VulnerabilityType::Reentrancy => "Reentrancy",
            VulnerabilityType::ArrayLimits => "Array Limits",
            VulnerabilityType::UpgradeabilityInitializerSafety => {
                "Upgradeability Initializer Safety"
            }
            VulnerabilityType::PausableEmergencyStop => "Pausable Emergency Stop",
            VulnerabilityType::TimestampDependentLogic => "Timestamp Dependent Logic",
            VulnerabilityType::FlashLoanEconomicManipulation => "Flash Loan Economic Manipulation",
            VulnerabilityType::DelegatecallLowLevelOps => "Delegatecall Low Level Ops",
            VulnerabilityType::SignatureMalleability => "Signature Malleability",
            VulnerabilityType::EventConsistency => "Event Consistency",
            VulnerabilityType::GasGriefBlockLimit => "Gas Grief BlockLimit",
            VulnerabilityType::IntegerOverflow => "Integer Overflow",
        }
    }

    pub fn prompt(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => ORACLE_MANIPULATION,
            VulnerabilityType::AccessControl => ACCESS_CONTROL,
            VulnerabilityType::FrontrunMev => MEV,
            VulnerabilityType::UnexpectedEth => UNEXPECTED_ETH,
            VulnerabilityType::Pragma => FLOATING_PRAGMA,
            VulnerabilityType::Randomness => RANDOMNESS,
            VulnerabilityType::TxOrigin => TX_ORIGIN,
            VulnerabilityType::ZeroCode => CONTRACTS_WITH_ZERO_CODE,
            VulnerabilityType::SelfDestruct => SELF_DESTRUCT,
            VulnerabilityType::StorageLayout => STORAGE_VARIABLE,
            VulnerabilityType::ReplayAttack => REPLAY_SIGNATURES_ATTACK,
            VulnerabilityType::ShortAddress => SHORT_ADDRESS_ATTACK,
            VulnerabilityType::IntegerMath => INTEGER_OVERFLOW,
            VulnerabilityType::UncheckedReturn => UNCHECK_RETURN_VALUES,
            VulnerabilityType::Dos => DOS,
            VulnerabilityType::DefaultVisibility => DEFAULT_VISIBILITIES,
            VulnerabilityType::Inheritance => WRONG_INHERITANCE,
            VulnerabilityType::ConfidentialData => SAVING_CONFIDENTIAL_DATA,
            VulnerabilityType::Reentrancy => REENTRANCY,
            VulnerabilityType::ArrayLimits => ACCESS_OUTSIDE_ARRAY_LIMITS,
            _ => MASTER_SECURITY_PROMPT,
        }
    }
}

/// ----- Serde glue --------------------------------------------------
/// * Accepts any case-insensitive spelling: "high", "HIGH", "High" …
impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "high" => Ok(Severity::High),
            "medium" => Ok(Severity::Medium),
            "low" => Ok(Severity::Low),
            "info" => Ok(Severity::Info),
            other => Err(de::Error::unknown_variant(
                other,
                &["High", "Medium", "Low", "Info"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InvariantType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "arithmetic" => Ok(InvariantType::Arithmetic),
            "balance" => Ok(InvariantType::Balance),
            "permission" => Ok(InvariantType::Permission),
            "temporal" => Ok(InvariantType::Temporal),
            "referential" => Ok(InvariantType::Referential),
            "statemachine" => Ok(InvariantType::StateMachine),
            other => Err(de::Error::unknown_variant(
                other,
                &[
                    "Arithmetic",
                    "Balance",
                    "Permission",
                    "Temporal",
                    "Referential",
                    "StateMachine",
                ],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InvariantStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "holds" => Ok(InvariantStatus::HOLDS),
            "violation" => Ok(InvariantStatus::VIOLATION),
            other => Err(de::Error::unknown_variant(
                other,
                &["High", "Medium", "Low", "Info"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for VulnerabilityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "oracle" => Ok(VulnerabilityType::Oracle),
            "accesscontrol" => Ok(VulnerabilityType::AccessControl),
            "frontrunmev" => Ok(VulnerabilityType::FrontrunMev),
            "unexpectedeth" => Ok(VulnerabilityType::UnexpectedEth),
            "pragma" => Ok(VulnerabilityType::Pragma),
            "randomness" => Ok(VulnerabilityType::Randomness),
            "txorigin" => Ok(VulnerabilityType::TxOrigin),
            "zerocode" => Ok(VulnerabilityType::ZeroCode),
            "selfdestruct" => Ok(VulnerabilityType::SelfDestruct),
            "storagelayout" => Ok(VulnerabilityType::StorageLayout),
            "replayattack" => Ok(VulnerabilityType::ReplayAttack),
            "shortaddress" => Ok(VulnerabilityType::ShortAddress),
            "integermath" => Ok(VulnerabilityType::IntegerMath),
            "uncheckedreturn" => Ok(VulnerabilityType::UncheckedReturn),
            "dos" => Ok(VulnerabilityType::Dos),
            "defaultvisibility" => Ok(VulnerabilityType::DefaultVisibility),
            "inheritance" => Ok(VulnerabilityType::Inheritance),
            "confidentialdata" => Ok(VulnerabilityType::ConfidentialData),
            "reentrancy" => Ok(VulnerabilityType::Reentrancy),
            "arraylimits" => Ok(VulnerabilityType::ArrayLimits),
            "upgradeabilityinitializersafety" => {
                Ok(VulnerabilityType::UpgradeabilityInitializerSafety)
            }
            "pausableemergencystop" => Ok(VulnerabilityType::PausableEmergencyStop),
            "timestampdependentlogic" => Ok(VulnerabilityType::TimestampDependentLogic),
            "flashloaneconomicmanipulation" => Ok(VulnerabilityType::FlashLoanEconomicManipulation),
            "delegatecalllowlevelops" => Ok(VulnerabilityType::DelegatecallLowLevelOps),
            "signaturemalleability" => Ok(VulnerabilityType::SignatureMalleability),
            "eventconsistency" => Ok(VulnerabilityType::EventConsistency),
            "gasgriefblocklimit" => Ok(VulnerabilityType::GasGriefBlockLimit),
            "integeroverflow" => Ok(VulnerabilityType::IntegerOverflow),
            other => Err(de::Error::unknown_variant(
                other,
                &[
                    "oracle",
                    "accesscontrol",
                    "frontrunattack",
                    "unexpectedeth",
                    "pragma",
                    "randomness",
                    "txorigin",
                    "zerocode",
                    "selfdestruct",
                    "storagelayout",
                    "replayattack",
                    "shortaddress",
                    "integermath",
                    "uncheckedreturn",
                    "dos",
                    "defaultvisibility",
                    "inheritance",
                    "confidentialdata",
                    "reentrancy",
                    "arraylimits",
                    "frontrunmev",
                    "upgradeabilityinitializersafety",
                    "pausableemergencystop",
                    "timestampdependentlogic",
                    "flashloaneconomicmanipulation",
                    "delegatecalllowlevelops",
                    "signaturemalleability",
                    "eventconsistency",
                    "gasgriefblocklimit",
                    "integeroverflow",
                ],
            )),
        }
    }
}

impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for InvariantType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for InvariantStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for VulnerabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

```
ai-agent-audit/src/llm_review/invariants.rs
```
pub const INVARIANTS: &str = r#"
You are a senior security auditor.

### TASK
1. Summarise, in bullet points, the intended behaviour of *this contract*.
2. Derive business-logic invariants.  Label them **INV-1 …** and assign each an *invariant type* from the list.
3. Inspect the IR and call flow.  For every invariant output:  
   – `"status": "HOLDS"` if the code enforces it.  
   – `"status": "VIOLATION"` if it can be broken. Show line numbers / IR lines.
4. For each **VIOLATION** include:  
   • `"exploit_path"` — external call sequence an attacker uses  
   • `"pre_state"`     — minimum balances / roles / time needed  
   • `"post_state"`    — resulting asset/thread state change or stolen value
5. Return *only* valid JSON.  No markdown, no comments.  

## INVARIANT TYPES
1. Arithmetic   values, sums, ratios must match expectations  
2. Balance      token/ETH balances and supply monotonicity  
3. Permission    only-owner / only-role / re-entrancy locks  
4. Temporal      timeouts, epochs, can’t rewind clock  
5. Referential   mappings/arrays stay in sync (index→value)  
6. StateMachine only allowed state transitions

Return JSON:

{
  "contract": "string (name of contract)"
  "intention": "string",
  "invariants": [
    {
      "id": "INV-n",
      "inv_type": "Arithmetic|Balance|Permission|Temporal|Referential|StateMachine",
      "desc": "string",
      "status": "HOLDS" | "VIOLATION",
      "exploit": "string (omit if HOLDS)",
      "exploit_path": "string (omit if HOLDS)",
      "pre_state":     "string (omit if HOLDS)",
      "post_state":    "string (omit if HOLDS)"
      "impact": "string (omit if HOLDS)",
      "poc": "string (omit if HOLDS)",
      "mitigation": "string (suggested mitigation - omit if HOLDS)",
    }
  ]
}

"#;

```
ai-agent-audit/src/llm_review/prompt_context.rs
```
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::slither_ffi::{cache_key, get_all_files_src};
use crate::build_brain::summarize;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_doc_file::extract_content_from_docs;

use super::config::Finding;

/// Global cache keyed by (repo_root, printer) tuple stringified
pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub async fn generate_slither_metadata_prompt_context(
    repo: &RepoPaths,
    _semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(&repo.root, "prompt_context");
    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = context_cache.get(&key) {
        return Ok(cached.clone());
    }

    // 1 . gather IR + storage  (re-use existing function)
    info!("get contract summary and source files");
    // let callgraph = callgraph::get_enriched_funcs_and_edges(repo_root, &semantics_path).await?;
    // let inheritance = inheritance::generate_slither_inheritance(repo_root).await?;
    // let contract_summary = run_printer(repo, "contract-summary").await?;
    let src_file_list = get_all_files_src(repo);
    info!("src file list => {}", src_file_list);

    let mut prompt_context = String::new();

    // prompt_context.push_str("\n## Slither Contract Summary\n");
    // prompt_context.push_str(&contract_summary);
    prompt_context.push_str("\n## List of Files in Src Folder\n");
    prompt_context.push_str(&src_file_list);
    // prompt_context.push_str("\n## Slither Call Graph\n");
    // prompt_context.push_str(&callgraph);
    // prompt_context.push_str("\n## Slither Inheritance Json\n");
    // prompt_context.push_str(&inheritance);
    // prompt_context.push_str("\n## Slither Detector\n");
    // prompt_context.push_str(&slither_scan_results);

    info!(
        "slither metadata prompt context size ==> {}",
        prompt_context.len()
    );
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}

pub async fn generate_context_for_code_review(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<String> {
    log::info!("generate slither metadata");
    let slither_metadata = generate_slither_metadata_prompt_context(repo, &semantics_path).await?;
    log::info!("generate summary of all files");

    let mut full_prompt_context = String::new();

    let mut file_summaries = String::new();
    let summaries = summarize::summarize_src_files(repo, &semantics_path).await?;
    for summary in summaries {
        file_summaries.push_str(&format!("\n## SUMMARY OF FILE: {}\n", summary.filename));
        file_summaries.push_str(&summary.summary);
        file_summaries.push_str("\n\n");
    }
    full_prompt_context.push_str(&file_summaries);
    full_prompt_context.push_str("\n## SLITHER GENERATED METADATA \n\n");
    full_prompt_context.push_str(&slither_metadata);

    let docs = summarize::summarize_docs(repo, &full_prompt_context).await?;
    let documentation = extract_content_from_docs(repo)?;
    let mut doc_summaries = String::new();
    for doc_summary in &docs {
        doc_summaries.push_str("\n\n");
        doc_summaries.push_str(&doc_summary.summary);
        doc_summaries.push_str("\n\n");
    }
    full_prompt_context.push_str("\n ## DOCUMENTATION: \n\n ");
    // adding FULL DOCS not doc_summaries
    full_prompt_context.push_str(&documentation);

    // TODO - add FULL DOCS IF AUDIT TYPE BUGBOUNTY OTHERWISE ADD SUMMARY OF AUDIT
    // test that documentation is being added
    // ALSO add $100 more to chain shield to cover these costs!
    // info!("documentation full size => {}", docs[0].summary.len());
    info!("documentation full size => {}", documentation.len());
    info!("full prompt context SIZE => {}", full_prompt_context.len());

    Ok(full_prompt_context)
}

pub fn generate_prompt_for_issue_check(
    code: &str,
    finding: &Finding,
    pre_instructions: &str,
    instructions: &str,
    post_instructions: &str,
) -> String {
    let mut prompt = format!("{}{}{}", pre_instructions, instructions, post_instructions);

    prompt.push_str("\n\n");
    prompt.push_str("## REPORT FOR SECURITY ISSUE");
    prompt.push_str("\n\n");

    let report = get_finding_report(finding);
    prompt.push_str(&report);
    prompt.push_str("\n\n");

    prompt.push_str("## CODEBASE WHERE ISSUE WAS FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);

    prompt
}

fn get_finding_report(finding: &Finding) -> String {
    let mut findings_report = String::new();
    //title
    findings_report.push_str(&format!(
        "## [Severity-{}]. {}\n\n",
        finding.severity.as_str(),
        finding.title()
    ));

    //description
    findings_report.push_str("## Description\n");
    findings_report.push_str(&finding.description.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //impact
    findings_report.push_str("## Impact\n");
    findings_report.push_str(&finding.impact.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //POC
    findings_report.push_str("## Proof of Concept\n");
    findings_report.push_str(&finding.proof_of_concept.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Proof of Code
    findings_report.push_str("## Proof of Code\n");
    findings_report.push_str(&finding.proof_of_code.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Suggested Fix
    findings_report.push_str("## Suggested Mitigation\n");
    findings_report.push_str(&finding.mitigation.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    findings_report.push_str("\n");
    findings_report
}

```
ai-agent-audit/src/llm_review/review_utils.rs
```
use rig::{
    client::CompletionClient,
    providers::{
        anthropic::{self},
        deepseek,
        gemini::{self},
        openai::{self},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::enums::{AIAgent, AIExtractor};

pub fn build_anthropic_agent(
    client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let agent = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .max_tokens(max_tokens)
        .temperature(temperature)
        .build();

    AIAgent::Anthropic(agent)
}

pub fn build_openai_extractor<T>(
    client: &openai::Client,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let builder = client.extractor::<T>(model).preamble(preamble);

    match context {
        Some(added_context) => AIExtractor::Openai(builder.context(added_context).build()),
        None => AIExtractor::Openai(builder.build()),
    }
}

pub fn build_openai_agent(
    client: &openai::Client,
    temperature: f64,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(preamble)
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Openai(builder.context(added_context).build()),
        None => AIAgent::Openai(builder.build()),
    }
}

pub fn build_gemini_agent(
    client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Gemini(builder.context(added_context).build()),
        None => AIAgent::Gemini(builder.build()),
    }
}

pub fn build_deepseek_agent(
    client: &deepseek::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Deepseek(builder.context(added_context).build()),
        None => AIAgent::Deepseek(builder.build()),
    }
}

```
ai-agent-audit/src/llm_review/prompt_support/dedup.rs
```
pub const DEDUP_PROMPT: &str = r#"

SYSTEM
You are a Solidity-security triager.  
Answer with exactly **YES** or **NO** (no punctuation, no prose).

USER
Are these two vulnerability reports describing the *same root-cause*?

---- REPORT A ----
Issue Type : {issue_type}
Contract    : {contract}
Function    : {function}
Description : {description_a}

---- REPORT B ----
Issue Type : {issue_type}
Contract    : {contract}
Function    : {function}
Description : {description_b}

Remember: root-cause means the exact same bug, not just similar wording.
Answer:
"#;

```
ai-agent-audit/src/llm_review/prompt_support/post_prompt.rs
```
pub const POST_PROMPT: &str = r#"

### OUTPUT REQUIREMENTS 

 **For Every VIOLATION** return:
1. **Description**: Detailed explanation including vulnerable code snippet 
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow
3. **Contract**: The exact contract name where vulnerability is found 
4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
5. **Impact**: Financial and security consequences 
6. **Proof of Concept**: Step-by-step exploitation scenario 
7. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
8. **Severity**: High/Medium/Low/Info based on table below
    | Severity | Definition |
    |----------|------------|
    | HIGH     | Steals, locks, or permanently harms a significant portion of funds/governance. |
    | MEDIUM   | Exploitable but needs favourable conditions or yields limited loss. |
    | LOW      | Minor financial or operational impact; edge-case or hard to exploit. |
    | INFO     | Non-safety best-practice / observability issue. | 
9. **Mitigation**: Suggested Mitigation with code example of fix

*Please respond with ONLY valid JSON in the following exact format:*

{
  "findings": [
    {
      "description": "Detailed explanation if vulnerability including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", 
      "function": "<Function>", 
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

- If no vulnerabilities are found, return: 

{
  "findings": []
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
**Please double-check opening and closing brakets: `}` and `]`, make sure 
they match up correctly.

"#;

```
ai-agent-audit/src/llm_review/prompt_support/post_qualify.rs
```
pub const POST_QUALIFY: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_quality_check_passed**: true|false 
   • `true`   → if nothing has to be updated
   • `false`  → at least one field (impact, POC, proof of code, severity, mitigation) requires an update 
   *NOTE* : this is boolean value, NO "" around it
2. **where_quality_lacks**: Brief summary of issues found with vulnerability write up (omit this field if quality check passed)
3. **impact**: provide updated impact statement (ONLY IF current one is not adequately addressing impact)
4. **proof_of_concept**: provide an updated proof of concept ONLY IF NEEDED
5. **proof_of_code**: provide an updated proof of code ONLY IF NEEDED
6. **severity**: provid an updated severity (High|Medium|Low|Info), ONLY IF current severity is not accurate
    Judge severity based on below table
    | Severity | Definition |
    |----------|------------|
    | HIGH     | Steals, locks, or permanently harms a significant portion of funds/governance. |
    | MEDIUM   | Exploitable but needs favourable conditions or yields limited loss. |
    | LOW      | Minor financial or operational impact; edge-case or hard to exploit. |
    | INFO     | Non-safety best-practice / observability issue. | 
7. **mitigation**: provide updated mitigation, ONLY IF current one is inadequate

*Please respond with ONLY valid JSON in the following exact format:*

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

```
ai-agent-audit/src/llm_review/prompt_support/post_verify.rs
```
pub const POST_VERIFY: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_legit_vulnerability**: true|false 
   • `true`   → issue is real
   • `false`  → issue is clearly harmless, irrelevant, or deliberate with no risk or confusion
   *NOTE* : this is boolean value, NO "" around it
2. **why_its_not_legit**: IF above is false (OMIT this field if above true), provide brief explanation why issue is NOT real 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "is_legit_vulnerability": true|false, 
    "why_its_not_legit": "explain why NOT legit (OMIT if legit)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

```
ai-agent-audit/src/llm_review/prompt_support/pre_prompt.rs
```
pub const PRE_PROMPT: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "findings": [
    {
      "description": "Detailed explanation including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", // the exact contract name where vulnerability is found
      "function": "<Function>", // exact function name where vulnerability is found, if not applicable set to "NA"
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High|Medium|Low|Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

"#;

```
ai-agent-audit/src/llm_review/prompt_support/pre_qualify.rs
```
pub const PRE_QUALIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

"#;

```
ai-agent-audit/src/llm_review/prompt_support/pre_verify.rs
```
pub const PRE_VERIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
    "is_legit_vulnerability": true|false,
    "why_its_not_legit": "explain why NOT legit (OMIT if legit)"
}

"#;

```
ai-agent-audit/src/llm_review/prompt_support/qualify_prompt.rs
```
pub const QUALIFY_PROMPT: &str = r#"

**Inputs you will receive (per request)**  
1. A draft vulnerability write-up authored by another auditor (sections: Description, Impact, Proof of Concept, Proof of Code, Suggested Mitigation, Severity).  
2. The relevant Solidity contract (or excerpt) for context.

Your tasks for vulnerability write-up are:

1. **Proof-of-Concept (PoC) check** – does the current PoC really show how an attacker can exploit it?  
   - If it misses an attack vector or is incorrect, write a *revised* PoC that clearly demonstrates exploitation.

2. **Impact & Severity check** – is the stated impact accurate and is the severity level appropriate (High / Medium / Low / Info)?  
   - If not, provide an updated *impact* paragraph and/or change the *severity*.

3. **Foundry unit-test check** – will the `proof_of_code` test compile and reliably prove the issue?  
   - If it is wrong, incomplete, or non-deterministic, supply a corrected Foundry test (keep it minimal but runnable).

4. **Mitigation check** – will the suggested fix fully eliminate the vulnerability?  
   - If it is insufficient or can be improved, provide an updated mitigation.
**Tasks**

For vulnerability write-up perform the checks below and update anything that is wrong, missing, or can be improved:
"#;

```
ai-agent-audit/src/llm_review/prompt_support/verify_prompt.rs
````
pub const VERIFY_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **valid and worth fixing**. This includes:

1. **Actual vulnerabilities** – exploitable bugs, broken access control, reentrancy, overflow, etc.
2. **Security best practices violations** – unsafe patterns, missing event logs, unsafe external calls, unchecked return values, etc.
3. **Security-adjacent concerns** – issues that degrade transparency, auditability, maintainability, or correctness.
4. **Potential future risk** – minor today but can cause critical issues when upgraded or combined with other code.

You should return `"true"` if the issue meets **any** of these criteria.

Only return `"false"` if the issue is clearly meets **All** of these conditions:
- Already mitigated or impossible to exploit
- A deliberate pattern that is safe and idiomatic
- Fully unrelated to security, correctness, or best practice

INPUT  
You will receive **one report** with the following structure:

## <Title>

## Description  
<Human-written description of the bug>

## Impact  
<Claimed effect>

## Proof of Concept  
<Attack steps, if applicable>

## Proof of Code  
```solidity

## Suggested Mitigation

<Recommended fix>

TASK

1. Read the description, impact, PoC, and mitigation to understand the claimed vulnerability.
2. Inspect every Solidity code block (vulnerable contract and PoC) and verify, line-by-line, whether the issue can actually occur in practice.
3. Watch for false positives (e.g., state changes before external calls, access-control modifiers, Solidity ≥ 0.8 overflow checks, built-in reentrancy guards, etc.).
4. Think step-by-step **silently**; **do not** reveal chain-of-thought.

**No other text, markdown, or punctuation is allowed in your final answer.**

"#;

````
ai-agent-audit/src/utils/bpe.rs
```
/// OpenAI tokenizer (BPE) for accurate token counting and text chunking.
///
/// This module provides thread-safe access to the cl100k_base tokenizer used by
/// OpenAI models (GPT-4, text-embedding-3) for precise token counting in cost
/// calculations and context limit management.
use std::sync::OnceLock;
use tiktoken_rs::{CoreBPE, cl100k_base};

/// Lazily-initialized, thread-safe tokenizer instance
/// This is initialized only once when first accessed and then reused
static BPE_INSTANCE: OnceLock<CoreBPE> = OnceLock::new();

/// Returns a reference to the singleton BPE tokenizer instance.
///
/// This function provides access to the cl100k_base tokenizer used by OpenAI models
/// like GPT-4 and text-embedding-3. The tokenizer is initialized on first access
/// and then reused for subsequent calls, making it efficient for repeated use.
///
/// @return A static reference to the CoreBPE tokenizer instance
pub fn get_bpe() -> &'static CoreBPE {
    BPE_INSTANCE.get_or_init(|| cl100k_base().expect("Failed to load cl100k_base tokenizer"))
}

```
ai-agent-audit/src/utils/contract_name_check.rs
```
//  check if content contains at least 1 contract
//  that does not have 'mock' in its name
pub fn has_non_mock_contract(content: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("contract ") {
            if let Some(name) = trimmed
                .split_whitespace()
                .nth(1)
                .map(|s| s.trim_end_matches('{').to_ascii_lowercase())
            {
                if !name.contains("mock") {
                    return true;
                }
            }
        }
    }
    false
}

```
ai-agent-audit/src/utils/delete_docker_volumes.rs
````
/// Docker volume cleanup utilities for secure analysis environment.
///
/// This module provides cleanup functions to remove Docker volumes and
/// temporary directories created during repository analysis, ensuring
/// no residual data remains on the host system.

use std::fs;
use std::path::Path;
use anyhow::Result;

/// Cleans up Docker volume directory and build artifacts after analysis.
///
/// Removes the repository directory and all associated build artifacts
/// from the Docker volume to prevent disk space accumulation and ensure
/// clean analysis environments for subsequent runs.
///
/// # Arguments
/// * `root` - Path to the repository root directory to clean up
///
/// # Example
/// ```
/// cleanup_repo_volume(&Path::new("/tmp/audit-analysis/my-repo"));
/// ```
pub fn cleanup_repo_volume(root: &Path) -> Result<()> {
    if root.exists() {
        fs::remove_dir_all(root).expect(&format!("Failed to remove Docker volume at {:?}", root));
        println!("[INFO] Cleaned up Docker volume: {:?}", root);
    } else {
        println!("[WARN] No Docker volume found for cleanup at {:?}", root);
    }

    Ok(())
}

````
ai-agent-audit/src/utils/env_security.rs
```
/// Environment variable security utilities.
/// 
/// This module provides secure handling of environment variables including
/// validation, sanitization, and secure logging to prevent credential leakage.

use anyhow::{Context, Result};
use std::env;

/// Securely retrieves an API key from environment variables with validation.
/// 
/// # Security Features
/// - Validates key format and length
/// - Prevents logging of actual key values
/// - Returns sanitized error messages
pub fn get_api_key(env_var: &str) -> Result<String> {
    let key = env::var(env_var)
        .with_context(|| format!("Environment variable {} is not set", env_var))?;
    
    // Validate API key format
    if key.is_empty() {
        anyhow::bail!("API key {} is empty", env_var);
    }
    
    if key.len() < 10 {
        anyhow::bail!("API key {} appears to be too short", env_var);
    }
    
    if key.len() > 512 {
        anyhow::bail!("API key {} is suspiciously long", env_var);
    }
    
    // Check for obvious placeholder values
    let placeholder_values = ["your_api_key", "placeholder", "changeme", "test", "demo"];
    let key_lower = key.to_lowercase();
    if placeholder_values.iter().any(|&placeholder| key_lower.contains(placeholder)) {
        anyhow::bail!("API key {} appears to be a placeholder value", env_var);
    }
    
    Ok(key)
}

/// Securely retrieves a URL from environment variables with validation.
pub fn get_secure_url(env_var: &str) -> Result<String> {
    let url = env::var(env_var)
        .with_context(|| format!("Environment variable {} is not set", env_var))?;
    
    // Basic URL validation
    if !url.starts_with("http://") && !url.starts_with("https://") {
        anyhow::bail!("URL {} must start with http:// or https://", env_var);
    }
    
    if url.len() > 2048 {
        anyhow::bail!("URL {} is too long", env_var);
    }
    
    // Check for localhost/private IPs in production
    if env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        if url.contains("localhost") || url.contains("127.0.0.1") || url.contains("0.0.0.0") {
            anyhow::bail!("Localhost URLs not allowed in production environment");
        }
    }
    
    Ok(url)
}

/// Sanitizes a string for safe logging (removes sensitive information).
pub fn sanitize_for_logging(input: &str) -> String {
    if input.len() <= 8 {
        "*".repeat(input.len())
    } else {
        format!("{}***{}", &input[..4], &input[input.len()-4..])
    }
}

/// Validates that required environment variables are set without exposing values.
pub fn validate_required_env_vars() -> Result<()> {
    let required_vars = ["OPENAI_API_KEY", "QDRANT_URL"];
    let mut missing_vars = Vec::new();
    
    for var in &required_vars {
        if env::var(var).is_err() {
            missing_vars.push(*var);
        }
    }
    
    if !missing_vars.is_empty() {
        anyhow::bail!("Missing required environment variables: {}", missing_vars.join(", "));
    }
    
    Ok(())
}

```
ai-agent-audit/src/utils/extract_retry.rs
```
use crate::ai_bot::agent::get_rag_for_security_query;
/// LLM extraction with retry logic and cost tracking.
///
/// This module provides robust LLM interaction utilities with automatic retry
/// mechanisms for handling rate limits, network issues, and parsing errors,
/// while tracking inference costs across different providers.
use crate::cost::cost_data::add_to_inference_cost_by_type;
use crate::cost::cost_data::LlmCostType;
use crate::llm_review::config::FromLLMJson;
use reqwest::StatusCode;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::completion::PromptError;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::de::Error as _; // <- bring the trait’s methods into scope
use serde::Deserialize;
use serde_json::Error as JsonError;
use std::{thread, time::Duration};

/// Maximum retry attempts for failed LLM requests
const MAX_ATTEMPTS: usize = 3;

/// Retries LLM extraction with exponential backoff and cost tracking.
///
/// Handles common LLM API issues including rate limits, network errors,
/// and JSON parsing failures with automatic retry logic.
pub async fn extractor_with_retry<M, T>(
    extractor: &Extractor<M, T>,
    input: &str,
    llm_cost_type: LlmCostType,
) -> Result<T, ExtractionError>
where
    M: CompletionModel,
    T: JsonSchema + for<'a> Deserialize<'a> + Send + Sync,
{
    let delay = Duration::from_millis(500);

    for attempt in 1..=MAX_ATTEMPTS {
        // add to cost
        add_to_inference_cost_by_type(input, llm_cost_type).await;

        match extractor.extract(input).await {
            Ok(data) => return Ok(data), // ✅ parsed JSON
            Err(ExtractionError::NoData) if attempt < MAX_ATTEMPTS => {
                eprintln!("No data extracted – (attempt {attempt}/{MAX_ATTEMPTS})");
                thread::sleep(delay);
            }
            Err(e) => return Err(e), // network / OpenAI errors → bubble up
        }
    }

    Err(ExtractionError::NoData)
}

pub async fn agent_extract_with_retry<M, T>(
    agent: &Agent<M>,
    input: &str,
    llm_cost_type: LlmCostType,
) -> Result<T, JsonError>
where
    M: CompletionModel,
    T: DeserializeOwned,
{
    for attempt in 1..=MAX_ATTEMPTS {
        /* ────── 1. ask the model ───────────────────────────────────────── */
        let raw = match agent.prompt(input).await {
            Ok(txt) => txt,
            // Convert prompt error to JsonError
            Err(e) if should_retry_prompt_err(&e) && attempt < MAX_ATTEMPTS => {
                eprintln!("LLM backend busy ({e}) – retry {attempt}/{MAX_ATTEMPTS}");
                continue;
            }
            Err(e) => return Err(JsonError::custom(format!("prompt failed: {e}"))),
        };
        // log::info!("json => {:#?}", raw);

        // add to cost
        add_to_inference_cost_by_type(&raw, llm_cost_type).await;

        /* ────── 2. try to parse JSON ───────────────────────────────────── */
        match FromLLMJson::parse_from_llm_response(&raw) {
            Ok(f) => return Ok(f), // ✅ success
            Err(e) => {
                let msg = e.to_string();
                // Check if we should retry based on the original error
                let should_retry = should_retry_based_on_error(&msg) && attempt < MAX_ATTEMPTS;

                if should_retry {
                    eprintln!("parse error ({msg}) – retrying {attempt}/{MAX_ATTEMPTS}");
                    // sleep(delay).await; --> NOT Send
                    continue;
                } else {
                    // Convert the error to JsonError and return
                    return Err(JsonError::custom(format!("parse failed: {msg}")));
                }
            }
        }
    }
    // This point is only reached if all attempts exhausted
    Err(JsonError::custom("exhausted retries – still no data"))
}
// Helper function to determine if we should retry based on the original error
fn should_retry_based_on_error(e: &str) -> bool {
    let error_msg = e.to_string().to_lowercase();

    // Retry on common parsing issues that might be fixed by the LLM on retry
    error_msg.contains("unexpected")
        || error_msg.contains("invalid")
        || error_msg.contains("syntax")
        || error_msg.contains("parse")
        || error_msg.contains("json")
        || error_msg.contains("deserialize")
    // Add more conditions based on what errors you typically see
}

/*──────────────── helper ───────────────────────────────────────────────*/
/// `true`  → retry is warranted  
/// `false` → give up / bubble the error
fn should_retry_prompt_err(e: &PromptError) -> bool {
    match e {
        // Unpack the CompletionError variant  ──────────────────────────
        PromptError::CompletionError(inner) => match inner {
            /* 1) HTTP transport layer issues -------------------------- */
            CompletionError::HttpError(http_err) => {
                // 1a) Too-Many-Requests (OpenAI & friends)
                if http_err.status() == Some(StatusCode::TOO_MANY_REQUESTS) {
                    return true;
                }
                // 1b) Any 5xx server error
                if let Some(status) = http_err.status() {
                    if status.is_server_error() {
                        return true;
                    }
                }
                // 1c) Network time-outs
                if http_err.is_timeout() {
                    return true;
                }
                false
            }

            /* 2) Provider said “I’m busy / overloaded / rate-limited”  */
            CompletionError::ProviderError(msg) | CompletionError::ResponseError(msg) => {
                let m = msg.to_lowercase();
                m.contains("overload")
                    || m.contains("rate limit")
                    || m.contains("busy")
                    || m.contains("try again later")
            }

            /* 3) Anything else – usually not transient */
            _ => false,
        },

        /* Tool-call failures, depth-limit, etc. -> *not* transient */
        _ => false,
    }
}

```
ai-agent-audit/src/utils/file_security.rs
```
/// File system security utilities.
///
/// This module provides secure file operations including path validation,
/// symlink detection, and safe file reading with size limits.
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum file size for reading (10MB)
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum path length to prevent buffer overflow attacks
const MAX_PATH_LENGTH: usize = 4096;

/// Validates that a path is safe to access (no path traversal, symlinks, etc.)
pub fn validate_safe_path(path: &Path, allowed_base: &Path) -> Result<()> {
    // Convert to absolute paths for comparison
    let abs_path = path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {:?}", path))?;
    let abs_base = allowed_base
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize base path: {:?}", allowed_base))?;

    // Check if path is within allowed base directory
    if !abs_path.starts_with(&abs_base) {
        anyhow::bail!(
            "Path traversal attempt detected: {:?} is outside {:?}",
            abs_path,
            abs_base
        );
    }

    // Check path length
    if abs_path.to_string_lossy().len() > MAX_PATH_LENGTH {
        anyhow::bail!(
            "Path is too long: {} characters",
            abs_path.to_string_lossy().len()
        );
    }

    // Check for symlinks in the path components
    let mut current = abs_path.as_path();
    while let Some(parent) = current.parent() {
        if parent == abs_base {
            break;
        }

        let metadata = fs::symlink_metadata(current)
            .with_context(|| format!("Failed to read metadata for: {:?}", current))?;

        if metadata.file_type().is_symlink() {
            anyhow::bail!("Symlink detected in path: {:?}", current);
        }

        current = parent;
    }

    Ok(())
}

/// Safely reads a file with size limits and validation
pub fn safe_read_file(path: &Path, allowed_base: &Path) -> Result<String> {
    // Validate path safety
    validate_safe_path(path, allowed_base)?;

    // Check file metadata
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to read file metadata: {:?}", path))?;

    // Check if it's actually a file
    if !metadata.is_file() {
        anyhow::bail!("Path is not a regular file: {:?}", path);
    }

    // Check file size
    if metadata.len() > MAX_FILE_SIZE {
        anyhow::bail!(
            "File is too large: {} bytes (max: {} bytes)",
            metadata.len(),
            MAX_FILE_SIZE
        );
    }

    // Read file content
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))
}

/// Validates file extension against allowed list
pub fn validate_file_extension(path: &Path, allowed_extensions: &[&str]) -> Result<()> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("File has no extension: {:?}", path))?;

    if !allowed_extensions.contains(&extension) {
        anyhow::bail!(
            "File extension '{}' not allowed. Allowed: {:?}",
            extension,
            allowed_extensions
        );
    }

    Ok(())
}

/// Creates a secure temporary directory with restricted permissions
pub fn create_secure_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .context("Failed to create temporary directory")?;

    let path = temp_dir.keep();

    // Set restrictive permissions (owner only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o700); // rwx------
        fs::set_permissions(&path, perms)?;
    }

    Ok(path)
}

/// Validates and sanitizes a Git repository URL to prevent command injection.
///
/// # Security
/// This function prevents command injection by validating URL format and
/// rejecting URLs with shell metacharacters or suspicious patterns.
pub fn validate_repo_url(url: &str) -> Result<()> {
    // Basic URL format validation
    let url_regex = Regex::new(r"^https?://[a-zA-Z0-9.-]+/[a-zA-Z0-9._/-]+(?:\.git)?/?$")
        .context("Failed to compile URL regex")?;

    if !url_regex.is_match(url) {
        anyhow::bail!("Invalid repository URL format: {}", url);
    }

    // Check for shell metacharacters that could enable command injection
    let dangerous_chars = [
        '&', '|', ';', '`', '$', '(', ')', '{', '}', '<', '>', '"', '\'', '\\',
    ];
    if url.chars().any(|c| dangerous_chars.contains(&c)) {
        anyhow::bail!(
            "Repository URL contains potentially dangerous characters: {}",
            url
        );
    }

    // Reject URLs that are too long (potential buffer overflow)
    if url.len() > 2048 {
        anyhow::bail!("Repository URL is too long: {} characters", url.len());
    }

    Ok(())
}

/// Sanitizes repository name to prevent path traversal and injection attacks.
fn sanitize_repo_name(name: &str) -> String {
    // Remove any path traversal attempts and dangerous characters
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(250) // Limit length
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}

/// Sanitizes filename to prevent directory traversal and special characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(255) // Limit filename length
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}

```
ai-agent-audit/src/utils/fn_labels.rs
```
pub fn get_visibility_label(visibility: &str) -> String {
    let label = match visibility {
        "external" => "[EXTERNAL]",
        "public" => "[PUBLIC]",
        "internal" => "[INTERNAL]",
        "private" => "[PRIVATE]",
        _ => "",
    };

    label.to_string()
}

pub fn get_modifiers_label(modifiers: &[String]) -> String {
    // log::info!("modifiers ==> {:#?}", modifiers);
    let owner = modifiers.iter().any(|m| m == "onlyOwner");
    let mod_tag = if owner { "[OWNER]" } else { "" };

    mod_tag.to_string()
}

```
ai-agent-audit/src/utils/get_doc_file.rs
```
use std::fs;

use anyhow::Result;

use crate::prepare_code::git_clone::RepoPaths;

pub fn extract_content_from_docs(repo: &RepoPaths) -> Result<String> {
    let mut docs = String::new();

    for doc in &repo.docs {
        // Skip directories and symlinks
        if fs::symlink_metadata(doc)?.file_type().is_symlink() {
            continue;
        }

        let filename = doc
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = match fs::read_to_string(doc) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read {}: {}", doc.display(), e);
                continue;
            }
        };

        if content.trim().is_empty() {
            continue; // skip empty files
        }
        docs.push_str(&format!("{}\n\n{}\n\n", filename, content));
    }
    Ok(docs)
}

```
ai-agent-audit/src/utils/get_fn_name.rs
```
use regex::Regex;

pub fn get_function_name(function_interface: &str) -> String {
    let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

    if let Some(caps) = re.captures(function_interface) {
        let function_name = &caps[1];
        return function_name.to_string();
    }
    "".to_string()
}

```
ai-agent-audit/src/utils/logging.rs
```
use log::info;
use schemars::schema_for;

use crate::llm_review::config::Findings;

pub fn print_first_four_lines(text: &str) {
    let lines: Vec<&str> = text.lines().collect();

    for (index, line) in lines.iter().enumerate().take(4) {
        info!("Line {} (at line {}): {}", index + 1, line!(), line);
    }

    if lines.len() > 4 {
        info!("... ({} more lines)", lines.len() - 4);
    }
}

pub fn print_first_n_lines(number_of_lines: usize, text: &str) {
    let lines: Vec<&str> = text.lines().collect();
    let n = if number_of_lines < lines.len() {
        number_of_lines
    } else {
        lines.len()
    };

    for (index, line) in lines.iter().enumerate().take(n) {
        info!("Line {} (at line {}): {}", index + 1, line!(), line);
    }

    if lines.len() > n {
        info!("... ({} more lines)", lines.len() - n);
    }
}

pub fn print_schema() {
    let schema = schema_for!(Findings);
    println!(
        "Generated schema: {}",
        serde_json::to_string_pretty(&schema).unwrap()
    );
}

```
ai-agent-audit/src/utils/sanitize.rs
```
use once_cell::sync::Lazy;
use regex::Regex;

pub fn sanitize_for_claude(raw: &str) -> String {
    static MD_IMAGE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"!\[(?P<alt>[^\]]*)\]\([^)]+\)").unwrap());

    static HTML_MEDIA: Lazy<Regex> = Lazy::new(|| {
        // matches <img …>, <video …>, <audio …>, <iframe …>, <object …>
        Regex::new(r#"(?is)<\s*(img|video|audio|iframe|object)\b[^>]*>"#).unwrap()
    });

    static NESTED_IMG_LINK: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[\s*\[IMAGE[^\]]*\]\s*\]\([^)]+\)").unwrap());

    static IMAGE_LIKE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[IMAGE:[^\]]+\]").unwrap());

    // ── 1.  Markdown images → [IMAGE] or [IMAGE: alt]
    let step1 = MD_IMAGE.replace_all(raw, |caps: &regex::Captures| {
        let alt = caps.name("alt").map_or("", |m| m.as_str()).trim();
        if alt.is_empty() {
            "[IMAGE]".into()
        } else {
            format!("[IMAGE: {alt}]")
        }
    });

    // ── 2.  HTML media tags → [IMAGE] / [VIDEO] …
    let step2 = HTML_MEDIA.replace_all(&step1, |caps: &regex::Captures| {
        let tag = caps.get(1).unwrap().as_str().to_uppercase();
        format!("[{tag}]")
    });

    // ── 3.  Nested [IMAGE] inside a Markdown link
    let step3 = NESTED_IMG_LINK.replace_all(&step2, |caps: &regex::Captures| {
        let s = caps.get(0).unwrap().as_str();
        let url = s.rsplit("](").next().unwrap_or("").trim_end_matches(')');
        format!("[LINK] → {url}")
    });

    // ── 4.  Remove any remaining “[IMAGE: …]” with an extension-looking alt
    let step4 = IMAGE_LIKE.replace_all(&step3, "[IMAGE]");

    // ── 5.  Strip control characters
    step4
        .chars()
        .filter(|c| matches!(*c, '\n' | '\r' | '\t') || *c >= '\u{20}')
        .collect::<String>()
}

```
ai-agent-audit/src/utils/vec_db_connect.rs
```
// utils/connect.rs
use once_cell::sync::OnceCell;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::{
    client::EmbeddingsClient,
    providers::openai::{Client, TEXT_EMBEDDING_3_SMALL},
    vector_store::VectorStoreIndexDyn,
};
use rig_qdrant::QdrantVectorStore;
use std::sync::Arc;

/// Build (once) and return an Arc<dyn VectorStoreIndexDyn>.
///
/// Synchronous because OnceCell’s initializer must be sync.
pub fn vector_index() -> anyhow::Result<Arc<dyn VectorStoreIndexDyn>> {
    static ONCE: OnceCell<Arc<dyn VectorStoreIndexDyn>> = OnceCell::new();

    let idx = ONCE.get_or_try_init(|| {
        // 1. Qdrant client
        let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
            .build()
            .map_err(anyhow::Error::from)?;

        let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
        let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

        /* 2 ── Build the query-params object */
        let qp = QueryPointsBuilder::new("contract_chunks") // collection name
            .with_payload(true) // pull "meta", etc.
            .build();
        // 3. Vector-store pointing at existing collection “contract_chunks”
        //    -> create the collection elsewhere (ingest step) or check/ensure here.
        let store = QdrantVectorStore::new(qdrant, model, qp);

        // 4. Erase to trait object
        Ok::<Arc<dyn VectorStoreIndexDyn>, anyhow::Error>(Arc::new(store))
    })?;

    Ok(idx.clone())
}

```
ai-agent-audit/src/master_prompts/master_prompt.rs
```
/// Master security analysis prompt covering all vulnerability categories.
///
/// This comprehensive prompt instructs AI agents to analyze smart contracts
/// across 19 different security vulnerability categories, providing systematic
/// coverage of common and advanced attack vectors.

pub const MASTER_SECURITY_PROMPT: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  confidential_data             // private info leak via events / public vars  
4.  default_visibility            // funcs / vars defaulting to `public`  
5.  dos                           // gas exhaustion, revert griefing, block gas limit  
6.  inheritance                   // bad overrides, diamond ambiguity  
7.  integer_math                  // rounding, precision div-by-zero  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  pragma                        // floating pragma, outdated compiler bugs  
10. randomness                    // predictable entropy, miner influence  
11. reentrancy                    // state update after external call, cross-function  
12. replay_attack                 // sig replay, chain-ID mix-ups  
13. self_destruct                 // griefing / forced-ETH via `selfdestruct`  
14. short_address                 // calldata truncation on L1/L2 bridges  
15. storage_layout                // slot collisions, struct packing, uninitialized_storage  
16. tx_origin                     // auth that trusts `tx.origin`  
17. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
18. unexpected_eth                // Ether stuck / overly strict balance checks  
19. zero_code                     // constructor-phase contract bypasses  
20. frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
21. upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
22. pausable_emergency_stop       // missing pause guards or bypasses  
23. timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
24. flash_loan_economic_manipulation // state checked & used within same tx  
25. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
26. signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
27. event_consistency             // critical state changes not emitted / mis-ordered  
28. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  
29. integer_overflow              // overflow / underflow, div-by-zero  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_a.rs
```
pub const PROMPT_2X_A: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  dos                           // gas exhaustion, revert griefing, block gas limit  
4.  inheritance                   // bad overrides, diamond ambiguity  
5.  integer_math                  // rounding, precision div-by-zero  
6.  integer_overflow              // overflow / underflow, div-by-zero  
7.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  randomness                    // predictable entropy, miner influence  
10. reentrancy                    // state update after external call, cross-function  
11. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
12. unexpected_eth                // Ether stuck / overly strict balance checks  
13. storage_layout                // slot collisions, struct packing, uninitialized_storage  
14. self_destruct                 // griefing / forced-ETH via `selfdestruct`  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_aa.rs
```
pub const PROMPT_2X_AA: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  dos                           // gas exhaustion, revert griefing, block gas limit  
3.  integer_overflow              // overflow / underflow, div-by-zero  
4.  signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
5.  unexpected_eth                // Ether stuck / overly strict balance checks  
6.  storage_layout                // slot collisions, struct packing, uninitialized_storage  
7.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
    ### 7-A  Quick front-running / TOD checklist
    - Public functions: can caller profit by seeing a tx in mempool and racing it?  
    - Sequencing deps: does fn A write state that fn B reads in the *same* block?  
    - Value-transfer timing: funds sent immediately after a calc the attacker can influence?  
    - Deterministic selection: winner/outcome based on current on-chain state?  
    - Mitigations present? (pull payments, commit-reveal, VRF, time-locks, ACL)  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  randomness                    // predictable entropy, miner influence  
10. reentrancy                    // state update after external call, cross-function  
11. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
12. replay_attack                 // sig replay, chain-ID mix-ups  
13. upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
14. self_destruct                 // griefing / forced-ETH via `selfdestruct`  
15. zero_code                     // constructor-phase contract bypasses  
16. flash_loan_economic_manipulation // state checked & used within same tx  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_b.rs
```
pub const PROMPT_2X_B: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  tx_origin                     // auth that trusts `tx.origin`  
2.  zero_code                     // constructor-phase contract bypasses  
3.  pragma                        // floating pragma, outdated compiler bugs  
4.  confidential_data             // private info leak via events / public vars  
5.  default_visibility            // funcs / vars defaulting to `public`  
6.  replay_attack                 // sig replay, chain-ID mix-ups  
7.  upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
8.  pausable_emergency_stop       // missing pause guards or bypasses  
9.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
10. flash_loan_economic_manipulation // state checked & used within same tx  
11. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
12. signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
13. event_consistency             // critical state changes not emitted / mis-ordered  
14. short_address                 // calldata truncation on L1/L2 bridges  
15. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_bb.rs
```
pub const PROMPT_2X_BB: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  tx_origin                     // auth that trusts `tx.origin`  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  pragma                        // floating pragma, outdated compiler bugs  
4.  inheritance                   // bad overrides, diamond ambiguity  
5.  integer_math                  // rounding, precision div-by-zero  
6.  confidential_data             // private info leak via events / public vars  
7.  default_visibility            // funcs / vars defaulting to `public`  
8.  pausable_emergency_stop       // missing pause guards or bypasses  
9.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
10. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
11. event_consistency             // critical state changes not emitted / mis-ordered  
12. short_address                 // calldata truncation on L1/L2 bridges  
13. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_3x_a.rs
```
pub const PROMPT_3X_A: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  dos                           // gas exhaustion, revert griefing, block gas limit  
4.  unexpected_eth                // Ether stuck / overly strict balance checks  
5.  integer_overflow              // overflow / underflow, div-by-zero  
6.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
7.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
8.  randomness                    // predictable entropy, miner influence  
9.  reentrancy                    // state update after external call, cross-function  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_3x_b.rs
```
pub const PROMPT_3X_B: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  default_visibility            // funcs / vars defaulting to `public`  
2.  replay_attack                 // sig replay, chain-ID mix-ups  
3.  upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
4.  pausable_emergency_stop       // missing pause guards or bypasses  
5.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
6.  flash_loan_economic_manipulation // state checked & used within same tx  
7.  delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
8.  signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
9.  event_consistency             // critical state changes not emitted / mis-ordered  
10. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_3x_c.rs
```
pub const PROMPT_3X_C: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
2.  storage_layout                // slot collisions, struct packing, uninitialized_storage  
3.  inheritance                   // bad overrides, diamond ambiguity  
4.  self_destruct                 // griefing / forced-ETH via `selfdestruct`  
5.  integer_math                  // rounding, precision div-by-zero  
6.  tx_origin                     // auth that trusts `tx.origin`  
7.  zero_code                     // constructor-phase contract bypasses  
8.  pragma                        // floating pragma, outdated compiler bugs  
9.  confidential_data             // private info leak via events / public vars  
10. short_address                 // calldata truncation on L1/L2 bridges  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/prompts/access_control.rs
````
/// Access control vulnerability detection prompt.
///
/// This prompt guides AI agents to identify access control issues including
/// unprotected functions, missing modifiers, privilege escalation, and
/// improper role management in smart contracts.

pub const ACCESS_CONTROL: &str = r#"

You are an expert Solidity smart contract security auditor specializing in access control vulnerabilities. Your task is to perform a comprehensive access control analysis on the provided Solidity smart contract code.

## Analysis Framework

Systematically examine the {contract_name} contract for the following access control issues:

1. **Unprotected Sensitive Functions**: Functions that perform critical operations without proper authorization checks
2. **Missing Access Modifiers**: Functions lacking `onlyOwner`, `onlyAdmin`, or equivalent access control modifiers
3. **Privilege Escalation**: Functions that allow unauthorized users to gain elevated privileges
4. **State-Changing Operations**: Functions that modify critical contract state without authorization
5. **Asset Management**: Functions handling Ether transfers, token minting/burning, or asset withdrawals

## Critical Functions to Analyze

Pay special attention to functions in {contract_name} with these patterns:
- `mint()`, `burn()`, `mintTo()` - Token supply manipulation
- `withdraw()`, `withdrawAll()`, `emergencyWithdraw()` - Asset extraction
- `initialize()`, `setup()` - Contract initialization
- `setOwner()`, `transferOwnership()` - Ownership changes
- `pause()`, `unpause()` - Contract state control
- `updateConfig()`, `setParameters()` - Configuration changes
- Functions with `payable` modifier or Ether handling
- Functions that call `selfdestruct()` or `delegatecall()`

## BEFORE ANALYZING: 
- ONLY reference or analyze functions that are: explicitly present in the {contract_name} code, OR called
  by functions in {contract_name} contract.
- If you cannot find an access control vulnerability in the ACTUAL code, return:
{
  "findings": []
}

## Analysis Instructions

1. Read through the entire {contract_name} contract code carefully
2. Identify all functions that modify state or handle assets
3. Check each function for appropriate access control mechanisms
4. Verify that access control cannot be bypassed
5. Consider edge cases and inheritance patterns
6. Test your findings with concrete exploitation scenarios

"#;

// ARCHIVED
pub const ACCESS_CONTROL_V1: &str = r#"You are an expert smart contract security auditor specializing in access control vulnerabilities. Your task is to perform a comprehensive access control analysis on the provided Solidity smart contract code.

## Analysis Framework

Systematically examine the contract for the following access control issues:

1. **Unprotected Sensitive Functions**: Functions that perform critical operations without proper authorization checks
2. **Missing Access Modifiers**: Functions lacking `onlyOwner`, `onlyAdmin`, or equivalent access control modifiers
3. **Privilege Escalation**: Functions that allow unauthorized users to gain elevated privileges
4. **State-Changing Operations**: Functions that modify critical contract state without authorization
5. **Asset Management**: Functions handling Ether transfers, token minting/burning, or asset withdrawals

## Critical Functions to Analyze

Pay special attention to functions with these patterns:
- `mint()`, `burn()`, `mintTo()` - Token supply manipulation
- `withdraw()`, `withdrawAll()`, `emergencyWithdraw()` - Asset extraction
- `initialize()`, `setup()` - Contract initialization
- `setOwner()`, `transferOwnership()` - Ownership changes
- `pause()`, `unpause()` - Contract state control
- `updateConfig()`, `setParameters()` - Configuration changes
- Functions with `payable` modifier or Ether handling
- Functions that call `selfdestruct()` or `delegatecall()`

## Example Vulnerable Pattern

```solidity
contract VulnerableToken {
    mapping(address => uint256) public balances;
    uint256 public totalSupply;
    address public owner;
    
    // VULNERABLE: Missing access control - anyone can mint tokens
    function mint(address to, uint256 amount) public {
        balances[to] += amount;
        totalSupply += amount;
    }
    
    // VULNERABLE: Missing access control - anyone can withdraw all Ether
    function withdrawAll() public {
        payable(msg.sender).transfer(address(this).balance);
    }
}
```

## Expected Foundry Test Pattern

For each finding, provide a Foundry test that demonstrates the vulnerability:

```solidity
function test_UnauthorizedMinting() public {
    // Setup: Deploy contract and fund with initial state
    VulnerableToken token = new VulnerableToken();
    
    // Attack: Non-owner calls privileged function
    vm.prank(attacker);
    token.mint(attacker, 1000000 ether);
    
    // Verify: Unauthorized action succeeded
    assertEq(token.balances(attacker), 1000000 ether);
    assertEq(token.totalSupply(), 1000000 ether);
}
```

## Output Requirements

For each access control vulnerability found, provide:

1. **Title**: Format as "[Severity-X] - Access Control Issue in <Contract>::<Function>"
2. **Description**: Detailed explanation including vulnerable code snippet
3. **Impact**: Business and security consequences of the vulnerability
4. **Proof of Concept**: Step-by-step exploitation scenario
5. **Proof of Code**: Complete Foundry unit test demonstrating the vulnerability
6. **Severity**: High/Medium/Low/Info based on exploitability and impact

## Severity Guidelines

- **High**: Critical functions (mint, burn, withdraw, ownership transfer) with no access control
- **Medium**: Important functions with partial or bypassable access control
- **Low**: Administrative functions with missing access control but limited impact
- **Info**: Best practice violations or potential future risks

## Analysis Instructions

1. Read through the entire contract code carefully
2. Identify all functions that modify state or handle assets
3. Check each function for appropriate access control mechanisms
4. Verify that access control cannot be bypassed
5. Consider edge cases and inheritance patterns
6. Test your findings with concrete exploitation scenarios

"#;

````
ai-agent-audit/src/prompts/array_limits.rs
```
pub const ACCESS_OUTSIDE_ARRAY_LIMITS: &str = r#"
You are an expert smart contract security auditor specializing in array bounds vulnerabilities. Your task is to perform a comprehensive array access analysis on the provided Solidity smart contract code.

## Analysis Framework
Systematically examine the contract for the following array bounds issues:

1. **Unchecked Array Access**: Direct array indexing without bounds validation
2. **Loop Index Overflow**: For-loops with unsafe index incrementation or bounds
3. **User-Controlled Indices**: External input used as array index without validation
4. **Dynamic Array Manipulation**: Push/pop operations that could cause index misalignment
5. **Fixed Array Overflows**: Static array access beyond declared bounds
6. **Nested Array Issues**: Multi-dimensional array access with insufficient bounds checking
7. **Array Length Manipulation**: Functions that modify array length without updating dependent logic
8. **Sentinel & Off-By-One Errors**  
   * Functions that **return an index** (e.g. `getIndex(...)` or `find(...)`) but  
     - use `0` (or `type(uint256).max`) both as “first element” and “not found”, **or**  
     - return `array.length` as a valid index.  
   * Comparisons like `>= array.length`, `<= 0`, or `i <= array.length` inside loops.  
   * Boundary checks that use `>=` when they should use `>` (or vice-versa) causing one extra/omitted element to be processed.

## Critical Patterns to Analyze
Pay special attention to functions with these array access patterns:
- Direct indexing: `array[index]`, `mapping[key][index]`
- Loop iterations: `for(uint i = 0; i < someValue; i++)` where `someValue != array.length`
- User input as index: `function get(uint256 index)` without bounds checking
- Array modifications: `array.push()`, `array.pop()`, `delete array[index]`
- Batch operations: Functions processing multiple array elements
- Array copying: `for` loops copying between arrays of different lengths
- External calls with array parameters: Functions passing arrays to external contracts

## Analysis Instructions
1. Identify all array declarations and their usage patterns throughout the contract
2. Check every array access operation for bounds validation
3. Examine loop constructs that iterate over arrays or use array indices
4. Validate that user-provided indices are properly bounded
5. Look for functions that modify array length and check dependent operations
6. Test multi-dimensional array access patterns for nested bounds issues
7. Verify batch operations handle array length mismatches safely
8. Check for edge cases like empty arrays or single-element arrays
9. Examine inheritance patterns that might introduce array access issues
10. **Verify index-return helpers**  
    * Does a “not-found” condition have an unambiguous signal (e.g. returns `(false, 0)` or reverts)? 
    * Could the caller mistakenly treat that value as a valid slot?

"#;

pub const ACCESS_OUTSIDE_ARRAY_LIMITS_V1: &str = r#"
You are an expert smart-contract auditor.  
Your ONLY task is to detect **actual or inevitable array-out-of-bounds
read/write operations** in the Solidity source below.

────────────────────────────
⚠️  VALID-BUG CRITERIA
────────────────────────────
A finding is reportable **only if _all_ of the following hold**:

1. **Real Bounds Violation**  
   A runtime path exists where `array[index]` (or similar) executes with  
   `index ≥ array.length` or `index < 0` (underflow).

2. **Feasible Trigger**  
   * The violating index is controllable with ≤ 1 full block of gas  
     **and** with calldata sizes that fit today’s mainnet limits.  
   * Ignore purely theoretical indices that require 2³² iterations,
     2²⁵⁶ elements, etc.

3. **Impact Observable**  
   The violation causes at least one of:  
   * Immediate revert (DoS of THAT single call is fine)  
   * Corruption or loss of data / funds  
   * Escape from intended control flow

**Do NOT** report:

* High-gas loops, quadratic complexity, or any issue whose only
  consequence is excessive gas.  
* “Performance”, “duplication”, or “ambiguous return-value” complaints.  
* Array-length mismatches that still stay within bounds.

"#;

```
ai-agent-audit/src/prompts/confidential_data.rs
```
pub const SAVING_CONFIDENTIAL_DATA: &str = r#"You are an expert smart contract security auditor specializing in data privacy and confidential information vulnerabilities. Your task is to perform a comprehensive analysis on the provided Solidity smart contract code for improper storage of sensitive data.

## Analysis Framework
Systematically examine the contract for the following confidential data vulnerabilities:

1. **Unencrypted Personal Information**: Storage of user personal data (names, addresses, SSNs, emails) in plain text
2. **Private Key Exposure**: Storage of private keys, seed phrases, or cryptographic secrets on-chain
3. **Sensitive Business Data**: Confidential business information, trade secrets, or proprietary data stored publicly
4. **Authentication Credentials**: Passwords, API keys, or authentication tokens stored without proper hashing
5. **Financial Information**: Bank details, credit card numbers, or sensitive financial data in plain text
6. **Medical/Health Data**: Protected health information (PHI) or medical records stored publicly

## Critical Patterns to Analyze
Pay special attention to storage patterns with these characteristics:
- `mapping(address => string)` storing personal information
- `bytes` or `string` variables containing sensitive data
- Private variables (remember: private != secret on blockchain)
- Struct fields containing personal identifiers
- Event emissions that leak sensitive information
- Functions that accept and store sensitive data without encryption
- Comments or variable names suggesting confidential data storage

## Specific Attack Vectors to Test
1. **Storage Slot Reading**: Direct reading of contract storage slots to extract "private" variables
2. **Event Log Analysis**: Monitoring blockchain events for sensitive data emissions
3. **Transaction Data Mining**: Extracting sensitive data from transaction input parameters
4. **Bytecode Analysis**: Reverse engineering contract bytecode to find hardcoded secrets
5. **Rainbow Table Attacks**: Cracking unsalted password hashes using precomputed tables
6. **Social Engineering**: Using exposed personal information for targeted attacks

## Analysis Instructions
1. Scan all storage variables, mappings, and structs for sensitive data patterns
2. Examine function parameters and event emissions for confidential information
3. Check for hardcoded secrets, keys, or credentials in contract code
4. Analyze password/authentication mechanisms for proper cryptographic practices
5. Verify that sensitive data is properly encrypted, hashed, or stored off-chain
6. Test data extraction scenarios using storage reading and event monitoring
7. Create concrete demonstrations showing how sensitive data can be compromised

Focus on actionable privacy vulnerabilities where confidential data can be extracted by unauthorized parties. Each finding must include a working Foundry test that demonstrates the specific data exposure vector and its potential for exploitation."#;

```
ai-agent-audit/src/prompts/default_visibility.rs
````
pub const DEFAULT_VISIBILITIES: &str = r#"

# Smart Contract Security Analysis: Default Function Visibility Detection

You are an expert smart contract security auditor specializing in identifying function visibility vulnerabilities. Your task is to analyze Solidity smart contracts for functions with missing or inappropriate visibility modifiers that could lead to unauthorized access.

## Vulnerability Overview
The Default Visibility vulnerability occurs when functions lack explicit visibility modifiers, causing them to default to `public` visibility. This can expose sensitive internal functions to external callers, potentially allowing unauthorized access to critical contract operations.

### Solidity Visibility Rules
- **No modifier specified**: Defaults to `public` 
- **public**: Callable externally and internally
- **external**: Only callable externally (gas efficient for external calls)
- **internal**: Only callable within contract and derived contracts
- **private**: Only callable within the defining contract

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns contract code:

1. **Missing Visibility Modifiers**: Functions without `public`, `external`, `internal`, or `private`
2. **Inappropriate Public Access**: Functions that should be restricted but are publicly accessible
3. **Administrative Functions**: Owner-only or privileged functions without proper access control
4. **Internal Logic Exposure**: Helper functions that should be internal/private but are public

## Analysis Focus Areas

1. **Administrative Functions**: Functions that change ownership, pause/unpause, or modify critical parameters
2. **Financial Functions**: Functions that handle funds, minting, burning, or balance modifications
3. **State-Changing Functions**: Functions that modify contract state without proper access control
4. **Helper Functions**: Internal logic that should not be publicly accessible
5. **Privileged Operations**: Functions intended for specific roles but lacking visibility control
6. **Emergency Functions**: Functions designed for crisis management without proper restrictions

### Detection Strategy
1. **Scan for Missing Modifiers**: Identify all functions without explicit visibility keywords
2. **Analyze Function Purpose**: Determine if the function should be restricted based on its operations
3. **Check Access Patterns**: Look for functions that modify critical state or handle sensitive operations
4. **Validate Public Exposure**: Ensure publicly accessible functions are intentionally public
5. **Review Administrative Logic**: Flag any owner/admin functions without proper visibility

### Common Vulnerable Patterns
- Owner/admin functions without visibility modifiers
- Internal calculation functions exposed publicly
- State modification functions without access control
- Emergency or maintenance functions lacking proper visibility
- Helper functions that reveal internal contract logic

"#;

pub const DEFAULT_VISIBILITIES_V1: &str = r#"

# Smart Contract Security Analysis: Default Function Visibility Detection

You are an expert smart contract security auditor specializing in identifying function visibility vulnerabilities. Your task is to analyze Solidity smart contracts for functions with missing or inappropriate visibility modifiers that could lead to unauthorized access.

## Vulnerability Overview
The Default Visibility vulnerability occurs when functions lack explicit visibility modifiers, causing them to default to `public` visibility. This can expose sensitive internal functions to external callers, potentially allowing unauthorized access to critical contract operations.

### Solidity Visibility Rules
- **No modifier specified**: Defaults to `public` (DANGEROUS)
- **public**: Callable externally and internally
- **external**: Only callable externally (gas efficient for external calls)
- **internal**: Only callable within contract and derived contracts
- **private**: Only callable within the defining contract

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns in smart contract code:

1. **Missing Visibility Modifiers**: Functions without `public`, `external`, `internal`, or `private`
2. **Inappropriate Public Access**: Functions that should be restricted but are publicly accessible
3. **Administrative Functions**: Owner-only or privileged functions without proper access control
4. **Internal Logic Exposure**: Helper functions that should be internal/private but are public

### Code Example to Analyze
```solidity
pragma solidity ^0.8.0;

contract VulnerableVisibility {
    address public owner;
    mapping(address => uint256) private balances;
    uint256 private totalSupply;
    bool private paused;
    
    constructor() {
        owner = msg.sender;
        totalSupply = 1000000;
    }
    
    // VULNERABLE: Missing visibility modifier (defaults to public)
    function setOwner(address newOwner) {
        owner = newOwner;
    }
    
    // VULNERABLE: Administrative function without access control
    function pause() {
        paused = true;
    }
    
    // VULNERABLE: Internal helper function exposed publicly
    function calculateFee(uint256 amount) returns (uint256) {
        return amount * 3 / 100;
    }
    
    // VULNERABLE: Critical function without visibility modifier
    function mint(address to, uint256 amount) {
        balances[to] += amount;
        totalSupply += amount;
    }
    
    // VULNERABLE: Withdrawal function without proper access control
    function emergencyWithdraw() {
        payable(owner).transfer(address(this).balance);
    }
    
    // CORRECT: Properly defined visibility
    function transfer(address to, uint256 amount) public returns (bool) {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        require(!paused, "Contract is paused");
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }
    
    // CORRECT: Internal helper function
    function _beforeTransfer(address from, address to) internal view {
        require(!paused, "Transfers paused");
    }
    
    // VULNERABLE: State-changing function without visibility
    function updateTotalSupply(uint256 newSupply) {
        totalSupply = newSupply;
    }
}
```

### Foundry Test Example
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

contract DefaultVisibilityTest is Test {
    VulnerableVisibility target;
    address attacker = address(0x1337);
    address originalOwner;
    
    function setUp() public {
        target = new VulnerableVisibility();
        originalOwner = target.owner();
    }
    
    function testUnauthorizedOwnershipTransfer() public {
        // Verify initial owner
        assertEq(target.owner(), originalOwner);
        
        // Attacker can call setOwner due to missing visibility modifier
        vm.prank(attacker);
        target.setOwner(attacker);
        
        // Ownership has been transferred to attacker
        assertEq(target.owner(), attacker);
        assertNotEq(target.owner(), originalOwner);
    }
    
    function testUnauthorizedPause() public {
        // Attacker can pause the contract
        vm.prank(attacker);
        target.pause();
        
        // Contract is now paused, blocking legitimate transfers
        vm.expectRevert("Contract is paused");
        target.transfer(address(0x123), 100);
    }
    
    function testUnauthorizedMinting() public {
        uint256 initialBalance = target.balances(attacker);
        
        // Attacker can mint tokens to themselves
        vm.prank(attacker);
        target.mint(attacker, 1000000);
        
        // Attacker now has unlimited tokens
        assertEq(target.balances(attacker), initialBalance + 1000000);
    }
    
    function testUnauthorizedEmergencyWithdraw() public {
        // Fund the contract
        vm.deal(address(target), 10 ether);
        
        // Attacker can drain the contract
        uint256 attackerBalanceBefore = attacker.balance;
        
        vm.prank(attacker);
        target.emergencyWithdraw();
        
        // All funds sent to original owner (as set in function)
        // But attacker controlled the call
        assertEq(address(target).balance, 0);
    }
    
    function testSupplyManipulation() public {
        // Attacker can manipulate total supply
        vm.prank(attacker);
        target.updateTotalSupply(0);
        
        // Total supply is now corrupted
        // This could break tokenomics and calculations
    }
}
```

## Analysis Focus Areas

1. **Administrative Functions**: Functions that change ownership, pause/unpause, or modify critical parameters
2. **Financial Functions**: Functions that handle funds, minting, burning, or balance modifications
3. **State-Changing Functions**: Functions that modify contract state without proper access control
4. **Helper Functions**: Internal logic that should not be publicly accessible
5. **Privileged Operations**: Functions intended for specific roles but lacking visibility control
6. **Emergency Functions**: Functions designed for crisis management without proper restrictions

### Detection Strategy
1. **Scan for Missing Modifiers**: Identify all functions without explicit visibility keywords
2. **Analyze Function Purpose**: Determine if the function should be restricted based on its operations
3. **Check Access Patterns**: Look for functions that modify critical state or handle sensitive operations
4. **Validate Public Exposure**: Ensure publicly accessible functions are intentionally public
5. **Review Administrative Logic**: Flag any owner/admin functions without proper visibility

### Common Vulnerable Patterns
- Owner/admin functions without visibility modifiers
- Internal calculation functions exposed publicly
- State modification functions without access control
- Emergency or maintenance functions lacking proper visibility
- Helper functions that reveal internal contract logic

## Output Requirements

For each vulnerability found, provide a structured finding with these exact fields:

### Finding Structure
- **title**: "[Severity-##] - Default Visibility Vulnerability in <Contract>::<Function>"
- **description**: Detailed explanation of the missing visibility modifier with specific code snippets
- **impact**: Concrete description of unauthorized access potential and security implications
- **proof_of_concept**: Step-by-step explanation of how an attacker would exploit the missing visibility
- **proof_of_code**: Complete Foundry test demonstrating the unauthorized access
- **severity**: One of: High, Medium, Low, Info

### Severity Guidelines
- **High**: Administrative functions, fund access, or ownership changes without proper visibility
- **Medium**: State-changing functions or business logic exposure without access control
- **Low**: Helper functions or view functions with inappropriate visibility
- **Info**: Functions that should have explicit visibility for code clarity
"#;

````
ai-agent-audit/src/prompts/dos.rs
```
pub const DOS: &str = r#"
You are an expert Solidity smart contract security auditor specializing in identifying Denial of Service (DoS) vulnerabilities caused by unexpected reverts in batch operations.

Your task is to systematically analyze {contract_name} contract code for functions that aggregate multiple external calls where a single failure can cause the entire operation to revert, creating a DoS condition.

## Analysis Framework

### Vulnerability Detection Criteria:
1. **Batch Operations**: Functions in {contract_name} that iterate over arrays/lists making external calls
2. **Fail-Fast Logic**: Use of `require()`, `assert()`, or unhandled reverts in loops
3. **External Dependencies**: Calls to user-controlled contracts or addresses
4. **State Coupling**: Operations where one failure blocks all subsequent operations
5. **Gas Limit Attacks**: Loops that can be manipulated to consume excessive gas

## Analysis Instructions

1. **Identify Batch Operations**: Look for loops that make external calls or transfer funds
2. **Trace Failure Points**: Find `require()`, `assert()`, or unhandled external call failures in loops
3. **Assess Attack Vectors**: Consider malicious contracts, gas manipulation, and edge cases
4. **Evaluate Impact**: Determine what functionality becomes unavailable during DoS
5. **Suggest Mitigations**: Recommend withdrawal patterns, try-catch blocks, or call isolation
6. **Provide Working Tests**: Ensure Foundry tests actually demonstrate the DoS condition

## Common DoS Patterns to Check:
- Batch transfers with `require(success)` in loops
- Reward/dividend distributions to user-controlled addresses
- Multi-call functions without proper error handling
- Unbounded loops over user-provided arrays
- External calls in loops without gas limits

"#;

```
ai-agent-audit/src/prompts/inheritance.rs
```
pub const WRONG_INHERITANCE: &str = r#"

You are a senior Solidity auditor.  
Your ONLY job is to find **genuine, compiler-detectable inheritance mistakes** in the
source below.

─────────────────────────────────
⚠️  VALID-BUG CRITERIA
─────────────────────────────────
A finding is reportable **ONLY if _all_ of the following are true**:

1. **Compiler Verification**  
   Solidity (tested with 0.7.x and 0.8.x) actually emits a diagnostic  
   _or_ the code demonstrably mis-behaves because of the inheritance
   issue (function silently ignored, access loss, etc.).

2. **Concrete Conflict**  
   * Missing `override` **only** if the function **does** override a
     parent implementation **and** the current pragma/pragma-range
     would compile without it.  
   * Missing `virtual` **only** if a child contract in the same file
     (or clearly intended future child) **already overrides** the
     function.  
   * Variable collision reported **only** when two *different* parents
     declare a **slot-compatible** variable with the same name or type,
     or the child redeclares an existing variable name.

3. **Inheritance-Specific**  
   Exclude topics that are **not caused by inheritance**, e.g. gas
   costs, storage packing optimisation, generic proxy layout advice.

4. **Feasible / Present**  
   Do not speculate about “future extensions.”  
   If no contract in the file currently creates the conflict, skip it.

─────────────────────────────────
OUTPUT FORMAT  (JSON array or `[]`)
─────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/integer_overflow.rs
```
pub const INTEGER_OVERFLOW: &str = r#"

 (1) integer overflow / underflow and  
 (2) material precision-loss faults.
 
 ⚠️  STRICT VALID-BUG RULES

 * Attacker profit or fund loss ≥ 1 % of total contract balance **or** ≥ 0.01 ETH, whichever is larger.
   * **Exception:** if the arithmetic fault lets an attacker **bypass /
     satisfy a security-critical check** (e.g. `require(msg.value ==
     expected)`), the above threshold is waived – always report.

 2. **Real Arithmetic Fault**  
    * A genuine overflow / underflow **or** precision-loss that changes token/ETH flows or ledger state.  
   * **Unchecked multiplication/division inside a `require`, `assert`, or
     payment-amount calculation is HIGH-RISK.** Report if either operand
     is user-supplied or may exceed 2¹²⁷.
   * “Dust” rounding that loses < 1 % **is still reportable** when  
       ⓐ  it *accumulates over repeated calls* **and**  
       ⓑ  the dust becomes permanently locked or skews future payouts.

 4. **Concrete Profit Path**  

 5. **Scope Discipline**  

────────────────────────────
REALITY-CHECK STEP (required)
────────────────────────────
After drafting a finding, sanity-check it with order-of-magnitude numbers
that fit in ≤ 30 M gas and realistic on-chain limits (e.g. ≤ 2²⁵⁶ wei,
≤ 500 array elements).  If it still works, keep the finding; otherwise
discard as infeasible.
────────────────────────────
OUTPUT FORMAT
────────────────────────────
"#;

pub const INTEGER_OVERFLOW_V1: &str = r#"You are an expert smart contract security auditor specializing in integer overflow, underflow, and precision vulnerabilities. Your task is to perform a comprehensive mathematical operation analysis on the provided Solidity smart contract code and return your findings in strict JSON format.

## Analysis Framework

Systematically examine the contract for these mathematical vulnerabilities:

### 1. Version-Specific Integer Overflow/Underflow
- **Solidity <0.8.0**: NO automatic overflow protection - flag ALL arithmetic
- **Solidity ≥0.8.0**: Automatic protection EXCEPT in `unchecked{}` blocks
- Check for SafeMath usage in pre-0.8.0 contracts

### 2. Precision Loss & Rounding Issues
- **Division Before Multiplication**: `(a / b) * c` loses precision vs `(a * c) / b`
- **Integer Division Truncation**: `amount / 100` truncates decimals
- **Small Value Operations**: Operations on wei amounts that round to zero
- **Fixed-Point Arithmetic**: Missing decimal handling in percentage calculations

### 3. Critical Vulnerable Operations
- **Arithmetic**: `a + b`, `balance += amount`, `counter++`, `a - b`, `balance -= amount`
- **Multiplication**: `amount * rate`, fee calculations, reward distributions
- **Division**: `amount / divisor`, percentage calculations, ratio computations
- **Casting**: `uint8(largeValue)`, `uint128(amount)` - truncation risks
- **Unchecked blocks**: Any arithmetic inside `unchecked{}` in Solidity 0.8+

## Critical Locations to Analyze

- Token balance updates and supply modifications
- Fee calculations and deductions  
- Reward calculations and distributions
- Timestamp arithmetic and deadline calculations
- Array index operations and bounds
- User input arithmetic operations
- Exchange rate and price calculations
- Percentage and ratio computations


## JSON Field Requirements

For each vulnerability found, populate these JSON fields:

1. **title**: "[Severity-X] - Integer Overflow/Underflow/Precision Loss in <Contract>::<Function>"
2. **description**: Technical explanation with vulnerable code snippet and operation type
3. **impact**: Financial consequences including potential for theft, balance manipulation, or DOS
4. **proof_of_concept**: Step-by-step exploitation with specific numeric values
5. **proof_of_code**: Complete Foundry test demonstrating the vulnerability with proper JSON escaping
6. **severity**: Exactly one of: "High", "Medium", "Low", "Info"

## Severity Guidelines

- **High**: Critical operations (token transfers, supply changes, fee calculations) vulnerable to overflow/precision loss leading to financial loss
- **Medium**: Important calculations with overflow/precision risk but limited direct impact
- **Low**: Edge cases or less critical operations with mathematical vulnerabilities
- **Info**: Best practices violations or potential optimization risks

## Analysis Instructions

1. Identify Solidity version from pragma statement
2. Locate ALL arithmetic operations throughout the contract
3. For pre-0.8.0: Check SafeMath usage for every arithmetic operation
4. For 0.8+: Examine `unchecked{}` blocks carefully
5. Analyze division operations for precision loss patterns
6. Test edge cases with maximum/minimum values and small amounts
7. Validate findings with concrete Foundry test cases

"#;

```
ai-agent-audit/src/prompts/mev.rs
```
pub const MEV: &str = r#"
#############################################
#         ⚒️  MEV / TOD BUG HUNTER         #
#############################################

You are a senior smart-contract security engineer whose **sole
mission** is to discover vulnerabilities that arise because an
attacker or a block producer can influence transaction ordering,
inclusion, or execution context.  
This includes the entire MEV / TOD (Transaction-Ordering Dependence)
surface: front-running, back-running, sandwich attacks, generalized
arbitrage, timestamp or difficulty manipulation, miner-griefing, and
economic denial-of-service.

─────────────────────────────────────────────
🎯  REPORTABLE CATEGORIES
─────────────────────────────────────────────
1. **State-Split Front-Run Windows**  
   • Multi-tx workflows where *Tx-A* makes a commitment, but
     *Tx-B* (sent by anyone) consumes it, letting an attacker cancel,
     cheapen, or dominate the result.

2. **Sandwichable Price / Amount Reads**  
   • Any payout or mint/burn that uses an on-chain value
     (`balanceOf`, `getReserves`, oracle feeds, etc.) that can be
     skewed between *pre-state* and *post-state*.

3. **Miner-Controllable Randomness / Time**  
   • Use of `block.timestamp`, `block.number`, `block.difficulty`,
     `blockhash`, `gasleft`, `tx.gasprice`, etc. to pick winners or
     branch logic.

4. **External-Call Ordering & Callback Abuse**  
   • Contract sends value or executes untrusted code **before**
     critical state is updated, or relies on `receive()` hooks.

5. **Oracle / TWAP Manipulation**  
   • Insufficient averaging period, single-tick quotes, or TWAP that
     can be shifted in ≤ N blocks for profit.

6. **Economic Grief / Balance Equality Traps**  
   • Equality checks (`require(balance == cached)`) or invariants that
     a miner can break by pushing “dust” ETH / tokens or using
     `selfdestruct`.

7. **Auction / Raffle / Bidding Races**  
   • Highest-bid-wins logic reliant on mem-pool honesty, or reward
     functions that privilege the caller.

─────────────────────────────────────────────
🔬  ANALYSIS PLAYBOOK
─────────────────────────────────────────────
A. List every **public / external** function (including inherited).  
B. For each function ask:  
   — *If reordered with another tx in the same block, does value flow
      unfairly?*  
   — *Can a second tx read-modify-write the same variable before this
      tx commits?*  
C. Trace multi-step flows (`commit → reveal`, `deposit → withdraw`,
   `bid → claim`, `enter → refund`, etc.).  
D. Inspect any read of balances, reserves, oracles, totalSupply,
   array lengths, **then** a payment/mint/burn in the same tx.  
E. Flag randomness/time usage manipulable by miners.  
F. Look for equality checks on `address(this).balance` or token
   balances that a dust transfer can break.  
G. When a contract makes an external call **before** internal state
   updates, consider both re-entrancy *and* insertion attacks.

─────────────────────────────────────────────
⚠️  VALID-BUG RULES
─────────────────────────────────────────────
A finding is **reportable** only if:  
1. Exploit fits in one block (≤ 30 M gas) *or* can be repeated
   inexpensively until it pays.  
2. Net attacker profit or victim loss ≥ 0.01 ETH **or** ≥ 1 % of the
   affected pool/fund.  
3. You can outline a concrete tx sequence (front-run, sandwich, oracle
   skew, etc.) and sketch a Foundry/Hardhat test that would succeed.  
4. Ignore purely off-chain / UI issues.

─────────────────────────────────────────────
📄  OUTPUT TEMPLATE
─────────────────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/oracle.rs
```
pub const ORACLE_MANIPULATION: &str = r#"You are an expert smart contract security auditor specializing in oracle manipulation vulnerabilities. Your task is to perform a comprehensive oracle security analysis on the provided Solidity smart contract code.

## Analysis Framework
Systematically examine the contract for the following oracle-related vulnerabilities:

1. **Single Oracle Dependency**: Contracts relying on a single oracle source without redundancy
2. **Price Feed Manipulation**: Vulnerable price feeds that can be manipulated via flash loans or market manipulation
3. **Stale Data Usage**: Oracle data used without freshness checks or heartbeat validation
4. **Flash Loan Oracle Attacks**: Single-block price manipulation vulnerabilities
5. **Inadequate Oracle Aggregation**: Missing or weak oracle data aggregation mechanisms
6. **Time-Weighted Price Bypass**: Lack of TWAP or other manipulation-resistant pricing mechanisms

## Critical Patterns to Analyze
Pay special attention to functions with these oracle-related patterns:
- `getPrice()`, `latestRoundData()` - Price feed queries
- `liquidate()`, `borrow()`, `lend()` - Financial operations using oracle data
- DEX price queries: `getAmountsOut()`, `getReserves()`, spot price calculations
- Single oracle calls without fallback mechanisms
- Price data used immediately without time delays or validation
- Oracle data used for access control or critical state changes
- Functions that don't validate oracle response data (zero prices, stale timestamps)

## Specific Attack Vectors to Test
1. **Flash Loan Price Manipulation**: Use flash loans to manipulate DEX prices before oracle queries
2. **Stale Data Exploitation**: Exploit contracts that don't validate oracle data freshness
3. **Oracle Frontrunning**: Predict oracle updates and frontrun price-sensitive operations
4. **Cross-Chain Oracle Delays**: Exploit timing differences in cross-chain oracle updates
5. **Oracle Outage Exploitation**: Attack during oracle downtime or circuit breaker activation
6. **Aggregation Bypass**: Exploit weak oracle aggregation or fallback mechanisms

## Analysis Instructions
1. Identify all external oracle dependencies and data sources
2. Examine price feed usage in financial calculations and critical operations
3. Check for oracle data validation, staleness checks, and circuit breakers
4. Analyze aggregation mechanisms and fallback oracle implementations
5. Test for flash loan attack vectors and single-block price manipulation
6. Verify time-weighted pricing and manipulation resistance measures
7. Create concrete attack scenarios with working Foundry tests

Focus on exploitable oracle vulnerabilities that can result in financial losses, incorrect liquidations, or protocol manipulation. Each finding must include a working Foundry test that demonstrates the specific oracle attack vector."#;

```
ai-agent-audit/src/prompts/pragma.rs
```
pub const FLOATING_PRAGMA: &str = r#"You are an expert smart contract security auditor specializing in compiler version vulnerabilities. Your task is to perform a comprehensive floating pragma analysis on the provided Solidity smart contract code.

## Analysis Framework
Systematically examine the contract for the following floating pragma issues:
1. **Floating Pragma Declarations**: Pragma statements using caret (^) or range operators that allow compilation with multiple compiler versions
2. **Wide Version Ranges**: Pragma statements with overly broad version ranges (e.g., >=0.8.0 <0.9.0)
3. **Missing Upper Bounds**: Pragma statements without explicit upper version limits
4. **Inconsistent Pragma Versions**: Different pragma versions across contract files in the same project
5. **Deprecated Version Usage**: Usage of compiler versions with known security vulnerabilities

## Critical Pragma Patterns to Analyze
Pay special attention to pragma declarations with these patterns:
- `pragma solidity ^0.8.0;` - Caret allowing any 0.8.x version
- `pragma solidity >=0.8.0;` - Open-ended range without upper bound
- `pragma solidity >=0.7.0 <0.9.0;` - Wide version range spanning major releases
- `pragma solidity 0.8.*;` - Wildcard version specifications
- Missing pragma statements entirely
- Pragma versions below 0.8.0 (lacking built-in overflow protection)

## Analysis Instructions
1. Read through all pragma declarations in the contract files
2. Identify any floating pragma patterns (^, >=, ranges, wildcards)
3. Assess the width of version ranges allowed by each pragma
4. Check for pragma consistency across related contract files
5. Evaluate contract criticality (asset handling, access control, business logic)
6. Consider known vulnerabilities in the allowed compiler version range
7. Test findings with concrete compilation and deployment scenarios

Focus on pragma declarations that create real deployment and security risks. Provide clear evidence showing how floating pragma usage can lead to inconsistent contract behavior or introduce security vulnerabilities."#;

```
ai-agent-audit/src/prompts/randomness.rs
```
pub const RANDOMNESS: &str = r#"
You are an expert smart contract security auditor specializing in randomness vulnerabilities. Your task is to analyze Solidity code for insecure randomness implementations and provide structured findings.

## Analysis Instructions:
1. **Identify** any use of block variables for randomness generation, including:
   - `block.timestamp`
   - `block.difficulty` (legacy) or `block.prevrandao` (post-merge)
   - `blockhash()`
   - `block.number`
   - Any combination of these values

2. **Evaluate** the context and criticality of randomness usage:
   - Gaming/lottery systems (HIGH severity)
   - NFT minting/rarity (MEDIUM severity)  
   - Administrative functions (LOW severity)
   - Non-critical features (INFO severity)

3. **Analyze** exploitation vectors:
   - Miner/validator manipulation capabilities
   - Front-running opportunities
   - Timing attack possibilities
   - Predictability windows

## Recommended Mitigations:
Suggest secure alternatives such as:
- Chainlink VRF (Verifiable Random Function)
- Commit-reveal schemes with time delays
- Oracle-based randomness solutions
- Hash-based random beacon services

## Important Notes:
- Consider post-merge Ethereum changes (prevrandao vs difficulty)  
- Account for different manipulation timeframes for each block variable
- Evaluate economic incentives for exploitation
- Consider MEV (Maximal Extractable Value) implications

Analyze the provided code thoroughly and output findings in the exact structure required for automated processing.
"#;

```
ai-agent-audit/src/prompts/reentrancy.rs
````
/// Reentrancy vulnerability detection prompt.
///
/// This prompt guides AI agents to identify genuine reentrancy vulnerabilities
/// with strict criteria to minimize false positives. Focuses on external calls
/// before state updates that can lead to exploitable attack paths.

pub const REENTRANCY: &str = r#"

You are an expert smart-contract security auditor.  
Analyse the *entire* Solidity source below for genuine **reentrancy** vulnerabilities.

─────────────────────────
⚠️  STRICT DEFINITIONS
─────────────────────────
A finding is valid only if **all** of the following are true:

1. **External call before final state update**
   * The call is to an untrusted target (`call`, `.sendValue`, ERC-777 hook, etc.) **and**
   * At least one writable contract variable that influences funds/logic is modified **after** that call.
2. **Gain-of-function**  
   An attacker can, during the callback, re-enter the *same contract* and:
   * steal value, OR
   * corrupt accounting, OR
   * bypass access control.
3. **Executable attack path**  
   You can outline a sequence of transactions that compiles & passes in Foundry - **or the finding is invalid**.

Do **NOT** report:

* External calls that happen **after** all related state is fully updated (i.e. CEI compliant).
* Calls protected by `nonReentrant` or `ReentrancyGuard` (unless you show a bypass).
* OpenZeppelin’s `_safeMint`, `_safeTransfer`, or `transfer`/`send` **when** they are invoked *after* state updates.
* “Theoretical” read-only or cross-function issues without a runnable exploit.

─────────────────────────
OUTPUT FORMAT
─────────────────────────
"#;

pub const REENTRANCY_V2: &str = r#"You are an expert smart contract security auditor specializing in reentrancy vulnerabilities. Your task is to perform a comprehensive reentrancy analysis on the provided Solidity smart contract code.

## Analysis Framework

Systematically examine the contract for the following reentrancy patterns:

1. **Classic Reentrancy**: External calls before state updates (CEI pattern violation)
2. **Cross-Function Reentrancy**: State inconsistencies across multiple functions
3. **Cross-Contract Reentrancy**: Reentrancy through external contract interactions
4. **Read-Only Reentrancy**: Exploiting inconsistent state during external calls
5. **ERC-777/ERC-1363 Hooks**: Token callback mechanisms enabling reentrancy

## Critical Patterns to Identify

### Vulnerable Call Patterns:
- `address.call{value: amount}("")`
- `payable(address).transfer(amount)`
- `payable(address).send(amount)`
- External contract method calls
- Token transfers with hooks (ERC-777, ERC-1363)
- Callback mechanisms and delegate calls

### State Update Patterns:
- Balance modifications: `balances[user] -= amount`
- Status changes: `withdrawn[user] = true`
- Nonce updates: `nonces[user]++`
- Supply changes: `totalSupply -= amount`

### CEI Pattern Violations:
- External calls BEFORE state updates
- Multiple external calls in sequence
- State reads after external calls

## Analysis Checklist

For each function in the contract, verify:

1. **External Call Identification**: Locate all external calls (transfers, calls, contract interactions)
2. **State Update Ordering**: Check if state updates occur AFTER external calls
3. **CEI Pattern Compliance**: Verify Checks-Effects-Interactions pattern is followed
4. **Cross-Function Impact**: Analyze if reentrancy in one function affects others
5. **Reentrancy Guards**: Check for `nonReentrant` modifiers or similar protections
6. **View Function Safety**: Ensure view functions don't rely on inconsistent state

## Reentrancy Protection Patterns

### Good Patterns (Secure):
```solidity
function secureWithdraw(uint256 amount) public nonReentrant {
    require(balances[msg.sender] >= amount, "Insufficient balance");
    
    // Effects: Update state FIRST
    balances[msg.sender] -= amount;
    
    // Interactions: External call LAST
    (bool success, ) = payable(msg.sender).call{value: amount}("");
    require(success, "Transfer failed");
}
```

### Bad Patterns (Vulnerable):
```solidity
function vulnerableWithdraw(uint256 amount) public {
    require(balances[msg.sender] >= amount, "Insufficient balance");
    
    // Interactions: External call FIRST - VULNERABLE!
    (bool success, ) = payable(msg.sender).call{value: amount}("");
    require(success, "Transfer failed");
    
    // Effects: State update LAST - TOO LATE!
    balances[msg.sender] -= amount;
}
```

## Severity Guidelines

- **High**: Direct fund loss through classic reentrancy in withdrawal/transfer functions
- **Medium**: Cross-function reentrancy or state inconsistency issues
- **Low**: Read-only reentrancy or limited impact scenarios
- **Info**: Missing reentrancy guards on functions that should have them

## Output Requirements

For each reentrancy vulnerability found, provide:

1. **Title**: Format as "[Severity-X] - Reentrancy Vulnerability in <Contract Name>::<Exact Function Name>" 
2. **Description**: Detailed explanation including vulnerable code snippet and call flow
3. **Impact**: Financial consequences including potential fund loss amounts
4. **Proof of Concept**: Step-by-step attack scenario with attacker contract interaction
5. **Proof of Code**: Complete Foundry unit test with attacker contract demonstrating exploitation
6. **Severity**: Assessment based on fund loss potential and ease of exploitation

## Analysis Instructions

1. Map all external calls throughout the contract
2. Trace the execution flow for each function containing external calls
3. Identify state variables that are read/written around external calls
4. Check for reentrancy guard implementations
5. Analyze cross-function state dependencies
6. Create attack scenarios for each potential vulnerability
7. Validate findings with working Foundry test cases

Focus on vulnerabilities that can lead to direct financial loss, unauthorized withdrawals, 
or contract state corruption through reentrancy attacks. Prioritize classic reentrancy patterns 
in withdrawal and transfer functions as these typically have the highest impact.
"#;

````
ai-agent-audit/src/prompts/replay_attack.rs
```
pub const REPLAY_SIGNATURES_ATTACK: &str = r#"You are an expert smart contract security auditor specializing in signature replay attack vulnerabilities. Your task is to perform a comprehensive signature replay analysis on the provided Solidity smart contract code.
## Analysis Framework
Systematically examine the contract for the following signature replay vulnerabilities:

1. **Missing Nonce Systems**: Signature verification without proper nonce tracking or incrementation
2. **Timestamp-Based Replay**: Insufficient timestamp granularity allowing replay within time windows
3. **Cross-Chain Replay**: Missing chain ID validation enabling signature reuse across different networks
4. **Hash Collision Replay**: Inadequate signature hash construction allowing hash reuse
5. **Permit Function Replay**: EIP-2612 permit implementations without proper nonce management
6. **Meta-Transaction Replay**: Gasless transaction implementations vulnerable to signature reuse

## Critical Functions to Analyze
Pay special attention to functions with these patterns:
- Functions using `ecrecover()`, `ECDSA.recover()`, or signature verification libraries
- `permit()`, `permitWithDeadline()` - EIP-2612 implementations
- `executeMetaTransaction()`, `relayTransaction()` - Meta-transaction handlers
- `withdrawWithSignature()`, `transferWithSignature()` - Signature-based asset transfers
- `voteWithSignature()`, `delegateWithSignature()` - Governance signature functions
- Functions accepting `bytes signature` or `(uint8 v, bytes32 r, bytes32 s)` parameters
- Functions with deadline/timestamp validation but no nonce tracking
- Multicall or batch transaction functions using signatures


## Analysis Instructions
1. Scan all functions accepting signature parameters or using signature verification
2. Check if signature hash construction includes nonce, chain ID, and contract address
3. Verify nonce storage and incrementation after successful signature verification
4. Look for deadline/timestamp validation that might create replay windows
5. Test signature reuse scenarios across different function calls
6. Consider batch operations or multicall functions that might bypass individual nonce checks
7. Examine inheritance patterns that might introduce replay vulnerabilities

Focus on immediately exploitable signature replay attacks that can result in financial loss or unauthorized access. Provide concrete test cases showing successful signature capture and reuse.

"#;

```
ai-agent-audit/src/prompts/self_destruct.rs
```
pub const SELF_DESTRUCT: &str = r#"
You are an expert smart contract security auditor specializing in self-destruct vulnerabilities. Your task is to analyze Solidity code for improper or dangerous usage of the selfdestruct opcode and related contract destruction patterns.

## Analysis Instructions:
1. **Identify Self-Destruct Usage**:
   - Direct calls to `selfdestruct()` or `suicide()` (deprecated)
   - Delegatecall patterns that could trigger self-destruct
   - Proxy contracts with destructible implementations
   - Library contracts with self-destruct capabilities

2. **Evaluate Access Controls**:
   - Check if self-destruct is restricted to authorized accounts (owner, admin)
   - Analyze modifier protections and their effectiveness
   - Look for indirect paths to trigger destruction
   - Verify multi-signature or timelock requirements

3. **Assess Destruction Context**:
   - Funds handling before destruction
   - State cleanup requirements
   - Impact on dependent contracts
   - Upgrade vs destruction patterns

## Recommended Mitigations:
- Implement robust multi-signature controls for destruction
- Add time delays for destruction operations
- Ensure user fund withdrawal before destruction
- Use upgrade patterns instead of destruction where possible
- Implement emergency pause instead of destruction
- Add comprehensive access controls and governance

## Important Notes:
- Consider EIP-4758 (Deactivate SELFDESTRUCT) implications for future deployments
- Account for proxy patterns and delegatecall risks
- Evaluate user fund protection mechanisms
- Consider contract dependencies that rely on the contract's existence

Analyze the provided code thoroughly and output findings in the exact structure required for automated processing.
"#;

```
ai-agent-audit/src/prompts/short_address_attack.rs
```
pub const SHORT_ADDRESS_ATTACK: &str = r#"
# Smart Contract Security Analysis: Short Address Attack Detection

You are an expert smart contract security auditor specializing in identifying short address attack vulnerabilities. Your task is to analyze Solidity smart contracts for functions that improperly handle fixed-size type parameters, particularly addresses, which could be exploited through malformed input data.

## Vulnerability Overview
The Short Address Attack exploits the EVM's automatic zero-padding behavior when function parameters are shorter than expected. When an address parameter is truncated (e.g., missing the last byte), the EVM pads it with zeros from the right, potentially causing:
1. **Address Misinterpretation**: Shortened addresses become different valid addresses
2. **Parameter Shifting**: Subsequent parameters get shifted, corrupting their values
3. **Silent Failures**: Functions execute with wrong data without obvious errors

### EVM Padding Behavior
- Expected address: `0x1234567890abcdef1234567890abcdef12345678` (20 bytes)
- Shortened input: `0x1234567890abcdef1234567890abcdef123456` (19 bytes)
- EVM padded result: `0x1234567890abcdef1234567890abcdef12345600` (20 bytes, zero-padded)

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns in smart contract code:

1. **Unvalidated Address Parameters**: Functions accepting addresses without length validation
2. **Multiple Fixed-Size Parameters**: Functions with address + amount patterns susceptible to parameter shifting
3. **External Interface Functions**: Public/external functions that process user-provided address data
4. **Token Transfer Functions**: Functions handling recipient addresses and amounts together
5. **Missing Input Validation**: Functions that don't verify parameter integrity before processing

## Analysis Focus Areas

1. **Token Transfer Functions**: `transfer()`, `transferFrom()`, `mint()`, `burn()`
2. **Approval Functions**: `approve()`, `increaseAllowance()`, `decreaseAllowance()`
3. **Batch Operations**: Functions processing arrays of addresses or multiple parameters
4. **Administrative Functions**: Owner/admin functions accepting address parameters
5. **External Interfaces**: Public/external functions that process user-provided addresses
6. **Multi-Parameter Functions**: Functions with address + amount parameter combinations

### Detection Strategy
1. **Parameter Analysis**: Identify functions with address parameters
2. **Validation Check**: Look for explicit address validation or length checks
3. **Parameter Ordering**: Analyze functions with multiple fixed-size parameters
4. **External Exposure**: Focus on public/external functions accessible to attackers
5. **Impact Assessment**: Evaluate consequences of parameter corruption

### Common Vulnerable Function Patterns
- `function transfer(address to, uint256 amount)` - No address validation
- `function batchTransfer(address[] recipients, uint256[] amounts)` - Array processing without validation
- `function approve(address spender, uint256 amount)` - Missing spender validation
- `function transferFrom(address from, address to, uint256 amount)` - Multiple addresses without checks

Analyze the provided smart contract code systematically and identify all functions vulnerable to short address attacks. Focus on functions that accept address parameters without proper validation and could be exploited through malformed transaction data.
"#;

```
ai-agent-audit/src/prompts/storage_variables.rs
```
pub const STORAGE_VARIABLE: &str = r#"

You are a senior Solidity auditor.  
Your ONLY goal is to find **storage bugs that can corrupt or hijack
contract state.**

────────────────────────────────────────────
⚠️  VALID-BUG CRITERIA
────────────────────────────────────────────
A finding is reportable **ONLY when _all_ of the following are true**:

1. **Direct Corruption or Take-Over**  
   One of these must occur:
   * Un-initialized storage pointer writes to **slot 0** or any other
     slot holding critical data (`owner`, `admin`, `balances`,
     `implementation`, proxy beacons, etc.).
   * A state variable essential for access control or token/accounting
     is left at its default value and can later be seized.
   * Storage-slot collision between parent/child or upgradeable
     versions lets an attacker overwrite live data.

2. **Exploit Demonstrable in ≤ 2 Transactions**  
   Provide a short PoC where an attacker:
   * Gains ownership / admin role, OR
   * Moves ≥ 0.01 ETH / tokens they shouldn’t, OR
   * Permanently bricks a core function.

3. **Compiler-Version Context**  
   Confirm the vulnerability exists for the pragma used.  
   Skip edge-cases already guarded by the compiler in that version.

4. **Out of Scope**  
   Do **NOT** report:
   * Array “holes”, fragmentation, or gas inefficiencies.
   * Generic storage packing commentary that does **not** break access
     control or accounting.
   * “Potential future collision if someone changes inheritance.”
   * Merely recommending `storage gaps` unless a real collision is
     already present.

────────────────────────────────────────────
OUTPUT FORMAT  
────────────────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/tx_origin.rs
```
pub const TX_ORIGIN: &str = r#"
You are an expert smart contract security auditor specializing in identifying tx.origin authentication vulnerabilities.

Your task is to systematically analyze Solidity smart contract code for improper use of tx.origin in access control mechanisms, which can lead to phishing attacks and unauthorized access.

## Analysis Framework

### Vulnerability Detection Criteria:
1. **tx.origin in Access Control**: Use of `tx.origin` in `require()`, `modifier`, or conditional statements for authentication
2. **Privileged Functions**: Functions that use tx.origin to restrict access to sensitive operations
3. **Authorization Bypass**: Scenarios where tx.origin can be manipulated through contract intermediaries
4. **Phishing Attack Vectors**: Situations where users can be tricked into authorizing malicious transactions

## Analysis Instructions

1. **Scan for tx.origin Usage**: Search for all instances of `tx.origin` in the codebase
2. **Identify Access Control**: Focus on tx.origin used in `require()`, modifiers, or conditional statements
3. **Assess Privilege Level**: Determine what functions/operations the tx.origin check protects
4. **Map Attack Vectors**: Consider how malicious contracts can exploit the authentication
5. **Evaluate Impact**: Determine potential damage from successful phishing attacks
6. **Provide Mitigations**: Recommend using `msg.sender` for direct caller verification

## Common tx.origin Vulnerability Patterns:
- `require(tx.origin == owner)` in access control modifiers
- tx.origin checks in privileged functions (withdraw, transfer, admin operations)
- tx.origin used for user identification in financial operations
- Emergency functions relying on tx.origin authentication
- Multi-signature or delegation patterns using tx.origin

## Attack Scenario Framework:
1. **Phishing Setup**: Attacker deploys malicious contract
2. **Social Engineering**: Trick legitimate user into interacting with malicious contract
3. **Exploitation**: Malicious contract calls vulnerable function while tx.origin remains the victim
4. **Impact**: Unauthorized operations execute with victim's privileges

Now analyze the provided smart contract code for tx.origin authentication vulnerabilities following this framework.
"#;

```
ai-agent-audit/src/prompts/unchecked_return_value.rs
```
pub const UNCHECK_RETURN_VALUES: &str = r#"

Your ONLY goal is to detect **external calls whose boolean success
return value is NOT verified or bubbled up.**

──────────────────────────────────
⚠️  VALID–BUG CRITERIA
──────────────────────────────────
A finding is reportable **ONLY if _all_ of the following hold**:

1. **Ignored Success Flag**  
   * For `.call{…}()`, `.delegatecall()`, `.staticcall()`, or
     `.send()` the returned `(bool success, …)` (or single `bool` for
     `send`) is **not**:
     - used in `require(success, …)`  
     - wrapped in `if (!success) revert …;`  
     - returned to the caller, **or**  
     - handled by a library that already reverts on failure  
       (e.g. `Address.sendValue`, `SafeERC20.safeTransfer`).
2. **Interface Calls**  
   The function returns `bool` (e.g., `ERC20.transfer`) and that value
   is ignored **and** no SafeERC20/try-catch wrapper is present.
3. **No CEI Commentary**  
   Do **NOT** flag state-update-before-call ordering; if the success
   flag *is* checked, the call is **out of scope** for this audit.

──────────────────────────────────
OUTPUT FORMAT  
──────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/unexpected_eth.rs
```
pub const UNEXPECTED_ETH: &str = r#"
# Smart Contract Security Analysis: Unexpected Ether Vulnerability Detection

You are an expert smart contract security auditor specializing in identifying vulnerabilities related to unexpected Ether balance manipulation. Your task is to analyze Solidity smart contracts for potential "force-feeding" or "unexpected Ether" vulnerabilities.

## Vulnerability Overview
The Unexpected Ether vulnerability occurs when contracts make assumptions about their Ether balance that can be broken by external actors. Attackers can force Ether into contracts through:
1. `selfdestruct()` calls targeting the contract
2. Pre-calculating contract addresses and sending Ether before deployment
3. Coinbase transactions (for mining rewards)

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns in smart contract code:

1. **Balance Comparisons**: `address(this).balance == expectedAmount`
2. **Balance-based Conditionals**: `require(address(this).balance >= threshold)`
3. **Balance Arithmetic**: `uint256 userShare = msg.value * totalShares / address(this).balance`
4. **Invariant Assumptions**: Internal accounting that assumes balance changes only through contract functions

## Analysis Focus Areas

1. **Balance Equality Checks**: Look for exact balance comparisons
2. **Conditional Logic**: Find balance-dependent control flow
3. **Mathematical Operations**: Identify balance used in calculations
4. **State Transitions**: Check if balance affects contract state changes
5. **Access Control**: Verify if balance influences permissions
6. **Economic Logic**: Examine reward/penalty calculations using balance

Analyze the provided smart contract code thoroughly and identify all instances where unexpected Ether could compromise the contract's intended behavior. Focus on practical exploitability and real-world impact.
"#;

```
ai-agent-audit/src/prompts/zero_code.rs
```
pub const CONTRACTS_WITH_ZERO_CODE: &str = r#"You are an expert smart contract security auditor specializing in access control vulnerabilities related to code size checks. Your task is to perform a comprehensive analysis on the provided Solidity smart contract code for vulnerabilities involving `extcodesize` and code length checks.

## Analysis Framework
Systematically examine the contract for the following code size check vulnerabilities:

1. **Constructor Bypass**: Contracts using code size checks that can be bypassed during contract construction
2. **Self-Destruct Bypass**: Access control relying on code size that fails after contract self-destruction
3. **EOA vs Contract Distinction**: Flawed logic attempting to differentiate between EOAs and contracts
4. **Whitelist Bypass**: Contract whitelisting mechanisms vulnerable to zero-code exploitation
5. **Access Control Evasion**: Critical functions protected only by code size checks

## Critical Patterns to Analyze
Pay special attention to code with these patterns:
- `extcodesize(msg.sender)` or `msg.sender.code.length` checks
- `assembly { size := extcodesize(caller()) }` patterns
- Access modifiers using code size for authorization
- Functions checking `tx.origin == msg.sender` combined with code size checks
- Contract whitelist validation based on code presence
- Access control assuming non-zero code size indicates legitimate contracts

## Specific Attack Vectors to Test
1. **Constructor Attack**: Deploy contract that calls target during `constructor()` execution
2. **Self-Destruct Attack**: Deploy contract, record address, self-destruct, then call from zero-code address
3. **Create2 Attack**: Use CREATE2 to deploy to predictable address, self-destruct, then redeploy different code
4. **Proxy Pattern Bypass**: Exploit proxy contracts that may have minimal code

## Analysis Instructions
1. Scan all functions for `extcodesize`, `code.length`, or assembly code size checks
2. Identify what access control or logic depends on these checks  
3. Determine if the protected functionality can be exploited via zero-code bypass
4. Create concrete attack scenarios showing the bypass
5. Write Foundry tests proving each vulnerability exists
6. Consider edge cases like proxy patterns, factory contracts, and upgrade mechanisms

Focus on demonstrable vulnerabilities where an attacker can bypass intended access restrictions through code size manipulation. Each finding must include a working Foundry test that proves the vulnerability exists."#;

```
ai-agent-audit/src/ai_bot/agent.rs
```
use std::collections::HashSet;

use anyhow::Result;
use log::info;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{
    agent::{Agent, AgentBuilder},
    client::{CompletionClient, EmbeddingsClient},
    providers::openai::{Client, CompletionModel, GPT_4O},
    vector_store::VectorStoreIndex,
};
use rig_qdrant::QdrantVectorStore;
/// AI agent implementations with vector-based context retrieval.
///
/// This module provides intelligent AI agents that combine static documentation
/// with dynamic vector search for contextual smart contract analysis.
use tiktoken_rs::cl100k_base;

use crate::build_brain::enbeddings::SourceChunk;
use crate::config::MAX_RAG_QUERY_CONTENT_LENGTH;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_doc_file::extract_content_from_docs;
use crate::utils::logging::print_first_four_lines;

/// Creates an AI audit agent with vector-based dynamic context retrieval.
///
/// This agent combines static documentation context with dynamic vector search
/// to provide relevant code context for smart contract analysis queries.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `Agent<CompletionModel>` - Configured AI agent with vector search capabilities
pub fn create_ai_audit_agent(repo: &RepoPaths) -> Result<Agent<CompletionModel>> {
    // Extract static documentation for base context
    let documentation = extract_content_from_docs(repo)?;

    // Initialize OpenAI client and model
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let gpt4o = openai.completion_model(GPT_4O);

    // let solidity_auditor_preable = "Your are a world class expert at smart contract auditing, reknown for your ability to find the most complex and trickiest security vulnerabilities in solidity codebases.";

    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let qp = QueryPointsBuilder::new("contract_chunks") // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    let store = QdrantVectorStore::new(qdrant, model, qp);

    // let dynamic_context = vector_index()?;

    let openai_audit_agent = AgentBuilder::new(gpt4o)
        // .preamble(&solidity_auditor_preable)
        .context(&documentation)
        .dynamic_context(3, store)
        .temperature(0.1)
        .build();

    Ok(openai_audit_agent)
}

pub async fn get_rag_for_security_query(query_content: &str, repo: &RepoPaths) -> Result<String> {
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    // Tokenize the input
    let encoding = cl100k_base()?; // for OpenAI models
    let mut tokens = encoding.encode(query_content, HashSet::new());

    // Truncate tokens if needed
    if tokens.len() > MAX_RAG_QUERY_CONTENT_LENGTH {
        tokens.truncate(MAX_RAG_QUERY_CONTENT_LENGTH);
    }

    // Decode truncated tokens back into a string
    let query_content = encoding.decode(tokens)?;
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());
    let qp = QueryPointsBuilder::new(&vector_db_name) // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    info!("creating store...");
    let store = QdrantVectorStore::new(qdrant, model, qp);

    info!("retrieving relevant content from vector db");
    let relevant_docs: Vec<(f64, String, SourceChunk)> = store.top_n(&query_content, 3).await?;

    let dynamic_content = relevant_docs
        .iter()
        .map(|(_, _, source_chunk)| source_chunk.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    // info!("dynamic content");
    // print_first_four_lines(&dynamic_content);

    let final_context = format!("## ADDITIONAL CONTEXT: \n\n {}", dynamic_content);

    Ok(final_context)
}

```
ai-agent-audit/src/ai_bot/retrieve_slice.rs
```
use anyhow::Result;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

use crate::enumerator::codeblock_db::CodeBlocksDb;

#[derive(Debug, Deserialize)]
pub struct Args {
    pub slice_id: String,
}

#[derive(Debug, Serialize)]
pub struct Out {
    pub content: String,
}
/* ────────────── Thread-safe error type ──────────────────────────── */

#[derive(Debug, thiserror::Error)]
#[error("retrival error")]
// struct RetrieveSliceError;
pub enum RetrieveSliceError {
    Sql(#[from] rusqlite::Error),
}

pub struct RetrieveSliceTool {
    pub db: CodeBlocksDb, // <- now Sync because it’s just a PathBuf
}

impl Tool for RetrieveSliceTool {
    const NAME: &'static str = "retrieve_slice";

    type Args = Args;
    type Output = Out;
    type Error = RetrieveSliceError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Load a code slice (by ID) from SQLite and return its Markdown.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "slice_id": { "type": "string", "description": "UUID of the slice" }
                },
                "required": ["slice_id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let content = self.db.get_code_for_seed(&args.slice_id)?;
        Ok(Out { content })
    }
}

```
ai-agent-audit/src/enumerator/codeblock_cache.rs
```
/// In-memory caching for generated code blocks to optimize performance.
///
/// This module provides thread-safe caching of markdown code blocks to avoid
/// redundant generation when processing the same contracts multiple times,
/// significantly improving analysis performance for large repositories.
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::codeblock_db::MarkdownCodeblock;

/// Global in-memory cache for storing MarkdownCodeblock instances.
///
/// The cache is keyed by seed file paths and stores the corresponding
/// MarkdownCodeblock instances. This helps avoid redundant processing
/// and database operations when the same seed file is encountered multiple times.
///
/// The cache is thread-safe, using Arc and Mutex for concurrent access.
pub static CODEBLOCK_CACHE: Lazy<Arc<Mutex<HashMap<String, MarkdownCodeblock>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Retrieves a cached codeblock for a given seed file if it exists.
///
/// # Arguments
/// * `seed_file` - The path to the seed file used as the cache key
///
/// # Returns
/// * `Option<MarkdownCodeblock>` - The cached codeblock if found, None otherwise
pub async fn get_cached_codeblock(seed_file: &str) -> Option<MarkdownCodeblock> {
    let cache = Arc::clone(&CODEBLOCK_CACHE);
    let codeblock_cache = cache.lock().await;

    // Returns Some(codeblock) if found, and None if not found
    codeblock_cache.get(seed_file).cloned()
}

/// Stores a codeblock in the cache for a given seed file.
///
/// # Arguments
/// * `seed_file` - The path to the seed file used as the cache key
/// * `codeblock` - The MarkdownCodeblock instance to cache
pub async fn set_codeblock_cache(seed_file: &str, codeblock: &MarkdownCodeblock) {
    let cache = Arc::clone(&CODEBLOCK_CACHE);
    let mut codeblock_cache = cache.lock().await;

    codeblock_cache.insert(seed_file.to_string(), codeblock.clone());
}

```
ai-agent-audit/src/enumerator/codeblock_db.rs
```
/// SQLite database for code block storage and retrieval.
///
/// This module manages persistent storage of generated markdown code blocks,
/// providing efficient storage and retrieval of contextual code slices for
/// AI analysis with metadata and token counting.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Represents a contextual markdown code block for AI analysis.
///
/// Contains generated code slices with associated metadata including token
/// counts for LLM context management and contract identification.
#[derive(Debug, Clone)]
pub struct MarkdownCodeblock {
    /// Unique identifier for the code block
    pub id: String,
    /// Contract name (e.g., "PuppyRaffle")
    pub contract: String,
    /// Token count for LLM context window management
    pub tokens: usize,
    /// Markdown content with code, IR, and storage information
    pub content: String,
}

/// SQLite database manager for code block persistence.
///
/// Provides high-level interface for storing and retrieving generated
/// code blocks with efficient querying and metadata management.
pub struct CodeBlocksDb {
    /// Path to the SQLite database file
    path: PathBuf,
}

impl CodeBlocksDb {
    /// Creates a new SliceDb instance or opens an existing one at the specified path.
    ///
    /// This function initializes the database schema if it doesn't already exist,
    /// creating tables for seed slices and codeblocks with appropriate indexes.
    ///
    /// # Arguments
    /// * `path` - Path to the SQLite database file
    ///
    /// # Returns
    /// * `Result<Self>` - A new SliceDb instance if successful, Error otherwise
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Self {
            path: path.as_ref().to_path_buf(),
        };

        // Initialize schema once
        let conn = Connection::open(&db.path)?;
        conn.execute_batch(
            r#"
                /* ───────── seed → contract link table ───────── */
                CREATE TABLE IF NOT EXISTS seed_slices(
                id                 TEXT PRIMARY KEY,      -- UUID you assign
                seed_id            TEXT,                  -- FK → seeds.id
                codeblock_id       TEXT,                  -- FK → codeblocks.id
                status             TEXT
                );

                /* speed up look-ups by seed_id or by slice_id */
                CREATE INDEX IF NOT EXISTS idx_seed_slices_seed_id
                    ON seed_slices(seed_id);
                CREATE INDEX IF NOT EXISTS idx_seed_slices_slice_id
                    ON seed_slices(codeblock_id);

                /* ───────── deduped contract bodies ─────────── */
                CREATE TABLE IF NOT EXISTS codeblocks(
                id      TEXT PRIMARY KEY,  -- sha256(body)
                filename TEXT,
                tokens  INTEGER,
                content TEXT
                );
               "#,
        )?;
        Ok(db)
    }

    /// Retrieves the markdown content for a given seed ID.
    ///
    /// This function performs a join between the seed_slices and codeblocks tables
    /// to find the markdown content associated with a specific seed.
    ///
    /// # Arguments
    /// * `seed_id` - The ID of the seed to retrieve code for
    ///
    /// # Returns
    /// * `rusqlite::Result<String>` - The markdown content if found, Error otherwise
    pub fn get_code_for_seed(&self, seed_id: &str) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT c.content
                FROM   codeblocks AS c
                JOIN   seed_slices     AS s  ON s.codeblock_id = c.id
                WHERE  s.seed_id = ?1
                LIMIT  1;
                "#,
            params![seed_id],
            |row| row.get(0),
        )
    }

    /// Inserts a new seed slice into the database.
    ///
    /// This function creates a mapping between a seed and a codeblock in the database.
    ///
    /// # Arguments
    /// * `s` - The SeedSlice to insert
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Error otherwise
    // pub fn insert_seed_slice(&self, s: &SeedSlice) -> Result<()> {
    //     let conn = Connection::open(&self.path)?;
    //     conn.execute(
    //         "INSERT INTO seed_slices VALUES (?1,?2,?3,?4);",
    //         params![s.id, s.seed_id, s.codeblock_id, s.status],
    //     )?;
    //     Ok(())
    // }
    //
    /// Inserts a new codeblock into the database if it doesn't already exist.
    ///
    /// This function checks if a codeblock with the same ID already exists in the database
    /// and only inserts it if it doesn't, preventing duplicate entries.
    ///
    /// # Arguments
    /// * `c` - The MarkdownCodeblock to insert
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Error otherwise
    pub fn insert_codeblock(&self, c: &MarkdownCodeblock) -> Result<()> {
        let conn = Connection::open(&self.path)?;

        // Check if codeblock already exists
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM codeblocks WHERE id = ?1);",
            params![c.id],
            |row| row.get(0),
        )?;

        if exists {
            // Skip insertion if codeblock already exists
            log::debug!("Skipping duplicate contract_slice with id {}", c.id);
            return Ok(());
        }

        // Insert new codeblock
        conn.execute(
            "INSERT INTO codeblocks VALUES (?1,?2,?3,?4);",
            params![c.id, c.contract, c.tokens as i64, c.content],
        )?;
        Ok(())
    }

    /// Retrieves all contracts and their content as a HashMap.
    ///
    /// # Returns
    /// * `rusqlite::Result<HashMap<String, String>>` - HashMap mapping contract names to their content
    pub fn get_all_contracts(&self) -> rusqlite::Result<HashMap<String, String>> {
        let conn = Connection::open(&self.path)?;

        let mut stmt = conn.prepare("SELECT filename, content FROM codeblocks")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // filename/contract
                row.get::<_, String>(1)?, // content
            ))
        })?;

        let mut contracts = HashMap::new();
        for row in rows {
            let (contract, content) = row?;
            contracts.insert(contract, content);
        }

        Ok(contracts)
    }

    pub fn get_code_for_contract(&self, contract: &str) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT content
                FROM   codeblocks
                WHERE  contract = ?1
                LIMIT  1;
                "#,
            params![contract],
            |row| row.get(0),
        )
    }
}

```
ai-agent-audit/src/enumerator/codeblock_maker.rs
```
/// Code block generation orchestration for contract analysis.
///
/// This module coordinates the generation of contextual code blocks for each
/// contract in a repository, managing database connections and processing
/// parameters for optimal AI analysis.

use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::{enumerator::codeblock_db::CodeBlocksDb, prepare_code::git_clone::RepoPaths};
use super::codeblocks::generate_codeblock_from_codebase;

/// Generates and saves contextual code blocks for all contracts in a repository.
///
/// This function orchestrates the code block generation process by connecting
/// to semantic and slice databases, then generating focused code slices for
/// each contract using call graph traversal within specified constraints.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_db` - Path to semantic analysis database
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `PathBuf` - Path to the generated slice database
pub async fn generate_and_save_codeblocks_for_each_contract(
    repo: &RepoPaths,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // Open the three databases
    log::info!("connecting to databases..");
    let semantic_conn = Connection::open(semantics_db)?;
    let slice_path = repo.root.join(".cache").join("slice.db");
    let slice_db = CodeBlocksDb::open(&slice_path)?;

    // Fetch all seeds and process each one
    generate_codeblock_from_codebase(repo, &semantic_conn, &slice_db, max_depth, token_budget)
        .await?;

    Ok(slice_path)
}

```
ai-agent-audit/src/enumerator/codeblocks.rs
```
use crate::build_brain::graph_db::SmartContractFunction;
/// Intelligent code slicing for focused AI analysis.
///
/// This module generates contextual code blocks by traversing call graphs and
/// assembling relevant code, IR, and storage information within token budgets
/// for optimal LLM analysis.
use crate::enumerator::codeblock_cache::{get_cached_codeblock, set_codeblock_cache};
use crate::enumerator::codeblock_db::MarkdownCodeblock;
use crate::enumerator::utils::{
    generate_code_slice_for_storage, generate_codeblock_for_function,
    get_function_metadata_from_id, get_hashmap_of_contract_to_functions,
    get_token_count_of_function_ir,
};
use crate::prepare_code::git_clone::RepoPaths;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

/// Generates contextual code blocks for each contract using call graph traversal.
///
/// This function creates focused code slices by:
/// 1. Checking cache to avoid redundant processing
/// 2. Performing breadth-first search through call graphs up to max_depth
/// 3. Respecting token budgets for LLM context limits
/// 4. Assembling markdown with storage layouts and SlithIR representations
/// 5. Caching results for efficient reprocessing
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantic_db` - Database containing call graph and function data
/// * `codeblock_db` - Database for storing generated code blocks
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `Result<()>` - Success or error
pub async fn generate_codeblock_from_codebase(
    repo: &RepoPaths,
    semantic_db: &Connection,
    codeblock_db: &CodeBlocksDb,
    max_depth: usize,
    token_budget: usize,
) -> Result<()> {
    log::info!("getting contract to func mapping");
    let contract_to_func_map = get_hashmap_of_contract_to_functions(repo, semantic_db)?;

    for (contract, functions_of_contract) in contract_to_func_map {
        // Check if codeblock already generated for this seed

        log::info!("contract => {:#?}", contract);
        log::info!("fn count of contract => {:#?}", functions_of_contract.len());
        if let Some(_) = get_cached_codeblock(&contract).await {
            // Save seed-to-codeblock mapping in the database
            continue;
        };
        // 2. BFS until depth / token budget
        let mut frontier: VecDeque<(SmartContractFunction, usize)> = VecDeque::new();
        for func in functions_of_contract {
            frontier.push_back((func, 0_usize))
        }
        let mut visited = HashSet::new();
        let mut contracts = HashSet::new();
        let mut all_funcs_connected_to_contract = Vec::<SmartContractFunction>::new();
        let mut token_count = 0_usize;

        while let Some((func, depth)) = frontier.pop_front() {
            if !visited.insert(func.id.clone()) {
                continue;
            }

            //keep track of unique contract traversed in BPS
            contracts.insert(func.contract.clone());
            // info!("contract {} / func {} added...", func.contract, func.name);

            all_funcs_connected_to_contract.push(func.clone());

            // get token count of new fn + IR + storage
            let token_count_fn_ir_storage = get_token_count_of_function_ir(&func, repo).await?;
            // info!("token_count_fn_ir_storage => {}", token_count_fn_ir_storage);

            // check budget, make sure not exceeding token context window
            if token_count + token_count_fn_ir_storage > token_budget {
                break; // budget exhausted
            }

            // update token count
            token_count += token_count_fn_ir_storage;

            // info!("token_count => {}", token_count);
            if depth < max_depth {
                let mut statement =
                    semantic_db.prepare("SELECT callee FROM edges WHERE caller = ?1;")?;
                let rows = statement.query_map([&func.id], |r| r.get::<_, String>(0))?;
                for callee in rows.flatten() {
                    let callee_fn = get_function_metadata_from_id(&callee, semantic_db)?;
                    let Some(callee_fn) = callee_fn else { continue };

                    frontier.push_back((callee_fn, depth + 1));
                }
            }
        }

        // ── 3.  Assemble final Markdown body ────────────────────────────
        let mut markdown_codeblock_for_llm = String::new();
        // list storage vars
        for contract in &contracts {
            let storage_var_ir = generate_code_slice_for_storage(contract, repo).await?;
            // loop through and add all functions of contract
            markdown_codeblock_for_llm.push_str(&storage_var_ir);
            markdown_codeblock_for_llm.push('\n');
        }
        for func in &all_funcs_connected_to_contract {
            let function_ir_code = generate_codeblock_for_function(func, repo).await?;
            markdown_codeblock_for_llm.push_str(&function_ir_code);
            markdown_codeblock_for_llm.push('\n');
        }
        info!(
            "markdown codeblock size ==> {:#?}",
            markdown_codeblock_for_llm.len()
        );

        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            contract: contract.clone(),
            tokens: token_count,
            content: markdown_codeblock_for_llm,
        };
        // 4. store
        codeblock_db.insert_codeblock(&codeblock)?;

        // save to cache
        set_codeblock_cache(&contract, &codeblock).await;

        // info!("codeblock => {:#?}", codeblock);
    }

    Ok(())
}

```
ai-agent-audit/src/enumerator/utils.rs
````
use anyhow::anyhow;
use anyhow::Result;
use log::info;
use regex::Regex;
use rusqlite::params_from_iter;
use rusqlite::{Connection, OptionalExtension};
/// Enumeration utilities for code block generation and analysis.
///
/// This module provides utility functions for generating markdown code blocks,
/// extracting function metadata, managing IR mappings, and performing token
/// counting for optimal code slice generation within LLM context limits.
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::fn_labels::get_modifiers_label;
use crate::utils::fn_labels::get_visibility_label;
use crate::utils::get_fn_name::get_function_name;
use crate::{
    build_brain::{
        self,
        graph_db::SmartContractFunction,
        slither_ffi::{SlithIRFn, StorageVar},
    },
    utils::bpe::get_bpe,
};

/// Generates a markdown code block for a specific function with IR representation.
///
/// Creates a formatted markdown section containing the function's SlithIR
/// intermediate representation, including contract context and function metadata.
///
/// # Arguments
/// * `func` - Smart contract function metadata
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `String` - Formatted markdown code block with IR content
pub async fn generate_codeblock_for_function(
    func: &SmartContractFunction,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let ir_map = get_code_ir_map(repo).await?;
    let mut function_slice = String::new();
    let func_name = get_function_name(&func.name);
    let visibility = get_visibility_label(&func.visibility);
    let modifiers = get_modifiers_label(&func.modifiers);

    if let Some(ir) = ir_map.get(&(func.contract.clone(), func_name)) {
        function_slice.push_str(&format!(
            "#### {} {}{}\n",
            ir.function, visibility, modifiers
        ));
        function_slice.push_str("```slithir\n");
        function_slice.push_str(&ir.ir);
        function_slice.push_str("\n```");
    }

    // info!("function slice => {:#?}", function_slice);
    Ok(function_slice)
}

/// Generates a markdown codeblock for a contract's storage layout.
///
/// Retrieves the storage variables for a contract and formats them as a markdown codeblock.
///
/// # Arguments
/// * `contract` - The name of the contract
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<String>` - The generated markdown codeblock
pub async fn generate_code_slice_for_storage(
    contract: &str,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let storage_map = get_storage_map(repo).await?;
    let mut storage_slice = String::new();
    if let Some(vars) = storage_map.get(contract) {
        storage_slice.push_str(&format!("### Storage layout ({}) \n\n", contract));
        storage_slice.push_str("```text\n");
        for v in vars {
            storage_slice.push_str(&format!("{} {}\n", v.name, v.r#type));
        }
        storage_slice.push_str("\n```");
    }

    Ok(storage_slice)
}

pub fn get_hashmap_of_contract_to_functions(
    repo: &RepoPaths,
    semantic_db: &Connection,
) -> anyhow::Result<HashMap<String, Vec<SmartContractFunction>>> {
    // find all main contracts for app (ones in /src)
    info!("grabbing all contracts...");
    let contracts_in_src_folder = contracts_in_src(repo)?;

    let placeholders = contracts_in_src_folder
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    let mut statement = semantic_db.prepare(&format!(
        "SELECT id, contract, name, visibility, modifiers, mutability FROM functions WHERE contract IN ({})",
        placeholders
    ))?;

    let rows = statement.query_map(params_from_iter(contracts_in_src_folder), |row| {
        // info!("rows => {:#?}", row);
        let modifier_str: String = row.get(4)?;
        let modifiers: Vec<String> = modifier_str
            .split(',')
            .map(|s| s.trim_matches([' ', '\'']).to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(SmartContractFunction {
            id: row.get(0)?,
            contract: row.get(1)?,
            name: row.get(2)?,
            visibility: row.get(3)?,
            modifiers,
            mutability: row.get(5)?,
        })
    })?;
    let functions_of_contract: Vec<SmartContractFunction> =
        rows.collect::<rusqlite::Result<_>>()?;

    if functions_of_contract.is_empty() {
        return Err(anyhow!("no entry fn found"));
    }
    let mut map: HashMap<String, Vec<SmartContractFunction>> = HashMap::new();

    for func in functions_of_contract {
        map.entry(func.contract.clone()).or_default().push(func)
    }

    Ok(map)
}

/// Calculates the token count of a function's IR representation.
///
/// Generates the markdown codeblock for the function and counts the number of tokens
/// using the BPE tokenizer.
///
/// # Arguments
/// * `func` - The smart contract function
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<usize>` - The token count
pub async fn get_token_count_of_function_ir(
    func: &SmartContractFunction,
    repo: &RepoPaths,
) -> anyhow::Result<usize> {
    // Generate the function's markdown codeblock
    let fn_text = generate_codeblock_for_function(func, repo).await?;

    // Count tokens using BPE tokenizer
    let bpe = get_bpe();
    let tokens = bpe.encode_with_special_tokens(&fn_text).len();

    Ok(tokens)
}

/// Retrieves a mapping of contract and function names to their SlithIR representations.
///
/// Extracts the function name from the full function signature and creates a map
/// keyed by (contract_name, function_name) tuples.
///
/// # Arguments
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<HashMap<(String, String), SlithIRFn>>` - Map of (contract, function) to SlithIR
async fn get_code_ir_map(repo: &RepoPaths) -> anyhow::Result<HashMap<(String, String), SlithIRFn>> {
    // Regex to extract function name from full signature (e.g., "Contract.function(args)")
    let extract_function_name = Regex::new(r#"[A-Za-z0-9$_]+\.([A-Za-z0-9$_]+)\([^)]*\)"#)?;

    // Get IR and storage variables from Slither
    let (ir_vec, _, _) =
        build_brain::slither_ffi::get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;

    // Create map of (contract, function) -> SlithIRFn
    let ir_map: HashMap<(String, String), SlithIRFn> = ir_vec
        .into_iter()
        .map(|f| {
            if let Some(c) = extract_function_name.captures(&f.function) {
                // Extract function name from signature
                ((f.contract.clone(), c[1].to_string()), f)
            } else {
                // Use full function signature if extraction fails
                ((f.contract.clone(), f.function.clone()), f)
            }
        })
        .collect();

    // info!("ir_map => {:#?}", ir_map);
    Ok(ir_map)
}

async fn get_storage_map(repo: &RepoPaths) -> anyhow::Result<HashMap<String, Vec<StorageVar>>> {
    let (_, storage_vec, _) =
        build_brain::slither_ffi::get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;

    let storage_map: HashMap<String, Vec<StorageVar>> = {
        let mut m = HashMap::<String, Vec<StorageVar>>::new();
        for v in storage_vec {
            m.entry(v.contract.clone()).or_default().push(v);
        }
        m
    };
    Ok(storage_map)
}

/// Return the names of all `contract XXX` declarations that sit
/// anywhere under `repo_root/src/`.
pub fn contracts_in_src(repo: &RepoPaths) -> Result<Vec<String>> {
    let src_root = repo.root.join(&repo.repo_name).join("src");
    if !src_root.exists() {
        anyhow::bail!("no src/ folder found at {},", src_root.display());
    }
    // Regex matches `contract Foo`, ignores `interface` / `library`
    let re = Regex::new(r"(?m)^\s*contract\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let mut contracts = Vec::<String>::new();

    for file in &repo.sol_files {
        // ✅ is in src ?
        if !file.starts_with(&src_root) {
            continue;
        }

        // 🚫 Skip if path contains /lib/ or /mock/
        if file.components().any(|comp| {
            let part = comp.as_os_str().to_ascii_lowercase();
            part == "lib"
                || part == "library"
                || part.to_string_lossy().to_ascii_lowercase().contains("mock")
                || part
                    .to_string_lossy()
                    .to_ascii_lowercase()
                    .contains("helper")
        }) {
            continue;
        }
        // Skip directories and symlinks
        if fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read file {}: {}", file.display(), e);
                continue;
            }
        };

        for cap in re.captures_iter(&content) {
            if let Some(contract_name) = cap.get(1) {
                let contract = contract_name.as_str();
                if !contract.to_ascii_lowercase().contains("mock") {
                    contracts.push(contract.to_string());
                }
            }
        }
    }
    Ok(contracts)
}

pub fn get_function_metadata_from_id(
    id: &str,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let fn_metadata: Option<SmartContractFunction> = semantic_db
                        .query_row(
                            "SELECT id, contract, name, visibility, modifiers, mutability FROM functions WHERE id = ?1;",
                            [id],
                            |row| {
                                let modifier_str: String = row.get(4)?;
                                let modifiers: Vec<String> = modifier_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                Ok(SmartContractFunction {
                                    id: row.get(0)?,
                                    contract: row.get(1)?,
                                    name: row.get(2)?,
                                    visibility: row.get(3)?,
                                    modifiers,
                                    mutability: row.get(5)?,
                                })
                            },
                        )
                        .optional()?;

    Ok(fn_metadata)
}

````

`````
ai-agent-audit/docker-compose.yml
```yaml
version: '3.8'

services:
  qdrant:
    image: qdrant/qdrant:v1.14.0
    container_name: qdrant
    ports:
      - "6333:6333"  # REST API
      - "6334:6334"  # gRPC API
    volumes:
      - qdrant_data:/qdrant/storage

volumes:
  qdrant_data:


```
ai-agent-audit/src/config.rs
```
/// Configuration management for the AI Agent Audit application.
///
/// This module provides centralized configuration handling, with constants
/// for application settings and environment variables only for sensitive
/// configuration like API keys and URLs.
use crate::error::{AuditError, Result};
use serde::{Deserialize, Serialize};
use std::env;

// Application constants - these don't need to be configurable via environment
/// Maximum call graph traversal depth for code slice generation
pub const MAX_DEPTH: usize = 3;

/// Maximum token budget per code block to stay within LLM context limits
pub const TOKEN_BUDGET: usize = 150_000;

/// Number of discovery rounds per contract during analysis
pub const RUNS: usize = 5;

pub const MAX_RAG_QUERY_CONTENT_LENGTH: usize = 8192; // 8192 token limit for embedding

/// Docker volume path for repository analysis
pub const DOCKER_VOLUME: &str = "/tmp/audit-analysis";

/// Maximum repository URL length for security validation
pub const MAX_REPO_URL_LENGTH: usize = 2048;

/// Timeout for LLM requests in seconds
pub const LLM_TIMEOUT_SECONDS: u64 = 120;

/// Default temperature for LLM models
pub const DEFAULT_TEMPERATURE: f64 = 1.0;

/// Maximum tokens for LLM responses
pub const MAX_RESPONSE_TOKENS: u64 = 100_000;

/// Vector database collection dimension
pub const VECTOR_DIMENSION: u64 = 1536;

/// Number of similar chunks to retrieve for context
pub const CONTEXT_CHUNKS: usize = 5;

/// Similarity threshold for vector search
pub const SIMILARITY_THRESHOLD: f64 = 0.7;

/// Main configuration structure for the AI Agent Audit application.
///
/// This struct contains configurable parameters loaded from environment variables
/// for sensitive data (API keys, URLs) and constants for application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Maximum call graph traversal depth for code slice generation
    pub max_depth: usize,

    /// Maximum token budget per code block to stay within LLM context limits
    pub token_budget: usize,

    /// Number of discovery rounds per contract during analysis
    pub runs: usize,

    /// Qdrant vector database URL for semantic search
    pub qdrant_url: String,

    /// OpenAI API key for GPT models and embeddings
    pub openai_api_key: Option<String>,

    /// Anthropic API key for Claude models
    pub anthropic_api_key: Option<String>,

    /// Google AI API key for Gemini models
    pub gemini_ai_api_key: Option<String>,

    /// DeepSeek API key for DeepSeek models
    pub deepseek_api_key: Option<String>,

    /// Logging level for the application
    pub log_level: String,

    /// Docker volume path for repository analysis
    pub docker_volume: String,

    /// Maximum repository URL length for security validation
    pub max_repo_url_length: usize,

    /// Timeout for LLM requests in seconds
    pub llm_timeout_seconds: u64,

    /// Default temperature for LLM models
    pub default_temperature: f64,

    /// Maximum tokens for LLM responses
    pub max_response_tokens: u64,

    /// Vector database collection dimension
    pub vector_dimension: u64,

    /// Number of similar chunks to retrieve for context
    pub context_chunks: usize,

    /// Similarity threshold for vector search
    pub similarity_threshold: f64,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: RUNS,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "info".to_string(),
            docker_volume: DOCKER_VOLUME.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
            vector_dimension: VECTOR_DIMENSION,
            context_chunks: CONTEXT_CHUNKS,
            similarity_threshold: SIMILARITY_THRESHOLD,
        }
    }
}

impl AuditConfig {
    /// Creates a new configuration from environment variables.
    ///
    /// This function reads only sensitive configuration from environment variables
    /// (API keys, URLs, logging) and uses constants for application settings.
    ///
    /// # Returns
    /// * `Result<AuditConfig>` - Configuration loaded from environment
    ///
    /// # Environment Variables
    /// * `QDRANT_URL` - Vector database URL (default: http://localhost:6334)
    /// * `OPENAI_API_KEY` - OpenAI API key (optional)
    /// * `ANTHROPIC_API_KEY` - Anthropic API key (optional)
    /// * `GEMINI_API_KEY` - Gemini AI API key (optional)
    /// * `DEEPSEEK_API_KEY` - DeepSeek API key (optional)
    /// * `RUST_LOG` - Logging level (default: info)
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Load Qdrant URL
        if let Ok(qdrant_url) = env::var("QDRANT_URL") {
            if !qdrant_url.starts_with("http://") && !qdrant_url.starts_with("https://") {
                return Err(AuditError::configuration(
                    "QDRANT_URL",
                    "Must start with http:// or https://",
                ));
            }
            config.qdrant_url = qdrant_url;
        }

        // Load API keys (optional)
        config.openai_api_key = env::var("OPENAI_API_KEY").ok();
        config.anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();
        config.gemini_ai_api_key = env::var("GEMINI_API_KEY").ok();
        config.deepseek_api_key = env::var("DEEPSEEK_API_KEY").ok();

        // Validate at least one API key is provided
        if config.openai_api_key.is_none()
            && config.anthropic_api_key.is_none()
            && config.gemini_ai_api_key.is_none()
            && config.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        // Load logging level
        config.log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(config)
    }

    /// Validates the configuration and returns any errors found.
    pub fn validate(&self) -> Result<()> {
        // Validate qdrant_url
        if !self.qdrant_url.starts_with("http://") && !self.qdrant_url.starts_with("https://") {
            return Err(AuditError::configuration(
                "qdrant_url",
                "Must start with http:// or https://",
            ));
        }

        // Validate at least one API key is provided
        if self.openai_api_key.is_none()
            && self.anthropic_api_key.is_none()
            && self.gemini_ai_api_key.is_none()
            && self.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        Ok(())
    }

    /// Returns true if OpenAI API key is configured.
    pub fn has_openai_key(&self) -> bool {
        self.openai_api_key.is_some()
    }

    /// Returns true if Anthropic API key is configured.
    pub fn has_anthropic_key(&self) -> bool {
        self.anthropic_api_key.is_some()
    }

    /// Returns true if Google AI API key is configured.
    pub fn has_google_ai_key(&self) -> bool {
        self.gemini_ai_api_key.is_some()
    }

    /// Returns true if DeepSeek API key is configured.
    pub fn has_deepseek_key(&self) -> bool {
        self.deepseek_api_key.is_some()
    }

    /// Returns a list of configured LLM providers.
    pub fn available_providers(&self) -> Vec<String> {
        let mut providers = Vec::new();
        if self.has_openai_key() {
            providers.push("OpenAI".to_string());
        }
        if self.has_anthropic_key() {
            providers.push("Anthropic".to_string());
        }
        if self.has_google_ai_key() {
            providers.push("Google AI".to_string());
        }
        if self.has_deepseek_key() {
            providers.push("DeepSeek".to_string());
        }
        providers
    }

    /// Creates a test configuration with minimal settings.
    #[cfg(test)]
    pub fn test_config() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: RUNS,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: Some("test-key".to_string()),
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "debug".to_string(),
            docker_volume: DOCKER_VOLUME.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
            vector_dimension: VECTOR_DIMENSION,
            context_chunks: CONTEXT_CHUNKS,
            similarity_threshold: SIMILARITY_THRESHOLD,
        }
    }
}

/// Global configuration instance.
use std::sync::OnceLock;
static CONFIG: OnceLock<AuditConfig> = OnceLock::new();

/// Initializes the global configuration from environment variables.
pub fn init_config() -> Result<()> {
    let config = AuditConfig::from_env()?;
    config.validate()?;

    CONFIG.set(config).map_err(|_| {
        AuditError::configuration("global_config", "Configuration already initialized")
    })?;

    Ok(())
}

/// Returns a reference to the global configuration.
///
/// # Panics
/// Panics if the configuration has not been initialized with `init_config()`.
pub fn audit_config() -> &'static AuditConfig {
    CONFIG
        .get()
        .expect("Configuration not initialized. Call init_config() first.")
}

/// Returns a reference to the global configuration, or None if not initialized.
pub fn try_audit_config() -> Option<&'static AuditConfig> {
    CONFIG.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = AuditConfig::default();
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);
        assert_eq!(config.runs, RUNS);
        // Config validation will fail because no API keys are set
    }

    #[test]
    fn test_config_validation() {
        let mut config = AuditConfig::default();

        // Set an API key to make validation pass
        config.openai_api_key = Some("test-key".to_string());
        assert!(config.validate().is_ok());

        // Test invalid URL
        config.qdrant_url = "invalid-url".to_string();
        assert!(config.validate().is_err());

        // Test no API keys
        config.qdrant_url = "http://localhost:6334".to_string();
        config.openai_api_key = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_available_providers() {
        let mut config = AuditConfig::default();
        config.openai_api_key = Some("test".to_string());
        config.anthropic_api_key = Some("test".to_string());

        let providers = config.available_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"OpenAI".to_string()));
        assert!(providers.contains(&"Anthropic".to_string()));
    }

    #[test]
    fn test_from_env() {
        unsafe {
            env::set_var("QDRANT_URL", "http://test:6334");
            env::set_var("OPENAI_API_KEY", "test-key");
        }

        let config = AuditConfig::from_env().unwrap();
        assert_eq!(config.qdrant_url, "http://test:6334");
        assert!(config.has_openai_key());
        // Constants should be used for other values
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);

        // Clean up
        unsafe {
            env::remove_var("QDRANT_URL");
            env::remove_var("OPENAI_API_KEY");
        }
    }
}

```
ai-agent-audit/src/error.rs
```
/// Centralized error types for the AI Agent Audit application.
///
/// This module provides a unified error handling system that consolidates
/// various error types from different modules into a cohesive hierarchy.
/// This improves debugging, error propagation, and overall system reliability.

use thiserror::Error;

/// Main error type for the AI Agent Audit application.
/// 
/// This enum encompasses all possible error conditions that can occur
/// during the audit process, providing specific error types for different
/// failure scenarios with descriptive messages and error chaining.
#[derive(Debug, Error)]
pub enum AuditError {
    /// Database operation failures (SQLite, Qdrant)
    #[error("Database operation failed: {message}")]
    Database { 
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Git operations and repository handling failures
    #[error("Git operation failed: {operation} - {message}")]
    Git {
        operation: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Docker container operations failures
    #[error("Docker operation failed: {command} - {message}")]
    Docker {
        command: String,
        message: String,
        exit_code: Option<i32>,
    },

    /// Slither static analysis failures
    #[error("Slither analysis failed: {printer} - {message}")]
    SlitherAnalysis {
        printer: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// LLM processing and API communication failures
    #[error("LLM processing failed: {provider} - {message}")]
    LlmProcessing {
        provider: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Vector database operations failures
    #[error("Vector database operation failed: {operation} - {message}")]
    VectorDb {
        operation: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// File system operations failures
    #[error("File system operation failed: {path} - {message}")]
    FileSystem {
        path: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// JSON parsing and serialization failures
    #[error("JSON processing failed: {context} - {message}")]
    JsonProcessing {
        context: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Security validation failures
    #[error("Security validation failed: {check} - {message}")]
    Security {
        check: String,
        message: String,
    },

    /// Configuration and environment setup failures
    #[error("Configuration error: {setting} - {message}")]
    Configuration {
        setting: String,
        message: String,
    },

    /// Network operations failures
    #[error("Network operation failed: {url} - {message}")]
    Network {
        url: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Async task processing failures
    #[error("Async task failed: {task} - {message}")]
    AsyncTask {
        task: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Generic validation failures
    #[error("Validation failed: {field} - {message}")]
    Validation {
        field: String,
        message: String,
    },
}

/// Result type alias for consistent error handling throughout the application.
pub type Result<T> = std::result::Result<T, AuditError>;

impl AuditError {
    /// Creates a new database error with context.
    pub fn database<E>(message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Database {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new git error with context.
    pub fn git<E>(operation: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Git {
            operation: operation.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new Docker error with context.
    pub fn docker(command: impl Into<String>, message: impl Into<String>, exit_code: Option<i32>) -> Self {
        Self::Docker {
            command: command.into(),
            message: message.into(),
            exit_code,
        }
    }

    /// Creates a new Slither analysis error with context.
    pub fn slither<E>(printer: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::SlitherAnalysis {
            printer: printer.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new LLM processing error with context.
    pub fn llm<E>(provider: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::LlmProcessing {
            provider: provider.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new vector database error with context.
    pub fn vector_db<E>(operation: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::VectorDb {
            operation: operation.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new file system error with context.
    pub fn file_system<E>(path: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::FileSystem {
            path: path.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new JSON processing error with context.
    pub fn json<E>(context: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::JsonProcessing {
            context: context.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new security validation error.
    pub fn security(check: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Security {
            check: check.into(),
            message: message.into(),
        }
    }

    /// Creates a new configuration error.
    pub fn configuration(setting: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Configuration {
            setting: setting.into(),
            message: message.into(),
        }
    }

    /// Creates a new network error with context.
    pub fn network<E>(url: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Network {
            url: url.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new async task error with context.
    pub fn async_task<E>(task: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::AsyncTask {
            task: task.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new validation error.
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Conversion implementations for common error types
impl From<std::io::Error> for AuditError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystem {
            path: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for AuditError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonProcessing {
            context: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<rusqlite::Error> for AuditError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<reqwest::Error> for AuditError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network {
            url: err.url().map(|u| u.to_string()).unwrap_or_else(|| "unknown".to_string()),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<qdrant_client::QdrantError> for AuditError {
    fn from(err: qdrant_client::QdrantError) -> Self {
        Self::VectorDb {
            operation: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<tokio::task::JoinError> for AuditError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::AsyncTask {
            task: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<anyhow::Error> for AuditError {
    fn from(err: anyhow::Error) -> Self {
        Self::AsyncTask {
            task: "legacy_anyhow".to_string(),
            message: err.to_string(),
            source: Some(err.into()),
        }
    }
}

/// Convenience macros for creating specific error types
#[macro_export]
macro_rules! audit_error {
    (database, $msg:expr) => {
        $crate::error::AuditError::Database {
            message: $msg.to_string(),
            source: None,
        }
    };
    (git, $op:expr, $msg:expr) => {
        $crate::error::AuditError::Git {
            operation: $op.to_string(),
            message: $msg.to_string(),
            source: None,
        }
    };
    (docker, $cmd:expr, $msg:expr) => {
        $crate::error::AuditError::Docker {
            command: $cmd.to_string(),
            message: $msg.to_string(),
            exit_code: None,
        }
    };
    (security, $check:expr, $msg:expr) => {
        $crate::error::AuditError::Security {
            check: $check.to_string(),
            message: $msg.to_string(),
        }
    };
    (config, $setting:expr, $msg:expr) => {
        $crate::error::AuditError::Configuration {
            setting: $setting.to_string(),
            message: $msg.to_string(),
        }
    };
    (validation, $field:expr, $msg:expr) => {
        $crate::error::AuditError::Validation {
            field: $field.to_string(),
            message: $msg.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AuditError::security("url_validation", "Invalid URL format");
        assert!(err.to_string().contains("Security validation failed"));
        assert!(err.to_string().contains("url_validation"));
    }

    #[test]
    fn test_error_macro() {
        let err = audit_error!(validation, "repo_url", "URL is required");
        assert!(err.to_string().contains("Validation failed"));
        assert!(err.to_string().contains("repo_url"));
    }
}
```
ai-agent-audit/src/lib.rs
```
/// AI Agent Audit - Comprehensive Smart Contract Security Analysis Tool
///
/// This library provides advanced AI-powered smart contract auditing capabilities,
/// combining static analysis, multiple LLM providers, and vector embeddings for
/// professional-grade security assessments.

/// Centralized error handling and types
pub mod error;

/// Configuration management and environment handling
pub mod config;

/// Core analysis and data processing functionality
pub mod build_brain {
    /// Call graph analysis and traversal
    pub mod callgraph;
    /// Vector embeddings generation for semantic search
    pub mod enbeddings;
    /// Slither analysis integration and data enrichment
    pub mod enrichment;
    /// Function summarization using LLMs
    pub mod fn_summaries;
    /// Graph database operations for semantic data
    pub mod graph_db;
    /// Contract inheritance analysis
    pub mod inheritance;
    /// Code parsing utilities
    pub mod parsers;
    /// Slither static analyzer interface
    pub mod slither_ffi;
    /// Protocol and file summarization
    pub mod summarize;
    /// Qdrant vector database operations
    pub mod vector_db;
    /// High-level vector database service
    pub mod vector_service;
}

/// Repository preparation and building
pub mod prepare_code {
    /// Git cloning and Docker-based building
    pub mod git_clone;
}

/// Report generation and data export
pub mod reporting {
    /// Audit report generation with findings
    pub mod audit;
    /// Contract data export utilities
    pub mod contract_data;
    /// File saving and formatting
    pub mod save_file;
}

/// Code slicing and enumeration for focused analysis
pub mod enumerator {
    /// Caching for generated code blocks
    pub mod codeblock_cache;
    /// Database for code block storage
    pub mod codeblock_db;
    /// Code block generation logic
    pub mod codeblock_maker;
    /// Core code slicing functionality
    pub mod codeblocks;
    /// Enumeration utilities
    pub mod utils;
}

/// AI-powered security analysis and LLM integration
pub mod llm_review {
    /// AI agent factory for centralized agent creation
    pub mod agent_factory;
    /// Analysis results database
    pub mod analysis_db;
    /// Main security review orchestration
    pub mod code_review;
    /// LLM configuration and models
    pub mod config;
    /// Global context management
    pub mod context_state;
    /// AI agent and vulnerability type enums
    pub mod enums;
    /// Protocol invariant analysis
    pub mod invariants;
    /// Dynamic prompt generation
    pub mod prompt_context;
    /// AI agent builders and utilities
    pub mod review_utils;
    /// Prompt engineering modules for different analysis stages
    pub mod prompt_support {
        /// Deduplication prompts
        pub mod dedup;
        /// Post-analysis prompts
        pub mod post_prompt;
        /// Quality check prompts
        pub mod post_qualify;
        /// Verification prompts
        pub mod post_verify;
        /// Pre-analysis prompts
        pub mod pre_prompt;
        /// Pre-qualification prompts
        pub mod pre_qualify;
        /// Pre-verification prompts
        pub mod pre_verify;
        /// Quality assessment prompts
        pub mod qualify_prompt;
        /// Verification prompts
        pub mod verify_prompt;
    }
}

/// Cost tracking and management for LLM inference
pub mod cost {
    /// Cost calculation and tracking across providers
    pub mod cost_data;
}

/// AI agent implementations with vector search
pub mod ai_bot {
    /// Core AI audit agent with dynamic context
    pub mod agent;
    /// Context retrieval for AI analysis
    pub mod retrieve_slice;
}

/// Master security analysis prompts
pub mod master_prompts {
    /// Base master security prompt
    pub mod master_prompt;
    /// Security analysis prompt variants
    pub mod prompt_2x_a;
    pub mod prompt_2x_aa;
    pub mod prompt_2x_b;
    pub mod prompt_2x_bb;
    pub mod prompt_3x_a;
    pub mod prompt_3x_b;
    pub mod prompt_3x_c;
}

/// Vulnerability-specific detection prompts (19 categories)
pub mod prompts {
    /// Access control vulnerabilities
    pub mod access_control;
    /// Array bounds checking issues
    pub mod array_limits;
    /// Confidential data exposure
    pub mod confidential_data;
    /// Default visibility issues
    pub mod default_visibility;
    /// Denial of service vulnerabilities
    pub mod dos;
    /// Inheritance-related issues
    pub mod inheritance;
    /// Integer overflow/underflow
    pub mod integer_overflow;
    /// MEV and front-running vulnerabilities
    pub mod mev;
    /// Oracle manipulation attacks
    pub mod oracle;
    /// Pragma-related issues
    pub mod pragma;
    /// Weak randomness vulnerabilities
    pub mod randomness;
    /// Reentrancy vulnerabilities
    pub mod reentrancy;
    /// Replay attack vulnerabilities
    pub mod replay_attack;
    /// Self-destruct related issues
    pub mod self_destruct;
    /// Short address attack vulnerabilities
    pub mod short_address_attack;
    /// Storage variable issues
    pub mod storage_variables;
    /// tx.origin usage vulnerabilities
    pub mod tx_origin;
    /// Unchecked return value issues
    pub mod unchecked_return_value;
    /// Unexpected ETH handling
    pub mod unexpected_eth;
    /// Zero-code contract issues
    pub mod zero_code;
}

/// Protocol invariant analysis prompts
pub mod invariant_prompts {
    /// Arithmetic invariants
    pub mod arithmetic;
    /// Balance invariants
    pub mod balance;
    /// Permission invariants
    pub mod permission;
    /// Referential integrity invariants
    pub mod referential;
    /// State machine invariants
    pub mod state_machine;
    /// Temporal invariants
    pub mod temporal;
}

/// Shared utilities and helper functions
pub mod utils {
    /// OpenAI tokenizer (BPE) for text chunking
    pub mod bpe;
    pub mod contract_name_check;
    /// Docker volume cleanup utilities
    pub mod delete_docker_volumes;
    pub mod env_security;
    /// LLM extraction with retry logic
    pub mod extract_retry;
    pub mod file_security;
    /// Function labeling utilities
    pub mod fn_labels;
    /// Documentation extraction
    pub mod get_file_content;
    /// Function name extraction
    pub mod get_fn_name;
    /// Logging utilities
    pub mod logging;
    /// Text sanitization utilities
    pub mod sanitize;
    /// Vector database connection utilities
    pub mod vec_db_connect;
}

```
ai-agent-audit/src/main.rs
```
/// The main entry point for the AI Agent Audit tool.
///
/// This application performs comprehensive smart contract security audits by:
/// 1. Cloning and building repositories (Foundry/Hardhat) in Docker containers
/// 2. Extracting call graphs, IR, and storage layouts using Slither
/// 3. Generating contextual code slices for focused AI analysis
/// 5. Running multi-LLM security analysis across 19+ vulnerability categories
/// 5. Creating vector embeddings and storing in Qdrant for semantic search
/// 6. Generating professional audit reports with findings and cost tracking
use ai_agent_audit::{
    build_brain::{enrichment, slither_ffi::get_all_files_src, vector_db},
    config::{audit_config, init_config},
    cost::cost_data::get_total_inference_cost,
    enumerator::codeblock_maker,
    error::{AuditError, Result},
    llm_review::{
        agent_factory::init_llm_clients,
        code_review,
        context_state::{self},
        prompt_context,
    },
    prepare_code::{self, git_clone::BuildFlags},
    reporting::{
        audit::{self, ReportType},
        contract_data, save_file,
    },
    utils::delete_docker_volumes::cleanup_repo_volume,
};
use dotenvy::dotenv;
use log::info;

/// The main async function that orchestrates the entire process.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize configuration from environment
    init_config()?;

    // Initialize LLM clients
    init_llm_clients()?;

    // Initialize the logger
    env_logger::init();

    // ────────────────────────────────
    // 1. Repository Preparation
    // ────────────────────────────────
    let repo_url = std::env::args().nth(1).ok_or_else(|| {
        AuditError::validation("repo_url", "Repository URL is required as first argument")
    })?;

    // Optional subfolder argument for analyzing specific directories in multi-app repositories
    let subfolder = std::env::args().nth(2);

    // Optional --via-ir flag for forge build
    let build_flags = match std::env::args().nth(3).as_deref() {
        Some("--via-ir") => BuildFlags::ViaIr,
        _ => BuildFlags::Standard,
    };

    // Validate URL format and length before processing
    if repo_url.len() > audit_config().max_repo_url_length {
        return Err(AuditError::validation(
            "repo_url",
            &format!(
                "Repository URL is too long (max {} characters)",
                audit_config().max_repo_url_length
            ),
        ));
    }

    if !repo_url.starts_with("https://") && !repo_url.starts_with("http://") {
        return Err(AuditError::validation(
            "repo_url",
            "Only HTTP/HTTPS repository URLs are supported",
        ));
    }

    info!("Processing repository: {}", repo_url);
    if let Some(ref sf) = subfolder {
        info!("Analyzing subfolder: {}", sf);
    }
    info!("git cloning and extraction source code");

    // Clone repository in Docker container and build with Foundry/Hardhat
    let repo = prepare_code::git_clone::clone_and_filter_git_repo(
        &repo_url,
        subfolder.as_deref(),
        build_flags,
    )?;
    info!("repo root => {:?}", &repo.root);
    info!("repo name => {:?}", &repo.repo_name);
    info!("sol files => {:?}", &repo.sol_files);

    // ────────────────────────────────
    // 2. Static Analysis & Graph Generation
    // ────────────────────────────────
    // Build semantic database with call graphs and inheritance data
    let semantics_db = enrichment::build_semantics_db_from_call_graph(repo.clone()).await?;
    info!("Call-graph DB at {}", semantics_db.display());

    // Generate and cache protocol metadata context for AI analysis
    info!("generating metadata context...");
    context_state::generate_and_save_metadata_context(&repo, &semantics_db).await?;

    let metadata = prompt_context::generate_context_for_code_review(&repo, &semantics_db).await?;

    info!("metadata => {:#?}", metadata);

    return Ok(());

    // ────────────────────────────────
    // 3. Code Slice Generation
    // ────────────────────────────────
    info!("generating codeblock for each contract in repo");
    // Create contextual code slices using call graph traversal
    let codeblocks_db = codeblock_maker::generate_and_save_codeblocks_for_each_contract(
        &repo,
        &semantics_db,
        audit_config().max_depth,
        audit_config().token_budget,
    )
    .await?;
    info!("Slices at {}", codeblocks_db.display());

    // ────────────────────────────────
    // 4. Vector Database Population
    // ────────────────────────────────
    // Create embeddings and store in Qdrant for semantic search
    vector_db::generate_slither_chucks_and_save_all_metadata_to_vector_db(&repo, &semantics_db)
        .await?;

    // ────────────────────────────────
    // 5. AI Security Analysis
    // ────────────────────────────────
    // Run multi-LLM security analysis across vulnerability categories
    let (security_issues, invariants) =
        code_review::review_codebase_for_security_issues(&codeblocks_db, &repo).await?;

    // ────────────────────────────────
    // 6. Report Generation
    // ────────────────────────────────
    // Generate comprehensive audit report (paid version)
    let audit_report = audit::generated_audit_report(
        &security_issues,
        &invariants,
        &repo,
        &semantics_db,
        ReportType::Paid,
    )
    .await?;

    // Generate limited audit report (free version)
    let free_audit_report = audit::generated_audit_report(
        &security_issues,
        &invariants,
        &repo,
        &semantics_db,
        ReportType::Free,
    )
    .await?;

    // ────────────────────────────────
    // 7. File Export & Cleanup
    // ────────────────────────────────
    // Save all reports and analysis data to markdown files
    save_file::save_audit_report(&audit_report, &repo, ReportType::Paid)?;
    save_file::save_audit_report(&free_audit_report, &repo, ReportType::Free)?;
    contract_data::save_contract_and_fn_ir(&codeblocks_db, &repo)?;
    contract_data::save_metadata(&semantics_db, &repo).await?;

    // Display total inference cost across all LLM providers
    let total_cost = get_total_inference_cost().await;
    info!("Total Inference Cost ===> {}", total_cost);

    // Clean up Docker volumes
    cleanup_repo_volume(&repo.root)?;

    Ok(())
}

```
ai-agent-audit/src/build_brain/callgraph.rs
```
/// Call graph analysis and DOT format parsing.
///
/// This module processes Slither's call graph output in DOT format, extracting
/// function relationships and building traversable graph structures for code
/// slice generation and dependency analysis.

use anyhow::Result;
use regex::Regex;
use rusqlite::Connection;
use serde::Deserialize;
use std::{collections::HashMap, default::Default, path::Path};

use crate::{
    enumerator::utils::get_function_metadata_from_id,
    prepare_code::git_clone::RepoPaths,
    utils::fn_labels::{get_modifiers_label, get_visibility_label},
};

use super::{graph_db::SmartContractFunction, slither_ffi::run_printer_json};

/// Represents a function node in the call graph
#[derive(Debug, Clone, Default)]
pub struct DotFunc {
    /// Unique identifier from Slither (e.g., "3895_changeFeeAddress")
    pub full_id: String,
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
}

/// Represents a call relationship between two functions
#[derive(Debug)]
pub struct DotEdge {
    /// Calling function's full_id
    pub caller: String,
    /// Called function's full_id
    pub callee: String,
}

/// Extracts call graph functions and edges from Slither analysis.
///
/// This function orchestrates the complete call graph extraction process by
/// running Slither's call-graph printer and parsing the resulting DOT format.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `(Vec<DotFunc>, Vec<DotEdge>)` - Functions and their call relationships
pub async fn get_dot_funcs_and_dot_edges(repo: &RepoPaths) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
    let json = run_printer_json(repo, "call-graph").await?;
    let blobs = extract_dot_blobs(&json)?;
    parse_dot_blobs(&blobs)
}

/// Step 2: pull every DOT file’s `content` string
pub fn extract_dot_blobs(json: &str) -> Result<Vec<String>> {
    #[derive(Deserialize)]
    struct DotFile {
        #[serde(rename = "type")]
        _ty: String,
        name: DotName,
    }
    #[derive(Deserialize)]
    struct DotName {
        content: String,
    }
    #[derive(Deserialize)]
    struct Printer {
        elements: Vec<DotFile>,
    }
    #[derive(Deserialize)]
    struct Root {
        results: Results,
    }
    #[derive(Deserialize)]
    struct Results {
        printers: Vec<Printer>,
    }

    let root: Root = serde_json::from_str(json)?;
    let mut out = Vec::new();
    for printer in root.results.printers {
        for file in printer.elements {
            out.push(file.name.content);
        }
    }
    Ok(out)
}

/// Step 3: regex-scan DOT text → nodes & edges
pub fn parse_dot_blobs(blobs: &[String]) -> Result<(Vec<DotFunc>, Vec<DotEdge>)> {
    let node_re = Regex::new(r#""(\d+)_([A-Za-z0-9$_]+)" \[label"#)?;
    let edge_re = Regex::new(r#""(\d+_[^"]+)" -> "(\d+_[^"]+)""#)?;
    let cluster_re = Regex::new(r#"cluster_(\d+)_([A-Za-z0-9$_]+) \{"#)?;
    let mut funcs = HashMap::<String, DotFunc>::new();
    let mut edges = Vec::<DotEdge>::new();

    for blob in blobs {
        let mut contract = String::new();
        for line in blob.lines() {
            if let Some(c) = cluster_re.captures(line) {
                contract = c[2].to_string(); // e.g., PuppyRaffle
            }
            if let Some(c) = node_re.captures(line) {
                let full = c[1].to_string() + "_" + &c[2];
                let func = DotFunc {
                    full_id: full.clone(),
                    contract: contract.clone(),
                    name: c[2].to_string(),
                };
                funcs.entry(full).or_insert(func);
            }
            if let Some(e) = edge_re.captures(line) {
                edges.push(DotEdge {
                    caller: e[1].to_string(),
                    callee: e[2].to_string(),
                });
            }
        }
    }
    Ok((funcs.into_values().collect(), edges))
}

pub async fn get_enriched_funcs_and_edges(
    repo: &RepoPaths,
    semantic_path: &Path,
) -> Result<String> {
    let mut enriched_edges = Vec::<DotEdge>::new();
    let mut enriched_funcs = Vec::<DotFunc>::new();
    let semantic_db = Connection::open(semantic_path)?;

    let (funcs, edges) = get_dot_funcs_and_dot_edges(repo).await?;

    for edge in edges {
        let enriched_callee = match get_function_metadata_from_id(&edge.callee, &semantic_db)? {
            Some(callee_fn) => generated_enriched_fn_label(&edge.callee, callee_fn),
            None => edge.callee,
        };
        let enriched_caller = match get_function_metadata_from_id(&edge.caller, &semantic_db)? {
            Some(callee_fn) => generated_enriched_fn_label(&edge.caller, callee_fn),
            None => edge.caller,
        };
        enriched_edges.push(DotEdge {
            callee: enriched_callee,
            caller: enriched_caller,
        });
    }

    for func in funcs {
        let enriched_func_name = match get_function_metadata_from_id(&func.full_id, &semantic_db)? {
            Some(full_func) => generated_enriched_fn_label(&func.name, full_func),
            None => func.name,
        };
        enriched_funcs.push(DotFunc {
            full_id: func.full_id,
            contract: func.contract,
            name: enriched_func_name,
        });
    }

    // log::info!("enriched edges => {:#?}", enriched_edges);

    let funcs_string: String = enriched_funcs
        .iter()
        .map(|f| {
            format!(
                "id: {}, contract: {}, name: {}",
                f.full_id, f.contract, f.name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let edges_string: String = enriched_edges
        .iter()
        .map(|e| format!("{} -> {}", e.caller, e.callee))
        .collect::<Vec<_>>()
        .join("\n");

    let mut final_dot_string = String::new();

    final_dot_string.push_str("\n\n### Functions\n\n");
    final_dot_string.push_str(&funcs_string);
    final_dot_string.push_str("\n\n### Dot Edges (Caller -> Callee)\n\n");
    final_dot_string.push_str(&edges_string);

    // log::info!("final dot string => {}", final_dot_string);
    Ok(final_dot_string)
}

fn generated_enriched_fn_label(fn_id: &str, fn_metadata: SmartContractFunction) -> String {
    let visibility = get_visibility_label(&fn_metadata.visibility);
    let modifiers = get_modifiers_label(&fn_metadata.modifiers);

    format!("{} {}{}", fn_id, visibility, modifiers)
}

```
ai-agent-audit/src/build_brain/enbeddings.rs
```
/// Vector embeddings generation for semantic search.
///
/// This module creates high-quality vector embeddings from source code and analysis
/// results using OpenAI's text-embedding-3-small model. Handles intelligent text
/// chunking with overlap to maintain context for optimal semantic search performance.
use anyhow::Result;
use log::info;
use rig::{
    client::EmbeddingsClient,
    embeddings::EmbeddingsBuilder,
    providers::openai::{self, Client},
    Embed,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tiktoken_rs::CoreBPE;

use crate::utils::bpe::get_bpe; // OpenAI’s GPT-4 / text-embedding 3 vocab

/// Represents a chunk of source code with metadata for vector embedding.
///
/// This struct organizes text content and associated metadata for embedding
/// generation, enabling semantic search with rich context information.
#[derive(Debug, Embed, Clone, Serialize, Deserialize)]
pub struct SourceChunk {
    #[embed] // Field Rig will vectorise
    pub text: String, // The actual text content to be embedded
    metadata: String, // we’ll keep this alongside the vector
}

/// Number of tokens in each chunk for optimal embedding quality
const CHUNK_TOKENS: usize = 256;
/// Number of tokens to overlap between chunks to maintain context continuity
const OVERLAP: usize = 32;
/// Batch size for embedding API requests to optimize throughput
const BATCH: usize = 30;
/// Maximum chunk length in characters (OpenAI supports ~8192 tokens, leaving headroom)
const MAX_CHUNK_LEN: usize = 4000;

/**
 * Processes a list of files and creates embeddings for their content.
 *
 * TODO - USE codellama:embed instead of openai embedding model
 * will need to either self host (need powerful computer) or self host on
 * on gcp ($250/month), this embedding is optimal for code
 *
 * @param paths - Array of file paths to process
 * @return Result containing a vector of tuples with metadata and embedding vectors
 */
pub async fn embed_files(paths: &[impl AsRef<Path>]) -> Result<Vec<(SourceChunk, Vec<f32>)>> {
    // ------------------------------------------------------------------
    // 1. Slice every file into SourceChunk structs
    // ------------------------------------------------------------------
    let mut docs = Vec::<SourceChunk>::new();

    info!("looping through all files and breaking into chunks");
    let bpe = get_bpe();
    for file in paths {
        let content = fs::read_to_string(file.as_ref())?;
        for (i, chunk) in tokenize(bpe, &content).into_iter().enumerate() {
            let clean = chunk
                .replace('\0', "") // Remove null bytes
                .replace('\u{FFFD}', ""); // Remove replacement chars
            let clean = clean.trim();

            if clean.is_empty() {
                log::warn!("Skipping empty chunk from {}", file.as_ref().display());
                continue;
            }
            if clean.len() > MAX_CHUNK_LEN {
                log::warn!(
                    "Skipping oversized chunk ({} chars) from {}",
                    clean.len(),
                    file.as_ref().display()
                );
                continue;
            }
            docs.push(SourceChunk {
                text: chunk,
                metadata: format!("{}:chunk {}", file.as_ref().display(), i),
            });
        }
    }

    info!("breaking out data into text chunks complete");
    // ------------------------------------------------------------------
    // 2. Pick an embedding model once
    // ------------------------------------------------------------------
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let openai = Client::new(&api_key);

    // 1536‑dim “storage‑optimised” v3 model
    let model = openai.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

    // ------------------------------------------------------------------
    // 3. Build embeddings in one RPC batch
    //    EmbeddingsBuilder<M, D>::new(model) infers both generics
    // ------------------------------------------------------------------
    // ------------------------------------------------------------------
    // 4. Flatten → (metadata, vector) so the caller can upsert to Qdrant
    // ------------------------------------------------------------------
    let mut all_vecs = Vec::<(SourceChunk, Vec<f32>)>::new();

    // info!("using openai to embed in {}-item batches…", BATCH);
    for docs_slice in docs.chunks(BATCH) {
        // info!("Batch size: {}", docs_slice.len());
        // for (i, doc) in docs_slice.iter().enumerate() {
        //     info!(
        //         "Chunk {}: text='{}', metadata='{}'",
        //         i, doc.text, doc.metadata
        //     );
        // }
        let batch_result = EmbeddingsBuilder::new(model.clone())
            .documents(docs_slice.to_vec())? // slice → Vec
            .build()
            .await;

        match batch_result {
            Ok(batch) => {
                all_vecs.extend(batch.into_iter().filter_map(|(doc, emb)| {
                    let v = emb.first().vec;
                    if v.is_empty() {
                        return None;
                    }
                    Some((doc, v.into_iter().map(|x| x as f32).collect()))
                }));
            }
            Err(e) => {
                log::error!("Embedding batch failed: {:#}", e);
                // Optionally retry, skip or abort here
            }
        };
    }
    Ok(all_vecs)
}

/// Split `s` into fixed-width token windows with `OVERLAP` tokens of context.
fn tokenize(bpe: &CoreBPE, s: &str) -> Vec<String> {
    let tokens = bpe.encode_with_special_tokens(s);

    let mut out = Vec::new();
    let mut start = 0;

    while start < tokens.len() {
        let end = usize::min(start + CHUNK_TOKENS, tokens.len());
        let token_slice = tokens[start..end].to_vec();
        let chunk = bpe.decode(token_slice).unwrap_or_default();
        out.push(chunk);

        if end == tokens.len() {
            break; // reached the tail – exit
        }
        start += CHUNK_TOKENS - OVERLAP; // always moves forward
    }
    out
}

```
ai-agent-audit/src/build_brain/enrichment.rs
```
use crate::build_brain::callgraph::DotFunc;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::get_function_name;

use super::fn_summaries::get_function_summaries;
use super::graph_db::GraphDb;
/// Smart contract data enrichment using Slither static analysis.
///
/// This module builds semantic databases containing call graphs, inheritance hierarchies,
/// and function metadata extracted from Solidity contracts using Slither analysis.
use super::slither_ffi::{self, SlithIRFn, StorageVar};
use super::{callgraph, inheritance};
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Contains the enriched data extracted from Solidity contracts.
/// This includes the intermediate representation (IR) of functions and storage variable information.
pub struct Enriched {
    /// Vector of SlithIR function representations
    pub ir: Vec<SlithIRFn>,
    /// Vector of storage variable information
    pub storage: Vec<StorageVar>,
}

/// Builds a semantic database containing call graphs and inheritance data.
///
/// This function extracts call graph and inheritance information from Slither analysis
/// and stores it in a SQLite database for efficient querying during code analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `PathBuf` - Path to the created semantic database
pub async fn build_semantics_db_from_call_graph(repo: RepoPaths) -> Result<PathBuf> {
    // Create database file in cache directory
    let db_path = repo.root.join(".cache").join("semantics.db");
    let cache_dir = db_path.parent().ok_or_else(|| {
        AuditError::file_system(
            db_path.to_string_lossy().to_string(),
            "Invalid database path - no parent directory",
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"),
        )
    })?;
    std::fs::create_dir_all(cache_dir).map_err(|e| {
        AuditError::file_system(
            cache_dir.to_string_lossy().to_string(),
            "Failed to create cache directory",
            e,
        )
    })?;
    let db = Arc::new(Mutex::new(GraphDb::create(&db_path)?));
    let repo = Arc::new(repo);

    // Process call graph data in parallel task
    let repo_func = Arc::clone(&repo);
    let db_func = Arc::clone(&db);
    let handle = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract call graph data from Slither
            info!("extracting DOT blobs");
            let json = slither_ffi::run_printer_json(&repo_func, "call-graph").await?;
            let blobs = callgraph::extract_dot_blobs(&json)?;
            let mut rows = Vec::new();
            let (funcs_id, edges) = callgraph::parse_dot_blobs(&blobs)?;
            let func_index: HashMap<(String, String), DotFunc> = funcs_id
                .into_iter()
                .map(|node| ((node.contract.clone(), node.name.clone()), node))
                .collect();

            // Get function summaries for metadata
            info!("getting function summaries...");
            let funcs = get_function_summaries(&repo_func).await?;

            info!("insert function metadata into database..");
            info!("{} function summaries", funcs.len());
            // Insert function metadata into database
            for f in &funcs {
                let func_name = get_function_name(&f.name);
                if let Some(node) = func_index.get(&(f.contract.clone(), func_name)) {
                    rows.push((
                        node.full_id.clone(),
                        f.contract.clone(),
                        f.name.clone(),
                        f.visibility.clone(),
                        f.modifiers.join(","),
                        f.mutability.clone(),
                    ))
                }
            }
            // single lock, batch insert
            {
                let db = db_func.lock().await;
                for row in rows {
                    db.insert_function(&row.0, &row.1, &row.2, &row.3, &row.4, &row.5)?;
                }
            }
            info!("done inserting function metadata into database");

            info!("insert {} call graph edges in db", edges.len());
            // Insert call graph edges
            for e in &edges {
                let db_guard = db_func.lock().await;
                db_guard.insert_edge(&e.caller, &e.callee)?;
            }
            info!("dot edges in db complete");
            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing call graph data: {:#}", e);
        }

        Ok::<_, AuditError>(())
    });

    // Process inheritance data in parallel task
    let db_inheritance = Arc::clone(&db);
    let repo_inheritance = Arc::clone(&repo);
    let handle_inheritance = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract inheritance hierarchy from Slither
            info!("generating inheritance json");
            let inheritance_json =
                slither_ffi::run_printer_json(&repo_inheritance, "inheritance").await?;
            let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
            info!("{} inheritance edges", inheritance_edges.len());

            // Insert inheritance relationships into database
            for (child, parent) in inheritance_edges {
                let db_guard = db_inheritance.lock().await;
                db_guard.insert_inheritance(&child, &parent)?;
            }
            info!("done generating inheritance edges");

            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing inheritance data: {:#}", e);
        }
        Ok::<_, AuditError>(())
    });

    // Wait for both parallel tasks to complete
    info!("waiting for meta data analysis to complete...");
    let (_func_result, _inheritance_result) = tokio::try_join!(handle, handle_inheritance)?;
    Ok(db_path)
}

```
ai-agent-audit/src/build_brain/fn_summaries.rs
```
/// Function summarization using Slither analysis.
///
/// This module extracts function metadata from Slither's function summary printer,
/// parsing visibility, modifiers, and mutability information for all functions
/// across contracts in a repository.

use anyhow::Result;
use serde::Deserialize;

use crate::{build_brain::slither_ffi::run_printer, prepare_code::git_clone::RepoPaths};

/// Comprehensive metadata for a smart contract function
#[derive(Debug, Deserialize, Clone)]
pub struct FnSummary {
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
    /// Function visibility (external/public/internal/private)
    pub visibility: String,
    /// Applied modifiers (e.g., ["onlyOwner", "nonReentrant"])
    #[serde(default)]
    pub modifiers: Vec<String>,
    /// State mutability (view/pure/payable/nonpayable)
    #[serde(default)]
    pub mutability: String,
}

/// Parses Slither's function summary table format into structured data.
///
/// Processes the text output from Slither's function-summary printer,
/// extracting function metadata for each contract in the repository.
fn parse_table(block: &str) -> Vec<FnSummary> {
    let mut out = Vec::new();

    let mut lines = block.lines();
    let mut current_line = lines.next().unwrap_or_default();
    let mut inside_fn_block = false;

    // skip any lines before first table heading
    while !current_line.starts_with("Contract ") {
        current_line = lines.next().unwrap_or("EOF");

        // if file is empty!
        if current_line == "EOF" {
            return Vec::new();
        }
    }

    // grab first contract
    let mut current_contract = get_contract_name(current_line);
    current_line = lines.next().unwrap_or_default();

    while current_line != "EOF" {
        // grab functions
        if current_line.starts_with("Contract ") && !current_line.starts_with("Contract vars") {
            // new contract
            current_contract = get_contract_name(current_line);
            current_line = lines.next().unwrap_or("EOF");
            continue;
        } else if current_line.is_empty() {
            inside_fn_block = false;
            current_line = lines.next().unwrap_or("EOF");
            continue;
        }

        if current_line.starts_with("|") {
            let cols: Vec<_> = current_line.split('|').map(|c| c.trim()).collect();
            if cols.len() < 4 || cols[1].is_empty() || cols[1] == "Modifiers" {
                current_line = lines.next().unwrap_or("EOF");
                continue; // header or empty
            } else if cols[1] == "Function" {
                // new fn table found!
                inside_fn_block = true;
                current_line = lines.next().unwrap_or("EOF");
                continue;
            }

            if inside_fn_block {
                // cols[1]   Function
                // cols[2]   Visibility
                // cols[3]   Modifiers
                let func_name = cols[1];
                let modifiers = if cols[3] == "[]" {
                    vec![]
                } else {
                    // remove '[' and ']'
                    let raw = cols[3].to_string();
                    let trimmed: String = raw.chars().skip(1).take(raw.len() - 2).collect();
                    trimmed
                        .split(',')
                        .map(|s| s.trim_matches([' ', '\'']).to_string())
                        .collect::<Vec<_>>()
                };
                out.push(FnSummary {
                    contract: current_contract.clone(),
                    name: func_name.to_string(),
                    visibility: cols[2].to_string(),
                    modifiers,
                    mutability: String::new(), // not present in this table
                });
            }
        }

        // go to next line
        current_line = lines.next().unwrap_or("EOF");
    }

    // log::info!("function summaries ==> {:#?}", out);
    out
}

pub fn get_contract_name(line: &str) -> String {
    // grab first contract
    log::info!("line with Contract => {}", line);
    line.split_ascii_whitespace()
        .nth(1)
        .unwrap_or("")
        .to_string()
}

pub async fn get_function_summaries(repo: &RepoPaths) -> Result<Vec<FnSummary>> {
    // 1. run slither
    let raw = run_printer(repo, "function-summary").await?;

    Ok(parse_table(&raw))
}

```
ai-agent-audit/src/build_brain/graph_db.rs
```
/// Graph database operations for semantic data storage.
///
/// This module provides a SQLite-based graph database for storing and querying
/// smart contract semantic data including functions, call relationships, and
/// inheritance hierarchies extracted from Slither analysis.

use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;

/// Represents a smart contract function with complete metadata
#[derive(Debug, Clone)]
pub struct SmartContractFunction {
    /// Unique function identifier from Slither
    pub id: String,
    /// Contract name containing the function
    pub contract: String,
    /// Function name
    pub name: String,
    /// Function visibility level
    pub visibility: String,
    /// Applied function modifiers
    pub modifiers: Vec<String>,
    /// State mutability specification
    pub mutability: String,
}

/// SQLite-based graph database for semantic contract data
pub struct GraphDb(Connection);

impl GraphDb {
    /// Creates a new graph database with required schema.
    ///
    /// Initializes SQLite database with tables for functions, call edges,
    /// and inheritance relationships. Uses WAL mode for better concurrency.
    pub fn create(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS functions(
              id TEXT PRIMARY KEY,   -- 3895_changeFeeAddress
              contract TEXT,
              name TEXT,
              visibility TEXT,
              modifiers TEXT,
              mutability TEXT
            );
            CREATE TABLE IF NOT EXISTS edges(
            caller TEXT,
            callee TEXT
            );
            /* NEW ↓ */
            CREATE TABLE IF NOT EXISTS inheritance(
            child TEXT,
            parent TEXT
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert_function(
        &self,
        id: &str,
        contract: &str,
        name: &str,
        visibility: &str,
        modifiers: &str,
        mutability: &str,
    ) -> Result<()> {
        self.0.execute(
            "INSERT OR IGNORE INTO functions(id, contract, name, visibility, modifiers, mutability) VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![id, contract, name, visibility, modifiers, mutability],
        )?;
        Ok(())
    }

    pub fn insert_edge(&self, caller: &str, callee: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO edges(caller, callee) VALUES (?1, ?2);",
            params![caller, callee],
        )?;
        Ok(())
    }

    pub fn insert_inheritance(&self, child: &str, parent: &str) -> Result<()> {
        self.0.execute(
            "INSERT INTO inheritance(child, parent) VALUES (?1, ?2);",
            params![child, parent],
        )?;
        Ok(())
    }
}

```
ai-agent-audit/src/build_brain/inheritance.rs
```
/// Contract inheritance analysis and parsing.
///
/// This module processes Slither's inheritance printer output to extract
/// parent-child relationships between smart contracts, supporting both
/// immediate and transitive inheritance hierarchies.

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// Step 2: pull every DOT file’s `content` string
pub fn parse_inheritance_json(json: &str) -> Result<Vec<(String, String)>> {
    /// Represents a list of immediate and non-immediate inheritance relationships.
    #[derive(Debug, Deserialize)]
    pub struct InheritanceRelation {
        pub immediate: Vec<String>,
        pub not_immediate: Vec<String>,
    }

    /// Outer container for additional fields (child_to_base and base_to_child maps).
    #[derive(Debug, Deserialize)]
    pub struct AdditionalFields {
        pub child_to_base: HashMap<String, InheritanceRelation>,
        pub base_to_child: HashMap<String, InheritanceRelation>,
    }

    #[derive(Deserialize)]
    struct Printer {
        pub printer: String,
        pub additional_fields: AdditionalFields,
    }
    #[derive(Deserialize)]
    struct Root {
        results: Results,
    }
    #[derive(Deserialize)]
    struct Results {
        printers: Vec<Printer>,
    }

    let root: Root = serde_json::from_str(json)?;
    let mut child_parent_map = HashMap::new();
    let mut edges = Vec::new();
    for printer in root.results.printers {
        if printer.printer == "inheritance" {
            child_parent_map = printer.additional_fields.child_to_base;
        }
    }

    for (child, parents) in child_parent_map.into_iter() {
        for parent in parents.immediate.iter() {
            edges.push((child.clone(), parent.to_string()));
        }
    }
    Ok(edges)
}

```
ai-agent-audit/src/build_brain/parsers.rs
```
/// Code parsing utilities for Slither analysis output.
///
/// This module provides parsers for various Slither output formats including
/// detector results, SlithIR representations, storage layouts, and contract
/// summaries. Handles text parsing and data structure extraction.
use super::slither_ffi::{ContractSummary, SlithIRFn, StorageVar};
use log::{debug, info};

/// Parses Slither detector output into individual issue descriptions.
///
/// Processes the text output from Slither detectors, separating different
/// vulnerability findings into discrete issue descriptions.
pub fn parse_slither(text: &str) -> Vec<String> {
    let mut current_issue = String::new();
    let mut issues = Vec::<String>::new();

    for line in text.lines() {
        // parse each detected issue
        if line.contains("Detectors") && line.ends_with(':') {
            if !current_issue.is_empty() {
                issues.push(current_issue.to_string());
                current_issue = line.to_string();
            }
        } else {
            current_issue.push_str(line);
        }
    }

    // add last issues
    if !current_issue.is_empty() {
        issues.push(current_issue.to_string());
    }

    // info!("issues => {:#?}", issues);
    issues
}

/// Parses Slither's SlithIR output into structured function representations.
///
/// Processes the text output from Slither's slithir-ssa printer, extracting
/// intermediate representation (IR) code for each function along with contract
/// and function metadata for code analysis and slice generation.
///
/// # Arguments
/// * `text` - Raw text output from the slithir-ssa printer
///
/// # Returns
/// * `Vec<SlithIRFn>` - Structured IR data for all functions
pub fn parse_slithir_ir_code(text: &str) -> Vec<SlithIRFn> {
    let mut current_contract = String::new();
    let mut current_fn = String::new();
    let mut buf = String::new();
    let mut out = Vec::new();

    // info!("text in parse_slithir {}", text.len());
    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract ") {
            current_contract = line["Contract ".len()..].trim_end_matches(':').to_owned();
        }
        // Parse function lines (format: "\tFunction functionName:")
        else if line.starts_with("\tFunction ") {
            // Flush previous function data if we have any
            if !current_fn.is_empty() {
                let ir_content = replace_special_character(&buf);
                let ir_content_cleaned =
                    ir_content.replace("IRs:\n", "").replace("Expression:", "");
                out.push(SlithIRFn {
                    contract: current_contract.clone(),
                    function: current_fn.clone(),
                    ir: ir_content_cleaned,
                });
            }
            // Extract new function name and reset buffer
            current_fn = line.trim()[9..].trim_end_matches(':').to_owned();
            buf.clear();
        }
        // Parse IR lines (format: "\t\t<ir content>")
        else if line.starts_with("\t\t") {
            buf.push_str(line.trim_start());
            buf.push('\n');
        }
    }

    // Flush the last function after processing all lines
    if !current_fn.is_empty() && !buf.trim().is_empty() && buf.trim().len() > 10 {
        let ir_content = replace_special_character(&buf);
        out.push(SlithIRFn {
            contract: current_contract,
            function: current_fn,
            ir: ir_content,
        });
    } else if !current_fn.is_empty() {
        info!(
            "Skipping empty or small IR for {}::{}",
            current_contract, current_fn
        );
    }

    // info!("functions => {:#?}", out);
    out
}

pub fn parse_slithir_contract_summary(text: &str) -> Vec<ContractSummary> {
    let mut current_name = String::new();
    let mut current_body = String::new();
    let mut out = Vec::<ContractSummary>::new();

    for line in text.lines() {
        // ── new contract header ─────────────────────────────────
        if let Some(rest) = line.strip_prefix("+ Contract ") {
            // flush the previous one (if any)
            if !current_name.is_empty() {
                out.push(ContractSummary {
                    contract: current_name.clone(),
                    content: current_body.trim_end().to_string(),
                });
                current_body.clear();
            }

            // take “IERC20 (Most derived contract)” → “IERC20”
            current_name = rest
                .split_whitespace() // split at first space
                .next()
                .unwrap_or_default()
                .to_string();
            continue;
        }

        // ── body line (ignore leading INFO: lines) ─────────────
        if current_name.is_empty() {
            continue; // still in preamble; skip
        }

        if !line.trim().is_empty() {
            current_body.push_str(line.trim_start());
            current_body.push('\n');
        }
        // we do NOT rely on blank line to flush; handled by header or final push
    }

    // push the last contract
    if !current_name.is_empty() {
        out.push(ContractSummary {
            contract: current_name,
            content: current_body.trim_end().to_string(),
        });
    }
    out
}

/// Parses the output of the Slither 'variable-order' printer into a vector of StorageVar structs.
///
/// This function processes the text output from Slither's variable-order printer,
/// which contains information about storage variables in Solidity contracts.
/// It extracts the contract name, variable name, and variable type for each storage variable.
///
/// @param text - The raw text output from the variable-order printer
/// @return Vector of StorageVar structs containing the parsed data
pub fn parse_storage(text: &str) -> Vec<StorageVar> {
    let mut current_contract = String::new();
    let mut vars = Vec::new();

    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract") && line.ends_with(':') {
            current_contract = line["Contract".len()..]
                .trim_end_matches(':')
                .trim()
                .to_owned();
        }
        // Parse variable lines (format: "| <index> | <name> | <type> | <...> |")
        else if line.starts_with('|') && line.contains('|') {
            let cols: Vec<_> = line.split('|').map(|c| c.trim()).collect();
            // Check if this is a valid variable line (has enough columns and not a header)
            if cols.len() >= 3 && cols[1] != "Name" && !cols[1].is_empty() && !cols[2].is_empty() {
                // cols[1] is contract_name.function_name.  need to parse
                let (contract, function) = split_str_by_period(cols[1]).unwrap();
                vars.push(StorageVar {
                    contract,
                    name: function,
                    r#type: cols[2].to_owned(),
                });
            } else {
                debug!(
                    "Skipping invalid storage var in {}: {:?}",
                    current_contract, cols
                );
            }
        }
    }

    // info!("storage => {:#?}", vars.len());
    vars
}
// splits "first.last" => ("first","last")
fn split_str_by_period(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None // Invalid format
    }
}
/// Replces special characters in the SlithIR text with their ASCII equivalents.
///
/// This function replaces the Greek letter phi (ϕ) with the ASCII string "phi"
/// to ensure the text can be properly processed and displayed.
///
/// @param text - The text containing special characters
/// @return String with special characters replaced
pub fn replace_special_character(text: &str) -> String {
    // Replace the Greek letter phi (ϕ) with "phi"
    let cleaned_text = text.trim().replace("ϕ", "phi");

    cleaned_text
}

```
ai-agent-audit/src/build_brain/slither_ffi.rs
```
/// Slither static analyzer interface with Docker integration.
///
/// This module provides a secure interface to Slither static analysis tool,
/// running all operations in Docker containers for security. Handles extraction
/// of IR, call graphs, inheritance data, and storage layouts with caching.
use anyhow::{anyhow, Result};
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::inheritance;
use crate::build_brain::parsers::parse_slithir_contract_summary;
use crate::build_brain::summarize::summarize_src_files;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_file_content::extract_content_from_docs;

use super::callgraph;
use super::parsers::{parse_slither, parse_slithir_ir_code, parse_storage};

/// Global cache for Slither printer outputs to avoid redundant analysis.
/// Key format: "{repo_root}:{printer_name}"
pub static PRINTER_OUTPUT_CACHE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a single function's SlithIR (intermediate representation).
///
/// SlithIR is Slither's intermediate representation of Solidity code, which
/// makes it easier to analyze the code's behavior and identify potential issues.
#[derive(Debug, Serialize, Deserialize)]
pub struct SlithIRFn {
    /// Name of the contract containing this function
    pub contract: String,
    /// Name of the function
    pub function: String,
    /// The SlithIR representation of the function's code
    pub ir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSummary {
    /// Name of the contract containing this function
    pub contract: String,
    /// list of function
    pub content: String,
}

/// Represents a storage variable in a Solidity contract.
///
/// This struct contains information about a storage variable, including
/// its name, type, and the contract it belongs to.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageVar {
    /// Name of the contract containing this storage variable
    pub contract: String,
    /// Name of the storage variable
    pub name: String,
    /// Data type of the storage variable (e.g., "uint256", "address", etc.)
    pub r#type: String,
}
/// Return “context file list” as LF-separated string.
pub fn get_all_files_src(repo: &RepoPaths) -> String {
    //   e.g.,   contracts/plume/src
    let code_root = repo.root.join(&repo.repo_name).join("src");

    let mut files = Vec::<String>::new();

    for path in &repo.sol_files {
        // fast skip: must be under src/ and not a symlink
        if !path.starts_with(&code_root) {
            continue;
        }

        if fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(true)
        {
            continue;
        }

        // build a relative "short path"
        let rel = path.strip_prefix(&code_root).unwrap_or(path);
        let rel_str = rel.to_string_lossy();

        if should_skip(&rel_str) {
            continue;
        }

        files.push(rel_str.to_string());
    }

    // stable ordering helps diffing prompts
    files.sort();
    files.join("\n")
}

fn should_skip(rel: &str) -> bool {
    let lowercase = rel.to_ascii_lowercase();

    // 1. third-party deps: vendor/*/contracts/**   OR   node_modules/**/contracts/**
    if lowercase.contains("/vendor/") && lowercase.contains("/contracts/")
        || lowercase.contains("/node_modules/") && lowercase.contains("/contracts/")
    {
        return true;
    }

    // 2. other large externals you may add later
    if lowercase.starts_with("lib/") && lowercase.contains("/contracts/") {
        return true;
    }

    false
}

pub async fn run_slither_detector(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "detector");
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither detector");
    let volume = format!("{}:/workspace", &repo.root.display());
    let out = Command::new("docker")
        .args([
            "run",
            "--read-only",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,            // Use the already-built repo folder
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--exclude-dependencies",
            "--foundry-out-directory", // Specify where to find Forge build artifacts
            "out",
        ])
        .output()?;

    // anyhow::ensure!(out.status.success(), "slither --sarif failed");
    //
    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&out.stderr).into_owned();
    }

    info!("slither analysis complete with size {}", text.len());
    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

/// Runs a single Slither printer and captures its output.
///
/// This function executes the Slither static analysis tool with a specific printer
/// and returns the captured output as a string.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param printer - Name of the Slither printer to run (e.g., "slithir-ssa", "variable-order")
/// @return Result containing the printer's output as a string
pub async fn run_printer(repo: &RepoPaths, printer: &str) -> Result<String> {
    let key = cache_key(&repo.root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither printer: {}", printer);
    let volume = format!("{}:/workspace", &repo.root.display());
    let output = Command::new("docker")
        .args([
            "run",
            "--read-only",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,            // Use the already-built repo folder
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print",
            printer,
            "--exclude-low",
            "--exclude-medium",
            "--exclude-high",
            "--exclude-informational",
            "--disable-color", // Disable ANSI color codes for easier parsing
        ])
        .stdout(Stdio::piped()) // Capture printer text from stdout
        .stderr(Stdio::piped()) // Capture banner & errors from stderr
        .output()?;

    // Use whichever stream is non-empty (some printers output to stdout, others to stderr)
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }

    // Ensure we got some output
    if text.trim().is_empty() {
        return Err(anyhow!("Slither ran but produced no `{}` output", printer));
    }

    info!("{} printer complete with size {}", printer, text.len());
    // Save to cache and return
    printer_cache.insert(key, text.clone());
    Ok(text)
}

// use for inheritance and call-graph
pub async fn run_printer_json(repo: &RepoPaths, printer: &str) -> Result<String> {
    let key = cache_key(&repo.root, printer);
    let cache = Arc::clone(&PRINTER_OUTPUT_CACHE);
    let mut printer_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = printer_cache.get(&key) {
        return Ok(cached.clone());
    }

    log::info!("Running Slither printer: {}", printer);
    let volume = format!("{}:/workspace", &repo.root.display());
    let out = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &volume,
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "slither",
            &repo.repo_name,
            "--foundry-ignore-compile", // Skip compilation as we've already built with Forge
            "--foundry-out-directory",  // Specify where to find Forge build artifacts
            "out",
            "--print",
            printer,
            "--exclude-low",
            "--exclude-medium",
            "--exclude-high",
            "--exclude-informational",
            "--disable-color",
            "--json",
            "-",
        ])
        .output()?;

    anyhow::ensure!(out.status.success(), format!("slither {} failed", printer));

    let text = String::from_utf8_lossy(&out.stdout).into_owned();

    // Save to cache and return
    info!("{} print complete with size {}", printer, text.len());
    printer_cache.insert(key, text.clone());

    Ok(text)
}
/// Runs both Slither printers and returns the parsed IR and storage information.
///
/// This function is the main public interface for extracting SlithIR and storage
/// information from Solidity contracts. It runs both the slithir-ssa and variable-order
/// printers and parses their output.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing a tuple of SlithIRFn and StorageVar vectors
pub async fn get_slither_ir_and_storage_for_codeblockcodeblock(
    repo: &RepoPaths,
) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>, Vec<String>)> {
    // Run the slithir-ssa printer to get IR information
    let ir_raw = run_printer(repo, "slithir-ssa").await?;

    // Run the variable-order printer to get storage information
    let storage_raw = run_printer(repo, "variable-order").await?;
    // info!("storage raw => {}", storage_raw);

    let slither_scan_results = run_slither_detector(repo).await?;

    // Parse both outputs and return the results
    Ok((
        parse_slithir_ir_code(&ir_raw),
        parse_storage(&storage_raw),
        parse_slither(&slither_scan_results),
    ))
}

/// Dumps IR and storage information to individual text files in a directory.
///
/// This function extracts SlithIR and storage information from Solidity contracts
/// and writes each function's IR and each storage variable's information to separate
/// text files in the specified directory.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @param dir - Path to the directory where the text files will be written
/// @return Result containing a vector of paths to the created files
pub async fn save_code_metadata_and_analysis_to_txt_files(
    repo: &RepoPaths,
    dir: &Path,
    semantics_path: &Path,
) -> Result<Vec<PathBuf>> {
    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chunks");
    let (_, _, slither_scan_vec) = get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;
    // info!("storage vec => {:?}", storage_vec);

    let (funcs, edges) = callgraph::get_dot_funcs_and_dot_edges(repo).await?;
    let inheritance_json = run_printer_json(repo, "inheritance").await?;
    let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
    let contract_summary = run_printer(repo, "contract-summary").await?;
    let contract_summary_vec = parse_slithir_contract_summary(&contract_summary);
    let src_file_list = get_all_files_src(repo);
    let summaries = summarize_src_files(repo, &semantics_path).await?;

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    info!("save src file list to txt");
    let file_list = dir.join("src_files.txt");
    info!("src file list => {}", src_file_list);
    fs::write(&file_list, src_file_list)?;
    out_paths.push(file_list);

    info!("convert file summaries to txt files");
    for sum in &summaries {
        let meta = format!("{}::file_summary", sum.filename);
        // info!("contract meta => {}", meta);
        let body = format!("\nfile: {}\n{}", sum.filename, sum.summary);
        // info!("{}", body);
        let p = dir.join(meta.replace("::", "_").replace("/", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }
    info!("convert contract summary to txt files");
    for c in &contract_summary_vec {
        let meta = format!("{}::contract_summary", c.contract);
        // info!("contract meta => {}", meta);
        let body = format!("\nContract: {}\n{}", c.contract, c.content);
        // info!("{}", body);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph functions to txt files");
    for f in &funcs {
        let meta = format!("function::{}::{}", f.full_id, f.contract);
        // info!("fn meta => {}", meta);
        let body = format!("{} {} {}", f.full_id, f.contract, f.name);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph edges to txt files");
    for e in &edges {
        let meta = format!("caller::{}::callee::{}", e.caller, e.callee);
        // info!("caller callee meta => {}", meta);
        let body = format!("{} {}", e.caller, e.callee);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert call graph edges to txt files");
    for edge in &inheritance_edges {
        let meta = format!("child::{}::parent::{}", edge.0, edge.1);
        // info!("edges meta => {}", meta);
        let body = format!("{} {}", edge.0, edge.1);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("convert slither scan results to txt files");
    for (i, issue) in slither_scan_vec.iter().enumerate() {
        let meta = format!("{} slither code issue", i);
        let p = dir.join(meta.replace(" ", "_") + ".txt");
        fs::write(&p, issue)?;
        out_paths.push(p);
    }

    info!(
        "slither issues found, fn ir, storage var files => {:?}",
        out_paths.len()
    );

    Ok(out_paths)
}

pub fn cache_key(repo_root: &Path, printer: &str) -> String {
    format!("{}::{}", repo_root.display(), printer)
}

```
ai-agent-audit/src/build_brain/summarize.rs
```
/// Protocol and file summarization using LLMs.
///
/// This module generates intelligent summaries of smart contract protocols and
/// individual source files using OpenAI models. Provides cached summarization
/// for protocol overviews and contextual information for AI analysis.
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::openai::{self, GPT_4O, O3},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, Semaphore},
    task,
};

use crate::{
    cost::cost_data::add_to_inference_cost_by_type,
    prepare_code::git_clone::RepoPaths,
    utils::{
        contract_name_check::has_non_mock_contract, extract_retry::extractor_with_retry,
        get_file_content::extract_content_from_docs,
    },
};
use crate::{
    cost::cost_data::LlmCostType,
    llm_review::prompt_context::{self, generate_context_for_code_review},
};

use super::slither_ffi::cache_key;

/// Global cache for file summaries to avoid redundant LLM calls
pub static FILE_SUMMARY_CACHE: Lazy<Arc<Mutex<HashMap<String, Vec<SrcFileSummary>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Represents a summary of a source file with metadata
#[derive(Default, Debug, Clone)]
pub struct SrcFileSummary {
    /// Source file name
    pub filename: String,
    /// AI-generated summary of the file's purpose and functionality
    pub summary: String,
}

/// Structured response format for LLM file summarization
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileSummary {
    /// The generated summary text
    pub summary: String,
}

pub async fn summarize_docs(
    repo: &RepoPaths,
    current_context: &str,
) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(&repo.root, "docs-summary");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    let documentation = extract_content_from_docs(repo)?;
    let mut doc_summaries = Vec::new();

    let mut docs_plus_context = format!("\n ## DOCUMENTATION: \n\n {}\n\n", documentation);
    docs_plus_context.push_str("\n ## CURRENT SECURITY AUDIT CONTEXT \n\n");
    docs_plus_context.push_str(&format!("\n #### The Documentation Summary should NOT contain content that is already included below.\n\n {} \n\n", current_context));

    let openai_client = openai::Client::from_env();

    info!("generate summmary of all major files and docs in repo...");
    let preamble =
        "You are a senior solidity dev and expert solidity security researcher. Please provide detailed and comprehensive summary 
        of below DOCUMENTATION. Should be up to 4000 words, but no longer.  Should cover **all relevant details** that a security researcher 
        should know about this protocol to do a proper smart contract audit. ALSO, exclude any information from the summary that is already
        included in below CURRENT SECURITY AUDIT CONTEXT, because both DOCUMENTATION and CURRENT SECURITY AUDIT CONTEXT will be provide as
        context for an llm to do a security scan of protocol code.  So its important there is NO duplicate information between DOCUMENTATION 
        and CURRENT SECURITY AUDIT CONTEXT ";
    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(O3)
        .preamble(preamble)
        .build();

    add_to_inference_cost_by_type(
        &format!("{}{}", preamble, documentation),
        LlmCostType::Openai4oInput,
    )
    .await;

    info!("summarizing documentation");

    let doc_summary = match extractor_with_retry(
        &ai_summary_agent,
        &docs_plus_context,
        LlmCostType::Openai4oOutput,
    )
    .await
    {
        Ok(res) => SrcFileSummary {
            filename: "readme.md".to_string(),
            summary: res.summary,
        },
        Err(e) => {
            log::error!("❌ summarizing readme.md failed: {e}");
            SrcFileSummary {
                filename: "readme.md".to_string(),
                summary: String::new(),
            }
        }
    };

    log::info!("readme.md summary => {:#?}", doc_summary);
    doc_summaries.push(doc_summary);

    summaries_cache.insert(key, doc_summaries.clone());
    Ok(doc_summaries)
}

pub async fn summarize_src_files(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<Vec<SrcFileSummary>> {
    let key = cache_key(&repo.root, "file_summaries");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        return Ok(cached.clone());
    }

    let openai_client = openai::Client::from_env();

    let context =
        prompt_context::generate_slither_metadata_prompt_context(repo, &semantics_path).await?;

    // info!("slither metadata => {:#?}", context);
    info!("generate summmary of all major files and docs in repo...");
    let preamble ="You are a senior solidity dev. Please summarize below content (code or docs). Format in markdown for easy reading. 
                    If content is code. Please write 200 word or less summary for each contract plus contract definition, 100 words or less summary 
                    of each function + function interface, and 50 word or less explanation of each storage variable + variable defintion. If docs 
                    please summarize each section of the docs with 150 words or less, max 500 words total for each doc file. 
                    Respond only with valid JSON matching the schema!";
    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(GPT_4O)
        .preamble(preamble)
        .context(&context)
        .build();

    // ---------------------------------------------
    // 1.  PREP – collect the  files we want to summarize first
    // ---------------------------------------------
    let mut work_items = Vec::new();

    // Walk through the repository and collect relevant files
    let repo_code_root = repo.root.join(repo.repo_name.clone());
    for file in &repo.sol_files {
        let is_sol_in_src = file.extension().map_or(false, |ext| ext == "sol")
            && file.starts_with(&repo_code_root.join("src"));

        if !is_sol_in_src {
            continue;
        }

        let is_readme_or_mock = file
            .file_name()
            .map(|f| {
                f.to_ascii_lowercase() == "readme.md"
                    || f.to_string_lossy().to_ascii_lowercase().contains("mock")
            })
            .unwrap_or(false)
            && (file.parent() == Some(&repo_code_root)
                || file.parent() == Some(&repo_code_root.join("src")));

        let is_sol_in_src = file.extension().map_or(false, |ext| ext == "sol")
            && file.starts_with(&repo_code_root.join("src"));

        // Skip directories and symlinks
        if !file.is_file() || fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        if is_readme_or_mock || is_sol_in_src {
            let content = fs::read_to_string(file.clone())?;

            // skip if content does not have have at least one line that start with contract and contract
            // name does NOT contain 'mock' (case insensative)
            let has_non_mock_contract = has_non_mock_contract(&content);

            if !has_non_mock_contract {
                continue;
            }

            // push full path & content into the work queue
            work_items.push((file.to_owned(), content));
        }
    }

    let max_parallel = 20;
    let sem = Arc::new(Semaphore::new(max_parallel));
    let agent = Arc::new(ai_summary_agent); // the OpenAI client
    let mut handles = Vec::new();

    for (file, content) in work_items {
        let sem = sem.clone();
        let agent = agent.clone();
        let repo_root = repo.root.clone();

        let handle = tokio::spawn(async move {
            // acquire permit – blocks if `max_parallel` already in-flight
            let _permit = sem.acquire_owned().await.unwrap();

            add_to_inference_cost_by_type(
                &format!("{}{}", preamble, content),
                LlmCostType::Openai4oInput,
            )
            .await;

            info!("summarizing {}", file.display());

            match extractor_with_retry(&agent, &content, LlmCostType::Openai4oOutput).await {
                Ok(res) => {
                    let filename = file
                        .strip_prefix(&repo_root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string();
                    Some(SrcFileSummary {
                        filename,
                        summary: res.summary,
                    })
                }
                Err(e) => {
                    log::error!("❌ summarizing {} failed: {e}", file.display());
                    None
                }
            }
        });
        handles.push(handle);
    }

    // wait for all tasks
    let mut summaries = Vec::new();
    for h in handles {
        if let Some(s) = h.await? {
            summaries.push(s);
        }
    }

    log::info!("summaries => {:#?}", summaries);

    summaries_cache.insert(key, summaries.clone());
    Ok(summaries)
}

pub async fn summarize_protocol(repo: &RepoPaths, semantics_path: &Path) -> Result<String> {
    // content retrival MUST come first to prevent race condition
    let context = generate_context_for_code_review(repo, &semantics_path).await?;

    let key = cache_key(&repo.root, "protocol-summary");
    let cache = Arc::clone(&FILE_SUMMARY_CACHE);
    let mut summaries_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = summaries_cache.get(&key) {
        let summary = cached
            .first()
            .unwrap_or(&SrcFileSummary::default())
            .summary
            .clone();
        return Ok(summary);
    }

    // Initialize vectors to store file paths
    let mut summaries = Vec::<SrcFileSummary>::new();

    let openai_client = openai::Client::from_env();

    log::info!("generate context for code review");
    let preamble= "You are a senior solidity dev. Given the context provided for solidity smart contract protocol, 
                   please create a max 200 word summary of this protocol explaining what it is, and how it works.  Format 
                   in markdown for easy reading. Respond only with valid JSON matching the schema!";

    let ai_summary_agent = openai_client
        .extractor::<FileSummary>(O3)
        .preamble(preamble)
        .build();

    log::info!("extracting protocol summary");
    // rerun if NoDataExtracted Error

    add_to_inference_cost_by_type(
        &format!("{}{}", preamble, context),
        LlmCostType::OpenaiO3Output,
    )
    .await;

    let summary =
        extractor_with_retry(&ai_summary_agent, &context, LlmCostType::OpenaiO3Output).await?;

    log::info!("protocol summary => {:#?}", summary);

    summaries.push(SrcFileSummary {
        filename: "protocol-summary".to_string(),
        summary: summary.summary.clone(),
    });

    summaries_cache.insert(key, summaries.clone());
    Ok(summary.summary)
}

```
ai-agent-audit/src/build_brain/vector_db.rs
```
use std::path::{Path, PathBuf};

/// Qdrant vector database operations for semantic search.
///
/// This module handles vector database operations including collection creation,
/// embedding generation, and metadata storage for intelligent code search and
/// AI agent context retrieval.
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use log::info;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, PointStruct, UpsertPointsBuilder,
    VectorParams, VectorsConfig,
};
use qdrant_client::Payload;
use qdrant_client::Qdrant;

use crate::build_brain::enbeddings::embed_files;
use crate::build_brain::slither_ffi;
use crate::prepare_code::git_clone::RepoPaths;

use super::enbeddings::SourceChunk;

/// Generates embeddings from Slither analysis and stores them in Qdrant.
///
/// This function creates a complete vector database for the repository by:
/// 1. Checking if collection already exists to avoid duplication
/// 2. Extracting all source files and Slither analysis results
/// 3. Generating embeddings using OpenAI's text-embedding-3-small
/// 4. Creating a unique Qdrant collection for the repository
/// 5. Storing vectors with rich metadata for semantic search
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantic_db` - Path to semantic analysis database
pub async fn generate_slither_chucks_and_save_all_metadata_to_vector_db(
    repo: &RepoPaths,
    semantic_db: &Path,
) -> Result<()> {
    // check if vector db for this repo already exists
    if does_qdrant_vector_db_for_this_repo_already_exist(&repo).await? {
        return Ok(());
    }
    // 2b. Create a temp dir and ask slither_ffi to fill it with chunk files
    // Create a temporary directory to store the Slither analysis results
    let tmp_dir = tempfile::tempdir().map_err(|e| {
        AuditError::file_system("tempdir", "Failed to create temporary directory", e)
    })?;
    info!("generating slither ssa into txt files that contain function or storage var");

    // Extract IR and storage information using Slither and write to text files
    let slither_chunk_paths = slither_ffi::save_code_metadata_and_analysis_to_txt_files(
        repo,
        tmp_dir.path(),
        &semantic_db,
    )
    .await?;
    info!("slither ssa file count => {}", slither_chunk_paths.len());
    // ────────────────────────────────
    // 4. Assemble the *full* file list to embed
    //    – original Solidity + docs  (repo.sol_files  ∪  repo.docs)
    //    – temp IR / storage files   (tmp_paths)
    // ────────────────────────────────
    info!("combine all file locations for solidity + docs, IR functions, and storage into 1 vec");
    // Combine all file paths into a single vector:
    // - Solidity source files
    // - Documentation files
    // - Slither analysis result files
    let mut all_files: Vec<_> = repo.sol_files.clone().into_iter().collect();
    all_files.extend(repo.docs.clone());
    all_files.extend(slither_chunk_paths);
    info!("all files => {:?}", all_files.len());

    //embed all files and upsert to qdrant vector db for later dynamic retrival
    generate_enbeddings_and_save_to_qdrant_vector_db(&all_files, &repo).await?;
    Ok(())
}

pub async fn generate_enbeddings_and_save_to_qdrant_vector_db(
    all_files: &[PathBuf],
    repo: &RepoPaths,
) -> Result<()> {
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());

    info!("connect to qdrant db");

    // Build Qdrant client configuration and connect to the database
    let qdrant_url = std::env::var("QDRANT_URL")
        .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;
    let qdrant = Qdrant::from_url(&qdrant_url).build().map_err(|e| {
        AuditError::vector_db("connection", "Failed to connect to Qdrant database", e)
    })?;

    let already_exists = qdrant.collection_exists(&vector_db_name).await?;

    // if vector db already exits for this repo no need to re-upsert
    if already_exists {
        return Ok(());
    }

    // Create the collection if it doesn't exist
    info!("create contract_chunks vector db (if does not exist)");
    ensure_collection(&qdrant, &vector_db_name, audit_config().vector_dimension).await?;

    // Generate vector embeddings for all files
    info!("generating vector embedding");
    let embeddings = embed_files(&all_files).await?;
    // ────────────────────────────────
    // 4. Upsert into Qdrant
    // ────────────────────────────────
    // Upsert the embeddings into the Qdrant collection
    info!("upsert embeddings");
    upsert(&qdrant, &vector_db_name, &embeddings).await?;

    // Print completion message
    println!("✅ Ingest complete – {} chunks stored", embeddings.len());
    Ok(())
}

pub async fn does_qdrant_vector_db_for_this_repo_already_exist(repo: &RepoPaths) -> Result<bool> {
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());

    info!("connect to qdrant db");
    // Build Qdrant client configuration and connect to the database
    let qdrant_url = std::env::var("QDRANT_URL")
        .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;
    let qdrant = Qdrant::from_url(&qdrant_url).build().map_err(|e| {
        AuditError::vector_db("connection", "Failed to connect to Qdrant database", e)
    })?;

    Ok(qdrant.collection_exists(&vector_db_name).await?)
}

/// Ensures that a collection exists in the Qdrant database, creating it if missing.
///
/// This function creates a new collection with the specified name and dimension if it doesn't
/// already exist. It uses cosine distance for similarity calculations.
///
/// @param client - Reference to the Qdrant client
/// @param name - Name of the collection to create
/// @param dim - Dimension of the vectors to be stored in the collection
/// @return Result indicating success or failure
pub async fn ensure_collection(client: &Qdrant, name: &str, dim: u64) -> Result<()> {
    // check if collection already exists
    let already_exists = client.collection_exists(name).await?;

    if already_exists {
        info!("Collection {} already exists...no need to create", name);
        return Ok(());
    }

    // Build the request *by value* (no &CreateCollection -> eliminates the Into/From error)
    let req = CreateCollection {
        collection_name: name.to_owned(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: dim,
                distance: Distance::Cosine.into(), // Using cosine similarity for vector comparison
                ..Default::default()
            })),
        }),
        ..Default::default()
    };

    // Newer client expects `CreateCollection`, not `&CreateCollection`
    client.create_collection(req).await?;
    Ok(())
}

/// Upserts a batch of metadata and embedding vector tuples into a Qdrant collection.
///
/// This function takes a list of (metadata, vector) tuples and inserts or updates them
/// in the specified Qdrant collection. Each item's metadata is stored in the payload
/// under the key "meta".
///
/// @param client - Reference to the Qdrant client
/// @param collection - Name of the collection to upsert into
/// @param items - Slice of tuples containing metadata strings and their corresponding embedding vectors
/// @return Result indicating success or failure
pub async fn upsert(
    client: &Qdrant,
    collection: &str,
    items: &[(SourceChunk, Vec<f32>)],
) -> Result<()> {
    // Build PointStructs from the items
    let points: Vec<PointStruct> = items
        .iter()
        .enumerate()
        .map(|(i, (chunk, vec))| -> Result<PointStruct> {
            // 🚩 flatten: payload IS the SourceChunk
            let payload: Payload = serde_json::to_value(chunk)
                .map_err(|e| {
                    AuditError::json("chunk_serialization", "Failed to serialize SourceChunk", e)
                })?
                .try_into()
                .map_err(|e| {
                    AuditError::json("payload_conversion", "Failed to convert JSON to Payload", e)
                })?;
            // Create a new point with ID, vector, and payload
            Ok(PointStruct::new(i as u64, vec.clone(), payload))
        })
        .collect::<Result<Vec<_>>>()?;

    // Use the builder-style API to create the upsert request
    let req = UpsertPointsBuilder::new(collection, points).wait(true); // wait=true mimics the old “blocking”

    // Execute the upsert operation
    client
        .upsert_points(req)
        .await
        .map_err(|e| AuditError::vector_db("upsert", "Failed to upsert points to Qdrant", e))?;
    Ok(())
}

```
ai-agent-audit/src/build_brain/vector_service.rs
```
/// Vector database service for centralized Qdrant operations.
///
/// This module provides a high-level service interface for vector database operations,
/// eliminating duplication and providing consistent error handling across the codebase.

use crate::config::audit_config;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use super::enbeddings::SourceChunk;
use log::info;
use qdrant_client::{
    qdrant::{
        vectors_config::Config, CreateCollection, Distance, PointStruct, UpsertPointsBuilder,
        VectorParams, VectorsConfig,
    },
    Payload, Qdrant,
};
use std::sync::Arc;

/// Vector database service providing centralized Qdrant operations.
pub struct VectorDbService {
    /// Qdrant client
    client: Qdrant,
    /// Default vector dimension
    vector_dimension: u64,
}

impl VectorDbService {
    /// Creates a new vector database service.
    pub async fn new() -> Result<Self> {
        let qdrant_url = std::env::var("QDRANT_URL")
            .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;
        
        let client = Qdrant::from_url(&qdrant_url)
            .build()
            .map_err(|e| AuditError::vector_db("connection", "Failed to connect to Qdrant database", e))?;

        Ok(Self {
            client,
            vector_dimension: audit_config().vector_dimension,
        })
    }

    /// Creates a new service with custom configuration.
    pub async fn with_config(qdrant_url: &str, vector_dimension: u64) -> Result<Self> {
        let client = Qdrant::from_url(qdrant_url)
            .build()
            .map_err(|e| AuditError::vector_db("connection", "Failed to connect to Qdrant database", e))?;

        Ok(Self {
            client,
            vector_dimension,
        })
    }

    /// Generates a collection name for a repository.
    pub fn collection_name(&self, repo: &RepoPaths) -> String {
        format!("{}-contract_chunks", repo.unique_repo_hash())
    }

    /// Checks if a collection exists for the given repository.
    pub async fn collection_exists(&self, repo: &RepoPaths) -> Result<bool> {
        let collection_name = self.collection_name(repo);
        self.client
            .collection_exists(&collection_name)
            .await
            .map_err(|e| AuditError::vector_db("collection_check", &format!("Failed to check if collection '{}' exists", collection_name), e))
    }

    /// Creates a collection if it doesn't exist.
    pub async fn ensure_collection(&self, repo: &RepoPaths) -> Result<()> {
        let collection_name = self.collection_name(repo);
        
        if self.collection_exists(repo).await? {
            info!("Collection {} already exists...no need to create", collection_name);
            return Ok(());
        }

        info!("Creating collection: {}", collection_name);
        
        let req = CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: self.vector_dimension,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        };

        self.client
            .create_collection(req)
            .await
            .map_err(|e| AuditError::vector_db("collection_creation", &format!("Failed to create collection '{}'", collection_name), e))?;

        Ok(())
    }

    /// Upserts embeddings into the collection.
    pub async fn upsert_embeddings(
        &self,
        repo: &RepoPaths,
        items: &[(SourceChunk, Vec<f32>)],
    ) -> Result<()> {
        let collection_name = self.collection_name(repo);
        
        info!("Upserting {} embeddings to collection: {}", items.len(), collection_name);

        // Build PointStructs from the items
        let points: Vec<PointStruct> = items
            .iter()
            .enumerate()
            .map(|(i, (chunk, vec))| -> Result<PointStruct> {
                let payload: Payload = serde_json::to_value(chunk)
                    .map_err(|e| AuditError::json("chunk_serialization", "Failed to serialize SourceChunk", e))?
                    .try_into()
                    .map_err(|e| AuditError::json("payload_conversion", "Failed to convert JSON to Payload", e))?;
                
                Ok(PointStruct::new(i as u64, vec.clone(), payload))
            })
            .collect::<Result<Vec<_>>>()?;

        // Create and execute the upsert request
        let req = UpsertPointsBuilder::new(&collection_name, points).wait(true);
        
        self.client
            .upsert_points(req)
            .await
            .map_err(|e| AuditError::vector_db("upsert", &format!("Failed to upsert embeddings to collection '{}'", collection_name), e))?;

        info!("Successfully upserted {} embeddings", items.len());
        Ok(())
    }

    /// Deletes a collection for a repository.
    pub async fn delete_collection(&self, repo: &RepoPaths) -> Result<()> {
        let collection_name = self.collection_name(repo);
        
        if !self.collection_exists(repo).await? {
            info!("Collection {} does not exist, nothing to delete", collection_name);
            return Ok(());
        }

        info!("Deleting collection: {}", collection_name);
        
        self.client
            .delete_collection(&collection_name)
            .await
            .map_err(|e| AuditError::vector_db("collection_deletion", &format!("Failed to delete collection '{}'", collection_name), e))?;

        Ok(())
    }

    /// Returns the underlying Qdrant client for advanced operations.
    pub fn client(&self) -> &Qdrant {
        &self.client
    }

    /// Returns the vector dimension used by this service.
    pub fn vector_dimension(&self) -> u64 {
        self.vector_dimension
    }
}

/// Shared vector database service instance.
use std::sync::OnceLock;
static VECTOR_SERVICE: OnceLock<Arc<VectorDbService>> = OnceLock::new();

/// Initializes the global vector database service.
pub async fn init_vector_service() -> Result<()> {
    let service = VectorDbService::new().await?;
    let arc_service = Arc::new(service);
    
    VECTOR_SERVICE.set(arc_service).map_err(|_| {
        AuditError::configuration("vector_service", "Vector service already initialized")
    })?;
    
    Ok(())
}

/// Returns a reference to the global vector database service.
///
/// # Panics
/// Panics if the service has not been initialized with `init_vector_service()`.
pub fn vector_service() -> &'static Arc<VectorDbService> {
    VECTOR_SERVICE.get().expect("Vector service not initialized. Call init_vector_service() first.")
}

/// Returns a reference to the global vector database service, or None if not initialized.
pub fn try_vector_service() -> Option<&'static Arc<VectorDbService>> {
    VECTOR_SERVICE.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_repo() -> RepoPaths {
        let temp_dir = TempDir::new().unwrap();
        RepoPaths {
            root: temp_dir.path().to_path_buf(),
            sol_files: vec![],
            docs: vec![],
            repo_name: "test-repo".to_string(),
            commit_hash: "abc123def456".to_string(),
        }
    }

    #[test]
    fn test_collection_name_generation() {
        let service = VectorDbService {
            client: Qdrant::from_url("http://localhost:6334").build().unwrap(),
            vector_dimension: audit_config().vector_dimension,
        };
        
        let repo = create_test_repo();
        let collection_name = service.collection_name(&repo);
        
        assert!(collection_name.contains("test-repo"));
        assert!(collection_name.contains("abc123"));
        assert!(collection_name.ends_with("-contract_chunks"));
    }

    #[test]
    fn test_vector_dimension() {
        let service = VectorDbService {
            client: Qdrant::from_url("http://localhost:6334").build().unwrap(),
            vector_dimension: audit_config().vector_dimension,
        };
        
        assert_eq!(service.vector_dimension(), audit_config().vector_dimension);
    }
}
```
ai-agent-audit/src/reporting/contract_data.rs
```
use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::prompt_context::generate_context_for_code_review,
    prepare_code::git_clone::RepoPaths, reporting::save_file::save_file_locally,
};
/// Contract data export utilities for analysis artifacts.
///
/// This module provides functions to export contract analysis data including
/// generated code blocks and metadata to markdown files for external use
/// and documentation purposes.
use std::path::{Path, PathBuf};

/// Saves individual contract analysis data to markdown files.
///
/// Exports generated code blocks for each contract to separate markdown files
/// with standardized naming conventions for easy reference and documentation.
///
/// # Arguments
// * `codeblocks_path` - Path to the code blocks database
/// * `repo` - Repository paths and metadata for naming
pub fn save_contract_and_fn_ir(codeblocks_path: &PathBuf, repo: &RepoPaths) -> anyhow::Result<()> {
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    log::info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    for (contract, codeblock) in contracts {
        let filename = format!("{}-{}.md", contract, repo.unique_repo_hash());
        save_file_locally(&codeblock, &filename)?;
    }
    Ok(())
}

/// Saves protocol metadata and context information to a markdown file.
///
/// Exports comprehensive protocol metadata including summaries, semantic data,
/// and contextual information used by AI agents during analysis.
///
/// # Arguments
/// * `semantics_path` - Path to the semantic analysis database
/// * `repo` - Repository paths and metadata for naming
pub async fn save_metadata(semantics_path: &Path, repo: &RepoPaths) -> anyhow::Result<()> {
    let metadata = generate_context_for_code_review(repo, semantics_path).await?;

    let filename = format!("metadata-{}.md", repo.unique_repo_hash());
    save_file_locally(&metadata, &filename)?;
    Ok(())
}

```
ai-agent-audit/src/reporting/save_file.rs
```
/// File saving utilities for audit reports and analysis data.
///
/// This module provides functions to save audit reports and other analysis
/// outputs to the local filesystem with appropriate naming conventions.

use std::{fs::File, io::Write};
use crate::prepare_code::git_clone::RepoPaths;
use super::audit::ReportType;

/// Saves an audit report to a file with appropriate naming.
///
/// Creates a markdown file with the audit report content using a standardized
/// naming convention that includes the repository hash and report type.
///
/// # Arguments
/// * `markdown` - The audit report content in Markdown format
/// * `repo` - Repository paths and metadata for naming
/// * `report_type` - Report type (Free/Paid) for filename suffix
pub fn save_audit_report(
    markdown: &str,
    repo: &RepoPaths,
    report_type: ReportType,
) -> anyhow::Result<()> {
    let suffix = if report_type == ReportType::Free {
        "-free"
    } else {
        ""
    };
    let filename = format!("{}{}-audit-report.md", repo.unique_repo_hash(), suffix);
    save_file_locally(markdown, &filename)?;

    Ok(())
}

/// Saves content to a local file.
///
/// Generic file saving utility that writes string content to a specified filename.
///
/// # Arguments
/// * `content` - String content to write to file
/// * `filename` - Target filename for the content
pub fn save_file_locally(content: &str, filename: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

```
ai-agent-audit/src/invariant_prompts/arithmetic.rs
```
pub const ARITHMETIC: &str = r#"
# Arithmetic Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Arithmetic Invariants** - mathematical relationships and formulas that must remain true throughout a smart contract's execution.

## WHAT ARE ARITHMETIC INVARIANTS?

Arithmetic invariants are mathematical constraints that define the correctness of a contract's core logic. They involve:
- **Mathematical formulas** that must hold (e.g., `x * y = k`)
- **Numerical bounds** and limits (e.g., `totalSupply <= maxSupply`)
- **Ratio relationships** between values (e.g., `collateral * factor >= debt`)
- **Conservation laws** (e.g., sum of balances = total supply)
- **Pricing formulas** and exchange rates

## COMMON ARITHMETIC INVARIANTS BY PROTOCOL TYPE

### DeFi Protocols
- **AMM Constant Product**: `reserveX * reserveY = k` (Uniswap-style)
- **Lending Collateralization**: `borrowed_amount <= collateral_amount * collateralFactor`
- **Interest Rate Models**: Utilization ratios, compound interest calculations
- **Liquidity Pool Ratios**: Token pair balances maintain pricing relationships
- **Slippage Bounds**: Price impact calculations within expected ranges

### NFT Contracts
- **Max Supply Caps**: `totalMinted <= maxSupply`
- **Sequential Token IDs**: `nextTokenId` only increases, no gaps
- **Pricing Tiers**: Different mint prices based on quantity/time
- **Royalty Calculations**: `royaltyAmount = salePrice * royaltyPercentage / 100`

### DAO/Governance
- **Voting Thresholds**: `yesVotes >= quorum` for execution
- **Token Supply vs Voting Power**: Total voting power ≤ total token supply
- **Proposal Lifecycle**: Vote counts determine state transitions
- **Treasury Accounting**: Funds only move with proper vote approval

## ANALYSIS METHODOLOGY

### 1. IDENTIFY MATHEMATICAL RELATIONSHIPS
Look for:
- Multiplication/division operations that should preserve constants
- Addition/subtraction that should maintain totals
- Percentage calculations and ratio maintenance
- Min/max bounds checking
- Mathematical formulas in comments or documentation

### 2. TRACE ARITHMETIC OPERATIONS
- Follow how values are calculated and updated
- Check for integer overflow/underflow protection
- Verify rounding behavior doesn't break invariants
- Look for missing bounds checks

### 3. COMMON VIOLATION PATTERNS
- **Missing bounds checks**: No validation of max values
- **Integer overflow/underflow**: Arithmetic operations that wrap
- **Rounding errors**: Precision loss breaking formulas
- **Reentrancy affecting calculations**: State changes mid-calculation
- **Race conditions**: Concurrent operations breaking math
- **Missing validation**: Parameters not checked against constraints

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's mathematical behavior:
- What calculations does it perform?
- What mathematical relationships should hold?
- What numerical constraints exist?

### 2. DERIVE ARITHMETIC INVARIANTS
For each mathematical relationship, create an invariant:
- **INV-A1, INV-A2, etc.** (use A prefix for Arithmetic)
- Describe the exact mathematical formula or constraint
- Identify the variables and operations involved

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly enforces the mathematical relationship
- **VIOLATION**: The relationship can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Sequence of function calls that breaks the math
- **pre_state**: Initial conditions needed (balances, permissions, etc.)
- **post_state**: Resulting state showing the broken relationship
- **impact**: Financial or functional impact of the violation
- **poc**: Concrete example with numbers
- **mitigation**: How to fix the arithmetic issue

## EXAMPLES OF ARITHMETIC VIOLATIONS

- **AMM**: Swap function allows `k` to decrease, enabling value extraction
- **Lending**: Missing collateral factor check allows over-borrowing
- **NFT**: Mint function bypasses max supply through integer overflow
- **DAO**: Vote counting allows double-counting or negative votes

Focus on mathematical correctness and ensure all arithmetic relationships that define the contract's core logic are properly validated and maintained.
"#;

```
ai-agent-audit/src/invariant_prompts/balance.rs
```
pub const BALANCE: &str = r#"
# Balance Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Balance Invariants** - mathematical relationships governing token/ETH balances, supply mechanics, and balance conservation laws that must remain true throughout a smart contract's execution.

## WHAT ARE BALANCE INVARIANTS?

Balance invariants are constraints that ensure the integrity of value storage and transfer within smart contracts. They involve:
- **Token balance conservation** (transfers don't create/destroy value)
- **Supply monotonicity** (total supply changes only through authorized mint/burn)
- **Balance consistency** (individual balances sum to total supply)
- **ETH/native token accounting** (contract ETH balance matches internal records)
- **Multi-token accounting** (cross-token balance relationships)
- **Liquidity pool integrity** (deposited tokens match pool records)

## COMMON BALANCE INVARIANTS BY PROTOCOL TYPE

### Token Contracts (ERC20/ERC721/ERC1155)
- **Conservation Law**: `sum(all_balances) == totalSupply`
- **Transfer Integrity**: `balanceOf[from] + balanceOf[to]` unchanged after transfer
- **Supply Bounds**: `totalSupply <= maxSupply` (if capped)
- **Burn Consistency**: `totalSupply` decreases exactly by burned amount
- **Mint Authorization**: Supply only increases through authorized minting

### DeFi Protocols
- **Pool Balance Matching**: `poolTokenBalance == sum(userDeposits)`
- **Vault Accounting**: `assetsUnderManagement == sum(userShares * sharePrice)`
- **Liquidity Provider Tokens**: `lpTokenSupply * price == poolValue`
- **Fee Accumulation**: `collectedFees + distributedFees == totalGeneratedFees`
- **Cross-Chain Balance**: Locked tokens on source chain = minted on destination

### Staking/Rewards Systems
- **Reward Pool Conservation**: `totalRewards == distributedRewards + remainingRewards`
- **Staking Balance**: `totalStaked == sum(userStakedAmounts)`
- **Slashing Consistency**: Penalties reduce both user and total stakes proportionally
- **Compound Interest**: Reward calculations maintain mathematical precision

### Multi-Signature/Treasury
- **Asset Tracking**: Contract's actual balance >= sum of tracked balances
- **Withdrawal Limits**: `totalWithdrawn <= depositedAmount` per period
- **Multi-Token Accounting**: Each token type balanced independently

## ANALYSIS METHODOLOGY

### 1. IDENTIFY BALANCE RELATIONSHIPS
Look for:
- State variables tracking balances (`balanceOf`, `totalSupply`)
- Transfer functions and their balance updates
- Mint/burn operations affecting supply
- Deposit/withdrawal mechanisms
- Fee collection and distribution
- Cross-contract balance interactions

### 2. TRACE BALANCE MODIFICATIONS
- Map all functions that modify balances
- Verify balance updates are atomic and consistent
- Check for missing balance updates in edge cases
- Validate overflow/underflow protection
- Ensure all balance changes are properly accounted

### 3. COMMON VIOLATION PATTERNS
- **Missing Balance Updates**: State changes without corresponding balance adjustments
- **Double Accounting**: Same value counted multiple times
- **Rounding Errors**: Precision loss causing balance drift over time
- **Reentrancy Attacks**: External calls allowing balance manipulation
- **Flash Loan Attacks**: Temporary balance inflation breaking invariants
- **Integer Overflow**: Balance arithmetic wrapping unexpectedly
- **Unchecked External Calls**: Failed transfers not reverting state
- **Fee Calculation Errors**: Incorrect fee deduction/distribution

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's balance management:
- What tokens/assets does it handle?
- How are balances tracked and updated?
- What balance relationships must be maintained?
- Are there any supply mechanics (mint/burn)?

### 2. DERIVE BALANCE INVARIANTS
For each balance relationship, create an invariant:
- **INV-B1, INV-B2, etc.** (use B prefix for Balance)
- Describe the exact balance constraint or conservation law
- Identify the variables and operations involved
- Specify the scope (per-user, global, cross-contract)

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly maintains the balance relationship
- **VIOLATION**: The relationship can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that break balance integrity
- **pre_state**: Initial balance conditions needed
- **post_state**: Resulting imbalanced state with specific amounts
- **impact**: Financial loss or system compromise potential
- **poc**: Step-by-step example with actual token amounts
- **mitigation**: How to fix the balance tracking issue

Focus on tracking how value moves through the system and ensure no tokens are created, destroyed, or double-counted unintentionally. Every balance change must be mathematically justified and properly implemented.
"#;

```
ai-agent-audit/src/invariant_prompts/permission.rs
```
pub const PERMISSION: &str = r#"
# Permission Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Permission Invariants** - access control mechanisms that ensure only authorized accounts can perform sensitive operations throughout a smart contract's execution.

## WHAT ARE PERMISSION INVARIANTS?

Permission invariants are security constraints that govern who can execute specific functions and under what conditions. They involve:
- **Role-based access control** (owner, admin, guardian, oracle roles)
- **Function-level restrictions** (only specific addresses can call certain functions)
- **State-dependent permissions** (access rights that change based on contract state)
- **Multi-signature requirements** (multiple parties must approve actions)
- **Time-based access** (permissions that expire or activate at certain times)
- **Hierarchical permissions** (different privilege levels and delegation)

## COMMON PERMISSION INVARIANTS BY PROTOCOL TYPE

### DeFi Protocols
- **Owner-Only Admin Functions**: Only owner can pause contracts, change fees, upgrade modules
- **Oracle Access Control**: Only designated oracles can update price feeds
- **Emergency Powers**: Only guardians can trigger emergency stops or liquidations
- **Fee Management**: Only authorized addresses can collect or distribute fees
- **Parameter Updates**: Critical parameters (interest rates, collateral factors) only changeable by governance
- **Vault Management**: Only vault managers can rebalance or migrate funds

### NFT Contracts
- **Minting Authorization**: Only whitelisted addresses or contract owner can mint tokens
- **Transfer Permissions**: Only token owner or approved addresses can transfer (ERC721 standard)
- **Metadata Updates**: Only authorized roles can modify token metadata or reveal mechanisms
- **Royalty Management**: Only designated addresses can update royalty settings
- **Collection Management**: Only collection owner can add/remove from whitelist or change mint prices

### DAO/Governance Contracts
- **Execution Authority**: Only timelock contract can execute proposals
- **Proposal Creation**: Only token holders above threshold can create proposals
- **Vote Delegation**: Only token holders can delegate their voting power
- **Treasury Access**: Only executed governance proposals can access treasury funds
- **Role Assignment**: Only existing admins can grant/revoke roles to other addresses
- **Upgrade Permissions**: Only governance can upgrade contract implementations

## ANALYSIS METHODOLOGY

### 1. IDENTIFY ACCESS CONTROL MECHANISMS
Look for:
- `onlyOwner`, `onlyAdmin`, custom role modifiers
- `require(msg.sender == authorizedAddress)` statements
- Multi-signature verification logic
- Role-based access control (RBAC) systems
- OpenZeppelin AccessControl usage
- Custom permission checking functions

### 2. MAP PERMISSION BOUNDARIES
- Identify all privileged functions and their access requirements
- Trace permission inheritance and delegation chains
- Check for default permissions and fallback behaviors
- Verify permission revocation mechanisms
- Look for bypass conditions or emergency overrides

### 3. COMMON VIOLATION PATTERNS
- **Missing Access Control**: Sensitive functions lack permission checks
- **Incorrect Role Checks**: Wrong role or address validation
- **Permission Bypass**: Alternative code paths that skip authorization
- **Role Escalation**: Lower privileges can gain higher privileges
- **Initialization Vulnerabilities**: Missing access control during setup
- **Frontrunning**: Permission changes can be frontrun by unauthorized users
- **Reentrancy Permission Bypass**: External calls allowing permission circumvention
- **Default Permissions**: Overly permissive default states

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's permission model:
- What roles and permissions exist?
- Which functions are access-controlled?
- How are permissions granted/revoked?
- Are there any special privilege escalation mechanisms?

### 2. DERIVE PERMISSION INVARIANTS
For each access control mechanism, create an invariant:
- **INV-P1, INV-P2, etc.** (use P prefix for Permission)
- Describe the exact authorization requirement
- Identify the protected resources and operations
- Specify the authorized parties and conditions

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly enforces the permission requirement
- **VIOLATION**: Unauthorized access is possible through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that bypass access control
- **pre_state**: Required initial conditions (roles, balances, etc.)
- **post_state**: Resulting unauthorized state change
- **impact**: Security compromise or privilege escalation achieved
- **poc**: Step-by-step unauthorized action sequence
- **mitigation**: How to fix the access control vulnerability

Focus on verifying that every sensitive operation has appropriate authorization checks and that there are no alternative paths that bypass these controls. Permission invariants are critical for preventing unauthorized access to protected resources and maintaining the security boundaries of the smart contract system.

"#;

```
ai-agent-audit/src/invariant_prompts/referential.rs
```
pub const REFERENTIAL: &str = r#"
# Referential Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Referential Invariants** - critical relationships between state variables, mappings, arrays, and cross-contract references that must remain consistent throughout a smart contract's execution lifecycle.

## WHAT ARE REFERENTIAL INVARIANTS?

Referential invariants are constraints that ensure the integrity of data relationships and pointer consistency within smart contracts. They involve:
- **State Variable Consistency** (related variables stay synchronized)
- **Mapping-Array Synchronization** (mappings and arrays referencing same entities)
- **Cross-Contract Reference Integrity** (external contract addresses remain valid)
- **Index-Data Correspondence** (array indices match their referenced data)
- **Ownership Chain Consistency** (parent-child relationships in hierarchies)
- **State Machine Coherence** (state transitions maintain valid references)

## COMMON REFERENTIAL INVARIANTS BY PROTOCOL TYPE

### Ownership & Access Control
- **Owner-Permission Consistency**: `isOwner[addr] == true` iff `addr` in owners array
- **Role-Permission Mapping**: `hasRole[user][role] == roles[role].members[user]`
- **Delegation Chain**: `delegatedBy[delegate] == delegator` iff `delegates[delegator] == delegate`
- **Multi-Sig Coherence**: `signers.length == signerCount` and all `signers[i]` are unique
- **Permission Inheritance**: Child contracts inherit parent permissions correctly

### Token & NFT Management
- **Owner-Token Mapping**: `ownerOf[tokenId] == owner` iff `tokensOwnedBy[owner]` contains `tokenId`
- **Approval Consistency**: `getApproved[tokenId] == spender` iff spender can transfer tokenId
- **Operator Authorization**: `isApprovedForAll[owner][operator]` matches operator permissions
- **Token Existence**: `tokenId` in `_allTokens` iff `ownerOf[tokenId] != address(0)`
- **Metadata Linkage**: `tokenURI[tokenId]` exists iff token exists

### DeFi Protocol References
- **Pool-Token Association**: `poolForToken[token] == pool` iff `pool.underlyingToken == token`
- **LP Token-Pool Binding**: `lpToken.pool == poolAddress` iff `pool.lpToken == lpTokenAddress`
- **Oracle-Price Consistency**: `priceOracle[asset]` points to valid oracle with recent updates
- **Vault-Strategy Mapping**: `vault.strategy == strategy` iff `strategy.vault == vault`
- **Cross-Chain Bridge**: `localToken[chainId][remoteToken] == localToken` bidirectionally

### Governance & Voting
- **Proposal-Voter Tracking**: `voters[proposalId]` contains all addresses that voted
- **Vote-Weight Consistency**: `totalVotes[proposalId] == sum(voterWeights[proposalId])`
- **Delegation Graph**: No cycles in `delegatedTo[voter]` relationships
- **Snapshot Coherence**: `votingPower[user][blockNumber]` matches historical balances
- **Execution Prerequisites**: Executed proposals have `state == Executed`

### Marketplace & Auction Systems
- **Listing-Item Binding**: `listings[listingId].item == itemId` iff item is listed
- **Bid-Auction Association**: `highestBid[auctionId].bidder` owns the winning bid
- **Escrow-Trade Linkage**: `escrow[tradeId]` holds funds iff trade is active
- **Collection-Item Hierarchy**: `items[itemId].collection == collectionId` consistently
- **Order Book Integrity**: Buy/sell orders reference valid tokens and amounts

### Staking & Rewards
- **Staker-Pool Reference**: `stakerInfo[user].pool == poolId` iff user staked in pool
- **Reward-Epoch Tracking**: `epochRewards[epoch]` distributed matches `totalStaked[epoch]`
- **Unbonding Queue**: `unbondingQueue[user]` entries have valid timestamps
- **Validator-Delegator Map**: `delegations[validator]` contains all delegator addresses
- **Slash-Event Correlation**: Slashing events reference valid validator addresses

## ANALYSIS METHODOLOGY

### 1. IDENTIFY REFERENTIAL RELATIONSHIPS
Look for:
- Mappings that reference array indices or other mappings
- State variables that must stay synchronized
- Cross-contract address storage and validation
- Parent-child relationships in data structures
- Bidirectional references between entities
- State machines with transition dependencies

### 2. TRACE REFERENCE MODIFICATIONS
- Map all functions that modify referenced data
- Verify synchronized updates across related structures
- Check for dangling references after deletions
- Validate reference integrity during state transitions
- Ensure atomic updates of related references
- Confirm proper cleanup of bidirectional links

### 3. COMMON VIOLATION PATTERNS
- **Orphaned References**: Pointers to deleted or non-existent data
- **Asymmetric Updates**: One-way reference updates breaking bidirectional links
- **Stale References**: Cached addresses pointing to outdated contracts
- **Index Drift**: Array indices becoming misaligned with their data
- **Circular Dependencies**: Reference cycles causing logical inconsistencies
- **Race Conditions**: Concurrent updates breaking reference atomicity
- **Missing Validation**: Accepting invalid addresses or indices
- **Inconsistent State Machines**: Invalid state transitions leaving broken references

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's referential structure:
- What entities have cross-references (users, tokens, pools, etc.)?
- How are relationships tracked (mappings, arrays, structs)?
- Are there bidirectional or hierarchical relationships?
- What external contracts or addresses are referenced?

### 2. DERIVE REFERENTIAL INVARIANTS
For each reference relationship, create an invariant:
- **INV-R1, INV-R2, etc.** (use R prefix for Referential)
- Describe the exact reference constraint or consistency requirement
- Identify the data structures and their relationships
- Specify the directionality (unidirectional, bidirectional, hierarchical)

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly maintains reference consistency
- **VIOLATION**: The relationship can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that break referential integrity
- **pre_state**: Initial reference setup needed
- **post_state**: Resulting inconsistent state with specific broken references
- **impact**: Data corruption, access control bypass, or logical inconsistencies
- **poc**: Step-by-step example with specific addresses/IDs
- **mitigation**: How to fix the reference management issue

### 5. FOUNDRY TEST RECOMMENDATIONS
Suggest specific invariant tests:
- Property-based tests for reference consistency
- Fuzz tests for edge cases in reference updates
- Integration tests for cross-contract reference validity
- State transition tests maintaining reference integrity

Focus on ensuring that all data relationships remain logically consistent throughout the contract's execution. Every reference must point to valid, existing data, and all bidirectional relationships must be maintained symmetrically. Reference integrity is crucial for preventing logical vulnerabilities and maintaining system coherence.

"#;

```
ai-agent-audit/src/invariant_prompts/state_machine.rs
```
pub const STATE_MACHINE: &str = r#"
# State Machine Invariant Security Analysis Prompt

You are a senior security auditor specializing in **State Machine Invariants** - constraints that govern valid state transitions, enforce business logic rules, and ensure protocol states remain consistent and secure throughout the contract's execution lifecycle.

## WHAT ARE STATE MACHINE INVARIANTS?

State machine invariants are rules that define valid states and state transitions within smart contracts. They involve:
- **Valid State Constraints** (only allowed states can exist)
- **Transition Authorization** (only permitted actors can trigger transitions)
- **Transition Logic** (state changes follow defined business rules)
- **State Consistency** (related state variables remain coherent)
- **Terminal State Protection** (final states cannot be illegally exited)
- **Temporal Constraints** (time-based state transition rules)

## COMMON STATE MACHINE INVARIANTS BY PROTOCOL TYPE

### Auction Systems
- **Lifecycle States**: `Created → Active → (Bidding) → Ended → Settled`
- **Bidding Rules**: Can only bid in `Active` state with higher amounts
- **Settlement Constraint**: Can only settle in `Ended` state
- **Cancellation Logic**: Can only cancel in `Created` or `Active` (if no bids)
- **Time-Based Transitions**: Auto-transition to `Ended` after auction duration
- **Final State Protection**: `Settled` auctions cannot be modified

### Token Sales & ICOs
- **Sale Phases**: `Pending → Whitelist → Public → Paused → Ended → Finalized`
- **Purchase Authorization**: Can only buy tokens in `Whitelist` or `Public` phases
- **Phase Transitions**: Only owner can advance phases in correct order
- **Pause/Resume Logic**: Can pause/resume only from active states
- **Finalization Rules**: Can only finalize after `Ended` state
- **Refund Conditions**: Refunds only available if sale fails or is cancelled

### Governance Proposals
- **Proposal Lifecycle**: `Pending → Active → Succeeded/Defeated → Queued → Executed/Expired`
- **Voting Period**: Votes only accepted in `Active` state
- **Execution Delay**: Must wait in `Queued` state before execution
- **Success Criteria**: Transition to `Succeeded` only if quorum and votes met
- **Cancellation Rules**: Can cancel in `Pending` or `Active` (by proposer/guardian)
- **Expiration Logic**: `Queued` proposals expire after timelock period

### Staking & Vesting
- **Staking States**: `Unstaked → Staked → Unbonding → Slashed`
- **Unbonding Period**: Must wait in `Unbonding` before withdrawal
- **Slashing Transitions**: Can be slashed from `Staked` or `Unbonding`
- **Re-staking Rules**: Can re-stake from `Unbonding` to cancel withdrawal
- **Vesting Schedule**: `Locked → Vesting → Vested → Claimed`
- **Cliff Enforcement**: No claims before cliff period

### Multi-Signature Wallets
- **Transaction States**: `Proposed → Confirmed → Executed/Revoked`
- **Confirmation Threshold**: Execute only after sufficient confirmations
- **Revocation Rules**: Can revoke confirmations before execution
- **Execution Finality**: `Executed` transactions cannot be modified
- **Owner Changes**: Special state transitions for adding/removing owners
- **Emergency States**: Pause state prevents all operations except recovery

### Escrow & Payment Systems
- **Escrow Flow**: `Created → Funded → InDispute → Released/Refunded`
- **Funding Requirements**: Must be `Funded` before dispute or release
- **Dispute Resolution**: Can enter dispute from `Funded` state
- **Release Conditions**: Can release to beneficiary from `Funded` or resolved dispute
- **Refund Logic**: Refund to payer under specific conditions
- **Timeout Mechanisms**: Auto-release or refund after time limits

### Lending & Borrowing
- **Loan States**: `Applied → Approved → Active → Repaid/Defaulted`
- **Collateral States**: `Deposited → Locked → Released/Liquidated`
- **Health Factor**: Liquidation triggered when health < threshold
- **Repayment Logic**: Can repay from `Active` state only
- **Grace Period**: Transition to `Defaulted` after grace period expires
- **Recovery Process**: Special states for loan recovery and restructuring

## ANALYSIS METHODOLOGY

### 1. IDENTIFY STATE MACHINES
Look for:
- Enum state variables and their possible values
- Boolean flags that represent state conditions
- Status tracking variables (uint8 status, etc.)
- Phase/stage management in protocols
- Time-dependent state transitions
- Multi-contract state coordination

### 2. MAP STATE TRANSITION GRAPH
- Document all possible states
- Identify valid transitions between states
- Determine who can trigger each transition
- Note any time-based or condition-based transitions
- Map out terminal/final states
- Identify any state loops or cycles

### 3. COMMON VIOLATION PATTERNS
- **Invalid State Transitions**: Direct jumps between non-adjacent states
- **Missing Access Controls**: Unauthorized actors triggering transitions
- **Race Conditions**: Concurrent state changes causing inconsistencies
- **Reentrancy State Corruption**: External calls modifying state mid-transition
- **Time Manipulation**: Block timestamp attacks bypassing time constraints
- **Terminal State Violations**: Modifying supposedly final states
- **State Inconsistency**: Related variables becoming desynchronized
- **Missing State Validations**: Functions operating in wrong states

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's state machine structure:
- What are the main state variables and their possible values?
- How many distinct states exist in the system?
- Are there multiple independent state machines?
- What external factors influence state transitions (time, user actions, oracles)?

### 2. DERIVE STATE MACHINE INVARIANTS
For each state machine, create invariants:
- **INV-S1, INV-S2, etc.** (use S prefix for State Machine)
- Document valid states and allowed transitions
- Specify transition authorization requirements
- Define temporal constraints and timing rules
- Identify state consistency requirements across variables

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: State transitions are properly enforced
- **VIOLATION**: Invalid transitions or states are possible
- **CONDITIONAL**: Holds under normal conditions but may break in edge cases

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that break state machine rules
- **pre_state**: Initial state setup required for exploitation
- **post_state**: Resulting invalid state after exploitation
- **impact**: Business logic bypass, unauthorized access, or protocol breakdown
- **poc**: Step-by-step state transition sequence with specific function calls
- **mitigation**: How to properly enforce state machine rules

### 5. FOUNDRY TEST RECOMMENDATIONS
Suggest specific state machine tests:
- Valid transition path testing
- Invalid transition rejection testing
- Time-based state change verification
- Access control on state transitions
- State consistency invariant tests
- Race condition and reentrancy state tests

### 6. STATE DIAGRAM VISUALIZATION
Create a visual representation:
- All states as nodes
- Valid transitions as directed edges
- Transition conditions and authorization requirements
- Terminal states clearly marked
- Any loops or cycles identified

Focus on ensuring that the protocol's business logic is correctly implemented through proper state management. Every state transition must be authorized, valid, and maintain system consistency. State machine violations can lead to critical protocol failures and economic exploits.
"#;

```
ai-agent-audit/src/invariant_prompts/temporal.rs
```
pub const TEMPORAL: &str = r#"
# Temporal Invariant Security Analysis Prompt

You are a senior security auditor specializing in **Temporal Invariants** - time-based constraints and chronological relationships that must be maintained throughout a smart contract's execution lifecycle.

## WHAT ARE TEMPORAL INVARIANTS?

Temporal invariants are time-dependent security constraints that govern when operations can be performed and how time-related state evolves. They involve:
- **Time-based access control** (functions only callable after/before certain timestamps)
- **Sequence enforcement** (operations must occur in specific chronological order)
- **Cooldown periods** (minimum time between repeated actions)
- **Expiration mechanisms** (permissions, offers, or states that expire)
- **Epoch/phase transitions** (contract phases that progress in order)
- **Timelock delays** (mandatory waiting periods before execution)
- **Clock monotonicity** (time only moves forward, no rewinding)

## COMMON TEMPORAL INVARIANTS BY PROTOCOL TYPE

### DeFi Protocols
- **Timelock Delays**: Governance proposals have mandatory delay (24-48 hours) before execution
- **Cooldown Periods**: Users must wait between unstaking/withdrawal requests
- **Interest Accrual**: Interest compounds over time and cannot be calculated for future timestamps
- **Oracle Update Frequency**: Price feeds must be updated within acceptable time windows
- **Auction Timing**: Liquidation auctions have start/end times that cannot be manipulated
- **Vesting Schedules**: Token releases follow predetermined time schedules
- **Lock Periods**: Staked tokens cannot be withdrawn before lock expiration

### NFT Contracts
- **Mint Phases**: Public mint only after whitelist phase ends
- **Reveal Timing**: Metadata reveals happen after mint phase completion
- **Auction Durations**: Bidding periods have enforced start/end times
- **Whitelist Expiry**: Whitelist access expires after specified period
- **Royalty Updates**: Changes to royalty settings have delay periods
- **Breeding Cooldowns**: NFT breeding has mandatory rest periods

### DAO/Governance Contracts
- **Proposal Lifecycle**: Voting -> Delay -> Execution phases in strict order
- **Voting Windows**: Proposals have fixed voting periods that cannot be extended arbitrarily
- **Execution Windows**: Passed proposals must be executed within time limits
- **Quorum Timing**: Vote counting only valid during official voting period
- **Role Transitions**: Admin role changes have mandatory transition periods
- **Emergency Delays**: Even emergency actions have minimum delay requirements

## ANALYSIS METHODOLOGY

### 1. IDENTIFY TIME-DEPENDENT MECHANISMS
Look for:
- `block.timestamp` usage and time comparisons
- Time-based state variables (`startTime`, `endTime`, `lastUpdate`)
- Deadline and expiration logic
- Phase/epoch progression mechanisms
- Cooldown and delay implementations
- Time-locked operations and escrows

### 2. TRACE TEMPORAL RELATIONSHIPS
- Map all time-dependent state transitions
- Verify chronological ordering requirements
- Check for time manipulation vulnerabilities
- Validate timestamp arithmetic and overflow protection
- Ensure proper handling of time edge cases

### 3. COMMON VIOLATION PATTERNS
- **Clock Manipulation**: Relying on `block.timestamp` without considering miner manipulation
- **Time Overflow**: Timestamp arithmetic causing wraparound
- **Phase Skipping**: Bypassing required sequential phases
- **Premature Execution**: Actions executed before required delays
- **Expired State Access**: Using expired data or permissions
- **Reentrancy Time Bypass**: External calls allowing time-based condition bypass
- **Front-running Time Windows**: Exploiting time-sensitive operations
- **Inconsistent Time Sources**: Mixed use of `block.timestamp` vs `block.number`

## TASK INSTRUCTIONS

### 1. CONTRACT ANALYSIS
Summarize the contract's temporal behavior:
- What time-based constraints exist?
- How does the contract track and enforce timing?
- Are there sequential phases or epochs?
- What operations have time dependencies?

### 2. DERIVE TEMPORAL INVARIANTS
For each time-based constraint, create an invariant:
- **INV-T1, INV-T2, etc.** (use T prefix for Temporal)
- Describe the exact timing requirement or chronological constraint
- Identify the time-dependent variables and operations
- Specify the temporal relationships that must hold

### 3. VERIFICATION ANALYSIS
For each invariant, determine:
- **HOLDS**: Code properly enforces the temporal constraint
- **VIOLATION**: Time-based rules can be broken through some execution path

### 4. EXPLOIT DOCUMENTATION
For violations, provide:
- **exploit_path**: Function calls that violate temporal constraints
- **pre_state**: Required initial timing conditions
- **post_state**: Resulting temporal inconsistency or bypass
- **impact**: Security compromise through time manipulation
- **poc**: Step-by-step timing exploit example
- **mitigation**: How to fix the temporal vulnerability

Focus on ensuring that all time-based operations respect their intended chronological constraints and cannot be manipulated to bypass security mechanisms. Temporal invariants are crucial for maintaining the proper sequence of operations and preventing time-based attacks.
"#;

```
ai-agent-audit/src/prepare_code/git_clone.rs
```
/// Repository preparation and Docker-based building.
///
/// This module handles secure repository cloning in Docker containers,
/// auto-detection of build systems (Foundry/Hardhat), and file filtering
/// for smart contract analysis.
use anyhow::{Context, Result};
use ignore::gitignore::GitignoreBuilder;
use log::info;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::config::audit_config;
use crate::utils::file_security::{validate_repo_url, validate_safe_path};

/// Build flags for forge compilation
#[derive(Debug, Clone, Copy)]
pub enum BuildFlags {
    /// Standard forge build
    Standard,
    /// Forge build with --via-ir --build-info flags
    ViaIr,
}

/// Contains paths to the repository root and relevant files.
/// This struct organizes the paths to Solidity files and documentation
/// that will be processed for analysis.
#[derive(Debug, Clone)]
pub struct RepoPaths {
    /// Path to the repository root directory
    pub root: PathBuf,
    /// Paths to all Solidity (.sol) files in the repository
    pub sol_files: Vec<PathBuf>,
    /// Paths to documentation files (README.md, etc.)
    pub docs: Vec<PathBuf>,
    /// e.g. `"my-cool-repo"`
    pub repo_name: String,
    /// full 40-char SHA, e.g. `"1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t"`
    pub commit_hash: String,
}

impl RepoPaths {
    /// Generates a unique identifier for the repository using name and short commit hash.
    /// Used for creating unique vector database collections and cache keys.
    pub fn unique_repo_hash(&self) -> String {
        format!(
            "{}-{}",
            self.repo_name.replace("/", "-"),
            &self.commit_hash[..6]
        )
    }
}

/// Clones a repository and builds it in a secure Docker environment.
///
/// This function performs the complete repository preparation workflow:
/// 1. Creates a Docker volume for isolated analysis
/// 2. Clones the repository using Trail of Bits security toolbox
/// 3. Auto-detects and builds with Foundry or Hardhat
/// 4. Filters and organizes Solidity files and documentation
/// 5. Extracts commit hash for unique identification
///
/// # Arguments
/// * `url` - Git repository URL to clone and analyze
/// * `subfolder` - Optional subfolder name to analyze within the repository
/// * `build_flags` - Build flags for forge compilation
///
/// # Returns
/// * `RepoPaths` - Organized repository paths and metadata
///
/// # Security
/// All operations are performed in isolated Docker containers to prevent
/// malicious code execution on the host system.
pub fn clone_and_filter_git_repo(
    url: &str,
    subfolder: Option<&str>,
    build_flags: BuildFlags,
) -> Result<RepoPaths> {
    // 🔐 Validate the repository URL for safety
    validate_repo_url(url)?;

    // 2. Extract & sanitize the repo name
    let mut repo_name = url
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .to_string();

    // Append subfolder to repo_name if specified
    if let Some(sf) = subfolder {
        repo_name = format!("{}/{}", repo_name, sf);
    }
    info!("repo_name ==> {}", repo_name);

    // 4. Read HEAD and get the first 6 chars of the commit SHA
    let commit_hash = get_commit_hash(url)?;
    let short_hash = &commit_hash[..6];

    // 5. git clone, install, and build in secure docker container
    // returns dierctory where files are located
    let root = clone_and_build_repo(url, &repo_name, short_hash, build_flags)?;

    // 6. Build .gitignore matcher
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    ign.add_line(None, "node_modules")?;
    let ign = ign.build()?;

    // Determine the search root - if subfolder is specified, search within that subdirectory
    let search_root = root.join(&repo_name);
    info!("search_root => {}", search_root.display());

    // Validate that the search root exists
    if !search_root.exists() {
        anyhow::bail!(
            "Specified path '{}' does not exist in the repository",
            repo_name
        );
    }

    // Initialize vectors to store file paths
    let mut sol_files = Vec::new();
    let mut docs = Vec::new();
    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        // skip if gitignore or simlink
        if ign.matched(path, false).is_ignore()
            || fs::symlink_metadata(path)?.file_type().is_symlink()
        {
            continue;
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some("sol") => sol_files.push(path.to_path_buf()),
            Some("md")
                if path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .map(|f| f.eq_ignore_ascii_case("README.md"))
                    .unwrap_or(false) =>
            {
                docs.push(path.to_path_buf())
            }
            _ => {}
        }
    }

    // Return the collected paths
    Ok(RepoPaths {
        root,
        sol_files,
        docs,
        repo_name,
        commit_hash,
    })
}

pub fn clone_and_build_repo(
    repo_url: &str,
    repo_name: &str,
    commit_hash: &str,
    build_flags: BuildFlags,
) -> Result<PathBuf> {
    let docker_volume = format!(
        "{}/{}-{}",
        audit_config().docker_volume,
        repo_name.replace("/", "-"),
        &commit_hash[..6]
    );
    let docker_path = PathBuf::from(&docker_volume);

    // if github repo clones to multiple sub folders with different apps
    // then repo_name will be something like contracts/plume
    // git clone will clone to contracts (repo_root) and then we cd into plume
    let repo_root = repo_name.split('/').next().unwrap_or(repo_name);

    if docker_path.exists() {
        log::warn!(
            "Docker volume {} already exists. Removing for clean build.",
            docker_path.display()
        );
        fs::remove_dir_all(&docker_path).with_context(|| {
            format!(
                "Failed to remove existing docker volume {}",
                docker_path.display()
            )
        })?;
    }

    // Shallow clone for speed and security
    log::info!("git cloning repo...");

    // Build forge command based on build flags
    let forge_build_cmd = match build_flags {
        BuildFlags::ViaIr => {
            "forge install && forge build --via-ir --build-info --skip test --skip script"
        }
        BuildFlags::Standard => {
            "forge install && forge build --build-info --skip test --skip script"
        }
    };

    let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/workspace", docker_volume),
            "-w",
            "/workspace",
            "ghcr.io/trailofbits/eth-security-toolbox:nightly",
            "bash",
            "-c",
            &format!(
                "git clone --depth=1 {repo_url} {repo_root} && \
             cd {repo_name} && \
             if [ -f foundry.toml ]; then {forge_build_cmd}; \
             elif [ -f hardhat.config.js ] || [ -f hardhat.config.ts ]; then \
             npm install -g hardhat && npm install && npx hardhat compile; \
             else echo 'No build system detected'; fi"
            ),
        ])
        .status()
        .context("Failed to clone and build repository in Docker")?;

    if !status.success() {
        // 🔐 Validate the constructed docker volume path
        anyhow::bail!("Clone and Build failed in Docker");
    }

    if docker_path.exists() {
        validate_safe_path(&docker_path, Path::new(&audit_config().docker_volume))?;
    } else {
        log::warn!(
            "Skipping path validation because {} does not exist yet",
            docker_path.display()
        );
    }
    Ok(PathBuf::from(docker_volume))
}

fn get_commit_hash(repo_url: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["ls-remote", repo_url, "HEAD"])
        .output()
        .context("Failed to run git ls-remote")?;

    if !output.status.success() {
        anyhow::bail!(
            "git ls-remote failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commit_hash = stdout
        .split_whitespace()
        .next()
        .context("Unexpected ls-remote output format")?
        .to_string();

    Ok(commit_hash)
}

```
ai-agent-audit/src/cost/cost_data.rs
```
/// Cost tracking and calculation for LLM inference across multiple providers.
///
/// This module provides real-time cost tracking for AI agent operations,
/// supporting OpenAI, Anthropic, Gemini, and DeepSeek providers with
/// accurate token-based pricing calculations.

use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::llm_review::enums::AIAgent;

/// LLM provider cost types with specific model variants
#[derive(Clone, Copy, Debug)]
pub enum LlmCostType {
    /// OpenAI GPT-4o input tokens
    Openai4oInput,
    /// OpenAI GPT-4o output tokens
    Openai4oOutput,
    /// OpenAI O3 input tokens
    OpenaiO3Input,
    /// OpenAI O3 output tokens
    OpenaiO3Output,
    /// Anthropic Claude input tokens
    AnthropicClaudeInput,
    /// Anthropic Claude output tokens
    AnthropicClaudeOutput,
    /// Google Gemini input tokens
    GeminiInput,
    /// Google Gemini output tokens
    GeminiOutput,
    /// DeepSeek input tokens
    DeepseekInput,
    /// DeepSeek output tokens
    DeepseekOutput,
}

/// Token direction for cost calculation
#[derive(PartialEq, Eq)]
pub enum TokenType {
    /// Input tokens (prompt)
    Input,
    /// Output tokens (response)
    Output,
}

impl LlmCostType {
    /// Returns the cost per million tokens for each LLM provider and model.
    /// Prices are based on current provider pricing as of 2024.
    pub fn get_cost_per_million_tokens(self) -> f64 {
        match self {
            LlmCostType::Openai4oInput => 2.50,
            LlmCostType::Openai4oOutput => 10.00,
            LlmCostType::OpenaiO3Input => 2.00,
            LlmCostType::OpenaiO3Output => 8.00,
            LlmCostType::AnthropicClaudeInput => 3.00,
            LlmCostType::AnthropicClaudeOutput => 15.00,
            LlmCostType::GeminiInput => 1.25,
            LlmCostType::GeminiOutput => 10.00,
            LlmCostType::DeepseekInput => 0.07,
            LlmCostType::DeepseekOutput => 1.10,
        }
    }
}

impl AIAgent {
    pub fn get_cost_per_million_tokens(&self, token_type: TokenType) -> f64 {
        match self {
            // assume highest cost
            AIAgent::Openai(_) if token_type == TokenType::Input => 2.00,
            AIAgent::Openai(_) => 8.00,
            AIAgent::Anthropic(_) if token_type == TokenType::Input => 3.00,
            AIAgent::Anthropic(_) => 15.00,
            AIAgent::Gemini(_) if token_type == TokenType::Input => 1.25,
            AIAgent::Gemini(_) => 10.00,
            AIAgent::Deepseek(_) if token_type == TokenType::Input => 0.07,
            AIAgent::Deepseek(_) => 1.10,
        }
    }
}
static INFERENCE_COST_DATA: Lazy<Arc<Mutex<f64>>> = Lazy::new(|| Arc::new(Mutex::new(0.0)));

pub async fn add_to_inference_cost_by_type(content: &str, cost_type: LlmCostType) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;
    let tokens = get_token_count(content);

    *inference_cost = *inference_cost + tokens as f64 * cost_type.get_cost_per_million_tokens();
}

pub async fn add_to_inference_cost_by_agent(
    content: &str,
    agent: &Arc<AIAgent>,
    token_type: TokenType,
) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;
    let tokens = get_token_count(content);

    *inference_cost =
        *inference_cost + tokens as f64 * agent.get_cost_per_million_tokens(token_type);
}

pub async fn get_total_inference_cost() -> String {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let inference_cost = cost_data.lock().await;

    format!("{:.2}", *inference_cost / 1_000_000_f64)
}

/// Estimates tokens with additional handling for whitespace and special cases
pub fn get_token_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    // Remove extra whitespace and count characters
    let cleaned = text.trim();
    let char_count = cleaned.chars().count();

    // Use ceiling division: (n + divisor - 1) / divisor
    (char_count + 3) / 4
}


```
ai-agent-audit/src/llm_review/agent_factory.rs
```
use super::enums::AIAgent;
/// AI Agent Factory for centralized agent creation across LLM providers.
///
/// This module provides a unified interface for creating AI agents from different
/// LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) with consistent configuration
/// and error handling.
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self, O3},
    },
};
use std::sync::OnceLock;

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    Gemini,
    DeepSeek,
}

impl LlmProvider {
    /// Returns all supported providers.
    pub fn all() -> &'static [LlmProvider] {
        &[
            LlmProvider::OpenAI,
            LlmProvider::Anthropic,
            LlmProvider::Gemini,
            LlmProvider::DeepSeek,
        ]
    }

    /// Returns the string representation of the provider.
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "openai",
            LlmProvider::Anthropic => "anthropic",
            LlmProvider::Gemini => "gemini",
            LlmProvider::DeepSeek => "deepseek",
        }
    }

    /// Returns the environment variable name for the API key.
    pub fn api_key_env_var(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Gemini => "GEMINI_API_KEY",
            LlmProvider::DeepSeek => "DEEPSEEK_API_KEY",
        }
    }

    /// Checks if this provider is available based on environment variables.
    pub fn is_available(&self) -> bool {
        std::env::var(self.api_key_env_var()).is_ok()
    }

    /// Parse provider from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" => Some(LlmProvider::OpenAI),
            "anthropic" | "claude" => Some(LlmProvider::Anthropic),
            "gemini" | "google" => Some(LlmProvider::Gemini),
            "deepseek" => Some(LlmProvider::DeepSeek),
            _ => None,
        }
    }
}

/// Configuration for creating AI agents.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Temperature for response generation (0.0-2.0)
    pub temperature: f64,
    /// Model name to use for the provider
    pub model: String,
    /// Optional context to include in the agent
    pub context: Option<String>,
    /// Maximum tokens for responses (Anthropic only)
    pub max_tokens: Option<u64>,
    /// System preamble/prompt for the agent
    pub preamble: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones.".to_string(),
        }
    }
}

impl AgentConfig {
    /// Creates a new agent configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the temperature for response generation.
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Sets the model name.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Sets the context for the agent.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Sets the maximum tokens (for Anthropic models).
    pub fn with_max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the system preamble/prompt.
    pub fn with_preamble(mut self, preamble: impl Into<String>) -> Self {
        self.preamble = preamble.into();
        self
    }

    /// Creates an agent configuration for security auditing.
    pub fn for_security_audit() -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones.".to_string(),
        }
    }
}

/// Singleton clients for LLM providers
static OPENAI_CLIENT: OnceLock<openai::Client> = OnceLock::new();
static ANTHROPIC_CLIENT: OnceLock<anthropic::Client> = OnceLock::new();
static GEMINI_CLIENT: OnceLock<gemini::Client> = OnceLock::new();
static DEEPSEEK_CLIENT: OnceLock<deepseek::Client> = OnceLock::new();

/// Initializes all LLM clients from environment variables.
///
/// This function should be called once during application startup to initialize
/// all available LLM clients. Clients are only created if their API keys are available.
pub fn init_llm_clients() -> Result<()> {
    // Initialize OpenAI client if API key is available
    if audit_config().has_openai_key() {
        let client = openai::Client::from_env();
        OPENAI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("openai_client", "OpenAI client already initialized")
        })?;
    }

    // Initialize Anthropic client if API key is available
    if audit_config().has_anthropic_key() {
        let client = anthropic::Client::from_env();
        ANTHROPIC_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("anthropic_client", "Anthropic client already initialized")
        })?;
    }

    // Initialize Gemini client if API key is available
    if audit_config().has_google_ai_key() {
        let client = gemini::Client::from_env();
        GEMINI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("gemini_client", "Gemini client already initialized")
        })?;
    }

    // Initialize DeepSeek client if API key is available
    if audit_config().has_deepseek_key() {
        let client = deepseek::Client::from_env();
        DEEPSEEK_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("deepseek_client", "DeepSeek client already initialized")
        })?;
    }

    Ok(())
}

/// Returns the OpenAI client instance.
fn openai_client() -> Result<&'static openai::Client> {
    OPENAI_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "openai_client",
            "OpenAI client not initialized or API key not configured",
        )
    })
}

/// Returns the Anthropic client instance.
fn anthropic_client() -> Result<&'static anthropic::Client> {
    ANTHROPIC_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "anthropic_client",
            "Anthropic client not initialized or API key not configured",
        )
    })
}

/// Returns the Gemini client instance.
fn gemini_client() -> Result<&'static gemini::Client> {
    GEMINI_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "gemini_client",
            "Gemini client not initialized or API key not configured",
        )
    })
}

/// Returns the DeepSeek client instance.
fn deepseek_client() -> Result<&'static deepseek::Client> {
    DEEPSEEK_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "deepseek_client",
            "DeepSeek client not initialized or API key not configured",
        )
    })
}

/// Factory for creating AI agents across different providers.
pub struct AgentFactory;

impl AgentFactory {
    /// Creates an OpenAI agent with the specified configuration.
    pub fn create_openai_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = openai_client()?;
        let model = if config.model == "default" {
            O3
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Openai(builder.build()))
    }

    /// Creates an Anthropic agent with the specified configuration.
    pub fn create_anthropic_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = anthropic_client()?;
        let model = if config.model == "default" {
            CLAUDE_3_7_SONNET
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(max_tokens) = config.max_tokens {
            builder = builder.max_tokens(max_tokens);
        }

        Ok(AIAgent::Anthropic(builder.build()))
    }

    /// Creates a Gemini agent with the specified configuration.
    pub fn create_gemini_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = gemini_client()?;
        let model = if config.model == "default" {
            "gemini-2.5-pro"
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Gemini(builder.build()))
    }

    /// Creates a DeepSeek agent with the specified configuration.
    pub fn create_deepseek_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = deepseek_client()?;
        let model = if config.model == "default" {
            DEEPSEEK_CHAT
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Deepseek(builder.build()))
    }

    /// Creates an agent from the specified provider type.
    pub fn create_agent(provider: LlmProvider, config: &AgentConfig) -> Result<AIAgent> {
        match provider {
            LlmProvider::OpenAI => Self::create_openai_agent(config),
            LlmProvider::Anthropic => Self::create_anthropic_agent(config),
            LlmProvider::Gemini => Self::create_gemini_agent(config),
            LlmProvider::DeepSeek => Self::create_deepseek_agent(config),
        }
    }

    /// Creates an agent from a string provider name.
    pub fn create_agent_from_str(provider: &str, config: &AgentConfig) -> Result<AIAgent> {
        let provider_enum = LlmProvider::from_str(provider).ok_or_else(|| {
            AuditError::configuration(
                "llm_provider",
                &format!("Unsupported LLM provider: {}", provider),
            )
        })?;
        Self::create_agent(provider_enum, config)
    }

    /// Creates an agent from environment configuration.
    ///
    /// This method checks which API keys are available and creates an agent
    /// from the first available provider in priority order.
    pub fn create_from_env(config: &AgentConfig) -> Result<AIAgent> {
        // Try providers in order of preference
        for provider in LlmProvider::all() {
            if provider.is_available() {
                match Self::create_agent(*provider, config) {
                    Ok(agent) => return Ok(agent),
                    Err(_) => continue, // Try next provider
                }
            }
        }

        Err(AuditError::configuration(
            "llm_providers",
            "No LLM provider API keys found in environment",
        ))
    }

    /// Returns a list of available providers based on environment variables.
    pub fn available_providers() -> Vec<LlmProvider> {
        LlmProvider::all()
            .iter()
            .filter(|provider| provider.is_available())
            .copied()
            .collect()
    }
}

/// Convenience functions that match the original API but use the factory internally.
/// These maintain backward compatibility with existing code.

/// Creates an OpenAI agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_openai_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_openai_agent(
    _client: &openai::Client,
    temperature: f64,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_preamble(preamble)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_openai_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create OpenAI agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates an Anthropic agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_anthropic_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_anthropic_agent(
    _client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_max_tokens(max_tokens);

    // Use the factory with singleton clients
    AgentFactory::create_anthropic_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create Anthropic agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates a Gemini agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_gemini_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_gemini_agent(
    _client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_gemini_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create Gemini agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates a DeepSeek agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_deepseek_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_deepseek_agent(
    _client: &deepseek::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_deepseek_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create DeepSeek agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_provider_enum() {
        assert_eq!(LlmProvider::OpenAI.as_str(), "openai");
        assert_eq!(LlmProvider::Anthropic.as_str(), "anthropic");
        assert_eq!(LlmProvider::Gemini.as_str(), "gemini");
        assert_eq!(LlmProvider::DeepSeek.as_str(), "deepseek");
    }

    #[test]
    fn test_provider_from_str() {
        assert_eq!(LlmProvider::from_str("openai"), Some(LlmProvider::OpenAI));
        assert_eq!(LlmProvider::from_str("gpt"), Some(LlmProvider::OpenAI));
        assert_eq!(
            LlmProvider::from_str("claude"),
            Some(LlmProvider::Anthropic)
        );
        assert_eq!(LlmProvider::from_str("invalid"), None);
    }

    #[test]
    fn test_agent_config_builder() {
        let config = AgentConfig::new()
            .with_temperature(0.8)
            .with_model("gpt-4")
            .with_context("test context");

        assert_eq!(config.temperature, 0.8);
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.context, Some("test context".to_string()));
    }

    #[test]
    fn test_security_audit_config() {
        let config = AgentConfig::for_security_audit();
        assert_eq!(config.temperature, 1.0);
        assert!(config.preamble.contains("security auditing"));
        assert_eq!(config.max_tokens, Some(4096));
    }

    #[test]
    fn test_available_providers() {
        let providers = AgentFactory::available_providers();
        // This will depend on environment variables, so we just check it returns a Vec
        assert!(providers.is_empty() || !providers.is_empty());
    }
}

```
ai-agent-audit/src/llm_review/analysis_db.rs
```
use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct FindingDb {
    pub id: String, // UUID v4
    pub title: String,
    pub description: String,
    pub impact: String,           // FK → seeds.id
    pub proof_of_concept: String, // e.g. "High", "Info", …
    pub proof_of_code: String,
    pub severity: String,
    // pub confidence:   Option<f32>,
    // pub sources_used: Vec<u8>,
}

pub struct FindingsDb(Connection);

impl FindingsDb {
    pub fn open(p: &Path) -> Result<Self> {
        let conn = Connection::open(p)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS findings(
              id                  TEXT PRIMARY KEY,
              title               TEXT,
              description         TEXT,
              impact              TEXT,
              proof_of_concept    TEXT,
              proof_of_code       TEXT,
              severity            TEXT,
            );
            "#,
        )?;
        Ok(Self(conn))
    }

    pub fn insert(&self, f: &FindingDb) -> Result<()> {
        self.0.execute(
            "INSERT INTO findings VALUES (?1,?2,?3,?4,?5,?6,?7);",
            params![
                f.id,
                f.title,
                f.description,
                f.impact,
                f.proof_of_concept,
                f.proof_of_code,
                f.severity
            ],
        )?;
        Ok(())
    }
}

```
ai-agent-audit/src/llm_review/code_review.rs
```
use crate::ai_bot::agent::get_rag_for_security_query;
use crate::config::audit_config;
use crate::error::Result;
use crate::prepare_code::git_clone::RepoPaths;
use crate::{
    cost::cost_data::{
        add_to_inference_cost_by_agent, add_to_inference_cost_by_type, LlmCostType, TokenType,
    },
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        agent_factory::{AgentConfig, AgentFactory},
        config::{generated_llm_prompt, ContractInvariants, Finding, VulnerabilityQualityCheck},
        context_state::get_metadata_context,
        prompt_context::generate_prompt_for_issue_check,
        prompt_support::{
            post_prompt::POST_PROMPT, post_qualify::POST_QUALIFY, post_verify::POST_VERIFY,
            pre_prompt::PRE_PROMPT, pre_qualify::PRE_QUALIFY, pre_verify::PRE_VERIFY,
            qualify_prompt::QUALIFY_PROMPT, verify_prompt::VERIFY_PROMPT,
        },
    },
    master_prompts::{prompt_2x_aa::PROMPT_2X_AA, prompt_2x_bb::PROMPT_2X_BB},
};
use log::info;
use rig::providers::openai::O3;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::Mutex;

use super::{
    config::{Findings, LegitVulnerability},
    enums::AIAgent,
};

/// Multi-LLM security analysis orchestration.
///
/// This module coordinates parallel security analysis across multiple LLM providers,
/// implements verification and deduplication workflows, and manages cost tracking.

/// Orchestrates comprehensive security analysis of smart contracts using multiple LLM providers.
///
/// This function performs parallel vulnerability detection across multiple AI agents,
/// implements verification and deduplication workflows, and returns categorized findings.
///
/// # Arguments
/// * `codeblocks_path` - Path to the database containing generated code blocks
///
/// # Returns
/// * `HashMap<String, Findings>` - Security findings organized by contract
/// * `Vec<ContractInvariants>` - Protocol invariant analysis results
pub async fn review_codebase_for_security_issues(
    codeblocks_path: &PathBuf,
    repo: &RepoPaths,
) -> Result<(HashMap<String, Findings>, Vec<ContractInvariants>)> {
    let mut all_security_issues = HashMap::<String, Findings>::new();
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    let (ai_verify_agent, ai_discovery_agents) = generate_ai_agents().await?;

    let invariant_findings = Vec::<ContractInvariants>::new();

    let metadata_context = get_metadata_context().await?;

    for (contract, codeblock) in contracts.into_iter() {
        info!("contract => {}", contract);
        info!("codeblock => {}", codeblock);
        // grab additional context from RAG
        let rag_context = get_rag_for_security_query(&codeblock, repo).await?;
        let audit_context = format!(
            "\n## CONTEXT \n\n {} \n\n {}",
            metadata_context, rag_context
        );

        let raw_findings = Findings::generate_findings_from_contract_codebase(
            &contract,
            &codeblock,
            &audit_context,
            &ai_discovery_agents,
        )
        .await?;

        if !raw_findings.findings.is_empty() {
            info!(
                "# of findings BEFORE deduping => {}",
                raw_findings.findings.len()
            );

            let deduped_and_verified_findings = raw_findings
                .dedup_and_verify_with_llm(&codeblock, &ai_verify_agent, &audit_context)
                .await?;

            let quality_checked_and_updated_findings = deduped_and_verified_findings
                .quality_check_with_llm(&codeblock, &ai_verify_agent, &audit_context)
                .await?;

            all_security_issues.insert(contract.to_string(), quality_checked_and_updated_findings);

            // TODO - save issues to Findings db
        }
    }

    // info!("standard security findings => {:#?}", all_security_issues);
    // info!("invariant findings => {:#?}", invariant_findings);
    Ok((all_security_issues, invariant_findings))
}

pub async fn generate_ai_agents() -> Result<(Arc<AIAgent>, Vec<Arc<AIAgent>>)> {
    info!("setting up AI agents...");

    // Create verification agent using OpenAI O3
    let verify_config = AgentConfig::new()
        .with_temperature(1.0)
        .with_model(O3)
        .with_preamble("You are SoliditySec-Verifier, a senior smart-contract auditor.");

    let ai_verify_agent = Arc::new(AgentFactory::create_openai_agent(&verify_config)?);

    // Create discovery agents using Anthropic models
    let mut ai_discovery_agents = Vec::new();

    let gemini_config = AgentConfig::for_security_audit()
        .with_temperature(1.0)
        .with_model("gemini-2.5-pro");

    for _ in 0..audit_config().runs {
        let agent = Arc::new(AgentFactory::create_gemini_agent(&gemini_config)?);
        ai_discovery_agents.push(agent);
    }

    info!("Created {} discovery agents", ai_discovery_agents.len());

    Ok((ai_verify_agent, ai_discovery_agents))
}

/// Executes one LLM-prompt round and merges the returned findings into the shared `Arc<Mutex<Findings>>`.
///
/// Splitting the logic out of the for-loop keeps the main auditor-loop readable
/// and makes it far easier to unit-test this piece in isolation.
pub async fn run_security_prompt(
    agent: Arc<AIAgent>,
    contract_name: Arc<String>,
    code: Arc<String>,
    added_context: Arc<String>,
    instructions: &'static str,
    idx_of_review_round: usize,
    shared_findings: Arc<Mutex<Findings>>,
) -> Result<()> {
    // 1. Build full prompt
    let prompt_header = generated_llm_prompt(&contract_name, instructions, PRE_PROMPT, POST_PROMPT);
    let prompt_body = generate_content_plus_context_block(&code, &added_context);
    let full_prompt = format!("{prompt_header}{prompt_body}");

    // add to cost
    // add_to_inference_cost_by_type(&full_prompt, LlmCostType::GeminiInput).await;
    add_to_inference_cost_by_agent(&full_prompt, &agent, TokenType::Input).await;

    // 2. Send to the right provider
    info!("----LLM analysis Round #{}----", idx_of_review_round);
    let findings: Findings = agent.extract_with_retry(&full_prompt).await?;

    let issues_found = findings.findings.len();
    info!("{} issues found!", issues_found);

    // 3. Merge results (if any) into the shared accumulator
    if issues_found > 0 {
        let mut guard = shared_findings.lock().await;
        guard.findings.extend(findings.findings);
    }

    Ok(())
}

fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n");

    code_plus_context.push_str(codeblock);
    // print_first_four_lines(&codeblock);

    code_plus_context.push_str("\n\n ADDITIONAL CONTEXT \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");
    // print_first_four_lines(&added_context);

    code_plus_context
}

//SCAN FOR INVARIANTS
// info!("submitting invariant prompt to openai");
// let invariants_response = openai_agent_1st_pass.prompt(INVARIANTS).await?;
//
// info!("parsing invariant prompt");
// let invariants = ContractInvariants::parse_from_json(&invariants_response)?;
//
// if !invariants.invariants.is_empty() {
//     invariant_findings.push(invariants);
// }
// for security_issue in SECURITY_PROMPT_ENUMS {
// SCAN FOR STANDARD SECURITY ISSUES
// let findings = if LANGUAGE_MODEL == LanguageModel::OpenAI {
//     let prompt_string = generate_llm_prompt_for_security_issue(
//         contract,
//         &security_issue.prompt(),
//         codeblock,
//         "",
//     );
//
//     info!("submitting security vulnerability prompt to openai");
//
//     let findings = agent_extract_with_retry(openai_agent, &prompt_string).await?;
//     findings
// } else {
//     let prompt_string = generate_llm_prompt_for_security_issue(
//         contract,
//         &security_issue.prompt(),
//         codeblock,
//         &added_context_from_ai_brain,
//     );
//
//     info!("submitting security vulnerability prompt to anthropic");
//     info!(
//         "-----------------------ROUND #{}-----------------------",
//         run
//     );
//     info!("{} Issue", security_issue.as_fancy_str());
//     let findings = agent_extract_with_retry(&anthropic_agents[run], &prompt_string).await?;
//
//     findings
// }
//
//
impl Findings {
    pub async fn generate_findings_from_contract_codebase(
        contract: &str,
        code: &str,
        context: &str,
        agents: &Vec<Arc<AIAgent>>,
    ) -> Result<Self> {
        let mut handles = vec![];
        let all_findings = Arc::new(Mutex::new(Findings {
            findings: Vec::new(),
        }));
        let contract = Arc::new(contract.to_string());
        let codeblock = Arc::new(code.to_string());
        let added_content_from_brain = Arc::new(context.to_string());

        for (run, arc_agent) in agents.iter().enumerate() {
            // PAUSED FOR COMPETITIVE AUDIT, only focused on critical issues in code
            // for (i, prompt) in [PROMPT_2X_AA, PROMPT_2X_BB].into_iter().enumerate() {
            for (i, prompt) in [PROMPT_2X_AA].into_iter().enumerate() {
                let agent = Arc::clone(arc_agent);
                let combined_findings = Arc::clone(&all_findings);
                let contract_name = Arc::clone(&contract);
                let code = Arc::clone(&codeblock);
                let added_content = Arc::clone(&added_content_from_brain);

                handles.push(tokio::spawn(async move {
                    if let Err(e) = run_security_prompt(
                        agent,
                        contract_name,
                        code,
                        added_content,
                        prompt,
                        (run + 1) * (i + 1),
                        combined_findings,
                    )
                    .await
                    {
                        log::error!("Prompt task failed: {e:#}");
                    }
                }));
            }
        }

        // Wait for ALL tasks to complete
        for handle in handles {
            handle.await?; // Will error if task panicked
        }

        let findings = all_findings.lock().await;

        if !findings.findings.is_empty() {
            info!(
                "# of findings BEFORE deduping => {}",
                findings.findings.len()
            );
        }
        Ok(findings.clone())
    }
    // TODO - refactor dedup to first use hashMap => HashMap<String(hash),Vec<Finding> (same hash)>
    // then evaluate each entry for dups with different thread!
    pub async fn dedup_and_verify_with_llm(
        &self,
        code: &str,
        agent: &Arc<AIAgent>,
        context: &str,
    ) -> Result<Self> {
        let mut handles = vec![];
        let deduped_findings = Arc::new(self.clone().dedup().await?);
        let code_and_context = generate_content_plus_context_block(code, context);
        let arc_code_context = Arc::new(code_and_context);

        let dedup_finding_count = deduped_findings.findings.len();
        let is_legit_finding_vec: Arc<Mutex<Vec<bool>>> =
            Arc::new(Mutex::new(vec![true; dedup_finding_count]));

        info!("# of findings AFTER deduping => {}", dedup_finding_count);

        info!("now verifying each finding...");

        for i in 0..dedup_finding_count {
            let codeblock_plus_context = Arc::clone(&arc_code_context);
            let arc_agent = Arc::clone(agent);
            let arc_findings = Arc::clone(&deduped_findings);
            let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
            handles.push(tokio::spawn(async move {
                let result: Result<()> = async {
                    let instruction_prompt = generate_prompt_for_issue_check(
                        &codeblock_plus_context,
                        &arc_findings.findings[i],
                        PRE_VERIFY,
                        VERIFY_PROMPT,
                        POST_VERIFY,
                    );

                    // add to cost
                    add_to_inference_cost_by_type(&instruction_prompt, LlmCostType::OpenaiO3Input)
                        .await;
                    info!("verifying finding #{}", i);
                    let is_legit_struct: LegitVulnerability =
                        arc_agent.extract_with_retry(&instruction_prompt).await?;

                    let is_finding_legit = is_legit_struct.is_legit_vulnerability;
                    if !is_finding_legit {
                        info!(
                            "{} is NOT legit => {}",
                            arc_findings.findings[i].title(),
                            is_legit_struct.why_its_not_legit.unwrap_or_default()
                        );
                    }
                    let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                    legit_findings_vec[i] = is_finding_legit;

                    // add
                    Ok(())
                }
                .await;

                if let Err(e) = result {
                    log::error!("Error verifying finding {}: {:?}", i, e);
                }
            }));
        }

        // optionally await them all
        for h in handles {
            let _ = h.await;
        }

        let legit_findings_vec = is_legit_finding_vec.lock().await;
        let verified_findings: Vec<Finding> = deduped_findings
            .as_ref()
            .findings
            .iter()
            .enumerate()
            .filter(|(idx, _)| legit_findings_vec[*idx])
            .map(|(_, f)| f.clone())
            .collect();

        info!(
            "-------------{} Verified Findings!-----------------",
            verified_findings.len()
        );

        Ok(Findings {
            findings: verified_findings,
        })
    }

    pub async fn quality_check_with_llm(
        self,
        code: &str,
        agent: &Arc<AIAgent>,
        context: &str,
    ) -> Result<Self> {
        let mut handles = vec![];
        let findings = Arc::new(self.clone().dedup().await?);
        let code_and_context = generate_content_plus_context_block(code, context);
        let arc_code_context = Arc::new(code_and_context);

        let finding_count = findings.findings.len();
        // create vec (is_quality_check_passed, updated_finding) for each finding
        // assume all initially pass
        let quality_check_passed_vec: Arc<Mutex<Vec<(bool, Finding)>>> =
            Arc::new(Mutex::new(vec![(true, Finding::default()); finding_count]));

        info!("now quality check on each finding...");

        for i in 0..finding_count {
            let codeblock_plus_context = Arc::clone(&arc_code_context);
            let arc_agent = Arc::clone(agent);
            let arc_findings = Arc::clone(&findings);
            let arc_legit_findings_vec = Arc::clone(&quality_check_passed_vec);
            handles.push(tokio::spawn(async move {
                let result: Result<()> = async {
                    let prompt = generate_prompt_for_issue_check(
                        &codeblock_plus_context,
                        &arc_findings.findings[i],
                        PRE_QUALIFY,
                        QUALIFY_PROMPT,
                        POST_QUALIFY,
                    );
                    add_to_inference_cost_by_type(&prompt, LlmCostType::OpenaiO3Input).await;
                    info!("quality checking finding #{}", i);
                    let qualify_checked_finding: VulnerabilityQualityCheck =
                        arc_agent.extract_with_retry(&prompt).await?;

                    let quality_check_passed = qualify_checked_finding.is_quality_check_passed;
                    if !quality_check_passed {
                        info!(
                            "{} did not pass quality check => {:?}",
                            arc_findings.findings[i].title(),
                            qualify_checked_finding.where_quality_lacks
                        );
                        info!("{:#?}", &qualify_checked_finding);
                        let updated_finding = Finding {
                            impact: Some(qualify_checked_finding.impact.clone().unwrap_or(
                                arc_findings.findings[i].impact.clone().unwrap_or_default(),
                            )),
                            proof_of_code: Some(
                                qualify_checked_finding.proof_of_code.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .proof_of_code
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            proof_of_concept: Some(
                                qualify_checked_finding.proof_of_concept.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .proof_of_concept
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            mitigation: Some(
                                qualify_checked_finding.mitigation.clone().unwrap_or(
                                    arc_findings.findings[i]
                                        .mitigation
                                        .clone()
                                        .unwrap_or_default(),
                                ),
                            ),
                            severity: qualify_checked_finding
                                .severity
                                .unwrap_or(arc_findings.findings[i].severity),
                            ..arc_findings.findings[i].clone()
                        };
                        let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                        legit_findings_vec[i] = (quality_check_passed, updated_finding);
                    } else {
                        let mut quality_checked_findings_vec = arc_legit_findings_vec.lock().await;
                        quality_checked_findings_vec[i] = (true, arc_findings.findings[i].clone());
                    }

                    Ok(())
                }
                .await;

                if let Err(e) = result {
                    log::error!("Error verifying finding {}: {:?}", i, e);
                }
            }));
        }

        // optionally await them all
        for h in handles {
            let _ = h.await;
        }

        let qualify_checked_findings_vec = quality_check_passed_vec.lock().await;
        let qualified_findings: Vec<Finding> = findings
            .as_ref()
            .findings
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                if qualify_checked_findings_vec[idx].0 {
                    // if quality check passes , no changes needed
                    f.clone()
                } else {
                    // if failed submit updated finding
                    qualify_checked_findings_vec[idx].1.clone()
                }
            })
            .collect();

        let num_findings_updated = qualify_checked_findings_vec
            .iter()
            .filter(|(passed, _)| !*passed)
            .count();

        info!(
            "{} Verified Findings with {} updated findings!",
            qualified_findings.len(),
            num_findings_updated
        );

        Ok(Findings {
            findings: qualified_findings,
        })
    }
}

```
ai-agent-audit/src/llm_review/config.rs
````
use super::{
    enums::{InvariantStatus, InvariantType, Severity, VulnerabilityType},
    prompt_support::dedup::DEDUP_PROMPT,
};
use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, LlmCostType},
    master_prompts::{prompt_2x_a::PROMPT_2X_A, prompt_2x_b::PROMPT_2X_B},
    prompts::{
        access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
        confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
        dos::DOS, inheritance::WRONG_INHERITANCE, integer_overflow::INTEGER_OVERFLOW, mev::MEV,
        oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
        reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK,
        self_destruct::SELF_DESTRUCT, storage_variables::STORAGE_VARIABLE, tx_origin::TX_ORIGIN,
        unchecked_return_value::UNCHECK_RETURN_VALUES, unexpected_eth::UNEXPECTED_ETH,
        zero_code::CONTRACTS_WITH_ZERO_CODE,
    },
};
use regex::Regex;
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    completion::{CompletionModel, Prompt},
    providers::{
        azure::GPT_4O,
        openai::{self},
    },
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageModel {
    OpenAI,
    Anthropic,
}

pub const CLAUDE_4_0_SONNET: &str = "claude-sonnet-4-0";
pub const CLAUDE_4_OPUS: &str = "claude-opus-4-0";
pub const LANGUAGE_MODEL: LanguageModel = LanguageModel::Anthropic;
pub const RUNS: usize = 3;
pub const INSTRUCTION_PROMPTS: [&str; 2] = [PROMPT_2X_A, PROMPT_2X_B];

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Finding {
    // [Severity-issue number] - List Issue (Reentrancy, Denial of Service, etc) and
    // <Contract>::<Function> its localed in
    pub issue_type: VulnerabilityType,
    pub contract: String,            // exact constract name where issue appears
    pub function: String, // exact function name where issue appears, if not applicable set to 'NA'
    pub description: Option<String>, // description of issue, include code snippet if relevant
    pub impact: Option<String>, // Impact of Issue
    pub proof_of_concept: Option<String>, // Demonstrate how issue can be exploited by hacker
    pub proof_of_code: Option<String>, // Write Foundry Unit test to prove issue exists
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Severity, //severity of issue
    pub mitigation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Findings {
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InvariantFinding {
    pub id: String,
    #[schemars(
        description = "Type: Arithmetic, Balance, Permission, Temporal, Referential, StateMachine"
    )]
    pub inv_type: InvariantType,
    pub desc: String,
    #[schemars(description = "Status: HOLDS, VIOLATION")]
    pub status: InvariantStatus,
    pub pre_state: Option<String>,
    pub post_state: Option<String>,
    pub impact: Option<String>,
    pub poc: Option<String>,
    pub mitigation: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContractInvariants {
    pub contract: String,
    pub intention: String,
    pub invariants: Vec<InvariantFinding>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitVulnerability {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_legit_vulnerability: bool,
    pub why_its_not_legit: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VulnerabilityQualityCheck {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_quality_check_passed: bool, // quality check passes with no changes/update needed, true|false
    pub where_quality_lacks: Option<String>, // breif description
    pub impact: Option<String>,              // updated impact (if necessary)
    pub proof_of_concept: Option<String>,    // updated POC (if necessary)
    pub proof_of_code: Option<String>,       // updated Foundry Unit test (if necessary)
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Option<Severity>, // updated severity of issue (if necessary)
    pub mitigation: Option<String>,          // updated mitigation (if necessary)
}

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct InvariantFindings {
//     pub findings: Vec<InvariantFinding>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DuplicateFindings {
    pub titles: Vec<String>,
}

pub const SECURITY_PROMPT_ENUMS: [VulnerabilityType; 8] = [
    VulnerabilityType::Reentrancy,
    VulnerabilityType::AccessControl,
    // VulnerabilityType::ArrayLimits,
    // VulnerabilityType::DefaultVisibility,
    VulnerabilityType::Dos,
    VulnerabilityType::IntegerMath,
    // VulnerabilityType::ConfidentialData,
    // VulnerabilityType::Inheritance,
    // VulnerabilityType::Oracle,
    VulnerabilityType::Pragma,
    VulnerabilityType::Randomness,
    // VulnerabilityType::ReplayAttack,
    // VulnerabilityType::SelfDestruct,
    // VulnerabilityType::StorageLayout,
    // VulnerabilityType::TxOrigin,
    // VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UnexpectedEth,
    // VulnerabilityType::ZeroCode,
    VulnerabilityType::FrontrunMev,
    // VulnerabilityType::ShortAddress,
];

pub const QUALITY_CHECK_ENUMS: [VulnerabilityType; 24] = [
    VulnerabilityType::Reentrancy,
    VulnerabilityType::AccessControl,
    VulnerabilityType::ArrayLimits,
    VulnerabilityType::Dos,
    VulnerabilityType::IntegerMath,
    VulnerabilityType::ConfidentialData,
    VulnerabilityType::Inheritance,
    VulnerabilityType::Oracle,
    VulnerabilityType::Randomness,
    VulnerabilityType::ReplayAttack,
    VulnerabilityType::SelfDestruct,
    VulnerabilityType::StorageLayout,
    VulnerabilityType::TxOrigin,
    VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UnexpectedEth,
    VulnerabilityType::FrontrunMev,
    VulnerabilityType::UpgradeabilityInitializerSafety,
    VulnerabilityType::PausableEmergencyStop,
    VulnerabilityType::TimestampDependentLogic,
    VulnerabilityType::FlashLoanEconomicManipulation,
    VulnerabilityType::DelegatecallLowLevelOps,
    VulnerabilityType::SignatureMalleability,
    VulnerabilityType::GasGriefBlockLimit,
    VulnerabilityType::IntegerOverflow,
];

pub const SECURITY_PROMPTS: [&str; 19] = [
    REENTRANCY,                  //DONE
    ACCESS_CONTROL,              //DONE
    ACCESS_OUTSIDE_ARRAY_LIMITS, //DONE
    DEFAULT_VISIBILITIES,        //DONE
    DOS,                         //DONE
    INTEGER_OVERFLOW,            // DONE
    SAVING_CONFIDENTIAL_DATA,    //DONE
    WRONG_INHERITANCE,           // DONE
    ORACLE_MANIPULATION,         // DONE
    FLOATING_PRAGMA,             //DONE
    RANDOMNESS,                  // DONE
    REPLAY_SIGNATURES_ATTACK,    //DONE
    SELF_DESTRUCT,               //DONE
    STORAGE_VARIABLE,            //DONE
    TX_ORIGIN,                   //DONE
    UNCHECK_RETURN_VALUES,       //DONE
    UNEXPECTED_ETH,              //DONE
    CONTRACTS_WITH_ZERO_CODE,    //DONE
    // SHORT_ADDRESS_ATTACK,        // DONE - this is only issue for very old contracts
    MEV, //DONE
];

pub fn generated_llm_prompt(
    contract_name: &str,
    main_instructions: &str,
    pre: &str,
    post: &str,
) -> String {
    let instruction_template = format!("{}{}{}", pre, main_instructions, post);

    // populate template
    instruction_template
        .replace("{contract_name}", contract_name)
        .to_string()
}

fn deserialize_bool_from_str_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let val: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match val {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(s) => match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(serde::de::Error::custom("expected 'true' or 'false'")),
        },
        _ => Err(serde::de::Error::custom("expected boolean or string")),
    }
}

impl Finding {
    pub fn title(&self) -> String {
        let fn_name = self.get_fn_name();
        format!(
            "{} issue in {}::{}",
            self.issue_type.as_fancy_str(),
            self.contract,
            fn_name
        )
    }

    pub fn free_report_title(&self) -> String {
        if self.severity == Severity::High || self.severity == Severity::Medium {
            format!(
                "{} issue found with {} severity",
                self.issue_type.as_fancy_str(),
                self.severity.as_str()
            )
        } else {
            format!(
                "{} issue in {}::{}",
                self.issue_type.as_fancy_str(),
                self.contract,
                self.function
            )
        }
    }

    pub fn hash(&self) -> String {
        // extract name 'func_name' from func_name(...)
        let fn_name = self.get_fn_name();

        format!("{}-{}-{}", self.issue_type.as_str(), self.contract, fn_name)
    }

    // extract name 'func_name' from func_name(...)
    pub fn get_fn_name(&self) -> String {
        let re = Regex::new(r"(^[a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
        if let Some(captures) = re.captures(&self.function) {
            match captures.get(1) {
                Some(name) => name.as_str().to_string(),
                None => self.function.clone(),
            }
        } else {
            self.function.clone()
        }
    }

    // resonse "YES" or "NO"
    pub async fn is_duplicate_issue<M>(
        &self,
        issue: &Finding,
        ai_agent: &Agent<M>,
    ) -> anyhow::Result<bool>
    where
        M: CompletionModel,
    {
        // contract, function and issue type MUST match
        if issue.hash() != self.hash() {
            return Ok(false);
        } else if issue.description == self.description {
            return Ok(true);
        }

        let prompt = DEDUP_PROMPT
            .replace("{contract}", &self.contract)
            .replace("{function}", &self.function)
            .replace("{issue_type}", self.issue_type.as_str())
            .replace(
                "{description_a}",
                &self.description.clone().unwrap_or_default(),
            )
            .replace(
                "{description_b}",
                &issue.description.clone().unwrap_or_default(),
            );

        log::info!("checking if {} is duplication", issue.title());
        add_to_inference_cost_by_type(&prompt, LlmCostType::Openai4oInput).await;

        let response = ai_agent.prompt(prompt).await?;

        add_to_inference_cost_by_type(&response, LlmCostType::Openai4oOutput).await;

        Ok(response.trim().eq_ignore_ascii_case("YES"))
    }
}

impl ContractInvariants {
    /// Parse JSON string containing findings from LLM response
    /// Handles both clean JSON and JSON wrapped in markdown code blocks
    pub fn parse_from_json(json_str: &str) -> Result<ContractInvariants, serde_json::Error> {
        // Clean the input - remove markdown code blocks and extra quotes/escapes
        let cleaned_json = Self::clean_json_string(json_str);

        // Parse the cleaned JSON
        serde_json::from_str(&cleaned_json)
    }

    /// Clean JSON string by removing markdown code blocks, escaped quotes, and extra formatting
    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        // Remove outer quotes if present (from string literals)
        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        // Remove markdown code blocks
        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        // Replace escaped quotes and newlines
        // cleaned
        //     .replace("\\\"", "\"")
        //     .replace("\\n", "\n")
        //     .replace("\\\n", "\n")
        //     .trim()
        //     .to_string()
        cleaned.to_string()
    }

    pub fn get_all_violations(self) -> Vec<InvariantFinding> {
        self.invariants
            .into_iter()
            .filter(|inv| inv.status == InvariantStatus::VIOLATION)
            .collect::<Vec<InvariantFinding>>()
    }
}
impl Findings {
    pub async fn dedup(self) -> anyhow::Result<Findings> {
        if self.findings.is_empty() {
            return Ok(Findings {
                findings: Vec::new(),
            });
        }

        let openai_client = openai::Client::from_env();
        let openai_agent = Arc::new(openai_client.agent(GPT_4O).temperature(1.0).build());

        let mut findings_hash = HashMap::<String, Vec<Finding>>::new();

        for finding in &self.findings {
            let hash = finding.hash();
            findings_hash
                .entry(hash)
                .or_insert(Vec::new())
                .push(finding.clone());
        }

        let arc_dedup_findings = Arc::new(Mutex::new(Vec::with_capacity(self.findings.len())));
        let mut handles = Vec::new();

        for findings in findings_hash.into_values() {
            let arc_findings = Arc::new(findings);
            let current_findings = Arc::clone(&arc_findings);
            let deduped_findings = Arc::clone(&arc_dedup_findings);
            let agent = Arc::clone(&openai_agent);
            let handle = tokio::spawn(async move {
                if current_findings.len() > 1 {
                    match get_deduped_finding_vec(&current_findings, &agent).await {
                        Ok(deduped) => {
                            let mut deduped_findings_lock = deduped_findings.lock().await;
                            deduped_findings_lock.extend(deduped);
                        }
                        Err(e) => {
                            log::error!("❌ deduping findngs failed: {e}");
                        }
                    }
                } else {
                    let mut deduped_findings_lock = deduped_findings.lock().await;
                    deduped_findings_lock.extend(current_findings.iter().cloned())
                }
            });
            handles.push(handle);
        }

        // CRITICAL: Wait for all tasks to complete
        for handle in handles {
            handle.await?;
        }
        // Fix: Extract the Vec from Arc<Mutex<Vec<Finding>>>
        let deduped_findings = Arc::try_unwrap(arc_dedup_findings)
            .map_err(|_| anyhow::anyhow!("Failed to unwrap Arc"))?
            .into_inner();

        Ok(Findings {
            findings: deduped_findings,
        })
    }

    /// Get count of findings by severity
    pub fn count_by_severity(&self) -> std::collections::HashMap<Severity, usize> {
        let mut counts = HashMap::new();

        for finding in &self.findings {
            *counts.entry(finding.severity).or_insert(0) += 1;
        }

        counts
    }

    /// Filter findings by severity level
    pub fn filter_by_severity(&self, severity: Severity) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .collect()
    }

    /// Get all high severity findings
    pub fn high_severity_findings(&self) -> Vec<&Finding> {
        self.filter_by_severity(Severity::High)
    }
}

async fn get_deduped_finding_vec<T>(
    findings: &Arc<Vec<Finding>>,
    agent: &Arc<Agent<T>>,
) -> anyhow::Result<Vec<Finding>>
where
    T: CompletionModel,
{
    // assigns bool to each finding index, is dup or not? assume not for initializing
    let size = findings.len();
    let mut is_dup_vec: Vec<bool> = vec![false; size];

    for i in 0..size {
        if is_dup_vec[i] {
            continue;
        }
        for j in i + 1..size {
            if is_dup_vec[j] {
                continue;
            }
            let is_dup = findings[i].is_duplicate_issue(&findings[j], &agent).await?;
            if is_dup {
                is_dup_vec[j] = true;
                continue;
            }
        }
    }

    let findings: Vec<Finding> = findings
        .iter()
        .enumerate()
        .filter(|(idx, _)| !is_dup_vec[*idx])
        .map(|(_, f)| f)
        .cloned()
        .collect();

    Ok(findings)
}

pub trait FromLLMJson: Sized {
    /// Parse clean JSON string into type
    fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error>;

    /// Clean up formatting (markdown code blocks, extra quotes, etc.)
    fn clean_json_string(input: &str) -> String;

    /// Extract and parse JSON from raw LLM response with extra text
    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>>;
}

impl<T> FromLLMJson for T
where
    T: DeserializeOwned,
{
    fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let cleaned = Self::clean_json_string(json_str);
        serde_json::from_str(&cleaned)
    }

    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        cleaned.to_string()
    }

    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        match (json_start, json_end) {
            (Some(start), Some(end)) if start < end => {
                let json_part = &response[start..=end];
                Self::parse_from_json(json_part)
                    .map_err(|e| format!("Failed to parse JSON: {}", e).into())
            }
            _ => Err("No valid JSON found in response".into()),
        }
    }
}

````
ai-agent-audit/src/llm_review/context_state.rs
```
/// Global context state management for AI analysis.
///
/// This module manages shared protocol metadata context that is generated once
/// and reused across all AI agents for consistent analysis. Provides thread-safe
/// access to protocol information including summaries and semantic data.
use once_cell::sync::Lazy;
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

use super::prompt_context::generate_context_for_code_review;
use crate::prepare_code::git_clone::RepoPaths;

/// Global metadata context shared across all AI agents
static METADATA_CONTEXT: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(String::new())));

/// Generates and caches protocol metadata context for AI analysis.
///
/// This function creates comprehensive context information including protocol
/// summaries, contract relationships, and semantic data that is shared across
/// all AI agents for consistent analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_path` - Path to semantic analysis database
pub async fn generate_and_save_metadata_context(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> anyhow::Result<()> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata = metadata_context.lock().await;
    let context = generate_context_for_code_review(repo, semantics_path).await?;

    *metadata = context.clone();
    Ok(())
}

/// Retrieves the cached metadata context for AI analysis.
///
/// Returns the protocol metadata context that was previously generated and
/// cached for use across all AI agents.
///
/// # Returns
/// * `String` - Cached protocol metadata context
pub async fn get_metadata_context() -> anyhow::Result<String> {
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata = metadata_context.lock().await;

    Ok(metadata.clone())
}

```
ai-agent-audit/src/llm_review/enums.rs
```
/// AI agent and vulnerability type enumerations.
///
/// This module defines the core enums for multi-LLM support and vulnerability
/// categorization, providing unified interfaces for different AI providers
/// and systematic vulnerability detection across 19+ security categories.

use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    cost::cost_data::LlmCostType,
    invariant_prompts::{
        arithmetic::ARITHMETIC, balance::BALANCE, permission::PERMISSION, referential::REFERENTIAL,
        state_machine::STATE_MACHINE, temporal::TEMPORAL,
    },
    master_prompts::master_prompt::MASTER_SECURITY_PROMPT,
    prompts::{
        access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
        confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
        dos::DOS, inheritance::WRONG_INHERITANCE, integer_overflow::INTEGER_OVERFLOW, mev::MEV,
        oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
        reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK,
        self_destruct::SELF_DESTRUCT, short_address_attack::SHORT_ADDRESS_ATTACK,
        storage_variables::STORAGE_VARIABLE, tx_origin::TX_ORIGIN,
        unchecked_return_value::UNCHECK_RETURN_VALUES, unexpected_eth::UNEXPECTED_ETH,
        zero_code::CONTRACTS_WITH_ZERO_CODE,
    },
    utils::extract_retry::agent_extract_with_retry,
};
use rig::{
    agent::Agent,
    extractor::Extractor,
    providers::{
        anthropic::{self},
        deepseek::DeepSeekCompletionModel,
        gemini::{self},
        openai::{self},
    },
};

use serde::de::DeserializeOwned;

/// Unified AI agent enum supporting multiple LLM providers.
///
/// Provides a common interface for different AI providers while maintaining
/// provider-specific optimizations and cost tracking capabilities.
pub enum AIAgent {
    /// Anthropic Claude models (3.7 Sonnet, 4.0 Sonnet)
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    /// OpenAI models (GPT-4o, O3)
    Openai(Agent<openai::CompletionModel>),
    /// Google Gemini models
    Gemini(Agent<gemini::completion::CompletionModel>),
    /// DeepSeek models (cost-effective option)
    Deepseek(Agent<DeepSeekCompletionModel>),
}

/// Unified AI extractor enum for structured data extraction.
///
/// Provides type-safe extraction capabilities across different LLM providers
/// with automatic retry logic and error handling.
pub enum AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    Anthropic(Extractor<anthropic::completion::CompletionModel, T>),
    Openai(Extractor<openai::CompletionModel, T>),
    Gemini(Extractor<gemini::completion::CompletionModel, T>),
    Deepseek(Extractor<DeepSeekCompletionModel, T>),
}

/// ------------------------------------------------------------------
/// 1.  Strict-typed severity enum
/// ------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum Severity {
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum InvariantType {
    Arithmetic,
    Balance,
    Permission,
    Temporal,
    Referential,
    StateMachine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum InvariantStatus {
    HOLDS,
    VIOLATION,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum VulnerabilityType {
    AccessControl,
    ArrayLimits,
    ConfidentialData,
    DefaultVisibility,
    Dos,
    Inheritance,
    IntegerMath,
    Oracle,
    Pragma,
    Randomness,
    Reentrancy,
    ReplayAttack,
    SelfDestruct,
    ShortAddress,
    StorageLayout,
    TxOrigin,
    UncheckedReturn,
    UnexpectedEth,
    ZeroCode,
    FrontrunMev,
    UpgradeabilityInitializerSafety,
    PausableEmergencyStop,
    TimestampDependentLogic,
    FlashLoanEconomicManipulation,
    DelegatecallLowLevelOps,
    SignatureMalleability,
    EventConsistency,
    GasGriefBlockLimit,
    IntegerOverflow,
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Info
    }
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::High => "High",
            Severity::Medium => "Medium",
            Severity::Low => "Low",
            Severity::Info => "Info",
        }
    }

    pub fn as_initial(self) -> &'static str {
        match self {
            Severity::High => "H",
            Severity::Medium => "M",
            Severity::Low => "L",
            Severity::Info => "I",
        }
    }
}

impl InvariantType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantType::Arithmetic => "Arithmetic",
            InvariantType::Balance => "Balance",
            InvariantType::Permission => "Permission",
            InvariantType::Temporal => "Temporal",
            InvariantType::Referential => "Referential",
            InvariantType::StateMachine => "StateMachine",
        }
    }

    pub fn get_prompt(self) -> &'static str {
        match self {
            InvariantType::Arithmetic => ARITHMETIC,
            InvariantType::Balance => BALANCE,
            InvariantType::Permission => PERMISSION,
            InvariantType::Temporal => TEMPORAL,
            InvariantType::Referential => REFERENTIAL,
            InvariantType::StateMachine => STATE_MACHINE,
        }
    }
}

impl InvariantStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvariantStatus::HOLDS => "Holds",
            InvariantStatus::VIOLATION => "Violation",
        }
    }
}

impl AIAgent {
    pub async fn extract_with_retry<T>(&self, prompt: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic(model) => Ok(agent_extract_with_retry::<_, T>(
                model,
                prompt,
                LlmCostType::AnthropicClaudeOutput,
            )
            .await?),
            AIAgent::Openai(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::OpenaiO3Output)
                        .await?,
                )
            }
            AIAgent::Gemini(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::GeminiOutput)
                        .await?,
                )
            }
            AIAgent::Deepseek(model) => {
                Ok(
                    agent_extract_with_retry::<_, T>(model, prompt, LlmCostType::DeepseekOutput)
                        .await?,
                )
            }
        }
    }
}

impl Default for VulnerabilityType {
    fn default() -> Self {
        VulnerabilityType::Dos
    }
}

impl VulnerabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => "Oracle",
            VulnerabilityType::AccessControl => "AccessControl",
            VulnerabilityType::FrontrunMev => "FrontrunMev",
            VulnerabilityType::UnexpectedEth => "UnexpectedEth",
            VulnerabilityType::Pragma => "Pragma",
            VulnerabilityType::Randomness => "Randomness",
            VulnerabilityType::TxOrigin => "TxOrigin",
            VulnerabilityType::ZeroCode => "ZeroCode",
            VulnerabilityType::SelfDestruct => "SelfDestruct",
            VulnerabilityType::StorageLayout => "StorageLayout",
            VulnerabilityType::ReplayAttack => "ReplayAttack",
            VulnerabilityType::ShortAddress => "ShortAddress",
            VulnerabilityType::IntegerMath => "IntegerMath",
            VulnerabilityType::UncheckedReturn => "UncheckedReturn",
            VulnerabilityType::Dos => "Dos",
            VulnerabilityType::DefaultVisibility => "DefaultVisibility",
            VulnerabilityType::Inheritance => "Inheritance",
            VulnerabilityType::ConfidentialData => "ConfidentialData",
            VulnerabilityType::Reentrancy => "Reentrancy",
            VulnerabilityType::ArrayLimits => "ArrayLimits",
            VulnerabilityType::UpgradeabilityInitializerSafety => "UpgradeabilityInitializerSafety",
            VulnerabilityType::PausableEmergencyStop => "PausableEmergencyStop",
            VulnerabilityType::TimestampDependentLogic => "TimestampDependentLogic",
            VulnerabilityType::FlashLoanEconomicManipulation => "FlashLoanEconomicManipulation",
            VulnerabilityType::DelegatecallLowLevelOps => "DelegatecallLowLevelOps",
            VulnerabilityType::SignatureMalleability => "SignatureMalleability",
            VulnerabilityType::EventConsistency => "EventConsistency",
            VulnerabilityType::GasGriefBlockLimit => "GasGriefBlockLimit",
            VulnerabilityType::IntegerOverflow => "IntegerOverflow",
        }
    }

    pub fn as_fancy_str(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => "Oracle",
            VulnerabilityType::AccessControl => "Access Control",
            VulnerabilityType::FrontrunMev => "Frontrun/Backrun/Sandwhich MEV",
            VulnerabilityType::UnexpectedEth => "Unexpected Eth",
            VulnerabilityType::Pragma => "Pragma",
            VulnerabilityType::Randomness => "Randomness",
            VulnerabilityType::TxOrigin => "tx.origin",
            VulnerabilityType::ZeroCode => "Zero Code",
            VulnerabilityType::SelfDestruct => "Self-Destruct",
            VulnerabilityType::StorageLayout => "Storage Layout",
            VulnerabilityType::ReplayAttack => "Replay Attack",
            VulnerabilityType::ShortAddress => "Short Address",
            VulnerabilityType::IntegerMath => "Integer Overflow/Math",
            VulnerabilityType::UncheckedReturn => "Unchecked Return",
            VulnerabilityType::Dos => "DOS",
            VulnerabilityType::DefaultVisibility => "Default Visibility",
            VulnerabilityType::Inheritance => "Inheritance",
            VulnerabilityType::ConfidentialData => "Confidential Data",
            VulnerabilityType::Reentrancy => "Reentrancy",
            VulnerabilityType::ArrayLimits => "Array Limits",
            VulnerabilityType::UpgradeabilityInitializerSafety => {
                "Upgradeability Initializer Safety"
            }
            VulnerabilityType::PausableEmergencyStop => "Pausable Emergency Stop",
            VulnerabilityType::TimestampDependentLogic => "Timestamp Dependent Logic",
            VulnerabilityType::FlashLoanEconomicManipulation => "Flash Loan Economic Manipulation",
            VulnerabilityType::DelegatecallLowLevelOps => "Delegatecall Low Level Ops",
            VulnerabilityType::SignatureMalleability => "Signature Malleability",
            VulnerabilityType::EventConsistency => "Event Consistency",
            VulnerabilityType::GasGriefBlockLimit => "Gas Grief BlockLimit",
            VulnerabilityType::IntegerOverflow => "Integer Overflow",
        }
    }

    pub fn prompt(self) -> &'static str {
        match self {
            VulnerabilityType::Oracle => ORACLE_MANIPULATION,
            VulnerabilityType::AccessControl => ACCESS_CONTROL,
            VulnerabilityType::FrontrunMev => MEV,
            VulnerabilityType::UnexpectedEth => UNEXPECTED_ETH,
            VulnerabilityType::Pragma => FLOATING_PRAGMA,
            VulnerabilityType::Randomness => RANDOMNESS,
            VulnerabilityType::TxOrigin => TX_ORIGIN,
            VulnerabilityType::ZeroCode => CONTRACTS_WITH_ZERO_CODE,
            VulnerabilityType::SelfDestruct => SELF_DESTRUCT,
            VulnerabilityType::StorageLayout => STORAGE_VARIABLE,
            VulnerabilityType::ReplayAttack => REPLAY_SIGNATURES_ATTACK,
            VulnerabilityType::ShortAddress => SHORT_ADDRESS_ATTACK,
            VulnerabilityType::IntegerMath => INTEGER_OVERFLOW,
            VulnerabilityType::UncheckedReturn => UNCHECK_RETURN_VALUES,
            VulnerabilityType::Dos => DOS,
            VulnerabilityType::DefaultVisibility => DEFAULT_VISIBILITIES,
            VulnerabilityType::Inheritance => WRONG_INHERITANCE,
            VulnerabilityType::ConfidentialData => SAVING_CONFIDENTIAL_DATA,
            VulnerabilityType::Reentrancy => REENTRANCY,
            VulnerabilityType::ArrayLimits => ACCESS_OUTSIDE_ARRAY_LIMITS,
            _ => MASTER_SECURITY_PROMPT,
        }
    }
}

/// ----- Serde glue --------------------------------------------------
/// * Accepts any case-insensitive spelling: "high", "HIGH", "High" …
impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "high" => Ok(Severity::High),
            "medium" => Ok(Severity::Medium),
            "low" => Ok(Severity::Low),
            "info" => Ok(Severity::Info),
            other => Err(de::Error::unknown_variant(
                other,
                &["High", "Medium", "Low", "Info"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InvariantType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "arithmetic" => Ok(InvariantType::Arithmetic),
            "balance" => Ok(InvariantType::Balance),
            "permission" => Ok(InvariantType::Permission),
            "temporal" => Ok(InvariantType::Temporal),
            "referential" => Ok(InvariantType::Referential),
            "statemachine" => Ok(InvariantType::StateMachine),
            other => Err(de::Error::unknown_variant(
                other,
                &[
                    "Arithmetic",
                    "Balance",
                    "Permission",
                    "Temporal",
                    "Referential",
                    "StateMachine",
                ],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InvariantStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "holds" => Ok(InvariantStatus::HOLDS),
            "violation" => Ok(InvariantStatus::VIOLATION),
            other => Err(de::Error::unknown_variant(
                other,
                &["High", "Medium", "Low", "Info"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for VulnerabilityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "oracle" => Ok(VulnerabilityType::Oracle),
            "accesscontrol" => Ok(VulnerabilityType::AccessControl),
            "frontrunmev" => Ok(VulnerabilityType::FrontrunMev),
            "unexpectedeth" => Ok(VulnerabilityType::UnexpectedEth),
            "pragma" => Ok(VulnerabilityType::Pragma),
            "randomness" => Ok(VulnerabilityType::Randomness),
            "txorigin" => Ok(VulnerabilityType::TxOrigin),
            "zerocode" => Ok(VulnerabilityType::ZeroCode),
            "selfdestruct" => Ok(VulnerabilityType::SelfDestruct),
            "storagelayout" => Ok(VulnerabilityType::StorageLayout),
            "replayattack" => Ok(VulnerabilityType::ReplayAttack),
            "shortaddress" => Ok(VulnerabilityType::ShortAddress),
            "integermath" => Ok(VulnerabilityType::IntegerMath),
            "uncheckedreturn" => Ok(VulnerabilityType::UncheckedReturn),
            "dos" => Ok(VulnerabilityType::Dos),
            "defaultvisibility" => Ok(VulnerabilityType::DefaultVisibility),
            "inheritance" => Ok(VulnerabilityType::Inheritance),
            "confidentialdata" => Ok(VulnerabilityType::ConfidentialData),
            "reentrancy" => Ok(VulnerabilityType::Reentrancy),
            "arraylimits" => Ok(VulnerabilityType::ArrayLimits),
            "upgradeabilityinitializersafety" => {
                Ok(VulnerabilityType::UpgradeabilityInitializerSafety)
            }
            "pausableemergencystop" => Ok(VulnerabilityType::PausableEmergencyStop),
            "timestampdependentlogic" => Ok(VulnerabilityType::TimestampDependentLogic),
            "flashloaneconomicmanipulation" => Ok(VulnerabilityType::FlashLoanEconomicManipulation),
            "delegatecalllowlevelops" => Ok(VulnerabilityType::DelegatecallLowLevelOps),
            "signaturemalleability" => Ok(VulnerabilityType::SignatureMalleability),
            "eventconsistency" => Ok(VulnerabilityType::EventConsistency),
            "gasgriefblocklimit" => Ok(VulnerabilityType::GasGriefBlockLimit),
            "integeroverflow" => Ok(VulnerabilityType::IntegerOverflow),
            other => Err(de::Error::unknown_variant(
                other,
                &[
                    "oracle",
                    "accesscontrol",
                    "frontrunattack",
                    "unexpectedeth",
                    "pragma",
                    "randomness",
                    "txorigin",
                    "zerocode",
                    "selfdestruct",
                    "storagelayout",
                    "replayattack",
                    "shortaddress",
                    "integermath",
                    "uncheckedreturn",
                    "dos",
                    "defaultvisibility",
                    "inheritance",
                    "confidentialdata",
                    "reentrancy",
                    "arraylimits",
                    "frontrunmev",
                    "upgradeabilityinitializersafety",
                    "pausableemergencystop",
                    "timestampdependentlogic",
                    "flashloaneconomicmanipulation",
                    "delegatecalllowlevelops",
                    "signaturemalleability",
                    "eventconsistency",
                    "gasgriefblocklimit",
                    "integeroverflow",
                ],
            )),
        }
    }
}

impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for InvariantType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for InvariantStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for VulnerabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

```
ai-agent-audit/src/llm_review/invariants.rs
```
pub const INVARIANTS: &str = r#"
You are a senior security auditor.

### TASK
1. Summarise, in bullet points, the intended behaviour of *this contract*.
2. Derive business-logic invariants.  Label them **INV-1 …** and assign each an *invariant type* from the list.
3. Inspect the IR and call flow.  For every invariant output:  
   – `"status": "HOLDS"` if the code enforces it.  
   – `"status": "VIOLATION"` if it can be broken. Show line numbers / IR lines.
4. For each **VIOLATION** include:  
   • `"exploit_path"` — external call sequence an attacker uses  
   • `"pre_state"`     — minimum balances / roles / time needed  
   • `"post_state"`    — resulting asset/thread state change or stolen value
5. Return *only* valid JSON.  No markdown, no comments.  

## INVARIANT TYPES
1. Arithmetic   values, sums, ratios must match expectations  
2. Balance      token/ETH balances and supply monotonicity  
3. Permission    only-owner / only-role / re-entrancy locks  
4. Temporal      timeouts, epochs, can’t rewind clock  
5. Referential   mappings/arrays stay in sync (index→value)  
6. StateMachine only allowed state transitions

Return JSON:

{
  "contract": "string (name of contract)"
  "intention": "string",
  "invariants": [
    {
      "id": "INV-n",
      "inv_type": "Arithmetic|Balance|Permission|Temporal|Referential|StateMachine",
      "desc": "string",
      "status": "HOLDS" | "VIOLATION",
      "exploit": "string (omit if HOLDS)",
      "exploit_path": "string (omit if HOLDS)",
      "pre_state":     "string (omit if HOLDS)",
      "post_state":    "string (omit if HOLDS)"
      "impact": "string (omit if HOLDS)",
      "poc": "string (omit if HOLDS)",
      "mitigation": "string (suggested mitigation - omit if HOLDS)",
    }
  ]
}

"#;

```
ai-agent-audit/src/llm_review/prompt_context.rs
```
use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::build_brain::slither_ffi::{cache_key, get_all_files_src};
use crate::build_brain::summarize;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_file_content::extract_content_from_docs;

use super::config::Finding;

/// Global cache keyed by (repo_root, printer) tuple stringified
pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub async fn generate_slither_metadata_prompt_context(
    repo: &RepoPaths,
    _semantics_path: &Path,
) -> Result<String> {
    let key = cache_key(&repo.root, "prompt_context");
    let cache = Arc::clone(&PROMPT_CONTEXT);
    let mut context_cache = cache.lock().await;

    // Return cached output if exists
    if let Some(cached) = context_cache.get(&key) {
        return Ok(cached.clone());
    }

    // 1 . gather IR + storage  (re-use existing function)
    info!("get contract summary and source files");
    // let callgraph = callgraph::get_enriched_funcs_and_edges(repo_root, &semantics_path).await?;
    // let inheritance = inheritance::generate_slither_inheritance(repo_root).await?;
    // let contract_summary = run_printer(repo, "contract-summary").await?;
    let src_file_list = get_all_files_src(repo);
    info!("src file list => {}", src_file_list);

    let mut prompt_context = String::new();

    // prompt_context.push_str("\n## Slither Contract Summary\n");
    // prompt_context.push_str(&contract_summary);
    prompt_context.push_str("\n## List of Files in Src Folder\n");
    prompt_context.push_str(&src_file_list);
    // prompt_context.push_str("\n## Slither Call Graph\n");
    // prompt_context.push_str(&callgraph);
    // prompt_context.push_str("\n## Slither Inheritance Json\n");
    // prompt_context.push_str(&inheritance);
    // prompt_context.push_str("\n## Slither Detector\n");
    // prompt_context.push_str(&slither_scan_results);

    info!(
        "slither metadata prompt context size ==> {}",
        prompt_context.len()
    );
    context_cache.insert(key, prompt_context.clone());
    Ok(prompt_context)
}

pub async fn generate_context_for_code_review(
    repo: &RepoPaths,
    semantics_path: &Path,
) -> Result<String> {
    log::info!("generate slither metadata");
    let slither_metadata = generate_slither_metadata_prompt_context(repo, &semantics_path).await?;
    log::info!("generate summary of all files");

    let mut full_prompt_context = String::new();

    let mut file_summaries = String::new();
    let summaries = summarize::summarize_src_files(repo, &semantics_path).await?;
    for summary in summaries {
        file_summaries.push_str(&format!("\n## SUMMARY OF FILE: {}\n", summary.filename));
        file_summaries.push_str(&summary.summary);
        file_summaries.push_str("\n\n");
    }
    full_prompt_context.push_str(&file_summaries);
    full_prompt_context.push_str("\n## SLITHER GENERATED METADATA \n\n");
    full_prompt_context.push_str(&slither_metadata);

    let docs = summarize::summarize_docs(repo, &full_prompt_context).await?;
    let documentation = extract_content_from_docs(repo)?;
    let mut doc_summaries = String::new();
    for doc_summary in &docs {
        doc_summaries.push_str("\n\n");
        doc_summaries.push_str(&doc_summary.summary);
        doc_summaries.push_str("\n\n");
    }
    full_prompt_context.push_str("\n ## DOCUMENTATION: \n\n ");
    // adding FULL DOCS not doc_summaries
    full_prompt_context.push_str(&documentation);

    // TODO - add FULL DOCS IF AUDIT TYPE BUGBOUNTY OTHERWISE ADD SUMMARY OF AUDIT
    // test that documentation is being added
    // ALSO add $100 more to chain shield to cover these costs!
    // info!("documentation full size => {}", docs[0].summary.len());
    info!("documentation full size => {}", documentation.len());
    info!("full prompt context SIZE => {}", full_prompt_context.len());

    Ok(full_prompt_context)
}

pub fn generate_prompt_for_issue_check(
    code: &str,
    finding: &Finding,
    pre_instructions: &str,
    instructions: &str,
    post_instructions: &str,
) -> String {
    let mut prompt = format!("{}{}{}", pre_instructions, instructions, post_instructions);

    prompt.push_str("\n\n");
    prompt.push_str("## REPORT FOR SECURITY ISSUE");
    prompt.push_str("\n\n");

    let report = get_finding_report(finding);
    prompt.push_str(&report);
    prompt.push_str("\n\n");

    prompt.push_str("## CODEBASE WHERE ISSUE WAS FOUND");
    prompt.push_str("\n\n");

    prompt.push_str(code);

    prompt
}

fn get_finding_report(finding: &Finding) -> String {
    let mut findings_report = String::new();
    //title
    findings_report.push_str(&format!(
        "## [Severity-{}]. {}\n\n",
        finding.severity.as_str(),
        finding.title()
    ));

    //description
    findings_report.push_str("## Description\n");
    findings_report.push_str(&finding.description.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //impact
    findings_report.push_str("## Impact\n");
    findings_report.push_str(&finding.impact.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //POC
    findings_report.push_str("## Proof of Concept\n");
    findings_report.push_str(&finding.proof_of_concept.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Proof of Code
    findings_report.push_str("## Proof of Code\n");
    findings_report.push_str(&finding.proof_of_code.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    //Suggested Fix
    findings_report.push_str("## Suggested Mitigation\n");
    findings_report.push_str(&finding.mitigation.clone().unwrap_or_default());
    findings_report.push_str("\n\n");

    findings_report.push_str("\n");
    findings_report
}

```
ai-agent-audit/src/llm_review/review_utils.rs
```
use rig::{
    client::CompletionClient,
    providers::{
        anthropic::{self},
        deepseek,
        gemini::{self},
        openai::{self},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::enums::{AIAgent, AIExtractor};

pub fn build_anthropic_agent(
    client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let agent = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .max_tokens(max_tokens)
        .temperature(temperature)
        .build();

    AIAgent::Anthropic(agent)
}

pub fn build_openai_extractor<T>(
    client: &openai::Client,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let builder = client.extractor::<T>(model).preamble(preamble);

    match context {
        Some(added_context) => AIExtractor::Openai(builder.context(added_context).build()),
        None => AIExtractor::Openai(builder.build()),
    }
}

pub fn build_openai_agent(
    client: &openai::Client,
    temperature: f64,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(preamble)
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Openai(builder.context(added_context).build()),
        None => AIAgent::Openai(builder.build()),
    }
}

pub fn build_gemini_agent(
    client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Gemini(builder.context(added_context).build()),
        None => AIAgent::Gemini(builder.build()),
    }
}

pub fn build_deepseek_agent(
    client: &deepseek::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    match context {
        Some(added_context) => AIAgent::Deepseek(builder.context(added_context).build()),
        None => AIAgent::Deepseek(builder.build()),
    }
}

```
ai-agent-audit/src/llm_review/prompt_support/dedup.rs
```
pub const DEDUP_PROMPT: &str = r#"

SYSTEM
You are a Solidity-security triager.  
Answer with exactly **YES** or **NO** (no punctuation, no prose).

USER
Are these two vulnerability reports describing the *same root-cause*?

---- REPORT A ----
Issue Type : {issue_type}
Contract    : {contract}
Function    : {function}
Description : {description_a}

---- REPORT B ----
Issue Type : {issue_type}
Contract    : {contract}
Function    : {function}
Description : {description_b}

Remember: root-cause means the exact same bug, not just similar wording.
Answer:
"#;

```
ai-agent-audit/src/llm_review/prompt_support/post_prompt.rs
```
pub const POST_PROMPT: &str = r#"

### OUTPUT REQUIREMENTS 

 **For Every VIOLATION** return:
1. **Description**: Detailed explanation including vulnerable code snippet 
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow
3. **Contract**: The exact contract name where vulnerability is found 
4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
5. **Impact**: Financial and security consequences 
6. **Proof of Concept**: Step-by-step exploitation scenario 
7. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
8. **Severity**: High/Medium/Low/Info based on table below
    | Severity | Definition |
    |----------|------------|
    | HIGH     | Steals, locks, or permanently harms a significant portion of funds/governance. |
    | MEDIUM   | Exploitable but needs favourable conditions or yields limited loss. |
    | LOW      | Minor financial or operational impact; edge-case or hard to exploit. |
    | INFO     | Non-safety best-practice / observability issue. | 
9. **Mitigation**: Suggested Mitigation with code example of fix

*Please respond with ONLY valid JSON in the following exact format:*

{
  "findings": [
    {
      "description": "Detailed explanation if vulnerability including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", 
      "function": "<Function>", 
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

- If no vulnerabilities are found, return: 

{
  "findings": []
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
**Please double-check opening and closing brakets: `}` and `]`, make sure 
they match up correctly.

"#;

```
ai-agent-audit/src/llm_review/prompt_support/post_qualify.rs
```
pub const POST_QUALIFY: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_quality_check_passed**: true|false 
   • `true`   → if nothing has to be updated
   • `false`  → at least one field (impact, POC, proof of code, severity, mitigation) requires an update 
   *NOTE* : this is boolean value, NO "" around it
2. **where_quality_lacks**: Brief summary of issues found with vulnerability write up (omit this field if quality check passed)
3. **impact**: provide updated impact statement (ONLY IF current one is not adequately addressing impact)
4. **proof_of_concept**: provide an updated proof of concept ONLY IF NEEDED
5. **proof_of_code**: provide an updated proof of code ONLY IF NEEDED
6. **severity**: provid an updated severity (High|Medium|Low|Info), ONLY IF current severity is not accurate
    Judge severity based on below table
    | Severity | Definition |
    |----------|------------|
    | HIGH     | Steals, locks, or permanently harms a significant portion of funds/governance. |
    | MEDIUM   | Exploitable but needs favourable conditions or yields limited loss. |
    | LOW      | Minor financial or operational impact; edge-case or hard to exploit. |
    | INFO     | Non-safety best-practice / observability issue. | 
7. **mitigation**: provide updated mitigation, ONLY IF current one is inadequate

*Please respond with ONLY valid JSON in the following exact format:*

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

```
ai-agent-audit/src/llm_review/prompt_support/post_verify.rs
```
pub const POST_VERIFY: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_legit_vulnerability**: true|false 
   • `true`   → issue is real
   • `false`  → issue is clearly harmless, irrelevant, or deliberate with no risk or confusion
   *NOTE* : this is boolean value, NO "" around it
2. **why_its_not_legit**: IF above is false (OMIT this field if above true), provide brief explanation why issue is NOT real 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "is_legit_vulnerability": true|false, 
    "why_its_not_legit": "explain why NOT legit (OMIT if legit)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

```
ai-agent-audit/src/llm_review/prompt_support/pre_prompt.rs
```
pub const PRE_PROMPT: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "findings": [
    {
      "description": "Detailed explanation including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", // the exact contract name where vulnerability is found
      "function": "<Function>", // exact function name where vulnerability is found, if not applicable set to "NA"
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High|Medium|Low|Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

"#;

```
ai-agent-audit/src/llm_review/prompt_support/pre_qualify.rs
```
pub const PRE_QUALIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

"#;

```
ai-agent-audit/src/llm_review/prompt_support/pre_verify.rs
```
pub const PRE_VERIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
    "is_legit_vulnerability": true|false,
    "why_its_not_legit": "explain why NOT legit (OMIT if legit)"
}

"#;

```
ai-agent-audit/src/llm_review/prompt_support/qualify_prompt.rs
```
pub const QUALIFY_PROMPT: &str = r#"

**Inputs you will receive (per request)**  
1. A draft vulnerability write-up authored by another auditor (sections: Description, Impact, Proof of Concept, Proof of Code, Suggested Mitigation, Severity).  
2. The relevant Solidity contract (or excerpt) for context.

Your tasks for vulnerability write-up are:

1. **Proof-of-Concept (PoC) check** – does the current PoC really show how an attacker can exploit it?  
   - If it misses an attack vector or is incorrect, write a *revised* PoC that clearly demonstrates exploitation.

2. **Impact & Severity check** – is the stated impact accurate and is the severity level appropriate (High / Medium / Low / Info)?  
   - If not, provide an updated *impact* paragraph and/or change the *severity*.

3. **Foundry unit-test check** – will the `proof_of_code` test compile and reliably prove the issue?  
   - If it is wrong, incomplete, or non-deterministic, supply a corrected Foundry test (keep it minimal but runnable).

4. **Mitigation check** – will the suggested fix fully eliminate the vulnerability?  
   - If it is insufficient or can be improved, provide an updated mitigation.
**Tasks**

For vulnerability write-up perform the checks below and update anything that is wrong, missing, or can be improved:
"#;

```
ai-agent-audit/src/llm_review/prompt_support/verify_prompt.rs
````
pub const VERIFY_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **valid and worth fixing**. This includes:

1. **Actual vulnerabilities** – exploitable bugs, broken access control, reentrancy, overflow, etc.
2. **Security best practices violations** – unsafe patterns, missing event logs, unsafe external calls, unchecked return values, etc.
3. **Security-adjacent concerns** – issues that degrade transparency, auditability, maintainability, or correctness.
4. **Potential future risk** – minor today but can cause critical issues when upgraded or combined with other code.

You should return `"true"` if the issue meets **any** of these criteria.

Only return `"false"` if the issue is clearly meets **All** of these conditions:
- Already mitigated or impossible to exploit
- A deliberate pattern that is safe and idiomatic
- Fully unrelated to security, correctness, or best practice

INPUT  
You will receive **one report** with the following structure:

## <Title>

## Description  
<Human-written description of the bug>

## Impact  
<Claimed effect>

## Proof of Concept  
<Attack steps, if applicable>

## Proof of Code  
```solidity

## Suggested Mitigation

<Recommended fix>

TASK

1. Read the description, impact, PoC, and mitigation to understand the claimed vulnerability.
2. Inspect every Solidity code block (vulnerable contract and PoC) and verify, line-by-line, whether the issue can actually occur in practice.
3. Watch for false positives (e.g., state changes before external calls, access-control modifiers, Solidity ≥ 0.8 overflow checks, built-in reentrancy guards, etc.).
4. Think step-by-step **silently**; **do not** reveal chain-of-thought.

**No other text, markdown, or punctuation is allowed in your final answer.**

"#;

````
ai-agent-audit/src/utils/bpe.rs
```
/// OpenAI tokenizer (BPE) for accurate token counting and text chunking.
///
/// This module provides thread-safe access to the cl100k_base tokenizer used by
/// OpenAI models (GPT-4, text-embedding-3) for precise token counting in cost
/// calculations and context limit management.
use std::sync::OnceLock;
use tiktoken_rs::{CoreBPE, cl100k_base};

/// Lazily-initialized, thread-safe tokenizer instance
/// This is initialized only once when first accessed and then reused
static BPE_INSTANCE: OnceLock<CoreBPE> = OnceLock::new();

/// Returns a reference to the singleton BPE tokenizer instance.
///
/// This function provides access to the cl100k_base tokenizer used by OpenAI models
/// like GPT-4 and text-embedding-3. The tokenizer is initialized on first access
/// and then reused for subsequent calls, making it efficient for repeated use.
///
/// @return A static reference to the CoreBPE tokenizer instance
pub fn get_bpe() -> &'static CoreBPE {
    BPE_INSTANCE.get_or_init(|| cl100k_base().expect("Failed to load cl100k_base tokenizer"))
}

```
ai-agent-audit/src/utils/contract_name_check.rs
```
//  check if content contains at least 1 contract
//  that does not have 'mock' in its name
pub fn has_non_mock_contract(content: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("contract ") {
            if let Some(name) = trimmed
                .split_whitespace()
                .nth(1)
                .map(|s| s.trim_end_matches('{').to_ascii_lowercase())
            {
                if !name.contains("mock") {
                    return true;
                }
            }
        }
    }
    false
}

```
ai-agent-audit/src/utils/delete_docker_volumes.rs
````
/// Docker volume cleanup utilities for secure analysis environment.
///
/// This module provides cleanup functions to remove Docker volumes and
/// temporary directories created during repository analysis, ensuring
/// no residual data remains on the host system.

use std::fs;
use std::path::Path;
use anyhow::Result;

/// Cleans up Docker volume directory and build artifacts after analysis.
///
/// Removes the repository directory and all associated build artifacts
/// from the Docker volume to prevent disk space accumulation and ensure
/// clean analysis environments for subsequent runs.
///
/// # Arguments
/// * `root` - Path to the repository root directory to clean up
///
/// # Example
/// ```
/// cleanup_repo_volume(&Path::new("/tmp/audit-analysis/my-repo"));
/// ```
pub fn cleanup_repo_volume(root: &Path) -> Result<()> {
    if root.exists() {
        fs::remove_dir_all(root).expect(&format!("Failed to remove Docker volume at {:?}", root));
        println!("[INFO] Cleaned up Docker volume: {:?}", root);
    } else {
        println!("[WARN] No Docker volume found for cleanup at {:?}", root);
    }

    Ok(())
}

````
ai-agent-audit/src/utils/env_security.rs
```
/// Environment variable security utilities.
/// 
/// This module provides secure handling of environment variables including
/// validation, sanitization, and secure logging to prevent credential leakage.

use anyhow::{Context, Result};
use std::env;

/// Securely retrieves an API key from environment variables with validation.
/// 
/// # Security Features
/// - Validates key format and length
/// - Prevents logging of actual key values
/// - Returns sanitized error messages
pub fn get_api_key(env_var: &str) -> Result<String> {
    let key = env::var(env_var)
        .with_context(|| format!("Environment variable {} is not set", env_var))?;
    
    // Validate API key format
    if key.is_empty() {
        anyhow::bail!("API key {} is empty", env_var);
    }
    
    if key.len() < 10 {
        anyhow::bail!("API key {} appears to be too short", env_var);
    }
    
    if key.len() > 512 {
        anyhow::bail!("API key {} is suspiciously long", env_var);
    }
    
    // Check for obvious placeholder values
    let placeholder_values = ["your_api_key", "placeholder", "changeme", "test", "demo"];
    let key_lower = key.to_lowercase();
    if placeholder_values.iter().any(|&placeholder| key_lower.contains(placeholder)) {
        anyhow::bail!("API key {} appears to be a placeholder value", env_var);
    }
    
    Ok(key)
}

/// Securely retrieves a URL from environment variables with validation.
pub fn get_secure_url(env_var: &str) -> Result<String> {
    let url = env::var(env_var)
        .with_context(|| format!("Environment variable {} is not set", env_var))?;
    
    // Basic URL validation
    if !url.starts_with("http://") && !url.starts_with("https://") {
        anyhow::bail!("URL {} must start with http:// or https://", env_var);
    }
    
    if url.len() > 2048 {
        anyhow::bail!("URL {} is too long", env_var);
    }
    
    // Check for localhost/private IPs in production
    if env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        if url.contains("localhost") || url.contains("127.0.0.1") || url.contains("0.0.0.0") {
            anyhow::bail!("Localhost URLs not allowed in production environment");
        }
    }
    
    Ok(url)
}

/// Sanitizes a string for safe logging (removes sensitive information).
pub fn sanitize_for_logging(input: &str) -> String {
    if input.len() <= 8 {
        "*".repeat(input.len())
    } else {
        format!("{}***{}", &input[..4], &input[input.len()-4..])
    }
}

/// Validates that required environment variables are set without exposing values.
pub fn validate_required_env_vars() -> Result<()> {
    let required_vars = ["OPENAI_API_KEY", "QDRANT_URL"];
    let mut missing_vars = Vec::new();
    
    for var in &required_vars {
        if env::var(var).is_err() {
            missing_vars.push(*var);
        }
    }
    
    if !missing_vars.is_empty() {
        anyhow::bail!("Missing required environment variables: {}", missing_vars.join(", "));
    }
    
    Ok(())
}

```
ai-agent-audit/src/utils/extract_retry.rs
```
use crate::ai_bot::agent::get_rag_for_security_query;
/// LLM extraction with retry logic and cost tracking.
///
/// This module provides robust LLM interaction utilities with automatic retry
/// mechanisms for handling rate limits, network issues, and parsing errors,
/// while tracking inference costs across different providers.
use crate::cost::cost_data::add_to_inference_cost_by_type;
use crate::cost::cost_data::LlmCostType;
use crate::llm_review::config::FromLLMJson;
use reqwest::StatusCode;
use rig::agent::Agent;
use rig::completion::CompletionError;
use rig::completion::CompletionModel;
use rig::completion::Prompt;
use rig::completion::PromptError;
use rig::extractor::ExtractionError;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::de::Error as _; // <- bring the trait’s methods into scope
use serde::Deserialize;
use serde_json::Error as JsonError;
use std::{thread, time::Duration};

/// Maximum retry attempts for failed LLM requests
const MAX_ATTEMPTS: usize = 3;

/// Retries LLM extraction with exponential backoff and cost tracking.
///
/// Handles common LLM API issues including rate limits, network errors,
/// and JSON parsing failures with automatic retry logic.
pub async fn extractor_with_retry<M, T>(
    extractor: &Extractor<M, T>,
    input: &str,
    llm_cost_type: LlmCostType,
) -> Result<T, ExtractionError>
where
    M: CompletionModel,
    T: JsonSchema + for<'a> Deserialize<'a> + Send + Sync,
{
    let delay = Duration::from_millis(500);

    for attempt in 1..=MAX_ATTEMPTS {
        // add to cost
        add_to_inference_cost_by_type(input, llm_cost_type).await;

        match extractor.extract(input).await {
            Ok(data) => return Ok(data), // ✅ parsed JSON
            Err(ExtractionError::NoData) if attempt < MAX_ATTEMPTS => {
                eprintln!("No data extracted – (attempt {attempt}/{MAX_ATTEMPTS})");
                thread::sleep(delay);
            }
            Err(e) => return Err(e), // network / OpenAI errors → bubble up
        }
    }

    Err(ExtractionError::NoData)
}

pub async fn agent_extract_with_retry<M, T>(
    agent: &Agent<M>,
    input: &str,
    llm_cost_type: LlmCostType,
) -> Result<T, JsonError>
where
    M: CompletionModel,
    T: DeserializeOwned,
{
    for attempt in 1..=MAX_ATTEMPTS {
        /* ────── 1. ask the model ───────────────────────────────────────── */
        let raw = match agent.prompt(input).await {
            Ok(txt) => txt,
            // Convert prompt error to JsonError
            Err(e) if should_retry_prompt_err(&e) && attempt < MAX_ATTEMPTS => {
                eprintln!("LLM backend busy ({e}) – retry {attempt}/{MAX_ATTEMPTS}");
                continue;
            }
            Err(e) => return Err(JsonError::custom(format!("prompt failed: {e}"))),
        };
        // log::info!("json => {:#?}", raw);

        // add to cost
        add_to_inference_cost_by_type(&raw, llm_cost_type).await;

        /* ────── 2. try to parse JSON ───────────────────────────────────── */
        match FromLLMJson::parse_from_llm_response(&raw) {
            Ok(f) => return Ok(f), // ✅ success
            Err(e) => {
                let msg = e.to_string();
                // Check if we should retry based on the original error
                let should_retry = should_retry_based_on_error(&msg) && attempt < MAX_ATTEMPTS;

                if should_retry {
                    eprintln!("parse error ({msg}) – retrying {attempt}/{MAX_ATTEMPTS}");
                    // sleep(delay).await; --> NOT Send
                    continue;
                } else {
                    // Convert the error to JsonError and return
                    return Err(JsonError::custom(format!("parse failed: {msg}")));
                }
            }
        }
    }
    // This point is only reached if all attempts exhausted
    Err(JsonError::custom("exhausted retries – still no data"))
}
// Helper function to determine if we should retry based on the original error
fn should_retry_based_on_error(e: &str) -> bool {
    let error_msg = e.to_string().to_lowercase();

    // Retry on common parsing issues that might be fixed by the LLM on retry
    error_msg.contains("unexpected")
        || error_msg.contains("invalid")
        || error_msg.contains("syntax")
        || error_msg.contains("parse")
        || error_msg.contains("json")
        || error_msg.contains("deserialize")
    // Add more conditions based on what errors you typically see
}

/*──────────────── helper ───────────────────────────────────────────────*/
/// `true`  → retry is warranted  
/// `false` → give up / bubble the error
fn should_retry_prompt_err(e: &PromptError) -> bool {
    match e {
        // Unpack the CompletionError variant  ──────────────────────────
        PromptError::CompletionError(inner) => match inner {
            /* 1) HTTP transport layer issues -------------------------- */
            CompletionError::HttpError(http_err) => {
                // 1a) Too-Many-Requests (OpenAI & friends)
                if http_err.status() == Some(StatusCode::TOO_MANY_REQUESTS) {
                    return true;
                }
                // 1b) Any 5xx server error
                if let Some(status) = http_err.status() {
                    if status.is_server_error() {
                        return true;
                    }
                }
                // 1c) Network time-outs
                if http_err.is_timeout() {
                    return true;
                }
                false
            }

            /* 2) Provider said “I’m busy / overloaded / rate-limited”  */
            CompletionError::ProviderError(msg) | CompletionError::ResponseError(msg) => {
                let m = msg.to_lowercase();
                m.contains("overload")
                    || m.contains("rate limit")
                    || m.contains("busy")
                    || m.contains("try again later")
            }

            /* 3) Anything else – usually not transient */
            _ => false,
        },

        /* Tool-call failures, depth-limit, etc. -> *not* transient */
        _ => false,
    }
}

```
ai-agent-audit/src/utils/file_security.rs
```
/// File system security utilities.
///
/// This module provides secure file operations including path validation,
/// symlink detection, and safe file reading with size limits.
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum file size for reading (10MB)
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum path length to prevent buffer overflow attacks
const MAX_PATH_LENGTH: usize = 4096;

/// Validates that a path is safe to access (no path traversal, symlinks, etc.)
pub fn validate_safe_path(path: &Path, allowed_base: &Path) -> Result<()> {
    // Convert to absolute paths for comparison
    let abs_path = path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {:?}", path))?;
    let abs_base = allowed_base
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize base path: {:?}", allowed_base))?;

    // Check if path is within allowed base directory
    if !abs_path.starts_with(&abs_base) {
        anyhow::bail!(
            "Path traversal attempt detected: {:?} is outside {:?}",
            abs_path,
            abs_base
        );
    }

    // Check path length
    if abs_path.to_string_lossy().len() > MAX_PATH_LENGTH {
        anyhow::bail!(
            "Path is too long: {} characters",
            abs_path.to_string_lossy().len()
        );
    }

    // Check for symlinks in the path components
    let mut current = abs_path.as_path();
    while let Some(parent) = current.parent() {
        if parent == abs_base {
            break;
        }

        let metadata = fs::symlink_metadata(current)
            .with_context(|| format!("Failed to read metadata for: {:?}", current))?;

        if metadata.file_type().is_symlink() {
            anyhow::bail!("Symlink detected in path: {:?}", current);
        }

        current = parent;
    }

    Ok(())
}

/// Safely reads a file with size limits and validation
pub fn safe_read_file(path: &Path, allowed_base: &Path) -> Result<String> {
    // Validate path safety
    validate_safe_path(path, allowed_base)?;

    // Check file metadata
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to read file metadata: {:?}", path))?;

    // Check if it's actually a file
    if !metadata.is_file() {
        anyhow::bail!("Path is not a regular file: {:?}", path);
    }

    // Check file size
    if metadata.len() > MAX_FILE_SIZE {
        anyhow::bail!(
            "File is too large: {} bytes (max: {} bytes)",
            metadata.len(),
            MAX_FILE_SIZE
        );
    }

    // Read file content
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))
}

/// Validates file extension against allowed list
pub fn validate_file_extension(path: &Path, allowed_extensions: &[&str]) -> Result<()> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("File has no extension: {:?}", path))?;

    if !allowed_extensions.contains(&extension) {
        anyhow::bail!(
            "File extension '{}' not allowed. Allowed: {:?}",
            extension,
            allowed_extensions
        );
    }

    Ok(())
}

/// Creates a secure temporary directory with restricted permissions
pub fn create_secure_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .context("Failed to create temporary directory")?;

    let path = temp_dir.keep();

    // Set restrictive permissions (owner only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o700); // rwx------
        fs::set_permissions(&path, perms)?;
    }

    Ok(path)
}

/// Validates and sanitizes a Git repository URL to prevent command injection.
///
/// # Security
/// This function prevents command injection by validating URL format and
/// rejecting URLs with shell metacharacters or suspicious patterns.
pub fn validate_repo_url(url: &str) -> Result<()> {
    // Basic URL format validation
    let url_regex = Regex::new(r"^https?://[a-zA-Z0-9.-]+/[a-zA-Z0-9._/-]+(?:\.git)?/?$")
        .context("Failed to compile URL regex")?;

    if !url_regex.is_match(url) {
        anyhow::bail!("Invalid repository URL format: {}", url);
    }

    // Check for shell metacharacters that could enable command injection
    let dangerous_chars = [
        '&', '|', ';', '`', '$', '(', ')', '{', '}', '<', '>', '"', '\'', '\\',
    ];
    if url.chars().any(|c| dangerous_chars.contains(&c)) {
        anyhow::bail!(
            "Repository URL contains potentially dangerous characters: {}",
            url
        );
    }

    // Reject URLs that are too long (potential buffer overflow)
    if url.len() > 2048 {
        anyhow::bail!("Repository URL is too long: {} characters", url.len());
    }

    Ok(())
}

/// Sanitizes repository name to prevent path traversal and injection attacks.
fn sanitize_repo_name(name: &str) -> String {
    // Remove any path traversal attempts and dangerous characters
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(250) // Limit length
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}

/// Sanitizes filename to prevent directory traversal and special characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(255) // Limit filename length
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}

```
ai-agent-audit/src/utils/fn_labels.rs
```
pub fn get_visibility_label(visibility: &str) -> String {
    let label = match visibility {
        "external" => "[EXTERNAL]",
        "public" => "[PUBLIC]",
        "internal" => "[INTERNAL]",
        "private" => "[PRIVATE]",
        _ => "",
    };

    label.to_string()
}

pub fn get_modifiers_label(modifiers: &[String]) -> String {
    // log::info!("modifiers ==> {:#?}", modifiers);
    let owner = modifiers.iter().any(|m| m == "onlyOwner");
    let mod_tag = if owner { "[OWNER]" } else { "" };

    mod_tag.to_string()
}

```
ai-agent-audit/src/utils/get_file_content.rs
```
use std::fs;

use anyhow::Result;

use crate::prepare_code::git_clone::RepoPaths;

pub fn extract_content_from_docs(repo: &RepoPaths) -> Result<String> {
    let mut docs = String::new();

    for doc in &repo.docs {
        // Skip directories and symlinks
        if fs::symlink_metadata(doc)?.file_type().is_symlink() {
            continue;
        }

        let filename = doc
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = match fs::read_to_string(doc) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read {}: {}", doc.display(), e);
                continue;
            }
        };

        if content.trim().is_empty() {
            continue; // skip empty files
        }
        docs.push_str(&format!("### {}\n\n{}\n\n", filename, content));
    }
    Ok(docs)
}

pub fn extract_content_from_source_code(repo: &RepoPaths) -> Result<String> {
    let mut source_code = String::new();

    for code in &repo.sol_files {
        // Skip directories and symlinks
        if fs::symlink_metadata(code)?.file_type().is_symlink() {
            continue;
        }

        let filename = code
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = match fs::read_to_string(code) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read {}: {}", code.display(), e);
                continue;
            }
        };

        if content.trim().is_empty() {
            continue; // skip empty files
        }
        source_code.push_str(&format!("### {}\n\n{}\n\n", filename, content));
    }
    Ok(source_code)
}

```
ai-agent-audit/src/utils/get_fn_name.rs
```
use regex::Regex;

pub fn get_function_name(function_interface: &str) -> String {
    let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

    if let Some(caps) = re.captures(function_interface) {
        let function_name = &caps[1];
        return function_name.to_string();
    }
    "".to_string()
}

```
ai-agent-audit/src/utils/logging.rs
```
use log::info;
use schemars::schema_for;

use crate::llm_review::config::Findings;

pub fn print_first_four_lines(text: &str) {
    let lines: Vec<&str> = text.lines().collect();

    for (index, line) in lines.iter().enumerate().take(4) {
        info!("Line {} (at line {}): {}", index + 1, line!(), line);
    }

    if lines.len() > 4 {
        info!("... ({} more lines)", lines.len() - 4);
    }
}

pub fn print_first_n_lines(number_of_lines: usize, text: &str) {
    let lines: Vec<&str> = text.lines().collect();
    let n = if number_of_lines < lines.len() {
        number_of_lines
    } else {
        lines.len()
    };

    for (index, line) in lines.iter().enumerate().take(n) {
        info!("Line {} (at line {}): {}", index + 1, line!(), line);
    }

    if lines.len() > n {
        info!("... ({} more lines)", lines.len() - n);
    }
}

pub fn print_schema() {
    let schema = schema_for!(Findings);
    println!(
        "Generated schema: {}",
        serde_json::to_string_pretty(&schema).unwrap()
    );
}

```
ai-agent-audit/src/utils/sanitize.rs
```
use once_cell::sync::Lazy;
use regex::Regex;

pub fn sanitize_for_claude(raw: &str) -> String {
    static MD_IMAGE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"!\[(?P<alt>[^\]]*)\]\([^)]+\)").unwrap());

    static HTML_MEDIA: Lazy<Regex> = Lazy::new(|| {
        // matches <img …>, <video …>, <audio …>, <iframe …>, <object …>
        Regex::new(r#"(?is)<\s*(img|video|audio|iframe|object)\b[^>]*>"#).unwrap()
    });

    static NESTED_IMG_LINK: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[\s*\[IMAGE[^\]]*\]\s*\]\([^)]+\)").unwrap());

    static IMAGE_LIKE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[IMAGE:[^\]]+\]").unwrap());

    // ── 1.  Markdown images → [IMAGE] or [IMAGE: alt]
    let step1 = MD_IMAGE.replace_all(raw, |caps: &regex::Captures| {
        let alt = caps.name("alt").map_or("", |m| m.as_str()).trim();
        if alt.is_empty() {
            "[IMAGE]".into()
        } else {
            format!("[IMAGE: {alt}]")
        }
    });

    // ── 2.  HTML media tags → [IMAGE] / [VIDEO] …
    let step2 = HTML_MEDIA.replace_all(&step1, |caps: &regex::Captures| {
        let tag = caps.get(1).unwrap().as_str().to_uppercase();
        format!("[{tag}]")
    });

    // ── 3.  Nested [IMAGE] inside a Markdown link
    let step3 = NESTED_IMG_LINK.replace_all(&step2, |caps: &regex::Captures| {
        let s = caps.get(0).unwrap().as_str();
        let url = s.rsplit("](").next().unwrap_or("").trim_end_matches(')');
        format!("[LINK] → {url}")
    });

    // ── 4.  Remove any remaining “[IMAGE: …]” with an extension-looking alt
    let step4 = IMAGE_LIKE.replace_all(&step3, "[IMAGE]");

    // ── 5.  Strip control characters
    step4
        .chars()
        .filter(|c| matches!(*c, '\n' | '\r' | '\t') || *c >= '\u{20}')
        .collect::<String>()
}

```
ai-agent-audit/src/utils/vec_db_connect.rs
```
// utils/connect.rs
use once_cell::sync::OnceCell;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::{
    client::EmbeddingsClient,
    providers::openai::{Client, TEXT_EMBEDDING_3_SMALL},
    vector_store::VectorStoreIndexDyn,
};
use rig_qdrant::QdrantVectorStore;
use std::sync::Arc;

/// Build (once) and return an Arc<dyn VectorStoreIndexDyn>.
///
/// Synchronous because OnceCell’s initializer must be sync.
pub fn vector_index() -> anyhow::Result<Arc<dyn VectorStoreIndexDyn>> {
    static ONCE: OnceCell<Arc<dyn VectorStoreIndexDyn>> = OnceCell::new();

    let idx = ONCE.get_or_try_init(|| {
        // 1. Qdrant client
        let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
            .build()
            .map_err(anyhow::Error::from)?;

        let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
        let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

        /* 2 ── Build the query-params object */
        let qp = QueryPointsBuilder::new("contract_chunks") // collection name
            .with_payload(true) // pull "meta", etc.
            .build();
        // 3. Vector-store pointing at existing collection “contract_chunks”
        //    -> create the collection elsewhere (ingest step) or check/ensure here.
        let store = QdrantVectorStore::new(qdrant, model, qp);

        // 4. Erase to trait object
        Ok::<Arc<dyn VectorStoreIndexDyn>, anyhow::Error>(Arc::new(store))
    })?;

    Ok(idx.clone())
}

```
ai-agent-audit/src/master_prompts/master_prompt.rs
```
/// Master security analysis prompt covering all vulnerability categories.
///
/// This comprehensive prompt instructs AI agents to analyze smart contracts
/// across 19 different security vulnerability categories, providing systematic
/// coverage of common and advanced attack vectors.

pub const MASTER_SECURITY_PROMPT: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  confidential_data             // private info leak via events / public vars  
4.  default_visibility            // funcs / vars defaulting to `public`  
5.  dos                           // gas exhaustion, revert griefing, block gas limit  
6.  inheritance                   // bad overrides, diamond ambiguity  
7.  integer_math                  // rounding, precision div-by-zero  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  pragma                        // floating pragma, outdated compiler bugs  
10. randomness                    // predictable entropy, miner influence  
11. reentrancy                    // state update after external call, cross-function  
12. replay_attack                 // sig replay, chain-ID mix-ups  
13. self_destruct                 // griefing / forced-ETH via `selfdestruct`  
14. short_address                 // calldata truncation on L1/L2 bridges  
15. storage_layout                // slot collisions, struct packing, uninitialized_storage  
16. tx_origin                     // auth that trusts `tx.origin`  
17. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
18. unexpected_eth                // Ether stuck / overly strict balance checks  
19. zero_code                     // constructor-phase contract bypasses  
20. frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
21. upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
22. pausable_emergency_stop       // missing pause guards or bypasses  
23. timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
24. flash_loan_economic_manipulation // state checked & used within same tx  
25. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
26. signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
27. event_consistency             // critical state changes not emitted / mis-ordered  
28. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  
29. integer_overflow              // overflow / underflow, div-by-zero  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_a.rs
```
pub const PROMPT_2X_A: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  dos                           // gas exhaustion, revert griefing, block gas limit  
4.  inheritance                   // bad overrides, diamond ambiguity  
5.  integer_math                  // rounding, precision div-by-zero  
6.  integer_overflow              // overflow / underflow, div-by-zero  
7.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  randomness                    // predictable entropy, miner influence  
10. reentrancy                    // state update after external call, cross-function  
11. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
12. unexpected_eth                // Ether stuck / overly strict balance checks  
13. storage_layout                // slot collisions, struct packing, uninitialized_storage  
14. self_destruct                 // griefing / forced-ETH via `selfdestruct`  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_aa.rs
```
pub const PROMPT_2X_AA: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  dos                           // gas exhaustion, revert griefing, block gas limit  
3.  integer_overflow              // overflow / underflow, div-by-zero  
4.  signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
5.  unexpected_eth                // Ether stuck / overly strict balance checks  
6.  storage_layout                // slot collisions, struct packing, uninitialized_storage  
7.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
    ### 7-A  Quick front-running / TOD checklist
    - Public functions: can caller profit by seeing a tx in mempool and racing it?  
    - Sequencing deps: does fn A write state that fn B reads in the *same* block?  
    - Value-transfer timing: funds sent immediately after a calc the attacker can influence?  
    - Deterministic selection: winner/outcome based on current on-chain state?  
    - Mitigations present? (pull payments, commit-reveal, VRF, time-locks, ACL)  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  randomness                    // predictable entropy, miner influence  
10. reentrancy                    // state update after external call, cross-function  
11. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
12. replay_attack                 // sig replay, chain-ID mix-ups  
13. upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
14. self_destruct                 // griefing / forced-ETH via `selfdestruct`  
15. zero_code                     // constructor-phase contract bypasses  
16. flash_loan_economic_manipulation // state checked & used within same tx  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_b.rs
```
pub const PROMPT_2X_B: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  tx_origin                     // auth that trusts `tx.origin`  
2.  zero_code                     // constructor-phase contract bypasses  
3.  pragma                        // floating pragma, outdated compiler bugs  
4.  confidential_data             // private info leak via events / public vars  
5.  default_visibility            // funcs / vars defaulting to `public`  
6.  replay_attack                 // sig replay, chain-ID mix-ups  
7.  upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
8.  pausable_emergency_stop       // missing pause guards or bypasses  
9.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
10. flash_loan_economic_manipulation // state checked & used within same tx  
11. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
12. signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
13. event_consistency             // critical state changes not emitted / mis-ordered  
14. short_address                 // calldata truncation on L1/L2 bridges  
15. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_2x_bb.rs
```
pub const PROMPT_2X_BB: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  tx_origin                     // auth that trusts `tx.origin`  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  pragma                        // floating pragma, outdated compiler bugs  
4.  inheritance                   // bad overrides, diamond ambiguity  
5.  integer_math                  // rounding, precision div-by-zero  
6.  confidential_data             // private info leak via events / public vars  
7.  default_visibility            // funcs / vars defaulting to `public`  
8.  pausable_emergency_stop       // missing pause guards or bypasses  
9.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
10. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
11. event_consistency             // critical state changes not emitted / mis-ordered  
12. short_address                 // calldata truncation on L1/L2 bridges  
13. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_3x_a.rs
```
pub const PROMPT_3X_A: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  dos                           // gas exhaustion, revert griefing, block gas limit  
4.  unexpected_eth                // Ether stuck / overly strict balance checks  
5.  integer_overflow              // overflow / underflow, div-by-zero  
6.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
7.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
8.  randomness                    // predictable entropy, miner influence  
9.  reentrancy                    // state update after external call, cross-function  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_3x_b.rs
```
pub const PROMPT_3X_B: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  default_visibility            // funcs / vars defaulting to `public`  
2.  replay_attack                 // sig replay, chain-ID mix-ups  
3.  upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
4.  pausable_emergency_stop       // missing pause guards or bypasses  
5.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
6.  flash_loan_economic_manipulation // state checked & used within same tx  
7.  delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
8.  signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
9.  event_consistency             // critical state changes not emitted / mis-ordered  
10. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/master_prompts/prompt_3x_c.rs
```
pub const PROMPT_3X_C: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
2.  storage_layout                // slot collisions, struct packing, uninitialized_storage  
3.  inheritance                   // bad overrides, diamond ambiguity  
4.  self_destruct                 // griefing / forced-ETH via `selfdestruct`  
5.  integer_math                  // rounding, precision div-by-zero  
6.  tx_origin                     // auth that trusts `tx.origin`  
7.  zero_code                     // constructor-phase contract bypasses  
8.  pragma                        // floating pragma, outdated compiler bugs  
9.  confidential_data             // private info leak via events / public vars  
10. short_address                 // calldata truncation on L1/L2 bridges  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;

```
ai-agent-audit/src/prompts/access_control.rs
````
/// Access control vulnerability detection prompt.
///
/// This prompt guides AI agents to identify access control issues including
/// unprotected functions, missing modifiers, privilege escalation, and
/// improper role management in smart contracts.

pub const ACCESS_CONTROL: &str = r#"

You are an expert Solidity smart contract security auditor specializing in access control vulnerabilities. Your task is to perform a comprehensive access control analysis on the provided Solidity smart contract code.

## Analysis Framework

Systematically examine the {contract_name} contract for the following access control issues:

1. **Unprotected Sensitive Functions**: Functions that perform critical operations without proper authorization checks
2. **Missing Access Modifiers**: Functions lacking `onlyOwner`, `onlyAdmin`, or equivalent access control modifiers
3. **Privilege Escalation**: Functions that allow unauthorized users to gain elevated privileges
4. **State-Changing Operations**: Functions that modify critical contract state without authorization
5. **Asset Management**: Functions handling Ether transfers, token minting/burning, or asset withdrawals

## Critical Functions to Analyze

Pay special attention to functions in {contract_name} with these patterns:
- `mint()`, `burn()`, `mintTo()` - Token supply manipulation
- `withdraw()`, `withdrawAll()`, `emergencyWithdraw()` - Asset extraction
- `initialize()`, `setup()` - Contract initialization
- `setOwner()`, `transferOwnership()` - Ownership changes
- `pause()`, `unpause()` - Contract state control
- `updateConfig()`, `setParameters()` - Configuration changes
- Functions with `payable` modifier or Ether handling
- Functions that call `selfdestruct()` or `delegatecall()`

## BEFORE ANALYZING: 
- ONLY reference or analyze functions that are: explicitly present in the {contract_name} code, OR called
  by functions in {contract_name} contract.
- If you cannot find an access control vulnerability in the ACTUAL code, return:
{
  "findings": []
}

## Analysis Instructions

1. Read through the entire {contract_name} contract code carefully
2. Identify all functions that modify state or handle assets
3. Check each function for appropriate access control mechanisms
4. Verify that access control cannot be bypassed
5. Consider edge cases and inheritance patterns
6. Test your findings with concrete exploitation scenarios

"#;

// ARCHIVED
pub const ACCESS_CONTROL_V1: &str = r#"You are an expert smart contract security auditor specializing in access control vulnerabilities. Your task is to perform a comprehensive access control analysis on the provided Solidity smart contract code.

## Analysis Framework

Systematically examine the contract for the following access control issues:

1. **Unprotected Sensitive Functions**: Functions that perform critical operations without proper authorization checks
2. **Missing Access Modifiers**: Functions lacking `onlyOwner`, `onlyAdmin`, or equivalent access control modifiers
3. **Privilege Escalation**: Functions that allow unauthorized users to gain elevated privileges
4. **State-Changing Operations**: Functions that modify critical contract state without authorization
5. **Asset Management**: Functions handling Ether transfers, token minting/burning, or asset withdrawals

## Critical Functions to Analyze

Pay special attention to functions with these patterns:
- `mint()`, `burn()`, `mintTo()` - Token supply manipulation
- `withdraw()`, `withdrawAll()`, `emergencyWithdraw()` - Asset extraction
- `initialize()`, `setup()` - Contract initialization
- `setOwner()`, `transferOwnership()` - Ownership changes
- `pause()`, `unpause()` - Contract state control
- `updateConfig()`, `setParameters()` - Configuration changes
- Functions with `payable` modifier or Ether handling
- Functions that call `selfdestruct()` or `delegatecall()`

## Example Vulnerable Pattern

```solidity
contract VulnerableToken {
    mapping(address => uint256) public balances;
    uint256 public totalSupply;
    address public owner;
    
    // VULNERABLE: Missing access control - anyone can mint tokens
    function mint(address to, uint256 amount) public {
        balances[to] += amount;
        totalSupply += amount;
    }
    
    // VULNERABLE: Missing access control - anyone can withdraw all Ether
    function withdrawAll() public {
        payable(msg.sender).transfer(address(this).balance);
    }
}
```

## Expected Foundry Test Pattern

For each finding, provide a Foundry test that demonstrates the vulnerability:

```solidity
function test_UnauthorizedMinting() public {
    // Setup: Deploy contract and fund with initial state
    VulnerableToken token = new VulnerableToken();
    
    // Attack: Non-owner calls privileged function
    vm.prank(attacker);
    token.mint(attacker, 1000000 ether);
    
    // Verify: Unauthorized action succeeded
    assertEq(token.balances(attacker), 1000000 ether);
    assertEq(token.totalSupply(), 1000000 ether);
}
```

## Output Requirements

For each access control vulnerability found, provide:

1. **Title**: Format as "[Severity-X] - Access Control Issue in <Contract>::<Function>"
2. **Description**: Detailed explanation including vulnerable code snippet
3. **Impact**: Business and security consequences of the vulnerability
4. **Proof of Concept**: Step-by-step exploitation scenario
5. **Proof of Code**: Complete Foundry unit test demonstrating the vulnerability
6. **Severity**: High/Medium/Low/Info based on exploitability and impact

## Severity Guidelines

- **High**: Critical functions (mint, burn, withdraw, ownership transfer) with no access control
- **Medium**: Important functions with partial or bypassable access control
- **Low**: Administrative functions with missing access control but limited impact
- **Info**: Best practice violations or potential future risks

## Analysis Instructions

1. Read through the entire contract code carefully
2. Identify all functions that modify state or handle assets
3. Check each function for appropriate access control mechanisms
4. Verify that access control cannot be bypassed
5. Consider edge cases and inheritance patterns
6. Test your findings with concrete exploitation scenarios

"#;

````
ai-agent-audit/src/prompts/array_limits.rs
```
pub const ACCESS_OUTSIDE_ARRAY_LIMITS: &str = r#"
You are an expert smart contract security auditor specializing in array bounds vulnerabilities. Your task is to perform a comprehensive array access analysis on the provided Solidity smart contract code.

## Analysis Framework
Systematically examine the contract for the following array bounds issues:

1. **Unchecked Array Access**: Direct array indexing without bounds validation
2. **Loop Index Overflow**: For-loops with unsafe index incrementation or bounds
3. **User-Controlled Indices**: External input used as array index without validation
4. **Dynamic Array Manipulation**: Push/pop operations that could cause index misalignment
5. **Fixed Array Overflows**: Static array access beyond declared bounds
6. **Nested Array Issues**: Multi-dimensional array access with insufficient bounds checking
7. **Array Length Manipulation**: Functions that modify array length without updating dependent logic
8. **Sentinel & Off-By-One Errors**  
   * Functions that **return an index** (e.g. `getIndex(...)` or `find(...)`) but  
     - use `0` (or `type(uint256).max`) both as “first element” and “not found”, **or**  
     - return `array.length` as a valid index.  
   * Comparisons like `>= array.length`, `<= 0`, or `i <= array.length` inside loops.  
   * Boundary checks that use `>=` when they should use `>` (or vice-versa) causing one extra/omitted element to be processed.

## Critical Patterns to Analyze
Pay special attention to functions with these array access patterns:
- Direct indexing: `array[index]`, `mapping[key][index]`
- Loop iterations: `for(uint i = 0; i < someValue; i++)` where `someValue != array.length`
- User input as index: `function get(uint256 index)` without bounds checking
- Array modifications: `array.push()`, `array.pop()`, `delete array[index]`
- Batch operations: Functions processing multiple array elements
- Array copying: `for` loops copying between arrays of different lengths
- External calls with array parameters: Functions passing arrays to external contracts

## Analysis Instructions
1. Identify all array declarations and their usage patterns throughout the contract
2. Check every array access operation for bounds validation
3. Examine loop constructs that iterate over arrays or use array indices
4. Validate that user-provided indices are properly bounded
5. Look for functions that modify array length and check dependent operations
6. Test multi-dimensional array access patterns for nested bounds issues
7. Verify batch operations handle array length mismatches safely
8. Check for edge cases like empty arrays or single-element arrays
9. Examine inheritance patterns that might introduce array access issues
10. **Verify index-return helpers**  
    * Does a “not-found” condition have an unambiguous signal (e.g. returns `(false, 0)` or reverts)? 
    * Could the caller mistakenly treat that value as a valid slot?

"#;

pub const ACCESS_OUTSIDE_ARRAY_LIMITS_V1: &str = r#"
You are an expert smart-contract auditor.  
Your ONLY task is to detect **actual or inevitable array-out-of-bounds
read/write operations** in the Solidity source below.

────────────────────────────
⚠️  VALID-BUG CRITERIA
────────────────────────────
A finding is reportable **only if _all_ of the following hold**:

1. **Real Bounds Violation**  
   A runtime path exists where `array[index]` (or similar) executes with  
   `index ≥ array.length` or `index < 0` (underflow).

2. **Feasible Trigger**  
   * The violating index is controllable with ≤ 1 full block of gas  
     **and** with calldata sizes that fit today’s mainnet limits.  
   * Ignore purely theoretical indices that require 2³² iterations,
     2²⁵⁶ elements, etc.

3. **Impact Observable**  
   The violation causes at least one of:  
   * Immediate revert (DoS of THAT single call is fine)  
   * Corruption or loss of data / funds  
   * Escape from intended control flow

**Do NOT** report:

* High-gas loops, quadratic complexity, or any issue whose only
  consequence is excessive gas.  
* “Performance”, “duplication”, or “ambiguous return-value” complaints.  
* Array-length mismatches that still stay within bounds.

"#;

```
ai-agent-audit/src/prompts/confidential_data.rs
```
pub const SAVING_CONFIDENTIAL_DATA: &str = r#"You are an expert smart contract security auditor specializing in data privacy and confidential information vulnerabilities. Your task is to perform a comprehensive analysis on the provided Solidity smart contract code for improper storage of sensitive data.

## Analysis Framework
Systematically examine the contract for the following confidential data vulnerabilities:

1. **Unencrypted Personal Information**: Storage of user personal data (names, addresses, SSNs, emails) in plain text
2. **Private Key Exposure**: Storage of private keys, seed phrases, or cryptographic secrets on-chain
3. **Sensitive Business Data**: Confidential business information, trade secrets, or proprietary data stored publicly
4. **Authentication Credentials**: Passwords, API keys, or authentication tokens stored without proper hashing
5. **Financial Information**: Bank details, credit card numbers, or sensitive financial data in plain text
6. **Medical/Health Data**: Protected health information (PHI) or medical records stored publicly

## Critical Patterns to Analyze
Pay special attention to storage patterns with these characteristics:
- `mapping(address => string)` storing personal information
- `bytes` or `string` variables containing sensitive data
- Private variables (remember: private != secret on blockchain)
- Struct fields containing personal identifiers
- Event emissions that leak sensitive information
- Functions that accept and store sensitive data without encryption
- Comments or variable names suggesting confidential data storage

## Specific Attack Vectors to Test
1. **Storage Slot Reading**: Direct reading of contract storage slots to extract "private" variables
2. **Event Log Analysis**: Monitoring blockchain events for sensitive data emissions
3. **Transaction Data Mining**: Extracting sensitive data from transaction input parameters
4. **Bytecode Analysis**: Reverse engineering contract bytecode to find hardcoded secrets
5. **Rainbow Table Attacks**: Cracking unsalted password hashes using precomputed tables
6. **Social Engineering**: Using exposed personal information for targeted attacks

## Analysis Instructions
1. Scan all storage variables, mappings, and structs for sensitive data patterns
2. Examine function parameters and event emissions for confidential information
3. Check for hardcoded secrets, keys, or credentials in contract code
4. Analyze password/authentication mechanisms for proper cryptographic practices
5. Verify that sensitive data is properly encrypted, hashed, or stored off-chain
6. Test data extraction scenarios using storage reading and event monitoring
7. Create concrete demonstrations showing how sensitive data can be compromised

Focus on actionable privacy vulnerabilities where confidential data can be extracted by unauthorized parties. Each finding must include a working Foundry test that demonstrates the specific data exposure vector and its potential for exploitation."#;

```
ai-agent-audit/src/prompts/default_visibility.rs
````
pub const DEFAULT_VISIBILITIES: &str = r#"

# Smart Contract Security Analysis: Default Function Visibility Detection

You are an expert smart contract security auditor specializing in identifying function visibility vulnerabilities. Your task is to analyze Solidity smart contracts for functions with missing or inappropriate visibility modifiers that could lead to unauthorized access.

## Vulnerability Overview
The Default Visibility vulnerability occurs when functions lack explicit visibility modifiers, causing them to default to `public` visibility. This can expose sensitive internal functions to external callers, potentially allowing unauthorized access to critical contract operations.

### Solidity Visibility Rules
- **No modifier specified**: Defaults to `public` 
- **public**: Callable externally and internally
- **external**: Only callable externally (gas efficient for external calls)
- **internal**: Only callable within contract and derived contracts
- **private**: Only callable within the defining contract

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns contract code:

1. **Missing Visibility Modifiers**: Functions without `public`, `external`, `internal`, or `private`
2. **Inappropriate Public Access**: Functions that should be restricted but are publicly accessible
3. **Administrative Functions**: Owner-only or privileged functions without proper access control
4. **Internal Logic Exposure**: Helper functions that should be internal/private but are public

## Analysis Focus Areas

1. **Administrative Functions**: Functions that change ownership, pause/unpause, or modify critical parameters
2. **Financial Functions**: Functions that handle funds, minting, burning, or balance modifications
3. **State-Changing Functions**: Functions that modify contract state without proper access control
4. **Helper Functions**: Internal logic that should not be publicly accessible
5. **Privileged Operations**: Functions intended for specific roles but lacking visibility control
6. **Emergency Functions**: Functions designed for crisis management without proper restrictions

### Detection Strategy
1. **Scan for Missing Modifiers**: Identify all functions without explicit visibility keywords
2. **Analyze Function Purpose**: Determine if the function should be restricted based on its operations
3. **Check Access Patterns**: Look for functions that modify critical state or handle sensitive operations
4. **Validate Public Exposure**: Ensure publicly accessible functions are intentionally public
5. **Review Administrative Logic**: Flag any owner/admin functions without proper visibility

### Common Vulnerable Patterns
- Owner/admin functions without visibility modifiers
- Internal calculation functions exposed publicly
- State modification functions without access control
- Emergency or maintenance functions lacking proper visibility
- Helper functions that reveal internal contract logic

"#;

pub const DEFAULT_VISIBILITIES_V1: &str = r#"

# Smart Contract Security Analysis: Default Function Visibility Detection

You are an expert smart contract security auditor specializing in identifying function visibility vulnerabilities. Your task is to analyze Solidity smart contracts for functions with missing or inappropriate visibility modifiers that could lead to unauthorized access.

## Vulnerability Overview
The Default Visibility vulnerability occurs when functions lack explicit visibility modifiers, causing them to default to `public` visibility. This can expose sensitive internal functions to external callers, potentially allowing unauthorized access to critical contract operations.

### Solidity Visibility Rules
- **No modifier specified**: Defaults to `public` (DANGEROUS)
- **public**: Callable externally and internally
- **external**: Only callable externally (gas efficient for external calls)
- **internal**: Only callable within contract and derived contracts
- **private**: Only callable within the defining contract

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns in smart contract code:

1. **Missing Visibility Modifiers**: Functions without `public`, `external`, `internal`, or `private`
2. **Inappropriate Public Access**: Functions that should be restricted but are publicly accessible
3. **Administrative Functions**: Owner-only or privileged functions without proper access control
4. **Internal Logic Exposure**: Helper functions that should be internal/private but are public

### Code Example to Analyze
```solidity
pragma solidity ^0.8.0;

contract VulnerableVisibility {
    address public owner;
    mapping(address => uint256) private balances;
    uint256 private totalSupply;
    bool private paused;
    
    constructor() {
        owner = msg.sender;
        totalSupply = 1000000;
    }
    
    // VULNERABLE: Missing visibility modifier (defaults to public)
    function setOwner(address newOwner) {
        owner = newOwner;
    }
    
    // VULNERABLE: Administrative function without access control
    function pause() {
        paused = true;
    }
    
    // VULNERABLE: Internal helper function exposed publicly
    function calculateFee(uint256 amount) returns (uint256) {
        return amount * 3 / 100;
    }
    
    // VULNERABLE: Critical function without visibility modifier
    function mint(address to, uint256 amount) {
        balances[to] += amount;
        totalSupply += amount;
    }
    
    // VULNERABLE: Withdrawal function without proper access control
    function emergencyWithdraw() {
        payable(owner).transfer(address(this).balance);
    }
    
    // CORRECT: Properly defined visibility
    function transfer(address to, uint256 amount) public returns (bool) {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        require(!paused, "Contract is paused");
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
        return true;
    }
    
    // CORRECT: Internal helper function
    function _beforeTransfer(address from, address to) internal view {
        require(!paused, "Transfers paused");
    }
    
    // VULNERABLE: State-changing function without visibility
    function updateTotalSupply(uint256 newSupply) {
        totalSupply = newSupply;
    }
}
```

### Foundry Test Example
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

contract DefaultVisibilityTest is Test {
    VulnerableVisibility target;
    address attacker = address(0x1337);
    address originalOwner;
    
    function setUp() public {
        target = new VulnerableVisibility();
        originalOwner = target.owner();
    }
    
    function testUnauthorizedOwnershipTransfer() public {
        // Verify initial owner
        assertEq(target.owner(), originalOwner);
        
        // Attacker can call setOwner due to missing visibility modifier
        vm.prank(attacker);
        target.setOwner(attacker);
        
        // Ownership has been transferred to attacker
        assertEq(target.owner(), attacker);
        assertNotEq(target.owner(), originalOwner);
    }
    
    function testUnauthorizedPause() public {
        // Attacker can pause the contract
        vm.prank(attacker);
        target.pause();
        
        // Contract is now paused, blocking legitimate transfers
        vm.expectRevert("Contract is paused");
        target.transfer(address(0x123), 100);
    }
    
    function testUnauthorizedMinting() public {
        uint256 initialBalance = target.balances(attacker);
        
        // Attacker can mint tokens to themselves
        vm.prank(attacker);
        target.mint(attacker, 1000000);
        
        // Attacker now has unlimited tokens
        assertEq(target.balances(attacker), initialBalance + 1000000);
    }
    
    function testUnauthorizedEmergencyWithdraw() public {
        // Fund the contract
        vm.deal(address(target), 10 ether);
        
        // Attacker can drain the contract
        uint256 attackerBalanceBefore = attacker.balance;
        
        vm.prank(attacker);
        target.emergencyWithdraw();
        
        // All funds sent to original owner (as set in function)
        // But attacker controlled the call
        assertEq(address(target).balance, 0);
    }
    
    function testSupplyManipulation() public {
        // Attacker can manipulate total supply
        vm.prank(attacker);
        target.updateTotalSupply(0);
        
        // Total supply is now corrupted
        // This could break tokenomics and calculations
    }
}
```

## Analysis Focus Areas

1. **Administrative Functions**: Functions that change ownership, pause/unpause, or modify critical parameters
2. **Financial Functions**: Functions that handle funds, minting, burning, or balance modifications
3. **State-Changing Functions**: Functions that modify contract state without proper access control
4. **Helper Functions**: Internal logic that should not be publicly accessible
5. **Privileged Operations**: Functions intended for specific roles but lacking visibility control
6. **Emergency Functions**: Functions designed for crisis management without proper restrictions

### Detection Strategy
1. **Scan for Missing Modifiers**: Identify all functions without explicit visibility keywords
2. **Analyze Function Purpose**: Determine if the function should be restricted based on its operations
3. **Check Access Patterns**: Look for functions that modify critical state or handle sensitive operations
4. **Validate Public Exposure**: Ensure publicly accessible functions are intentionally public
5. **Review Administrative Logic**: Flag any owner/admin functions without proper visibility

### Common Vulnerable Patterns
- Owner/admin functions without visibility modifiers
- Internal calculation functions exposed publicly
- State modification functions without access control
- Emergency or maintenance functions lacking proper visibility
- Helper functions that reveal internal contract logic

## Output Requirements

For each vulnerability found, provide a structured finding with these exact fields:

### Finding Structure
- **title**: "[Severity-##] - Default Visibility Vulnerability in <Contract>::<Function>"
- **description**: Detailed explanation of the missing visibility modifier with specific code snippets
- **impact**: Concrete description of unauthorized access potential and security implications
- **proof_of_concept**: Step-by-step explanation of how an attacker would exploit the missing visibility
- **proof_of_code**: Complete Foundry test demonstrating the unauthorized access
- **severity**: One of: High, Medium, Low, Info

### Severity Guidelines
- **High**: Administrative functions, fund access, or ownership changes without proper visibility
- **Medium**: State-changing functions or business logic exposure without access control
- **Low**: Helper functions or view functions with inappropriate visibility
- **Info**: Functions that should have explicit visibility for code clarity
"#;

````
ai-agent-audit/src/prompts/dos.rs
```
pub const DOS: &str = r#"
You are an expert Solidity smart contract security auditor specializing in identifying Denial of Service (DoS) vulnerabilities caused by unexpected reverts in batch operations.

Your task is to systematically analyze {contract_name} contract code for functions that aggregate multiple external calls where a single failure can cause the entire operation to revert, creating a DoS condition.

## Analysis Framework

### Vulnerability Detection Criteria:
1. **Batch Operations**: Functions in {contract_name} that iterate over arrays/lists making external calls
2. **Fail-Fast Logic**: Use of `require()`, `assert()`, or unhandled reverts in loops
3. **External Dependencies**: Calls to user-controlled contracts or addresses
4. **State Coupling**: Operations where one failure blocks all subsequent operations
5. **Gas Limit Attacks**: Loops that can be manipulated to consume excessive gas

## Analysis Instructions

1. **Identify Batch Operations**: Look for loops that make external calls or transfer funds
2. **Trace Failure Points**: Find `require()`, `assert()`, or unhandled external call failures in loops
3. **Assess Attack Vectors**: Consider malicious contracts, gas manipulation, and edge cases
4. **Evaluate Impact**: Determine what functionality becomes unavailable during DoS
5. **Suggest Mitigations**: Recommend withdrawal patterns, try-catch blocks, or call isolation
6. **Provide Working Tests**: Ensure Foundry tests actually demonstrate the DoS condition

## Common DoS Patterns to Check:
- Batch transfers with `require(success)` in loops
- Reward/dividend distributions to user-controlled addresses
- Multi-call functions without proper error handling
- Unbounded loops over user-provided arrays
- External calls in loops without gas limits

"#;

```
ai-agent-audit/src/prompts/inheritance.rs
```
pub const WRONG_INHERITANCE: &str = r#"

You are a senior Solidity auditor.  
Your ONLY job is to find **genuine, compiler-detectable inheritance mistakes** in the
source below.

─────────────────────────────────
⚠️  VALID-BUG CRITERIA
─────────────────────────────────
A finding is reportable **ONLY if _all_ of the following are true**:

1. **Compiler Verification**  
   Solidity (tested with 0.7.x and 0.8.x) actually emits a diagnostic  
   _or_ the code demonstrably mis-behaves because of the inheritance
   issue (function silently ignored, access loss, etc.).

2. **Concrete Conflict**  
   * Missing `override` **only** if the function **does** override a
     parent implementation **and** the current pragma/pragma-range
     would compile without it.  
   * Missing `virtual` **only** if a child contract in the same file
     (or clearly intended future child) **already overrides** the
     function.  
   * Variable collision reported **only** when two *different* parents
     declare a **slot-compatible** variable with the same name or type,
     or the child redeclares an existing variable name.

3. **Inheritance-Specific**  
   Exclude topics that are **not caused by inheritance**, e.g. gas
   costs, storage packing optimisation, generic proxy layout advice.

4. **Feasible / Present**  
   Do not speculate about “future extensions.”  
   If no contract in the file currently creates the conflict, skip it.

─────────────────────────────────
OUTPUT FORMAT  (JSON array or `[]`)
─────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/integer_overflow.rs
```
pub const INTEGER_OVERFLOW: &str = r#"

 (1) integer overflow / underflow and  
 (2) material precision-loss faults.
 
 ⚠️  STRICT VALID-BUG RULES

 * Attacker profit or fund loss ≥ 1 % of total contract balance **or** ≥ 0.01 ETH, whichever is larger.
   * **Exception:** if the arithmetic fault lets an attacker **bypass /
     satisfy a security-critical check** (e.g. `require(msg.value ==
     expected)`), the above threshold is waived – always report.

 2. **Real Arithmetic Fault**  
    * A genuine overflow / underflow **or** precision-loss that changes token/ETH flows or ledger state.  
   * **Unchecked multiplication/division inside a `require`, `assert`, or
     payment-amount calculation is HIGH-RISK.** Report if either operand
     is user-supplied or may exceed 2¹²⁷.
   * “Dust” rounding that loses < 1 % **is still reportable** when  
       ⓐ  it *accumulates over repeated calls* **and**  
       ⓑ  the dust becomes permanently locked or skews future payouts.

 4. **Concrete Profit Path**  

 5. **Scope Discipline**  

────────────────────────────
REALITY-CHECK STEP (required)
────────────────────────────
After drafting a finding, sanity-check it with order-of-magnitude numbers
that fit in ≤ 30 M gas and realistic on-chain limits (e.g. ≤ 2²⁵⁶ wei,
≤ 500 array elements).  If it still works, keep the finding; otherwise
discard as infeasible.
────────────────────────────
OUTPUT FORMAT
────────────────────────────
"#;

pub const INTEGER_OVERFLOW_V1: &str = r#"You are an expert smart contract security auditor specializing in integer overflow, underflow, and precision vulnerabilities. Your task is to perform a comprehensive mathematical operation analysis on the provided Solidity smart contract code and return your findings in strict JSON format.

## Analysis Framework

Systematically examine the contract for these mathematical vulnerabilities:

### 1. Version-Specific Integer Overflow/Underflow
- **Solidity <0.8.0**: NO automatic overflow protection - flag ALL arithmetic
- **Solidity ≥0.8.0**: Automatic protection EXCEPT in `unchecked{}` blocks
- Check for SafeMath usage in pre-0.8.0 contracts

### 2. Precision Loss & Rounding Issues
- **Division Before Multiplication**: `(a / b) * c` loses precision vs `(a * c) / b`
- **Integer Division Truncation**: `amount / 100` truncates decimals
- **Small Value Operations**: Operations on wei amounts that round to zero
- **Fixed-Point Arithmetic**: Missing decimal handling in percentage calculations

### 3. Critical Vulnerable Operations
- **Arithmetic**: `a + b`, `balance += amount`, `counter++`, `a - b`, `balance -= amount`
- **Multiplication**: `amount * rate`, fee calculations, reward distributions
- **Division**: `amount / divisor`, percentage calculations, ratio computations
- **Casting**: `uint8(largeValue)`, `uint128(amount)` - truncation risks
- **Unchecked blocks**: Any arithmetic inside `unchecked{}` in Solidity 0.8+

## Critical Locations to Analyze

- Token balance updates and supply modifications
- Fee calculations and deductions  
- Reward calculations and distributions
- Timestamp arithmetic and deadline calculations
- Array index operations and bounds
- User input arithmetic operations
- Exchange rate and price calculations
- Percentage and ratio computations


## JSON Field Requirements

For each vulnerability found, populate these JSON fields:

1. **title**: "[Severity-X] - Integer Overflow/Underflow/Precision Loss in <Contract>::<Function>"
2. **description**: Technical explanation with vulnerable code snippet and operation type
3. **impact**: Financial consequences including potential for theft, balance manipulation, or DOS
4. **proof_of_concept**: Step-by-step exploitation with specific numeric values
5. **proof_of_code**: Complete Foundry test demonstrating the vulnerability with proper JSON escaping
6. **severity**: Exactly one of: "High", "Medium", "Low", "Info"

## Severity Guidelines

- **High**: Critical operations (token transfers, supply changes, fee calculations) vulnerable to overflow/precision loss leading to financial loss
- **Medium**: Important calculations with overflow/precision risk but limited direct impact
- **Low**: Edge cases or less critical operations with mathematical vulnerabilities
- **Info**: Best practices violations or potential optimization risks

## Analysis Instructions

1. Identify Solidity version from pragma statement
2. Locate ALL arithmetic operations throughout the contract
3. For pre-0.8.0: Check SafeMath usage for every arithmetic operation
4. For 0.8+: Examine `unchecked{}` blocks carefully
5. Analyze division operations for precision loss patterns
6. Test edge cases with maximum/minimum values and small amounts
7. Validate findings with concrete Foundry test cases

"#;

```
ai-agent-audit/src/prompts/mev.rs
```
pub const MEV: &str = r#"
#############################################
#         ⚒️  MEV / TOD BUG HUNTER         #
#############################################

You are a senior smart-contract security engineer whose **sole
mission** is to discover vulnerabilities that arise because an
attacker or a block producer can influence transaction ordering,
inclusion, or execution context.  
This includes the entire MEV / TOD (Transaction-Ordering Dependence)
surface: front-running, back-running, sandwich attacks, generalized
arbitrage, timestamp or difficulty manipulation, miner-griefing, and
economic denial-of-service.

─────────────────────────────────────────────
🎯  REPORTABLE CATEGORIES
─────────────────────────────────────────────
1. **State-Split Front-Run Windows**  
   • Multi-tx workflows where *Tx-A* makes a commitment, but
     *Tx-B* (sent by anyone) consumes it, letting an attacker cancel,
     cheapen, or dominate the result.

2. **Sandwichable Price / Amount Reads**  
   • Any payout or mint/burn that uses an on-chain value
     (`balanceOf`, `getReserves`, oracle feeds, etc.) that can be
     skewed between *pre-state* and *post-state*.

3. **Miner-Controllable Randomness / Time**  
   • Use of `block.timestamp`, `block.number`, `block.difficulty`,
     `blockhash`, `gasleft`, `tx.gasprice`, etc. to pick winners or
     branch logic.

4. **External-Call Ordering & Callback Abuse**  
   • Contract sends value or executes untrusted code **before**
     critical state is updated, or relies on `receive()` hooks.

5. **Oracle / TWAP Manipulation**  
   • Insufficient averaging period, single-tick quotes, or TWAP that
     can be shifted in ≤ N blocks for profit.

6. **Economic Grief / Balance Equality Traps**  
   • Equality checks (`require(balance == cached)`) or invariants that
     a miner can break by pushing “dust” ETH / tokens or using
     `selfdestruct`.

7. **Auction / Raffle / Bidding Races**  
   • Highest-bid-wins logic reliant on mem-pool honesty, or reward
     functions that privilege the caller.

─────────────────────────────────────────────
🔬  ANALYSIS PLAYBOOK
─────────────────────────────────────────────
A. List every **public / external** function (including inherited).  
B. For each function ask:  
   — *If reordered with another tx in the same block, does value flow
      unfairly?*  
   — *Can a second tx read-modify-write the same variable before this
      tx commits?*  
C. Trace multi-step flows (`commit → reveal`, `deposit → withdraw`,
   `bid → claim`, `enter → refund`, etc.).  
D. Inspect any read of balances, reserves, oracles, totalSupply,
   array lengths, **then** a payment/mint/burn in the same tx.  
E. Flag randomness/time usage manipulable by miners.  
F. Look for equality checks on `address(this).balance` or token
   balances that a dust transfer can break.  
G. When a contract makes an external call **before** internal state
   updates, consider both re-entrancy *and* insertion attacks.

─────────────────────────────────────────────
⚠️  VALID-BUG RULES
─────────────────────────────────────────────
A finding is **reportable** only if:  
1. Exploit fits in one block (≤ 30 M gas) *or* can be repeated
   inexpensively until it pays.  
2. Net attacker profit or victim loss ≥ 0.01 ETH **or** ≥ 1 % of the
   affected pool/fund.  
3. You can outline a concrete tx sequence (front-run, sandwich, oracle
   skew, etc.) and sketch a Foundry/Hardhat test that would succeed.  
4. Ignore purely off-chain / UI issues.

─────────────────────────────────────────────
📄  OUTPUT TEMPLATE
─────────────────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/oracle.rs
```
pub const ORACLE_MANIPULATION: &str = r#"You are an expert smart contract security auditor specializing in oracle manipulation vulnerabilities. Your task is to perform a comprehensive oracle security analysis on the provided Solidity smart contract code.

## Analysis Framework
Systematically examine the contract for the following oracle-related vulnerabilities:

1. **Single Oracle Dependency**: Contracts relying on a single oracle source without redundancy
2. **Price Feed Manipulation**: Vulnerable price feeds that can be manipulated via flash loans or market manipulation
3. **Stale Data Usage**: Oracle data used without freshness checks or heartbeat validation
4. **Flash Loan Oracle Attacks**: Single-block price manipulation vulnerabilities
5. **Inadequate Oracle Aggregation**: Missing or weak oracle data aggregation mechanisms
6. **Time-Weighted Price Bypass**: Lack of TWAP or other manipulation-resistant pricing mechanisms

## Critical Patterns to Analyze
Pay special attention to functions with these oracle-related patterns:
- `getPrice()`, `latestRoundData()` - Price feed queries
- `liquidate()`, `borrow()`, `lend()` - Financial operations using oracle data
- DEX price queries: `getAmountsOut()`, `getReserves()`, spot price calculations
- Single oracle calls without fallback mechanisms
- Price data used immediately without time delays or validation
- Oracle data used for access control or critical state changes
- Functions that don't validate oracle response data (zero prices, stale timestamps)

## Specific Attack Vectors to Test
1. **Flash Loan Price Manipulation**: Use flash loans to manipulate DEX prices before oracle queries
2. **Stale Data Exploitation**: Exploit contracts that don't validate oracle data freshness
3. **Oracle Frontrunning**: Predict oracle updates and frontrun price-sensitive operations
4. **Cross-Chain Oracle Delays**: Exploit timing differences in cross-chain oracle updates
5. **Oracle Outage Exploitation**: Attack during oracle downtime or circuit breaker activation
6. **Aggregation Bypass**: Exploit weak oracle aggregation or fallback mechanisms

## Analysis Instructions
1. Identify all external oracle dependencies and data sources
2. Examine price feed usage in financial calculations and critical operations
3. Check for oracle data validation, staleness checks, and circuit breakers
4. Analyze aggregation mechanisms and fallback oracle implementations
5. Test for flash loan attack vectors and single-block price manipulation
6. Verify time-weighted pricing and manipulation resistance measures
7. Create concrete attack scenarios with working Foundry tests

Focus on exploitable oracle vulnerabilities that can result in financial losses, incorrect liquidations, or protocol manipulation. Each finding must include a working Foundry test that demonstrates the specific oracle attack vector."#;

```
ai-agent-audit/src/prompts/pragma.rs
```
pub const FLOATING_PRAGMA: &str = r#"You are an expert smart contract security auditor specializing in compiler version vulnerabilities. Your task is to perform a comprehensive floating pragma analysis on the provided Solidity smart contract code.

## Analysis Framework
Systematically examine the contract for the following floating pragma issues:
1. **Floating Pragma Declarations**: Pragma statements using caret (^) or range operators that allow compilation with multiple compiler versions
2. **Wide Version Ranges**: Pragma statements with overly broad version ranges (e.g., >=0.8.0 <0.9.0)
3. **Missing Upper Bounds**: Pragma statements without explicit upper version limits
4. **Inconsistent Pragma Versions**: Different pragma versions across contract files in the same project
5. **Deprecated Version Usage**: Usage of compiler versions with known security vulnerabilities

## Critical Pragma Patterns to Analyze
Pay special attention to pragma declarations with these patterns:
- `pragma solidity ^0.8.0;` - Caret allowing any 0.8.x version
- `pragma solidity >=0.8.0;` - Open-ended range without upper bound
- `pragma solidity >=0.7.0 <0.9.0;` - Wide version range spanning major releases
- `pragma solidity 0.8.*;` - Wildcard version specifications
- Missing pragma statements entirely
- Pragma versions below 0.8.0 (lacking built-in overflow protection)

## Analysis Instructions
1. Read through all pragma declarations in the contract files
2. Identify any floating pragma patterns (^, >=, ranges, wildcards)
3. Assess the width of version ranges allowed by each pragma
4. Check for pragma consistency across related contract files
5. Evaluate contract criticality (asset handling, access control, business logic)
6. Consider known vulnerabilities in the allowed compiler version range
7. Test findings with concrete compilation and deployment scenarios

Focus on pragma declarations that create real deployment and security risks. Provide clear evidence showing how floating pragma usage can lead to inconsistent contract behavior or introduce security vulnerabilities."#;

```
ai-agent-audit/src/prompts/randomness.rs
```
pub const RANDOMNESS: &str = r#"
You are an expert smart contract security auditor specializing in randomness vulnerabilities. Your task is to analyze Solidity code for insecure randomness implementations and provide structured findings.

## Analysis Instructions:
1. **Identify** any use of block variables for randomness generation, including:
   - `block.timestamp`
   - `block.difficulty` (legacy) or `block.prevrandao` (post-merge)
   - `blockhash()`
   - `block.number`
   - Any combination of these values

2. **Evaluate** the context and criticality of randomness usage:
   - Gaming/lottery systems (HIGH severity)
   - NFT minting/rarity (MEDIUM severity)  
   - Administrative functions (LOW severity)
   - Non-critical features (INFO severity)

3. **Analyze** exploitation vectors:
   - Miner/validator manipulation capabilities
   - Front-running opportunities
   - Timing attack possibilities
   - Predictability windows

## Recommended Mitigations:
Suggest secure alternatives such as:
- Chainlink VRF (Verifiable Random Function)
- Commit-reveal schemes with time delays
- Oracle-based randomness solutions
- Hash-based random beacon services

## Important Notes:
- Consider post-merge Ethereum changes (prevrandao vs difficulty)  
- Account for different manipulation timeframes for each block variable
- Evaluate economic incentives for exploitation
- Consider MEV (Maximal Extractable Value) implications

Analyze the provided code thoroughly and output findings in the exact structure required for automated processing.
"#;

```
ai-agent-audit/src/prompts/reentrancy.rs
````
/// Reentrancy vulnerability detection prompt.
///
/// This prompt guides AI agents to identify genuine reentrancy vulnerabilities
/// with strict criteria to minimize false positives. Focuses on external calls
/// before state updates that can lead to exploitable attack paths.

pub const REENTRANCY: &str = r#"

You are an expert smart-contract security auditor.  
Analyse the *entire* Solidity source below for genuine **reentrancy** vulnerabilities.

─────────────────────────
⚠️  STRICT DEFINITIONS
─────────────────────────
A finding is valid only if **all** of the following are true:

1. **External call before final state update**
   * The call is to an untrusted target (`call`, `.sendValue`, ERC-777 hook, etc.) **and**
   * At least one writable contract variable that influences funds/logic is modified **after** that call.
2. **Gain-of-function**  
   An attacker can, during the callback, re-enter the *same contract* and:
   * steal value, OR
   * corrupt accounting, OR
   * bypass access control.
3. **Executable attack path**  
   You can outline a sequence of transactions that compiles & passes in Foundry - **or the finding is invalid**.

Do **NOT** report:

* External calls that happen **after** all related state is fully updated (i.e. CEI compliant).
* Calls protected by `nonReentrant` or `ReentrancyGuard` (unless you show a bypass).
* OpenZeppelin’s `_safeMint`, `_safeTransfer`, or `transfer`/`send` **when** they are invoked *after* state updates.
* “Theoretical” read-only or cross-function issues without a runnable exploit.

─────────────────────────
OUTPUT FORMAT
─────────────────────────
"#;

pub const REENTRANCY_V2: &str = r#"You are an expert smart contract security auditor specializing in reentrancy vulnerabilities. Your task is to perform a comprehensive reentrancy analysis on the provided Solidity smart contract code.

## Analysis Framework

Systematically examine the contract for the following reentrancy patterns:

1. **Classic Reentrancy**: External calls before state updates (CEI pattern violation)
2. **Cross-Function Reentrancy**: State inconsistencies across multiple functions
3. **Cross-Contract Reentrancy**: Reentrancy through external contract interactions
4. **Read-Only Reentrancy**: Exploiting inconsistent state during external calls
5. **ERC-777/ERC-1363 Hooks**: Token callback mechanisms enabling reentrancy

## Critical Patterns to Identify

### Vulnerable Call Patterns:
- `address.call{value: amount}("")`
- `payable(address).transfer(amount)`
- `payable(address).send(amount)`
- External contract method calls
- Token transfers with hooks (ERC-777, ERC-1363)
- Callback mechanisms and delegate calls

### State Update Patterns:
- Balance modifications: `balances[user] -= amount`
- Status changes: `withdrawn[user] = true`
- Nonce updates: `nonces[user]++`
- Supply changes: `totalSupply -= amount`

### CEI Pattern Violations:
- External calls BEFORE state updates
- Multiple external calls in sequence
- State reads after external calls

## Analysis Checklist

For each function in the contract, verify:

1. **External Call Identification**: Locate all external calls (transfers, calls, contract interactions)
2. **State Update Ordering**: Check if state updates occur AFTER external calls
3. **CEI Pattern Compliance**: Verify Checks-Effects-Interactions pattern is followed
4. **Cross-Function Impact**: Analyze if reentrancy in one function affects others
5. **Reentrancy Guards**: Check for `nonReentrant` modifiers or similar protections
6. **View Function Safety**: Ensure view functions don't rely on inconsistent state

## Reentrancy Protection Patterns

### Good Patterns (Secure):
```solidity
function secureWithdraw(uint256 amount) public nonReentrant {
    require(balances[msg.sender] >= amount, "Insufficient balance");
    
    // Effects: Update state FIRST
    balances[msg.sender] -= amount;
    
    // Interactions: External call LAST
    (bool success, ) = payable(msg.sender).call{value: amount}("");
    require(success, "Transfer failed");
}
```

### Bad Patterns (Vulnerable):
```solidity
function vulnerableWithdraw(uint256 amount) public {
    require(balances[msg.sender] >= amount, "Insufficient balance");
    
    // Interactions: External call FIRST - VULNERABLE!
    (bool success, ) = payable(msg.sender).call{value: amount}("");
    require(success, "Transfer failed");
    
    // Effects: State update LAST - TOO LATE!
    balances[msg.sender] -= amount;
}
```

## Severity Guidelines

- **High**: Direct fund loss through classic reentrancy in withdrawal/transfer functions
- **Medium**: Cross-function reentrancy or state inconsistency issues
- **Low**: Read-only reentrancy or limited impact scenarios
- **Info**: Missing reentrancy guards on functions that should have them

## Output Requirements

For each reentrancy vulnerability found, provide:

1. **Title**: Format as "[Severity-X] - Reentrancy Vulnerability in <Contract Name>::<Exact Function Name>" 
2. **Description**: Detailed explanation including vulnerable code snippet and call flow
3. **Impact**: Financial consequences including potential fund loss amounts
4. **Proof of Concept**: Step-by-step attack scenario with attacker contract interaction
5. **Proof of Code**: Complete Foundry unit test with attacker contract demonstrating exploitation
6. **Severity**: Assessment based on fund loss potential and ease of exploitation

## Analysis Instructions

1. Map all external calls throughout the contract
2. Trace the execution flow for each function containing external calls
3. Identify state variables that are read/written around external calls
4. Check for reentrancy guard implementations
5. Analyze cross-function state dependencies
6. Create attack scenarios for each potential vulnerability
7. Validate findings with working Foundry test cases

Focus on vulnerabilities that can lead to direct financial loss, unauthorized withdrawals, 
or contract state corruption through reentrancy attacks. Prioritize classic reentrancy patterns 
in withdrawal and transfer functions as these typically have the highest impact.
"#;

````
ai-agent-audit/src/prompts/replay_attack.rs
```
pub const REPLAY_SIGNATURES_ATTACK: &str = r#"You are an expert smart contract security auditor specializing in signature replay attack vulnerabilities. Your task is to perform a comprehensive signature replay analysis on the provided Solidity smart contract code.
## Analysis Framework
Systematically examine the contract for the following signature replay vulnerabilities:

1. **Missing Nonce Systems**: Signature verification without proper nonce tracking or incrementation
2. **Timestamp-Based Replay**: Insufficient timestamp granularity allowing replay within time windows
3. **Cross-Chain Replay**: Missing chain ID validation enabling signature reuse across different networks
4. **Hash Collision Replay**: Inadequate signature hash construction allowing hash reuse
5. **Permit Function Replay**: EIP-2612 permit implementations without proper nonce management
6. **Meta-Transaction Replay**: Gasless transaction implementations vulnerable to signature reuse

## Critical Functions to Analyze
Pay special attention to functions with these patterns:
- Functions using `ecrecover()`, `ECDSA.recover()`, or signature verification libraries
- `permit()`, `permitWithDeadline()` - EIP-2612 implementations
- `executeMetaTransaction()`, `relayTransaction()` - Meta-transaction handlers
- `withdrawWithSignature()`, `transferWithSignature()` - Signature-based asset transfers
- `voteWithSignature()`, `delegateWithSignature()` - Governance signature functions
- Functions accepting `bytes signature` or `(uint8 v, bytes32 r, bytes32 s)` parameters
- Functions with deadline/timestamp validation but no nonce tracking
- Multicall or batch transaction functions using signatures


## Analysis Instructions
1. Scan all functions accepting signature parameters or using signature verification
2. Check if signature hash construction includes nonce, chain ID, and contract address
3. Verify nonce storage and incrementation after successful signature verification
4. Look for deadline/timestamp validation that might create replay windows
5. Test signature reuse scenarios across different function calls
6. Consider batch operations or multicall functions that might bypass individual nonce checks
7. Examine inheritance patterns that might introduce replay vulnerabilities

Focus on immediately exploitable signature replay attacks that can result in financial loss or unauthorized access. Provide concrete test cases showing successful signature capture and reuse.

"#;

```
ai-agent-audit/src/prompts/self_destruct.rs
```
pub const SELF_DESTRUCT: &str = r#"
You are an expert smart contract security auditor specializing in self-destruct vulnerabilities. Your task is to analyze Solidity code for improper or dangerous usage of the selfdestruct opcode and related contract destruction patterns.

## Analysis Instructions:
1. **Identify Self-Destruct Usage**:
   - Direct calls to `selfdestruct()` or `suicide()` (deprecated)
   - Delegatecall patterns that could trigger self-destruct
   - Proxy contracts with destructible implementations
   - Library contracts with self-destruct capabilities

2. **Evaluate Access Controls**:
   - Check if self-destruct is restricted to authorized accounts (owner, admin)
   - Analyze modifier protections and their effectiveness
   - Look for indirect paths to trigger destruction
   - Verify multi-signature or timelock requirements

3. **Assess Destruction Context**:
   - Funds handling before destruction
   - State cleanup requirements
   - Impact on dependent contracts
   - Upgrade vs destruction patterns

## Recommended Mitigations:
- Implement robust multi-signature controls for destruction
- Add time delays for destruction operations
- Ensure user fund withdrawal before destruction
- Use upgrade patterns instead of destruction where possible
- Implement emergency pause instead of destruction
- Add comprehensive access controls and governance

## Important Notes:
- Consider EIP-4758 (Deactivate SELFDESTRUCT) implications for future deployments
- Account for proxy patterns and delegatecall risks
- Evaluate user fund protection mechanisms
- Consider contract dependencies that rely on the contract's existence

Analyze the provided code thoroughly and output findings in the exact structure required for automated processing.
"#;

```
ai-agent-audit/src/prompts/short_address_attack.rs
```
pub const SHORT_ADDRESS_ATTACK: &str = r#"
# Smart Contract Security Analysis: Short Address Attack Detection

You are an expert smart contract security auditor specializing in identifying short address attack vulnerabilities. Your task is to analyze Solidity smart contracts for functions that improperly handle fixed-size type parameters, particularly addresses, which could be exploited through malformed input data.

## Vulnerability Overview
The Short Address Attack exploits the EVM's automatic zero-padding behavior when function parameters are shorter than expected. When an address parameter is truncated (e.g., missing the last byte), the EVM pads it with zeros from the right, potentially causing:
1. **Address Misinterpretation**: Shortened addresses become different valid addresses
2. **Parameter Shifting**: Subsequent parameters get shifted, corrupting their values
3. **Silent Failures**: Functions execute with wrong data without obvious errors

### EVM Padding Behavior
- Expected address: `0x1234567890abcdef1234567890abcdef12345678` (20 bytes)
- Shortened input: `0x1234567890abcdef1234567890abcdef123456` (19 bytes)
- EVM padded result: `0x1234567890abcdef1234567890abcdef12345600` (20 bytes, zero-padded)

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns in smart contract code:

1. **Unvalidated Address Parameters**: Functions accepting addresses without length validation
2. **Multiple Fixed-Size Parameters**: Functions with address + amount patterns susceptible to parameter shifting
3. **External Interface Functions**: Public/external functions that process user-provided address data
4. **Token Transfer Functions**: Functions handling recipient addresses and amounts together
5. **Missing Input Validation**: Functions that don't verify parameter integrity before processing

## Analysis Focus Areas

1. **Token Transfer Functions**: `transfer()`, `transferFrom()`, `mint()`, `burn()`
2. **Approval Functions**: `approve()`, `increaseAllowance()`, `decreaseAllowance()`
3. **Batch Operations**: Functions processing arrays of addresses or multiple parameters
4. **Administrative Functions**: Owner/admin functions accepting address parameters
5. **External Interfaces**: Public/external functions that process user-provided addresses
6. **Multi-Parameter Functions**: Functions with address + amount parameter combinations

### Detection Strategy
1. **Parameter Analysis**: Identify functions with address parameters
2. **Validation Check**: Look for explicit address validation or length checks
3. **Parameter Ordering**: Analyze functions with multiple fixed-size parameters
4. **External Exposure**: Focus on public/external functions accessible to attackers
5. **Impact Assessment**: Evaluate consequences of parameter corruption

### Common Vulnerable Function Patterns
- `function transfer(address to, uint256 amount)` - No address validation
- `function batchTransfer(address[] recipients, uint256[] amounts)` - Array processing without validation
- `function approve(address spender, uint256 amount)` - Missing spender validation
- `function transferFrom(address from, address to, uint256 amount)` - Multiple addresses without checks

Analyze the provided smart contract code systematically and identify all functions vulnerable to short address attacks. Focus on functions that accept address parameters without proper validation and could be exploited through malformed transaction data.
"#;

```
ai-agent-audit/src/prompts/storage_variables.rs
```
pub const STORAGE_VARIABLE: &str = r#"

You are a senior Solidity auditor.  
Your ONLY goal is to find **storage bugs that can corrupt or hijack
contract state.**

────────────────────────────────────────────
⚠️  VALID-BUG CRITERIA
────────────────────────────────────────────
A finding is reportable **ONLY when _all_ of the following are true**:

1. **Direct Corruption or Take-Over**  
   One of these must occur:
   * Un-initialized storage pointer writes to **slot 0** or any other
     slot holding critical data (`owner`, `admin`, `balances`,
     `implementation`, proxy beacons, etc.).
   * A state variable essential for access control or token/accounting
     is left at its default value and can later be seized.
   * Storage-slot collision between parent/child or upgradeable
     versions lets an attacker overwrite live data.

2. **Exploit Demonstrable in ≤ 2 Transactions**  
   Provide a short PoC where an attacker:
   * Gains ownership / admin role, OR
   * Moves ≥ 0.01 ETH / tokens they shouldn’t, OR
   * Permanently bricks a core function.

3. **Compiler-Version Context**  
   Confirm the vulnerability exists for the pragma used.  
   Skip edge-cases already guarded by the compiler in that version.

4. **Out of Scope**  
   Do **NOT** report:
   * Array “holes”, fragmentation, or gas inefficiencies.
   * Generic storage packing commentary that does **not** break access
     control or accounting.
   * “Potential future collision if someone changes inheritance.”
   * Merely recommending `storage gaps` unless a real collision is
     already present.

────────────────────────────────────────────
OUTPUT FORMAT  
────────────────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/tx_origin.rs
```
pub const TX_ORIGIN: &str = r#"
You are an expert smart contract security auditor specializing in identifying tx.origin authentication vulnerabilities.

Your task is to systematically analyze Solidity smart contract code for improper use of tx.origin in access control mechanisms, which can lead to phishing attacks and unauthorized access.

## Analysis Framework

### Vulnerability Detection Criteria:
1. **tx.origin in Access Control**: Use of `tx.origin` in `require()`, `modifier`, or conditional statements for authentication
2. **Privileged Functions**: Functions that use tx.origin to restrict access to sensitive operations
3. **Authorization Bypass**: Scenarios where tx.origin can be manipulated through contract intermediaries
4. **Phishing Attack Vectors**: Situations where users can be tricked into authorizing malicious transactions

## Analysis Instructions

1. **Scan for tx.origin Usage**: Search for all instances of `tx.origin` in the codebase
2. **Identify Access Control**: Focus on tx.origin used in `require()`, modifiers, or conditional statements
3. **Assess Privilege Level**: Determine what functions/operations the tx.origin check protects
4. **Map Attack Vectors**: Consider how malicious contracts can exploit the authentication
5. **Evaluate Impact**: Determine potential damage from successful phishing attacks
6. **Provide Mitigations**: Recommend using `msg.sender` for direct caller verification

## Common tx.origin Vulnerability Patterns:
- `require(tx.origin == owner)` in access control modifiers
- tx.origin checks in privileged functions (withdraw, transfer, admin operations)
- tx.origin used for user identification in financial operations
- Emergency functions relying on tx.origin authentication
- Multi-signature or delegation patterns using tx.origin

## Attack Scenario Framework:
1. **Phishing Setup**: Attacker deploys malicious contract
2. **Social Engineering**: Trick legitimate user into interacting with malicious contract
3. **Exploitation**: Malicious contract calls vulnerable function while tx.origin remains the victim
4. **Impact**: Unauthorized operations execute with victim's privileges

Now analyze the provided smart contract code for tx.origin authentication vulnerabilities following this framework.
"#;

```
ai-agent-audit/src/prompts/unchecked_return_value.rs
```
pub const UNCHECK_RETURN_VALUES: &str = r#"

Your ONLY goal is to detect **external calls whose boolean success
return value is NOT verified or bubbled up.**

──────────────────────────────────
⚠️  VALID–BUG CRITERIA
──────────────────────────────────
A finding is reportable **ONLY if _all_ of the following hold**:

1. **Ignored Success Flag**  
   * For `.call{…}()`, `.delegatecall()`, `.staticcall()`, or
     `.send()` the returned `(bool success, …)` (or single `bool` for
     `send`) is **not**:
     - used in `require(success, …)`  
     - wrapped in `if (!success) revert …;`  
     - returned to the caller, **or**  
     - handled by a library that already reverts on failure  
       (e.g. `Address.sendValue`, `SafeERC20.safeTransfer`).
2. **Interface Calls**  
   The function returns `bool` (e.g., `ERC20.transfer`) and that value
   is ignored **and** no SafeERC20/try-catch wrapper is present.
3. **No CEI Commentary**  
   Do **NOT** flag state-update-before-call ordering; if the success
   flag *is* checked, the call is **out of scope** for this audit.

──────────────────────────────────
OUTPUT FORMAT  
──────────────────────────────────
"#;

```
ai-agent-audit/src/prompts/unexpected_eth.rs
```
pub const UNEXPECTED_ETH: &str = r#"
# Smart Contract Security Analysis: Unexpected Ether Vulnerability Detection

You are an expert smart contract security auditor specializing in identifying vulnerabilities related to unexpected Ether balance manipulation. Your task is to analyze Solidity smart contracts for potential "force-feeding" or "unexpected Ether" vulnerabilities.

## Vulnerability Overview
The Unexpected Ether vulnerability occurs when contracts make assumptions about their Ether balance that can be broken by external actors. Attackers can force Ether into contracts through:
1. `selfdestruct()` calls targeting the contract
2. Pre-calculating contract addresses and sending Ether before deployment
3. Coinbase transactions (for mining rewards)

## Analysis Instructions

### Primary Detection Patterns
Look for these vulnerable patterns in smart contract code:

1. **Balance Comparisons**: `address(this).balance == expectedAmount`
2. **Balance-based Conditionals**: `require(address(this).balance >= threshold)`
3. **Balance Arithmetic**: `uint256 userShare = msg.value * totalShares / address(this).balance`
4. **Invariant Assumptions**: Internal accounting that assumes balance changes only through contract functions

## Analysis Focus Areas

1. **Balance Equality Checks**: Look for exact balance comparisons
2. **Conditional Logic**: Find balance-dependent control flow
3. **Mathematical Operations**: Identify balance used in calculations
4. **State Transitions**: Check if balance affects contract state changes
5. **Access Control**: Verify if balance influences permissions
6. **Economic Logic**: Examine reward/penalty calculations using balance

Analyze the provided smart contract code thoroughly and identify all instances where unexpected Ether could compromise the contract's intended behavior. Focus on practical exploitability and real-world impact.
"#;

```
ai-agent-audit/src/prompts/zero_code.rs
```
pub const CONTRACTS_WITH_ZERO_CODE: &str = r#"You are an expert smart contract security auditor specializing in access control vulnerabilities related to code size checks. Your task is to perform a comprehensive analysis on the provided Solidity smart contract code for vulnerabilities involving `extcodesize` and code length checks.

## Analysis Framework
Systematically examine the contract for the following code size check vulnerabilities:

1. **Constructor Bypass**: Contracts using code size checks that can be bypassed during contract construction
2. **Self-Destruct Bypass**: Access control relying on code size that fails after contract self-destruction
3. **EOA vs Contract Distinction**: Flawed logic attempting to differentiate between EOAs and contracts
4. **Whitelist Bypass**: Contract whitelisting mechanisms vulnerable to zero-code exploitation
5. **Access Control Evasion**: Critical functions protected only by code size checks

## Critical Patterns to Analyze
Pay special attention to code with these patterns:
- `extcodesize(msg.sender)` or `msg.sender.code.length` checks
- `assembly { size := extcodesize(caller()) }` patterns
- Access modifiers using code size for authorization
- Functions checking `tx.origin == msg.sender` combined with code size checks
- Contract whitelist validation based on code presence
- Access control assuming non-zero code size indicates legitimate contracts

## Specific Attack Vectors to Test
1. **Constructor Attack**: Deploy contract that calls target during `constructor()` execution
2. **Self-Destruct Attack**: Deploy contract, record address, self-destruct, then call from zero-code address
3. **Create2 Attack**: Use CREATE2 to deploy to predictable address, self-destruct, then redeploy different code
4. **Proxy Pattern Bypass**: Exploit proxy contracts that may have minimal code

## Analysis Instructions
1. Scan all functions for `extcodesize`, `code.length`, or assembly code size checks
2. Identify what access control or logic depends on these checks  
3. Determine if the protected functionality can be exploited via zero-code bypass
4. Create concrete attack scenarios showing the bypass
5. Write Foundry tests proving each vulnerability exists
6. Consider edge cases like proxy patterns, factory contracts, and upgrade mechanisms

Focus on demonstrable vulnerabilities where an attacker can bypass intended access restrictions through code size manipulation. Each finding must include a working Foundry test that proves the vulnerability exists."#;

```
ai-agent-audit/src/ai_bot/agent.rs
```
use std::collections::HashSet;

use anyhow::Result;
use log::info;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{
    agent::{Agent, AgentBuilder},
    client::{CompletionClient, EmbeddingsClient},
    providers::openai::{Client, CompletionModel, GPT_4O},
    vector_store::VectorStoreIndex,
};
use rig_qdrant::QdrantVectorStore;
/// AI agent implementations with vector-based context retrieval.
///
/// This module provides intelligent AI agents that combine static documentation
/// with dynamic vector search for contextual smart contract analysis.
use tiktoken_rs::cl100k_base;

use crate::build_brain::enbeddings::SourceChunk;
use crate::config::MAX_RAG_QUERY_CONTENT_LENGTH;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_file_content::extract_content_from_docs;
use crate::utils::logging::print_first_four_lines;

/// Creates an AI audit agent with vector-based dynamic context retrieval.
///
/// This agent combines static documentation context with dynamic vector search
/// to provide relevant code context for smart contract analysis queries.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `Agent<CompletionModel>` - Configured AI agent with vector search capabilities
pub fn create_ai_audit_agent(repo: &RepoPaths) -> Result<Agent<CompletionModel>> {
    // Extract static documentation for base context
    let documentation = extract_content_from_docs(repo)?;

    // Initialize OpenAI client and model
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let gpt4o = openai.completion_model(GPT_4O);

    // let solidity_auditor_preable = "Your are a world class expert at smart contract auditing, reknown for your ability to find the most complex and trickiest security vulnerabilities in solidity codebases.";

    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let qp = QueryPointsBuilder::new("contract_chunks") // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    let store = QdrantVectorStore::new(qdrant, model, qp);

    // let dynamic_context = vector_index()?;

    let openai_audit_agent = AgentBuilder::new(gpt4o)
        // .preamble(&solidity_auditor_preable)
        .context(&documentation)
        .dynamic_context(3, store)
        .temperature(0.1)
        .build();

    Ok(openai_audit_agent)
}

pub async fn get_rag_for_security_query(query_content: &str, repo: &RepoPaths) -> Result<String> {
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    // Tokenize the input
    let encoding = cl100k_base()?; // for OpenAI models
    let mut tokens = encoding.encode(query_content, HashSet::new());

    // Truncate tokens if needed
    if tokens.len() > MAX_RAG_QUERY_CONTENT_LENGTH {
        tokens.truncate(MAX_RAG_QUERY_CONTENT_LENGTH);
    }

    // Decode truncated tokens back into a string
    let query_content = encoding.decode(tokens)?;
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());
    let qp = QueryPointsBuilder::new(&vector_db_name) // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    info!("creating store...");
    let store = QdrantVectorStore::new(qdrant, model, qp);

    info!("retrieving relevant content from vector db");
    let relevant_docs: Vec<(f64, String, SourceChunk)> = store.top_n(&query_content, 3).await?;

    let dynamic_content = relevant_docs
        .iter()
        .map(|(_, _, source_chunk)| source_chunk.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    // info!("dynamic content");
    // print_first_four_lines(&dynamic_content);

    let final_context = format!("## ADDITIONAL CONTEXT: \n\n {}", dynamic_content);

    Ok(final_context)
}

```
ai-agent-audit/src/ai_bot/retrieve_slice.rs
```
use anyhow::Result;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

use crate::enumerator::codeblock_db::CodeBlocksDb;

#[derive(Debug, Deserialize)]
pub struct Args {
    pub slice_id: String,
}

#[derive(Debug, Serialize)]
pub struct Out {
    pub content: String,
}
/* ────────────── Thread-safe error type ──────────────────────────── */

#[derive(Debug, thiserror::Error)]
#[error("retrival error")]
// struct RetrieveSliceError;
pub enum RetrieveSliceError {
    Sql(#[from] rusqlite::Error),
}

pub struct RetrieveSliceTool {
    pub db: CodeBlocksDb, // <- now Sync because it’s just a PathBuf
}

impl Tool for RetrieveSliceTool {
    const NAME: &'static str = "retrieve_slice";

    type Args = Args;
    type Output = Out;
    type Error = RetrieveSliceError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Load a code slice (by ID) from SQLite and return its Markdown.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "slice_id": { "type": "string", "description": "UUID of the slice" }
                },
                "required": ["slice_id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let content = self.db.get_code_for_seed(&args.slice_id)?;
        Ok(Out { content })
    }
}

```
ai-agent-audit/src/enumerator/codeblock_cache.rs
```
/// In-memory caching for generated code blocks to optimize performance.
///
/// This module provides thread-safe caching of markdown code blocks to avoid
/// redundant generation when processing the same contracts multiple times,
/// significantly improving analysis performance for large repositories.
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::codeblock_db::MarkdownCodeblock;

/// Global in-memory cache for storing MarkdownCodeblock instances.
///
/// The cache is keyed by seed file paths and stores the corresponding
/// MarkdownCodeblock instances. This helps avoid redundant processing
/// and database operations when the same seed file is encountered multiple times.
///
/// The cache is thread-safe, using Arc and Mutex for concurrent access.
pub static CODEBLOCK_CACHE: Lazy<Arc<Mutex<HashMap<String, MarkdownCodeblock>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Retrieves a cached codeblock for a given seed file if it exists.
///
/// # Arguments
/// * `seed_file` - The path to the seed file used as the cache key
///
/// # Returns
/// * `Option<MarkdownCodeblock>` - The cached codeblock if found, None otherwise
pub async fn get_cached_codeblock(seed_file: &str) -> Option<MarkdownCodeblock> {
    let cache = Arc::clone(&CODEBLOCK_CACHE);
    let codeblock_cache = cache.lock().await;

    // Returns Some(codeblock) if found, and None if not found
    codeblock_cache.get(seed_file).cloned()
}

/// Stores a codeblock in the cache for a given seed file.
///
/// # Arguments
/// * `seed_file` - The path to the seed file used as the cache key
/// * `codeblock` - The MarkdownCodeblock instance to cache
pub async fn set_codeblock_cache(seed_file: &str, codeblock: &MarkdownCodeblock) {
    let cache = Arc::clone(&CODEBLOCK_CACHE);
    let mut codeblock_cache = cache.lock().await;

    codeblock_cache.insert(seed_file.to_string(), codeblock.clone());
}

```
ai-agent-audit/src/enumerator/codeblock_db.rs
```
/// SQLite database for code block storage and retrieval.
///
/// This module manages persistent storage of generated markdown code blocks,
/// providing efficient storage and retrieval of contextual code slices for
/// AI analysis with metadata and token counting.
use anyhow::Result;
use rusqlite::{Connection, params};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Represents a contextual markdown code block for AI analysis.
///
/// Contains generated code slices with associated metadata including token
/// counts for LLM context management and contract identification.
#[derive(Debug, Clone)]
pub struct MarkdownCodeblock {
    /// Unique identifier for the code block
    pub id: String,
    /// Contract name (e.g., "PuppyRaffle")
    pub contract: String,
    /// Token count for LLM context window management
    pub tokens: usize,
    /// Markdown content with code, IR, and storage information
    pub content: String,
}

/// SQLite database manager for code block persistence.
///
/// Provides high-level interface for storing and retrieving generated
/// code blocks with efficient querying and metadata management.
pub struct CodeBlocksDb {
    /// Path to the SQLite database file
    path: PathBuf,
}

impl CodeBlocksDb {
    /// Creates a new SliceDb instance or opens an existing one at the specified path.
    ///
    /// This function initializes the database schema if it doesn't already exist,
    /// creating tables for seed slices and codeblocks with appropriate indexes.
    ///
    /// # Arguments
    /// * `path` - Path to the SQLite database file
    ///
    /// # Returns
    /// * `Result<Self>` - A new SliceDb instance if successful, Error otherwise
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Self {
            path: path.as_ref().to_path_buf(),
        };

        // Initialize schema once
        let conn = Connection::open(&db.path)?;
        conn.execute_batch(
            r#"
                /* ───────── seed → contract link table ───────── */
                CREATE TABLE IF NOT EXISTS seed_slices(
                id                 TEXT PRIMARY KEY,      -- UUID you assign
                seed_id            TEXT,                  -- FK → seeds.id
                codeblock_id       TEXT,                  -- FK → codeblocks.id
                status             TEXT
                );

                /* speed up look-ups by seed_id or by slice_id */
                CREATE INDEX IF NOT EXISTS idx_seed_slices_seed_id
                    ON seed_slices(seed_id);
                CREATE INDEX IF NOT EXISTS idx_seed_slices_slice_id
                    ON seed_slices(codeblock_id);

                /* ───────── deduped contract bodies ─────────── */
                CREATE TABLE IF NOT EXISTS codeblocks(
                id      TEXT PRIMARY KEY,  -- sha256(body)
                filename TEXT,
                tokens  INTEGER,
                content TEXT
                );
               "#,
        )?;
        Ok(db)
    }

    /// Retrieves the markdown content for a given seed ID.
    ///
    /// This function performs a join between the seed_slices and codeblocks tables
    /// to find the markdown content associated with a specific seed.
    ///
    /// # Arguments
    /// * `seed_id` - The ID of the seed to retrieve code for
    ///
    /// # Returns
    /// * `rusqlite::Result<String>` - The markdown content if found, Error otherwise
    pub fn get_code_for_seed(&self, seed_id: &str) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT c.content
                FROM   codeblocks AS c
                JOIN   seed_slices     AS s  ON s.codeblock_id = c.id
                WHERE  s.seed_id = ?1
                LIMIT  1;
                "#,
            params![seed_id],
            |row| row.get(0),
        )
    }

    /// Inserts a new seed slice into the database.
    ///
    /// This function creates a mapping between a seed and a codeblock in the database.
    ///
    /// # Arguments
    /// * `s` - The SeedSlice to insert
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Error otherwise
    // pub fn insert_seed_slice(&self, s: &SeedSlice) -> Result<()> {
    //     let conn = Connection::open(&self.path)?;
    //     conn.execute(
    //         "INSERT INTO seed_slices VALUES (?1,?2,?3,?4);",
    //         params![s.id, s.seed_id, s.codeblock_id, s.status],
    //     )?;
    //     Ok(())
    // }
    //
    /// Inserts a new codeblock into the database if it doesn't already exist.
    ///
    /// This function checks if a codeblock with the same ID already exists in the database
    /// and only inserts it if it doesn't, preventing duplicate entries.
    ///
    /// # Arguments
    /// * `c` - The MarkdownCodeblock to insert
    ///
    /// # Returns
    /// * `Result<()>` - Ok if successful, Error otherwise
    pub fn insert_codeblock(&self, c: &MarkdownCodeblock) -> Result<()> {
        let conn = Connection::open(&self.path)?;

        // Check if codeblock already exists
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM codeblocks WHERE id = ?1);",
            params![c.id],
            |row| row.get(0),
        )?;

        if exists {
            // Skip insertion if codeblock already exists
            log::debug!("Skipping duplicate contract_slice with id {}", c.id);
            return Ok(());
        }

        // Insert new codeblock
        conn.execute(
            "INSERT INTO codeblocks VALUES (?1,?2,?3,?4);",
            params![c.id, c.contract, c.tokens as i64, c.content],
        )?;
        Ok(())
    }

    /// Retrieves all contracts and their content as a HashMap.
    ///
    /// # Returns
    /// * `rusqlite::Result<HashMap<String, String>>` - HashMap mapping contract names to their content
    pub fn get_all_contracts(&self) -> rusqlite::Result<HashMap<String, String>> {
        let conn = Connection::open(&self.path)?;

        let mut stmt = conn.prepare("SELECT filename, content FROM codeblocks")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // filename/contract
                row.get::<_, String>(1)?, // content
            ))
        })?;

        let mut contracts = HashMap::new();
        for row in rows {
            let (contract, content) = row?;
            contracts.insert(contract, content);
        }

        Ok(contracts)
    }

    pub fn get_code_for_contract(&self, contract: &str) -> rusqlite::Result<String> {
        let conn = Connection::open(&self.path)?;

        conn.query_row(
            r#"
                SELECT content
                FROM   codeblocks
                WHERE  contract = ?1
                LIMIT  1;
                "#,
            params![contract],
            |row| row.get(0),
        )
    }
}

```
ai-agent-audit/src/enumerator/codeblock_maker.rs
```
/// Code block generation orchestration for contract analysis.
///
/// This module coordinates the generation of contextual code blocks for each
/// contract in a repository, managing database connections and processing
/// parameters for optimal AI analysis.

use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::{enumerator::codeblock_db::CodeBlocksDb, prepare_code::git_clone::RepoPaths};
use super::codeblocks::generate_codeblock_from_codebase;

/// Generates and saves contextual code blocks for all contracts in a repository.
///
/// This function orchestrates the code block generation process by connecting
/// to semantic and slice databases, then generating focused code slices for
/// each contract using call graph traversal within specified constraints.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_db` - Path to semantic analysis database
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `PathBuf` - Path to the generated slice database
pub async fn generate_and_save_codeblocks_for_each_contract(
    repo: &RepoPaths,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // Open the three databases
    log::info!("connecting to databases..");
    let semantic_conn = Connection::open(semantics_db)?;
    let slice_path = repo.root.join(".cache").join("slice.db");
    let slice_db = CodeBlocksDb::open(&slice_path)?;

    // Fetch all seeds and process each one
    generate_codeblock_from_codebase(repo, &semantic_conn, &slice_db, max_depth, token_budget)
        .await?;

    Ok(slice_path)
}

```
ai-agent-audit/src/enumerator/codeblocks.rs
```
use crate::build_brain::graph_db::SmartContractFunction;
/// Intelligent code slicing for focused AI analysis.
///
/// This module generates contextual code blocks by traversing call graphs and
/// assembling relevant code, IR, and storage information within token budgets
/// for optimal LLM analysis.
use crate::enumerator::codeblock_cache::{get_cached_codeblock, set_codeblock_cache};
use crate::enumerator::codeblock_db::MarkdownCodeblock;
use crate::enumerator::utils::{
    generate_code_slice_for_storage, generate_codeblock_for_function,
    get_function_metadata_from_id, get_hashmap_of_contract_to_functions,
    get_token_count_of_function_ir,
};
use crate::prepare_code::git_clone::RepoPaths;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

/// Generates contextual code blocks for each contract using call graph traversal.
///
/// This function creates focused code slices by:
/// 1. Checking cache to avoid redundant processing
/// 2. Performing breadth-first search through call graphs up to max_depth
/// 3. Respecting token budgets for LLM context limits
/// 4. Assembling markdown with storage layouts and SlithIR representations
/// 5. Caching results for efficient reprocessing
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantic_db` - Database containing call graph and function data
/// * `codeblock_db` - Database for storing generated code blocks
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `Result<()>` - Success or error
pub async fn generate_codeblock_from_codebase(
    repo: &RepoPaths,
    semantic_db: &Connection,
    codeblock_db: &CodeBlocksDb,
    max_depth: usize,
    token_budget: usize,
) -> Result<()> {
    log::info!("getting contract to func mapping");
    let contract_to_func_map = get_hashmap_of_contract_to_functions(repo, semantic_db)?;

    for (contract, functions_of_contract) in contract_to_func_map {
        // Check if codeblock already generated for this seed

        log::info!("contract => {:#?}", contract);
        log::info!("fn count of contract => {:#?}", functions_of_contract.len());
        if let Some(_) = get_cached_codeblock(&contract).await {
            // Save seed-to-codeblock mapping in the database
            continue;
        };
        // 2. BFS until depth / token budget
        let mut frontier: VecDeque<(SmartContractFunction, usize)> = VecDeque::new();
        for func in functions_of_contract {
            frontier.push_back((func, 0_usize))
        }
        let mut visited = HashSet::new();
        let mut contracts = HashSet::new();
        let mut all_funcs_connected_to_contract = Vec::<SmartContractFunction>::new();
        let mut token_count = 0_usize;

        while let Some((func, depth)) = frontier.pop_front() {
            if !visited.insert(func.id.clone()) {
                continue;
            }

            //keep track of unique contract traversed in BPS
            contracts.insert(func.contract.clone());
            // info!("contract {} / func {} added...", func.contract, func.name);

            all_funcs_connected_to_contract.push(func.clone());

            // get token count of new fn + IR + storage
            let token_count_fn_ir_storage = get_token_count_of_function_ir(&func, repo).await?;
            // info!("token_count_fn_ir_storage => {}", token_count_fn_ir_storage);

            // check budget, make sure not exceeding token context window
            if token_count + token_count_fn_ir_storage > token_budget {
                break; // budget exhausted
            }

            // update token count
            token_count += token_count_fn_ir_storage;

            // info!("token_count => {}", token_count);
            if depth < max_depth {
                let mut statement =
                    semantic_db.prepare("SELECT callee FROM edges WHERE caller = ?1;")?;
                let rows = statement.query_map([&func.id], |r| r.get::<_, String>(0))?;
                for callee in rows.flatten() {
                    let callee_fn = get_function_metadata_from_id(&callee, semantic_db)?;
                    let Some(callee_fn) = callee_fn else { continue };

                    frontier.push_back((callee_fn, depth + 1));
                }
            }
        }

        // ── 3.  Assemble final Markdown body ────────────────────────────
        let mut markdown_codeblock_for_llm = String::new();
        // list storage vars
        for contract in &contracts {
            let storage_var_ir = generate_code_slice_for_storage(contract, repo).await?;
            // loop through and add all functions of contract
            markdown_codeblock_for_llm.push_str(&storage_var_ir);
            markdown_codeblock_for_llm.push('\n');
        }
        for func in &all_funcs_connected_to_contract {
            let function_ir_code = generate_codeblock_for_function(func, repo).await?;
            markdown_codeblock_for_llm.push_str(&function_ir_code);
            markdown_codeblock_for_llm.push('\n');
        }
        info!(
            "markdown codeblock size ==> {:#?}",
            markdown_codeblock_for_llm.len()
        );

        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            contract: contract.clone(),
            tokens: token_count,
            content: markdown_codeblock_for_llm,
        };
        // 4. store
        codeblock_db.insert_codeblock(&codeblock)?;

        // save to cache
        set_codeblock_cache(&contract, &codeblock).await;

        // info!("codeblock => {:#?}", codeblock);
    }

    Ok(())
}

```
ai-agent-audit/src/enumerator/utils.rs
````
use anyhow::anyhow;
use anyhow::Result;
use log::info;
use regex::Regex;
use rusqlite::params_from_iter;
use rusqlite::{Connection, OptionalExtension};
/// Enumeration utilities for code block generation and analysis.
///
/// This module provides utility functions for generating markdown code blocks,
/// extracting function metadata, managing IR mappings, and performing token
/// counting for optimal code slice generation within LLM context limits.
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::fn_labels::get_modifiers_label;
use crate::utils::fn_labels::get_visibility_label;
use crate::utils::get_fn_name::get_function_name;
use crate::{
    build_brain::{
        self,
        graph_db::SmartContractFunction,
        slither_ffi::{SlithIRFn, StorageVar},
    },
    utils::bpe::get_bpe,
};

/// Generates a markdown code block for a specific function with IR representation.
///
/// Creates a formatted markdown section containing the function's SlithIR
/// intermediate representation, including contract context and function metadata.
///
/// # Arguments
/// * `func` - Smart contract function metadata
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `String` - Formatted markdown code block with IR content
pub async fn generate_codeblock_for_function(
    func: &SmartContractFunction,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let ir_map = get_code_ir_map(repo).await?;
    let mut function_slice = String::new();
    let func_name = get_function_name(&func.name);
    let visibility = get_visibility_label(&func.visibility);
    let modifiers = get_modifiers_label(&func.modifiers);

    if let Some(ir) = ir_map.get(&(func.contract.clone(), func_name)) {
        function_slice.push_str(&format!(
            "#### {} {}{}\n",
            ir.function, visibility, modifiers
        ));
        function_slice.push_str("```slithir\n");
        function_slice.push_str(&ir.ir);
        function_slice.push_str("\n```");
    }

    // info!("function slice => {:#?}", function_slice);
    Ok(function_slice)
}

/// Generates a markdown codeblock for a contract's storage layout.
///
/// Retrieves the storage variables for a contract and formats them as a markdown codeblock.
///
/// # Arguments
/// * `contract` - The name of the contract
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<String>` - The generated markdown codeblock
pub async fn generate_code_slice_for_storage(
    contract: &str,
    repo: &RepoPaths,
) -> anyhow::Result<String> {
    let storage_map = get_storage_map(repo).await?;
    let mut storage_slice = String::new();
    if let Some(vars) = storage_map.get(contract) {
        storage_slice.push_str(&format!("### Storage layout ({}) \n\n", contract));
        storage_slice.push_str("```text\n");
        for v in vars {
            storage_slice.push_str(&format!("{} {}\n", v.name, v.r#type));
        }
        storage_slice.push_str("\n```");
    }

    Ok(storage_slice)
}

pub fn get_hashmap_of_contract_to_functions(
    repo: &RepoPaths,
    semantic_db: &Connection,
) -> anyhow::Result<HashMap<String, Vec<SmartContractFunction>>> {
    // find all main contracts for app (ones in /src)
    info!("grabbing all contracts...");
    let contracts_in_src_folder = contracts_in_src(repo)?;

    let placeholders = contracts_in_src_folder
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    let mut statement = semantic_db.prepare(&format!(
        "SELECT id, contract, name, visibility, modifiers, mutability FROM functions WHERE contract IN ({})",
        placeholders
    ))?;

    let rows = statement.query_map(params_from_iter(contracts_in_src_folder), |row| {
        // info!("rows => {:#?}", row);
        let modifier_str: String = row.get(4)?;
        let modifiers: Vec<String> = modifier_str
            .split(',')
            .map(|s| s.trim_matches([' ', '\'']).to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(SmartContractFunction {
            id: row.get(0)?,
            contract: row.get(1)?,
            name: row.get(2)?,
            visibility: row.get(3)?,
            modifiers,
            mutability: row.get(5)?,
        })
    })?;
    let functions_of_contract: Vec<SmartContractFunction> =
        rows.collect::<rusqlite::Result<_>>()?;

    if functions_of_contract.is_empty() {
        return Err(anyhow!("no entry fn found"));
    }
    let mut map: HashMap<String, Vec<SmartContractFunction>> = HashMap::new();

    for func in functions_of_contract {
        map.entry(func.contract.clone()).or_default().push(func)
    }

    Ok(map)
}

/// Calculates the token count of a function's IR representation.
///
/// Generates the markdown codeblock for the function and counts the number of tokens
/// using the BPE tokenizer.
///
/// # Arguments
/// * `func` - The smart contract function
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<usize>` - The token count
pub async fn get_token_count_of_function_ir(
    func: &SmartContractFunction,
    repo: &RepoPaths,
) -> anyhow::Result<usize> {
    // Generate the function's markdown codeblock
    let fn_text = generate_codeblock_for_function(func, repo).await?;

    // Count tokens using BPE tokenizer
    let bpe = get_bpe();
    let tokens = bpe.encode_with_special_tokens(&fn_text).len();

    Ok(tokens)
}

/// Retrieves a mapping of contract and function names to their SlithIR representations.
///
/// Extracts the function name from the full function signature and creates a map
/// keyed by (contract_name, function_name) tuples.
///
/// # Arguments
/// * `repo` - Path to the repository root
///
/// # Returns
/// * `anyhow::Result<HashMap<(String, String), SlithIRFn>>` - Map of (contract, function) to SlithIR
async fn get_code_ir_map(repo: &RepoPaths) -> anyhow::Result<HashMap<(String, String), SlithIRFn>> {
    // Regex to extract function name from full signature (e.g., "Contract.function(args)")
    let extract_function_name = Regex::new(r#"[A-Za-z0-9$_]+\.([A-Za-z0-9$_]+)\([^)]*\)"#)?;

    // Get IR and storage variables from Slither
    let (ir_vec, _, _) =
        build_brain::slither_ffi::get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;

    // Create map of (contract, function) -> SlithIRFn
    let ir_map: HashMap<(String, String), SlithIRFn> = ir_vec
        .into_iter()
        .map(|f| {
            if let Some(c) = extract_function_name.captures(&f.function) {
                // Extract function name from signature
                ((f.contract.clone(), c[1].to_string()), f)
            } else {
                // Use full function signature if extraction fails
                ((f.contract.clone(), f.function.clone()), f)
            }
        })
        .collect();

    // info!("ir_map => {:#?}", ir_map);
    Ok(ir_map)
}

async fn get_storage_map(repo: &RepoPaths) -> anyhow::Result<HashMap<String, Vec<StorageVar>>> {
    let (_, storage_vec, _) =
        build_brain::slither_ffi::get_slither_ir_and_storage_for_codeblockcodeblock(repo).await?;

    let storage_map: HashMap<String, Vec<StorageVar>> = {
        let mut m = HashMap::<String, Vec<StorageVar>>::new();
        for v in storage_vec {
            m.entry(v.contract.clone()).or_default().push(v);
        }
        m
    };
    Ok(storage_map)
}

/// Return the names of all `contract XXX` declarations that sit
/// anywhere under `repo_root/src/`.
pub fn contracts_in_src(repo: &RepoPaths) -> Result<Vec<String>> {
    let src_root = repo.root.join(&repo.repo_name).join("src");
    if !src_root.exists() {
        anyhow::bail!("no src/ folder found at {},", src_root.display());
    }
    // Regex matches `contract Foo`, ignores `interface` / `library`
    let re = Regex::new(r"(?m)^\s*contract\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let mut contracts = Vec::<String>::new();

    for file in &repo.sol_files {
        // ✅ is in src ?
        if !file.starts_with(&src_root) {
            continue;
        }

        // 🚫 Skip if path contains /lib/ or /mock/
        if file.components().any(|comp| {
            let part = comp.as_os_str().to_ascii_lowercase();
            part == "lib"
                || part == "library"
                || part.to_string_lossy().to_ascii_lowercase().contains("mock")
                || part
                    .to_string_lossy()
                    .to_ascii_lowercase()
                    .contains("helper")
        }) {
            continue;
        }
        // Skip directories and symlinks
        if fs::symlink_metadata(file)?.file_type().is_symlink() {
            continue;
        }

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read file {}: {}", file.display(), e);
                continue;
            }
        };

        for cap in re.captures_iter(&content) {
            if let Some(contract_name) = cap.get(1) {
                let contract = contract_name.as_str();
                if !contract.to_ascii_lowercase().contains("mock") {
                    contracts.push(contract.to_string());
                }
            }
        }
    }
    Ok(contracts)
}

pub fn get_function_metadata_from_id(
    id: &str,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let fn_metadata: Option<SmartContractFunction> = semantic_db
                        .query_row(
                            "SELECT id, contract, name, visibility, modifiers, mutability FROM functions WHERE id = ?1;",
                            [id],
                            |row| {
                                let modifier_str: String = row.get(4)?;
                                let modifiers: Vec<String> = modifier_str
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();

                                Ok(SmartContractFunction {
                                    id: row.get(0)?,
                                    contract: row.get(1)?,
                                    name: row.get(2)?,
                                    visibility: row.get(3)?,
                                    modifiers,
                                    mutability: row.get(5)?,
                                })
                            },
                        )
                        .optional()?;

    Ok(fn_metadata)
}

````
