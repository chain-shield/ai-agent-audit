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
    /// Inheritance relationship mapping with (contract, file) tuples
    pub mod inheritance_map;
    /// Code parsing utilities
    pub mod parsers;
    /// Slither static analyzer interface
    pub mod slither_ffi;
    /// Protocol and file summarization
    pub mod summarize;
    pub mod summarize_db;
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
    pub mod competition_reports;
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
    pub mod codeblocks;
    /// Core code slicing functionality
    pub mod extract_ir;
    /// Interface implementation detection
    pub mod interface_implementations;
    pub mod libraries;
    pub mod parse_solidity;
    /// Enumeration utilities
    pub mod utils;
}

/// AI-powered security analysis and LLM integration
pub mod llm_review {
    /// AI agent factory for centralized agent creation
    pub mod agent_factory;
    /// Analysis results database
    pub mod analysis_db;
    pub mod code_review_v2;
    /// Global context management
    pub mod context_state;
    pub mod contract_category;
    pub mod contract_file_map;
    /// AI agent and vulnerability type enums
    pub mod enums;
    /// LLM configuration and models
    pub mod findings;
    pub mod invariants;
    pub mod issues;
    pub mod pattern_category;
    pub mod patterns;
    pub mod semaphore;
    /// Security audit phases
    pub mod dynamic_prompts {
        pub mod findings_template;
        pub mod inv_findings;
        pub mod invariants;
        pub mod pattern_findings;
        pub mod patterns;
    }
    pub mod phases {
        pub mod add_poc_findings;
        /// Phase 1: AI-driven file selection and context prefetching
        pub mod prefetch_context;
        /// Phase 5: Quality assurance and final finding refinement
        pub mod quality_check;
        /// Phase 3: Deduplication and verification of discovered security findings
        pub mod verify_findings;
        // Phase 7: Create professaionl competition-grade report
        pub mod create_report;
    }
    pub mod pattern_phases {
        /// Phase 2: Parallel vulnerability detection across multiple AI agents
        pub mod generate_patterns;
        pub mod multipattern_to_findings;
        pub mod pattern_to_findings;
        /// Phase 3: Deduplication and verification of discovered security findings
        pub mod verify_patterns;
    }
    /// Utility functions for LLM review
    pub mod utils {
        pub mod contract_in_scope;
        /// Dynamic prompt generation and context management
        pub mod prompt_context;
        /// AI agent builders and utilities
        pub mod review_utils;
        pub mod save_run_poc;
    }
    /// Prompt engineering modules for different analysis stages
    pub mod prompt_support {
        pub mod create_report_prompt;
        /// Deduplication prompts
        pub mod dedup;
        pub mod extractor_prompt;
        pub mod make_poc_prompt;
        pub mod planner_prompt;
        pub mod post_file_select_prompt;
        pub mod post_poc;
        /// Verification prompts
        pub mod pre_file_select_prompt;
        pub mod pre_poc;
        pub mod report_templates;
        pub mod severity_rubics;
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

#[cfg(test)]
pub mod test_support {
    pub mod solidity_mocks;
}

/// Shared utilities and helper functions
pub mod utils {
    /// OpenAI tokenizer (BPE) for text chunking
    pub mod bpe;
    pub mod check_folder_name;
    pub mod contract_name_check;
    /// Docker volume cleanup utilities
    pub mod delete_docker_volumes;
    pub mod display_file;
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
    pub mod parse_library_file;
    pub mod read_file;
    /// Solidity import remapping utilities
    pub mod remapping;
    /// Text sanitization utilities
    pub mod sanitize;
    pub mod semantic_compare;
    /// Vector database connection utilities
    pub mod vec_db_connect;
}
