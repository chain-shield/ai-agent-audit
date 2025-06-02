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
    pub mod codeblock_cache;
    pub mod codeblocks;
    pub mod path_enum;
    pub mod slice_db;
    pub mod slice_maker;
    pub mod utils;
}

pub mod llm_review {
    pub mod analysis_db;
    pub mod code_review;
    pub mod config;
    pub mod prompt_support {
        pub mod post_prompt;
        pub mod pre_prompt;
    }
}

pub mod ai_bot {
    pub mod agent;
    pub mod rag;
    pub mod retrieve_slice;
}

pub mod prompts {
    pub mod access_control;
    pub mod array_limits;
    pub mod confidential_data;
    pub mod default_visibility;
    pub mod dos;
    pub mod inheritance;
    pub mod integer_overflow;
    pub mod oracle;
    pub mod pragma;
    pub mod randomness;
    pub mod reentrancy;
    pub mod replay_attack;
    pub mod self_destruct;
    pub mod short_address_attack;
    pub mod storage_variables;
    pub mod tx_origin;
    pub mod unchecked_return_value;
    pub mod unexpected_eth;
    pub mod zero_code;
}

/// The utils module contains utility functions used throughout the codebase.
pub mod utils {
    /// Provides access to the OpenAI tokenizer (BPE) used for text chunking
    pub mod bpe;
    pub mod get_doc_file;
    pub mod logging;
    pub mod vec_db_connect;
}
