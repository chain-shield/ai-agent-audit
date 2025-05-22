/// This is the main library crate for the AI Agent Audit tool.
/// It provides functionality for analyzing smart contracts, creating embeddings,
/// and storing them in a vector database for semantic search.

/// The build_brain module contains the core functionality for processing and analyzing smart contracts.
pub mod build_brain {
    pub mod callgraph;
    /// Handles creating embeddings for source code files
    pub mod enbeddings;
    /// Provides enrichment of smart contract data using Slither analysis
    pub mod enrichment;
    pub mod graph_db;
    pub mod inheritance;
    /// Handles repository cloning and file filtering
    pub mod intake;
    /// Interfaces with the Slither static analysis tool
    pub mod slither_ffi;
    /// Provides functionality for interacting with the Qdrant vector database
    pub mod vector_db;
}

pub mod static_scanning {
    pub mod seed_db;
    pub mod slither;
}

pub mod enumerator {
    pub mod path_enum;
    pub mod slice_db;
    pub mod slice_maker;
}

pub mod ai_bot {
    pub mod retrieve_slice;
}

/// The utils module contains utility functions used throughout the codebase.
pub mod utils {
    /// Provides access to the OpenAI tokenizer (BPE) used for text chunking
    pub mod bpe;
}
