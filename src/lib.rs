/// This is the main library crate for the AI Agent Audit tool.
/// It provides functionality for analyzing smart contracts, creating embeddings,
/// and storing them in a vector database for semantic search.

/// The build_brain module contains the core functionality for processing and analyzing smart contracts.
pub mod build_brain {
    /// Handles creating embeddings for source code files
    pub mod enbeddings;
    /// Provides enrichment of smart contract data using Slither analysis
    pub mod enrichment;
    /// Handles repository cloning and file filtering
    pub mod intake;
    /// Interfaces with the Slither static analysis tool
    pub mod slither_ffi;
    /// Provides functionality for interacting with the Qdrant vector database
    pub mod vector_db;
}

/// The utils module contains utility functions used throughout the codebase.
pub mod utils {
    /// Provides access to the OpenAI tokenizer (BPE) used for text chunking
    pub mod bpe;
}
