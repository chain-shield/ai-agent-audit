# AI Agent Audit v2.0

An advanced AI-powered smart contract auditing tool that combines static analysis, multiple LLM providers, and vector embeddings to perform comprehensive security audits of Solidity codebases with automated PoC generation and professional report writing.

## What's New in v2.0

### 🎲 Randomized Prompt Generation
- **Pattern Randomization**: Vulnerability patterns are now randomized in each analysis run, reducing LLM position bias and increasing finding diversity by 10-20%
- **Actor Randomization**: Threat actor lists are shuffled per prompt, ensuring all actor types get equal attention across multiple runs
- **Invariant Randomization**: Protocol invariants are randomized to maximize coverage and reduce primacy/recency effects

### 🏗️ Architectural Improvements
- **Structured Data Storage**: Refactored multimodal context system to store structured data (`Actors`, `ContractInvariants`) instead of pre-formatted strings
- **Dynamic Formatting**: Actors and invariants are now formatted at prompt generation time, enabling per-run randomization
- **Pattern Field Mapping**: New `PatternField` trait with `to_field()` method for converting vulnerability patterns to snake_case field names

### 📊 Enhanced Finding Tracking
- **PatternChecked Struct**: Complete tracking of all 73 vulnerability patterns (49 syntactic + 24 semantic) plus 6 invariant types
- **Deterministic Ordering**: Findings now use consistent ordering across summary and full reports via `OnceCell` initialization
- **Better Deduplication**: Improved semantic similarity scoring with clear thresholds (< 0.25 = different, > 0.5 = duplicate)

### 🔍 Improved Analysis Quality
- **Multi-Round Diversity**: Each of the 40 analysis runs now gets different pattern ordering, maximizing unique finding discovery
- **Reduced False Positives**: Enhanced verification gates with pre-checks for hallucinated bugs and invalid invariants
- **Better Coverage**: Randomization ensures all vulnerability patterns get fair representation across runs

## Overview

AI Agent Audit is a sophisticated Rust-based tool that performs comprehensive smart contract security audits through a **7-phase workflow**:

1. **Repository Analysis**: Cloning and building smart contract repositories with support for Foundry, Hardhat, and custom build systems
2. **Static Analysis**: Extracting detailed IR, call graphs, and storage information using Slither
3. **AI-Powered Security Review**: Leveraging multiple LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) for vulnerability detection across 29+ categories
4. **Verification & Quality Check**: Multi-stage AI verification to reduce false positives and enhance finding quality
5. **Automated PoC Generation**: AI-generated Proof-of-Concept tests with automatic compilation and validation
6. **Professional Report Writing**: Competition-grade markdown reports formatted for Code4rena, Sherlock, and other platforms
7. **Vector Embeddings**: Creating semantic search capabilities through Qdrant vector database for intelligent context retrieval

This tool provides professional-grade smart contract auditing capabilities with AI assistance, making it suitable for security researchers, auditors, bug bounty hunters, and development teams.

## Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/chain-shield/ai-agent-audit.git
cd ai-agent-audit

# 2. Set up environment variables
cat > .env << EOF
OPENAI_API_KEY=your_openai_api_key
ANTHROPIC_API_KEY=your_anthropic_api_key
QDRANT_URL=http://localhost:6334
RUST_LOG=info
EOF

# 3. Start Qdrant vector database
docker-compose up -d

# 4. Build the project
cargo build --release

# 5. Create a YAML config file
cat > audit.yaml << EOF
repo: "https://github.com/Cyfrin/4-puppy-raffle-audit.git"
audit_type: "Code4rena"
poc_instructions: "poc-guide.md"
EOF

# 6. Run the audit
cargo run --release -- --config audit.yaml
```

## Features

### Core Functionality
- **Multi-Platform Repository Support**: Automatically clone and build repositories with Foundry, Hardhat, or custom build commands
- **YAML Configuration**: Define audit configurations in reusable YAML files for consistent analysis
- **Advanced Static Analysis**: Deep integration with Slither for IR extraction, call graph analysis, and storage layout
- **Multi-LLM Security Analysis**: Parallel vulnerability detection using OpenAI (GPT-4o, O3-mini), Anthropic (Claude 3.7/4.0/4.5 Sonnet, Opus 4.0), Google Gemini, and DeepSeek
- **Comprehensive Vulnerability Detection**: Covers 73 distinct vulnerability patterns (49 syntactic + 24 semantic) with randomized ordering
- **Vector-Based Semantic Search**: High-quality embeddings with Qdrant for intelligent code search and context retrieval
- **Professional Audit Reports**: Generate competition-grade markdown reports formatted for Code4rena, Sherlock, and other platforms
- **Cost Optimization**: Real-time tracking of inference costs across different LLM providers

### Advanced Capabilities
- **7-Phase Analysis Workflow**: Pattern discovery → Verification → Deduplication → Quality check → PoC generation → Report writing → Vector storage
- **Randomized Analysis (v2.0)**: Pattern, actor, and invariant randomization per run for 10-20% more unique findings
- **Automated PoC Generation**: AI-generated Solidity test files with automatic compilation and validation (up to 5 retry attempts)
- **Intelligent Code Slicing**: Generate contextual code blocks with call graph traversal for focused analysis
- **AI-Powered Verification**: Multi-stage verification with hallucination detection and invariant validation
- **Structured Context Management (v2.0)**: Type-safe actor and invariant storage with dynamic formatting
- **Workspace Caching**: Intelligent caching of build artifacts and analysis results for faster re-runs
- **Docker Integration**: Secure, isolated analysis environment using Trail of Bits security toolbox
- **Concurrent Processing**: Parallel analysis across multiple AI agents with configurable semaphore limits
- **Private Repository Support**: GitHub token authentication for private repository audits

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

# Optional: For private GitHub repositories
GITHUB_TOKEN=your_github_personal_access_token

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

### YAML Configuration (Recommended)

For complex audits or reusable configurations, use YAML config files:

1. **Create a configuration file** (e.g., `audit-config.yaml`):

```yaml
# Git repository URL (required)
repo: "https://github.com/Cyfrin/4-puppy-raffle-audit.git"

# Audit platform type (Code4rena, Sherlock, etc)
audit_type: "Code4rena"

# Optional: Project subfolder
# subfolder: "contracts"

# Optional: Source code folders (defaults to ["src"])
# code_folders:
#   - "src"
#   - "contracts"

# Optional: Custom documentation
# custom_doc: "audit-docs/protocol-docs.md"

# Optional: Scoped files list
# scoped_files: "audit-docs/scope.txt"

# Optional: Audit scope documentation
# audit_scope: "audit-docs/scope.md"

# Optional: Builder configuration
# builder: "Custom"
# build_cmd: "pnpm install && pnpm build"

# Optional: PoC configuration
# poc_instructions: "poc-instructions.md"
# poc_template: "poc-template.sol"
# test_folder: "test"

# Optional: Exclude folders
# exclude_folders:
#   - "test"
#   - "script"

# Optional: Force rebuild
# force_rebuild: false

# Optional: Foundry via-ir flag
# via_ir: false
```

2. **Run the audit with config file**:

```bash
cargo run --release -- --config audit-config.yaml
```

### CLI Arguments

All YAML options can also be specified via command-line arguments:

```bash
cargo run --release -- https://github.com/example/solidity-project.git \
  --subfolder contracts \
  --custom-doc protocol-docs.md \
  --code-folders src,contracts \
  --scoped-files scope.txt \
  --audit-scope scope.md \
  --builder custom \
  --build-cmd "pnpm install && pnpm build" \
  --poc-instructions poc-guide.md \
  --exclude-folders test,script \
  --force-rebuild
```

**Note**: CLI arguments override YAML configuration values.

### Example YAML Configurations

#### Simple Foundry Project
```yaml
repo: "https://github.com/Cyfrin/4-puppy-raffle-audit.git"
audit_type: "Code4rena"
poc_instructions: "poc-guide.md"
```

#### Complex Monorepo with Custom Build
```yaml
repo: "https://github.com/example/complex-project.git"
subfolder: "contracts"
audit_type: "Sherlock"
code_folders:
  - "src"
  - "contracts"
custom_doc: "audit-docs/protocol-docs.md"
scoped_files: "audit-docs/scope.txt"
audit_scope: "audit-docs/scope.md"
builder: "Custom"
build_cmd: "pnpm install && pnpm build"
poc_instructions: "audit-docs/poc-guide.md"
poc_template: "audit-docs/poc-template.sol"
test_folder: "test"
exclude_folders:
  - "test"
  - "script"
  - "mock"
via_ir: true
```

#### Private Repository
```yaml
repo: "https://github.com/private-org/private-repo.git"
audit_type: "Immunefi"
# Set GITHUB_TOKEN environment variable before running
```

### Private Repository Support

To audit private GitHub repositories, set the `GITHUB_TOKEN` environment variable:

1. **Create a GitHub Personal Access Token**:
   - Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Click "Generate new token (classic)"
   - Select scopes: `repo` (for private repositories)
   - Copy the generated token

2. **Set the environment variable**:

```bash
# Option 1: Add to .env file
echo "GITHUB_TOKEN=ghp_your_token_here" >> .env

# Option 2: Export in your shell
export GITHUB_TOKEN=ghp_your_token_here

# Option 3: Inline with command
GITHUB_TOKEN=ghp_your_token_here cargo run --release -- --config audit-config.yaml
```

**Note**: The token is automatically injected into the git clone URL for authentication.

### Complete 7-Phase Workflow

The application performs a comprehensive audit workflow:

```
┌─────────────────────────────────────────────────────────────────┐
│                    PHASE 1: PREPARATION                         │
│  Repository Clone → Build → Slither Analysis → Code Slicing    │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│              PHASE 2: PATTERN DISCOVERY (29 types)              │
│  Parallel AI Agents → Vulnerability Detection → Initial Findings│
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│           PHASE 3: VERIFICATION & DEDUPLICATION                 │
│  AI Verification → Confidence Scoring → Similarity Analysis     │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                  PHASE 4: QUALITY CHECK                         │
│  Quality Assurance → Enhanced Details → Impact Analysis         │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│              PHASE 5: POC GENERATION (H/M only)                 │
│  Generate Test → Compile → Run → Retry (up to 5x) → Validate   │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│         PHASE 6: PROFESSIONAL REPORT WRITING                    │
│  Competition Format → GitHub URLs → Impact → Mitigation         │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│            PHASE 7: VECTOR DATABASE POPULATION                  │
│  Embeddings → Qdrant Storage → Semantic Search Ready            │
└─────────────────────────────────────────────────────────────────┘
```

#### Phase 1: Repository Preparation & Static Analysis
- Clone the repository using Docker for security isolation
- Auto-detect and build with Foundry, Hardhat, or custom build commands
- Intelligent workspace caching (reuses build artifacts if available)
- Extract call graphs and inheritance hierarchies using Slither
- Generate intermediate representation (IR) for all functions
- Analyze storage layouts and variable mappings
- Create semantic databases for efficient querying

#### Phase 2: Pattern Discovery
- Generate contextual code slices with call graph traversal
- Run parallel security analysis using multiple LLM providers
- Detect vulnerabilities across 73 distinct patterns (49 syntactic + 24 semantic)
- **Randomize pattern order** per run to reduce LLM position bias and increase finding diversity
- Use specialized prompts for each vulnerability type with dynamic actor and invariant context

#### Phase 3: Verification & Deduplication
- AI-powered verification of discovered findings with pre-gate sanity checks
- **Hallucination detection**: Verify bugs actually exist in code before proceeding
- **Invariant validation**: Confirm claimed invariants are real and documented
- Semantic similarity analysis to detect duplicates (threshold: 0.5 for high confidence)
- Confidence scoring for each finding
- Filter out false positives and low-confidence findings

#### Phase 4: Quality Check
- Final quality assurance on verified findings
- Enhance findings with improved details and impact analysis
- Ensure findings meet professional audit standards
- Add comprehensive mitigation strategies

#### Phase 5: Automated PoC Generation
- Generate Solidity test files for High and Medium severity findings
- Automatically compile and run tests using Forge
- Retry up to 5 times with AI-guided fixes if tests fail
- Validate that PoCs successfully demonstrate the vulnerability

#### Phase 6: Professional Report Writing
- Generate competition-grade markdown reports for validated findings
- **Deterministic ordering**: Findings use consistent order across summary and full reports
- Format according to audit platform (Code4rena, Sherlock, etc.)
- Include GitHub URLs with line numbers for all relevant code
- Add detailed impact analysis and mitigation recommendations
- Track which patterns were checked via `PatternChecked` metadata

#### Phase 7: Vector Database Population
- Create semantic embeddings for all code and analysis results
- Store in Qdrant for intelligent search and retrieval
- Enable future context-aware analysis

#### Cost Tracking
- Monitor and report total inference costs across all LLM providers
- Track costs per phase and per finding

## Project Structure

```
src/
├── ai_bot/                    # AI agent implementations
│   ├── agent.rs              # Core AI audit agent with vector search
│   └── retrieve_slice.rs     # Context retrieval for AI analysis
├── build_brain/              # Core analysis and data processing
│   ├── callgraph.rs          # Call graph analysis and traversal
│   ├── embeddings.rs         # Vector embeddings generation
│   ├── enrichment.rs         # Slither analysis integration
│   ├── fn_summaries.rs       # Function summarization
│   ├── graph_db.rs           # Graph database operations
│   ├── inheritance.rs        # Contract inheritance analysis
│   ├── parsers.rs            # Code parsing utilities
│   ├── slither_ffi.rs        # Slither static analyzer interface
│   ├── summarize.rs          # Protocol and file summarization
│   └── vector_db.rs          # Qdrant vector database operations
├── cli_args/                 # Command-line argument parsing
│   └── parse.rs              # CLI and YAML config parsing
├── cost/                     # Cost tracking and management
│   └── cost_data.rs          # LLM inference cost calculation
├── enumerator/               # Code slicing and enumeration
│   ├── codeblock_cache.rs    # Caching for generated code blocks
│   ├── codeblock_db.rs       # Database for code block storage
│   ├── codeblock_maker.rs    # Code block generation logic
│   ├── codeblocks.rs         # Core code slicing functionality
│   ├── parse_library_file.rs # Library file parsing
│   ├── parse_solidity.rs     # Solidity source parsing
│   └── utils.rs              # Enumeration utilities
├── llm_review/               # AI-powered security analysis
│   ├── agent_factory.rs      # AI agent initialization
│   ├── code_review_v2.rs     # Main security review orchestration (7-phase)
│   ├── config.rs             # LLM configuration and models
│   ├── context_state.rs      # Global context management
│   ├── enums.rs              # AI agent and vulnerability type enums
│   ├── findings.rs           # Finding data structures
│   ├── semaphore.rs          # Concurrency control
│   ├── phases/               # 7-phase analysis workflow
│   │   ├── discover_patterns.rs    # Phase 1: Pattern discovery
│   │   ├── verify_findings.rs      # Phase 2: Verification
│   │   ├── deduplicate_findings.rs # Phase 3: Deduplication
│   │   ├── quality_check.rs        # Phase 4: Quality assurance
│   │   ├── add_poc_findings.rs     # Phase 5: PoC generation
│   │   ├── create_report.rs        # Phase 6: Report writing
│   │   └── mod.rs                  # Phase module exports
│   ├── prompt_support/       # Prompt engineering modules
│   │   ├── create_report_prompt.rs # Report generation prompts
│   │   ├── make_poc_prompt.rs      # PoC generation prompts
│   │   ├── post_qualify.rs         # Quality check post-prompts
│   │   ├── qualify_prompt.rs       # Quality check prompts
│   │   └── verify_prompt.rs        # Verification prompts
│   └── utils/                # LLM review utilities
│       └── prompt_context.rs # Context generation for prompts
├── prepare_code/             # Repository preparation
│   └── git_clone.rs          # Git cloning, building, and workspace caching
├── reporting/                # Report generation
│   ├── audit.rs              # Audit report generation
│   ├── contract_data.rs      # Contract data export
│   └── save_file.rs          # File saving utilities
├── prompts/                  # Vulnerability-specific prompts (21 types)
├── master_prompts/           # Master security analysis prompts
├── invariant_prompts/        # Protocol invariant prompts (6 types)
├── config/                   # Configuration management
│   └── mod.rs                # Audit type and config
├── error/                    # Error handling
│   └── mod.rs                # Custom error types
├── utils/                    # Shared utilities
├── lib.rs                    # Library exports
└── main.rs                   # Application entry point (7-phase workflow)
```

## Vulnerability Detection

The tool analyzes smart contracts for **73 distinct vulnerability patterns** organized into two tiers:

### Pattern Organization (v2.0)
- **R1_PATTERNS (Syntactic)**: 49 patterns detected through code structure analysis
- **R2_PATTERNS (Semantic)**: 24 patterns requiring deeper semantic understanding
- **Invariant Types**: 6 protocol invariant categories (Arithmetic, Balance, Permission, Temporal, Referential, StateMachine)

All patterns are tracked via the `PatternChecked` struct with `Option<bool>` tri-state logic:
- `Some(true)`: Pattern checked and vulnerability found
- `Some(false)`: Pattern checked but no vulnerability found
- `None`: Pattern not checked in this analysis

### Core Vulnerability Categories

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
- Auto-detect and build with Foundry (`forge build`), Hardhat (`npx hardhat compile`), or custom build commands
- Intelligent workspace caching: reuses build artifacts if available, rebuilds only when necessary
- Extract and filter Solidity source files and documentation
- Support for monorepos, subfolders, and custom source directories

### 2. Advanced Static Analysis
- Generate comprehensive call graphs and inheritance hierarchies using Slither
- Extract SlithIR (intermediate representation) for every function
- Analyze storage layouts and variable mappings
- Create semantic databases for efficient querying
- Let Slither handle compilation automatically for maximum reliability

### 3. Intelligent Code Slicing
- Perform breadth-first search through call graphs
- Generate contextual code blocks with configurable depth and token budgets
- Include parent contracts, called contracts, and deployment scripts
- Cache results for efficient reprocessing

### 4. Multi-LLM Security Analysis (7-Phase Workflow)
- **Phase 1**: Deploy multiple AI agents in parallel for pattern discovery across 73 vulnerability patterns with randomized ordering
- **Phase 2**: AI-powered verification with confidence scoring, semantic similarity analysis, and hallucination detection
- **Phase 3**: Intelligent deduplication using vector embeddings and similarity thresholds (< 0.25 = different, > 0.5 = duplicate)
- **Phase 4**: Quality assurance with enhanced details, impact analysis, and mitigation strategies
- **Phase 5**: Automated PoC generation with up to 5 retry attempts and automatic compilation
- **Phase 6**: Professional report writing formatted for Code4rena, Sherlock, and other platforms with deterministic ordering
- **Phase 7**: Vector database population for semantic search and context retrieval

### 5. Vector-Based Context Retrieval
- Create high-quality embeddings using OpenAI's text-embedding-3-small
- Store in Qdrant with rich metadata for semantic search
- Enable AI agents to retrieve relevant context dynamically
- Unique collections per repository for isolated analysis

### 6. Professional Report Generation
- Generate competition-grade Markdown audit reports with severity classifications
- Include GitHub URLs with line numbers for all relevant code
- Add detailed impact analysis, PoC tests, and mitigation recommendations
- Support both comprehensive (paid) and limited (free) report formats
- Export contract data and metadata for further analysis

## Output Files

After analysis, the tool generates several output files:

### Audit Reports
- `{repo-name}-audit-{hash}-audit-report.md` - Comprehensive audit report with all findings
- `{repo-name}-audit-{hash}-free-audit-report.md` - Limited audit report (free version)
- Individual finding reports with PoC tests and GitHub URLs

### Contract Analysis Data
- `{ContractName}-{repo-name}-audit-{hash}.md` - Individual contract analysis
- `metadata-{repo-name}-audit-{hash}.md` - Protocol metadata and context

### PoC Test Files
- `test/{Severity}-{Finding-Title}.t.sol` - Generated Solidity test files
- Automatically compiled and validated using Forge
- Includes setup, exploit demonstration, and assertions

### Analysis Artifacts
- `callgraph.json` - Complete call graph data
- `inheritance.json` - Contract inheritance relationships
- `graph.json` - Semantic graph database
- `sarif.json` - SARIF format analysis results

### Workspace Artifacts
- `/tmp/audit-analysis/{repo-name}-{hash}/` - Cached workspace with build artifacts
- `.chainshield_build_ok` - Build stamp for workspace reuse validation

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
| OpenAI | $2.00 | $8.00 | GPT-4o, O3-mini |
| Anthropic | $3.00 | $15.00 | Claude 3.7 Sonnet, Claude 4.0 Sonnet |
| Gemini | $1.25 | $10.00 | Gemini 2.0 Flash Thinking |
| DeepSeek | $0.07 | $1.10 | DeepSeek Chat |

### Analysis Parameters

Key configuration constants (in source code):
- `MAX_DEPTH`: Call graph traversal depth (default: 3)
- `TOKEN_BUDGET`: Maximum tokens per code block (default: 150,000)
- `RUNS`: Number of discovery rounds per contract (default: 3)
- `MAX_CONCURRENTS_REVIEW`: Concurrent pattern discovery agents (default: 3)
- `MAX_CONCURRENTS_VERIFY`: Concurrent verification agents (default: 10)
- `MAX_CONCURRENTS_POC`: Concurrent PoC generation (default: 1)

### Audit Platform Types

Supported audit platforms (configured via `audit_type` in YAML or CLI):
- `Code4rena` - Code4rena bug bounty platform
- `Sherlock` - Sherlock audit contests
- `Cantina` - Cantina security competitions
- `Hats` - Hats Finance bug bounties
- `Immunefi` - Immunefi bug bounty program

## Performance and Costs

### Typical Analysis Times
- Small projects (< 10 contracts): 10-20 minutes
- Medium projects (10-50 contracts): 20-60 minutes
- Large projects (50+ contracts): 60+ minutes

**Note**: Times include all 7 phases (pattern discovery, verification, deduplication, quality check, PoC generation, report writing, and vector storage).

### Cost Estimation
- Small project: $2-10 USD
- Medium project: $10-30 USD
- Large project: $30+ USD

**Cost Breakdown by Phase**:
- Phase 1 (Pattern Discovery): ~40% of total cost
- Phase 2 (Verification): ~20% of total cost
- Phase 3 (Deduplication): ~5% of total cost
- Phase 4 (Quality Check): ~15% of total cost
- Phase 5 (PoC Generation): ~10% of total cost
- Phase 6 (Report Writing): ~10% of total cost
- Phase 7 (Vector Storage): Minimal cost (embeddings only)

*Costs vary significantly based on LLM provider choice, project complexity, and number of findings*

### Workspace Caching Benefits
- **First run**: Full analysis with all phases
- **Subsequent runs**: Reuses build artifacts, ~30% faster
- **Force rebuild**: Use `--force-rebuild` flag to ignore cache

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
   # Verify Qdrant is running
   curl http://localhost:6334/collections
   ```

3. **Slither Build Artifacts Issues**
   - The tool automatically lets Slither handle compilation for maximum reliability
   - If you encounter build errors, use `--force-rebuild` to clear cached artifacts
   - For custom build systems, specify `--builder custom --build-cmd "your build command"`

4. **Out of Memory Errors**
   - Reduce `TOKEN_BUDGET` in source code (default: 150,000)
   - Use fewer concurrent LLM agents (reduce `MAX_CONCURRENTS_*` values)
   - Increase Docker memory limits in Docker Desktop settings

5. **API Rate Limits**
   - The tool uses semaphores to control concurrency
   - Reduce `MAX_CONCURRENTS_REVIEW` and `MAX_CONCURRENTS_VERIFY` in source code
   - Use multiple API keys with rotation
   - Choose providers with higher rate limits (e.g., DeepSeek)

6. **PoC Generation Failures**
   - The tool retries up to 5 times with AI-guided fixes
   - Check that Forge is installed and accessible in Docker
   - Verify `poc_instructions` file exists and contains valid guidance
   - Review PoC test output in terminal for compilation errors

7. **YAML Configuration Errors**
   - Ensure YAML file uses correct syntax (spaces, not tabs)
   - Use PascalCase for enum values (e.g., `builder: "Custom"`)
   - Verify all file paths in YAML are relative to current directory
   - Check that `repo` field is provided either in YAML or CLI

8. **Workspace Caching Issues**
   - Use `--force-rebuild` to ignore cached build artifacts
   - Delete workspace manually: `rm -rf /tmp/audit-analysis/{repo-name}-{hash}`
   - Check for `.chainshield_build_ok` stamp file in workspace

## Advanced Features

### Semantic Similarity Deduplication (v2.0 Enhanced)
The tool uses vector embeddings to detect duplicate findings with clear thresholds:
- **Similarity score < 0.25**: Completely different issues (keep both)
- **Similarity score 0.25-0.5**: Gray zone (manual review recommended)
- **Similarity score > 0.5**: Highly likely same issue (automatically deduplicated)

The deduplication system now uses deterministic ordering to ensure consistent results across summary and full reports.

### Intelligent PoC Retry Logic
When PoC tests fail, the tool:
1. Captures compilation errors and test output
2. Feeds errors back to AI agent for analysis
3. Generates improved PoC with fixes
4. Retries up to 5 times total
5. Marks finding with PoC status (AllTestPass, SomeTestPass, NoTestPass, CannotCreate)

### Multi-Stage Verification (v2.0 Enhanced)
Each finding goes through multiple verification stages with improved quality gates:
1. **Initial Discovery**: Pattern-based detection across 73 patterns with randomized ordering
2. **Pre-Gate Sanity Check (NEW)**: Verify bug exists in code, invariant is real, and execution path is possible
3. **Verification**: AI-powered validation with confidence scoring and hallucination detection
4. **Deduplication**: Semantic similarity analysis with clear thresholds (< 0.25 = different, > 0.5 = duplicate)
5. **Quality Check**: Final quality assurance and enhancement
6. **PoC Validation**: Automated test generation and execution (up to 5 retries)
7. **Report Generation**: Professional markdown report writing with deterministic ordering

### Randomization Benefits (v2.0)
The randomization system addresses LLM position bias and improves finding diversity:

**Why Randomization Matters:**
- LLMs exhibit **primacy bias** (pay more attention to items at the beginning)
- LLMs exhibit **recency bias** (pay more attention to items at the end)
- Fixed ordering means the same patterns always get advantaged/disadvantaged positions

**How It Works:**
- Each analysis run (typically 3-5 runs per contract) gets a **different random order**
- Patterns, actors, and invariants are shuffled independently per prompt generation
- Uses cryptographically secure RNG (`rand::rng()`) for true randomness

**Expected Benefits:**
- **Without randomization**: ~70-80% coverage (some patterns always disadvantaged)
- **With randomization**: ~85-95% coverage (all patterns get fair chance)
- **Net improvement**: +10-20% more unique findings across multiple runs

**Example with 3 runs:**
```
Run 1: [Pattern A, Pattern B, Pattern C] → Pattern A gets "first position advantage"
Run 2: [Pattern C, Pattern A, Pattern B] → Pattern C gets "first position advantage"
Run 3: [Pattern B, Pattern C, Pattern A] → Pattern B gets "first position advantage"
```

All patterns get equal opportunity across runs, maximizing coverage!

### Workspace Management
The tool intelligently manages build artifacts:
- Checks for existing workspace at `/tmp/audit-analysis/{repo-name}-{hash}`
- Validates build artifacts (`out/`, `artifacts/`, or `build/` directories)
- Reuses workspace if artifacts are valid and `--force-rebuild` not set
- Rebuilds if artifacts missing, corrupted, or force rebuild requested
- Logs detailed reasons for rebuild decisions

## Version History

### v2.0 (Current Release)
- **Randomized Prompt Generation**: Pattern, actor, and invariant randomization for 10-20% more unique findings
- **Structured Context Management**: Type-safe storage with dynamic formatting
- **Enhanced Verification**: Pre-gate sanity checks for hallucination detection
- **Pattern Field Mapping**: `PatternField` trait for converting patterns to field names
- **Deterministic Ordering**: Consistent finding order across reports
- **Improved Deduplication**: Clear similarity thresholds and better tracking
- **73 Vulnerability Patterns**: Expanded from 29 to 73 distinct patterns (49 syntactic + 24 semantic)
- **Claude 4.0/4.5 Support**: Added latest Anthropic models (Sonnet 4.0, 4.5, Opus 4.0)

### v1.0 (Initial Release)
- 7-phase analysis workflow
- Multi-LLM support (OpenAI, Anthropic, Gemini, DeepSeek)
- Automated PoC generation
- Vector database integration
- Professional report writing
- 29 vulnerability categories

## Contributing

Contributions are welcome! Areas for improvement:
- Additional vulnerability detection patterns beyond the current 73 patterns
- New LLM provider integrations (GPT-5, Claude 5.0, etc.)
- Performance optimizations for large codebases
- Enhanced reporting formats and visualization
- Advanced prompt engineering for better detection accuracy
- Improved PoC generation strategies
- Additional audit platform support
- Better randomization strategies for finding diversity

Please feel free to submit a Pull Request.

## Acknowledgments

This tool builds upon excellent open-source projects:
- [Slither](https://github.com/crytic/slither) - Static analysis framework by Trail of Bits
- [Qdrant](https://qdrant.tech/) - Vector similarity search engine
- [Foundry](https://github.com/foundry-rs/foundry) - Ethereum development toolkit
- [OpenAI](https://openai.com/) - GPT models and embeddings
- [Anthropic](https://www.anthropic.com/) - Claude models
- [Google AI](https://ai.google.dev/) - Gemini models
- [DeepSeek](https://www.deepseek.com/) - Cost-effective AI models

## License

This project is licensed under the MIT License - see the LICENSE file for details.
