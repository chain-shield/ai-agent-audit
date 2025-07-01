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
    pub mod fn_summaries;
    pub mod parsers;
    pub mod summarize;

    /// Handles repository cloning and file filtering
    pub mod git_clone;
    pub mod graph_db;
    pub mod inheritance;
    /// Interfaces with the Slither static analysis tool
    pub mod slither_ffi;
    /// Provides functionality for interacting with the Qdrant vector database
    pub mod vector_db;
}

pub mod reporting {
    pub mod audit;
    pub mod contract_data;
    pub mod save_file;
}

pub mod enumerator {
    pub mod codeblock_cache;
    pub mod codeblock_db;
    pub mod codeblock_maker;
    pub mod codeblocks;
    pub mod utils;
}

pub mod llm_review {
    pub mod analysis_db;
    pub mod code_review;
    pub mod config;
    pub mod context_state;
    pub mod enums;
    pub mod invariants;
    pub mod prompt_content;
    pub mod review_utils;
    pub mod prompt_support {
        pub mod dedup;
        pub mod post_prompt;
        pub mod post_qualify;
        pub mod post_verify;
        pub mod pre_prompt;
        pub mod pre_qualify;
        pub mod pre_verify;
        pub mod qualify_prompt;
        pub mod verify_prompt;
    }
}

pub mod cost {
    pub mod cost_data;
}

pub mod ai_bot {
    pub mod agent;
    pub mod retrieve_slice;
}

pub mod master_prompts {
    pub mod master_prompt;
    pub mod prompt_2x_a;
    pub mod prompt_2x_aa;
    pub mod prompt_2x_b;
    pub mod prompt_2x_bb;
    pub mod prompt_3x_a;
    pub mod prompt_3x_b;
    pub mod prompt_3x_c;
}
pub mod prompts {
    pub mod access_control;
    pub mod array_limits;
    pub mod confidential_data;
    pub mod default_visibility;
    pub mod dos;
    pub mod inheritance;
    pub mod integer_overflow;
    pub mod mev;
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

pub mod invariant_prompts {
    pub mod arithmetic;
    pub mod balance;
    pub mod permission;
    pub mod referential;
    pub mod state_machine;
    pub mod temporal;
}

/// The utils module contains utility functions used throughout the codebase.
pub mod utils {
    /// Provides access to the OpenAI tokenizer (BPE) used for text chunking
    pub mod bpe;
    pub mod extract_retry;
    pub mod fn_labels;
    pub mod get_doc_file;
    pub mod get_fn_name;
    pub mod logging;
    pub mod sanitize;
    pub mod vec_db_connect;
}
