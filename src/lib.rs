/// AI Agent Audit - Comprehensive Smart Contract Security Analysis Tool
///
/// This library provides advanced AI-powered smart contract auditing capabilities,
/// combining static analysis, multiple LLM providers, and vector embeddings for
/// professional-grade security assessments.

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
    /// Code parsing utilities
    pub mod parsers;
    /// Protocol and file summarization
    pub mod summarize;
    /// Graph database operations for semantic data
    pub mod graph_db;
    /// Contract inheritance analysis
    pub mod inheritance;
    /// Slither static analyzer interface
    pub mod slither_ffi;
    /// Qdrant vector database operations
    pub mod vector_db;
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
    pub mod prompt_content;
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
    /// Docker volume cleanup utilities
    pub mod delete_docker_volumes;
    /// LLM extraction with retry logic
    pub mod extract_retry;
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
