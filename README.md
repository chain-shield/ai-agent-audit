# AI Agent Audit

A tool for analyzing smart contracts and creating semantic search capabilities through vector embeddings.

## Overview

AI Agent Audit is a Rust-based tool that analyzes Solidity smart contracts by:

1. Cloning a repository containing smart contracts
2. Building the contracts with Forge
3. Extracting intermediate representation (IR) and storage information using Slither
4. Creating embeddings for the source code and analysis results
5. Storing the embeddings in a Qdrant vector database for semantic search

This enables powerful semantic search capabilities for smart contract auditing and analysis.

## Features

- **Repository Intake**: Automatically clone and filter repositories containing Solidity contracts
- **Slither Integration**: Extract detailed IR and storage information from smart contracts
- **Text Chunking**: Break down source code and analysis results into optimal chunks for embedding
- **Vector Embeddings**: Create high-quality embeddings using OpenAI's embedding models
- **Vector Database**: Store and query embeddings using Qdrant for semantic search

## Prerequisites

Before running the application, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) for Forge
- [Slither](https://github.com/crytic/slither#how-to-install) static analyzer
- [Qdrant](https://qdrant.tech/documentation/quick-start/) vector database
- An OpenAI API key for generating embeddings

## Environment Setup

1. Create a `.env` file in the project root with the following variables:

```
OPENAI_API_KEY=your_openai_api_key
QDRANT_URL=http://localhost:6334
RUST_LOG=info
```

2. Ensure Qdrant is running. You can start it with Docker:

```bash
docker-compose up
```

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

Run the application with a Git repository URL containing Solidity contracts:

```bash
cargo run --release -- https://github.com/example/solidity-project.git
```

The application will:

1. Clone the repository
2. Build the contracts using Forge
3. Run Slither analysis to extract IR and storage information
4. Create embeddings for all the files
5. Store the embeddings in Qdrant

## Project Structure

```
src/
├── build_brain/
│   ├── enbeddings.rs    # Handles creating embeddings for source code
│   ├── enrichment.rs    # Enriches data using Slither analysis
│   ├── intake.rs        # Handles repository cloning and filtering
│   ├── slither_ffi.rs   # Interfaces with the Slither static analyzer
│   └── vector_db.rs     # Interacts with the Qdrant vector database
├── utils/
│   └── bpe.rs           # Provides access to the OpenAI tokenizer
├── lib.rs               # Library exports
└── main.rs              # Application entry point
```

## How It Works

### 1. Repository Intake

The application clones the specified Git repository and filters out irrelevant files, keeping only Solidity contracts and documentation.

### 2. Forge Build

It builds the Solidity contracts using Forge to ensure they compile correctly and to generate the necessary artifacts for Slither analysis.

### 3. Slither Analysis

Slither is used to extract:
- SlithIR (intermediate representation) for each function
- Storage variable information for each contract

This information is saved as individual text files.

### 4. Text Chunking and Embedding

All files (Solidity source, documentation, and Slither analysis results) are broken into chunks of appropriate size and embedded using OpenAI's embedding model.

### 5. Vector Database Storage

The embeddings are stored in a Qdrant collection, along with metadata that allows tracing back to the original source.

## Querying the Vector Database

After running the application, you can query the vector database using the Qdrant API or client libraries. For example:

```python
from qdrant_client import QdrantClient
from qdrant_client.http.models import Filter, FieldCondition, MatchValue

# Connect to Qdrant
client = QdrantClient(url="http://localhost:6334")

# Search for semantically similar content
search_result = client.search(
    collection_name="contract_chunks",
    query_vector=your_query_vector,  # Vector from embedding your query text
    limit=5
)

# Print results
for result in search_result:
    print(f"Score: {result.score}")
    print(f"Metadata: {result.payload['meta']}")
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
