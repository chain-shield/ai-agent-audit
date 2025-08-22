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
    pub mod repo_data;
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
    pub mod contract_file_map;
    /// AI agent and vulnerability type enums
    pub mod enums;
    pub mod patterns;
    pub mod semaphore;
    /// Security audit phases
    pub mod dynamic_prompts {
        pub mod inv_findings;
        pub mod invariants;
        pub mod pattern_findings;
        pub mod patterns;
    }
    pub mod phases {
        /// Phase 2: Parallel vulnerability detection across multiple AI agents
        pub mod generate_findings;
        /// Phase 1: AI-driven file selection and context prefetching
        pub mod prefetch_context;
        /// Phase 4: Quality assurance and final finding refinement
        pub mod quality_check;
        /// Phase 3a: Check finding are in scope, if scope is provided
        pub mod scope_findings;
        /// Phase 3: Deduplication and verification of discovered security findings
        pub mod verify_findings;
    }
    /// Utility functions for LLM review
    pub mod utils {
        /// Dynamic prompt generation and context management
        pub mod prompt_context;
        /// AI agent builders and utilities
        pub mod review_utils;
    }
    /// Prompt engineering modules for different analysis stages
    pub mod prompt_support {
        /// Deduplication prompts
        pub mod dedup;
        pub mod extractor_prompt;
        pub mod planner_prompt;
        pub mod post_file_select_prompt;
        /// Post-analysis prompts
        pub mod post_prompt;
        /// Quality check prompts
        pub mod post_qualify;
        /// Verification prompts
        pub mod post_verify;
        pub mod pre_file_select_prompt;
        /// Pre-analysis prompts
        pub mod pre_prompt;
        /// Pre-qualification prompts
        pub mod pre_qualify;
        /// Pre-verification prompts
        pub mod pre_verify;
        /// Quality assessment prompts
        pub mod qualify_prompt;
        pub mod severity_rubics;
        /// Verification prompts
        pub mod verify_prompt;
    }
}

pub mod cli_args {
    pub mod parse;
}

// /// Test module for rig-core API testing
// pub mod test_rig;
/// Cost tracking and management for LLM inference
pub mod cost {
    /// Cost calculation and tracking across providers
    pub mod cost_data;
}

/// AI agent implementations with vector search
pub mod ai_bot {
    pub mod file_picker;
    pub mod file_retrival;
    /// Context retrieval for AI analysis
    pub mod retrieve_slice;
}

/// Master security analysis prompts
pub mod master_prompts {
    /// Security analysis prompt variants
    pub mod code4rena;
    /// Base master security prompt
    pub mod master_prompt;
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
    pub mod check_folder_name;
    pub mod contract_name_check;
    /// Docker volume cleanup utilities
    pub mod delete_docker_volumes;
    pub mod env_security;
    /// LLM extraction with retry logic
    pub mod extract_retry;
    pub mod file_security;
    /// Function labeling utilities
    pub mod fn_labels;
    /// Function name extraction
    pub mod get_fn_name;
    /// Logging utilities
    pub mod logging;
    /// Text sanitization utilities
    pub mod sanitize;
    /// Vector database connection utilities
    pub mod vec_db_connect;
}
