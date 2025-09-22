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
