use crate::error::{AuditError, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Configuration management for the AI Agent Audit application.
///
/// This module provides centralized configuration handling, with constants
/// for application settings and environment variables only for sensitive
/// configuration like API keys and URLs.

#[derive(
    Debug, Clone, PartialEq, Eq, Deserialize, strum_macros::EnumString, strum_macros::Display,
)]
pub enum AuditType {
    Code4rena,
    Sherlock,
    Cantina,
    Client,
}

// Application constants - these don't need to be configurable via environment
/// Maximum call graph traversal depth for code slice generation
pub const MAX_DEPTH: usize = 3;

/// Maximum token budget per codeblock + context to stay within LLM context limits
pub const TOKEN_BUDGET: usize = 200_000;
pub const CREATE_TESTS: bool = false;
pub const NICHE_PATTERN_ANALYSIS_MODE: bool = true;

// if true set DISCOVERY_RUNS accordingly
pub const PATTERN_DISCOVERY_RUNS: usize = 5; // old value 10
pub const INVARIANT_DISCOVERY_RUNS: usize = 3; // old value 5
pub const ACTOR_DISCOVERY_RUNS: usize = 5; // old value 10

pub const OPENAI_MODEL: &str = "gpt-5.2";
pub const SKIP_LIBRARIES: bool = true;
pub const SKIP_INVARIANT_RUNS: bool = false;

// SKIP or RUN MAIN PATTERN RUNS
pub const SKIP_ACTOR_PATTERN_RUNS: bool = false;
// RUNS R1 (basic) and R2 (complex) patterns
pub const R1_RUNS: usize = 10; // default: 10 , testing: 5
pub const R2_RUNS: usize = 10; // default: 10 , testing: 5

// NOTE: for large protocols consider reducing scale, skip libs
/// Number of discovery rounds per contract during analysis
pub const INVARIANT_RUNS: usize = 3;
pub const ACTOR_RUNS: usize = 2;
pub const MAX_PATTERNS_FOR_PROMPT: usize = 32; // too many patterns and performance drops

pub const MAX_PATTERN_RUN_TOP: usize = 3; // 2 for large protocol, default: 3
pub const MAX_PATTERN_RUN_RARE: usize = 3; // 2 for large protocol, default: 3
pub const MAX_PATTERN_RUN_MOST: usize = 3; // 0 for large protocol, default: 3
pub const MAX_PATTERN_RUN_FREQUENT: usize = 3; // 2 for large protocol, default: 3
pub const MAX_PATTERN_RELEVANT_FREQUENT: usize = 20; // 2 for large protocol, default: 3
pub const MAX_PATTERN_LIBRARY: usize = 3;
pub const MAX_PATTERN_NICHE: usize = 4; // 3 for large protocol, default: 4
pub const MAX_PATTERN_GENERAL: usize = 4; // 2 for large protocol, default: 4

pub const MAX_FILE_RUNS: usize = 1;

pub const MAX_RAG_QUERY_CONTENT_LENGTH: usize = 8192; // 8192 token limit for embedding

/// Docker volume path for repository analysis
pub const DOCKER_VOLUME: &str = "/tmp/audit-analysis";

pub const CHAINSHIELD_DB_FOLDER: &str = "/Users/apmfree/chainshield_db";
pub const REPO_DATA_DB: &str = "repo_data.db";
pub const SUMMARY_DB: &str = "summary.db";
pub const SEMANTIC_DB: &str = "semantic.db";
pub const CODEBLOCK_DB: &str = "codeblock.db";
pub const FINDINGS_DB: &str = "findings.db";

/// Maximum repository URL length for security validation
pub const MAX_REPO_URL_LENGTH: usize = 2048;

/// Timeout for LLM requests in seconds
pub const LLM_TIMEOUT_SECONDS: u64 = 120;

/// Default temperature for LLM models
pub const DEFAULT_TEMPERATURE: f64 = 1.0;

/// Maximum tokens for LLM responses
pub const MAX_RESPONSE_TOKENS: u64 = 100_000;

/// Vector database collection dimension
pub const VECTOR_DIMENSION: u64 = 1536;

/// Number of similar chunks to retrieve for context
pub const CONTEXT_CHUNKS: usize = 5;

/// Similarity threshold for vector search
pub const SIMILARITY_THRESHOLD: f64 = 0.7;

/// Main configuration structure for the AI Agent Audit application.
///
/// This struct contains configurable parameters loaded from environment variables
/// for sensitive data (API keys, URLs) and constants for application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Maximum call graph traversal depth for code slice generation
    pub max_depth: usize,

    /// Maximum token budget per code block to stay within LLM context limits
    pub token_budget: usize,

    /// Number of discovery rounds per contract during analysis
    pub runs: usize,

    /// Qdrant vector database URL for semantic search
    pub qdrant_url: String,

    /// OpenAI API key for GPT models and embeddings
    pub openai_api_key: Option<String>,

    /// Anthropic API key for Claude models
    pub anthropic_api_key: Option<String>,

    /// Google AI API key for Gemini models
    pub gemini_ai_api_key: Option<String>,

    /// DeepSeek API key for DeepSeek models
    pub deepseek_api_key: Option<String>,

    /// Logging level for the application
    pub log_level: String,

    /// Docker volume path for repository analysis
    pub docker_volume: String,

    /// Maximum repository URL length for security validation
    pub max_repo_url_length: usize,

    /// Timeout for LLM requests in seconds
    pub llm_timeout_seconds: u64,

    /// Default temperature for LLM models
    pub default_temperature: f64,

    /// Maximum tokens for LLM responses
    pub max_response_tokens: u64,

    /// Vector database collection dimension
    pub vector_dimension: u64,

    /// Number of similar chunks to retrieve for context
    pub context_chunks: usize,

    /// Similarity threshold for vector search
    pub similarity_threshold: f64,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: PATTERN_DISCOVERY_RUNS,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "info".to_string(),
            docker_volume: DOCKER_VOLUME.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
            vector_dimension: VECTOR_DIMENSION,
            context_chunks: CONTEXT_CHUNKS,
            similarity_threshold: SIMILARITY_THRESHOLD,
        }
    }
}

impl AuditConfig {
    /// Creates a new configuration from environment variables.
    ///
    /// This function reads only sensitive configuration from environment variables
    /// (API keys, URLs, logging) and uses constants for application settings.
    ///
    /// # Returns
    /// * `Result<AuditConfig>` - Configuration loaded from environment
    ///
    /// # Environment Variables
    /// * `QDRANT_URL` - Vector database URL (default: http://localhost:6334)
    /// * `OPENAI_API_KEY` - OpenAI API key (optional)
    /// * `ANTHROPIC_API_KEY` - Anthropic API key (optional)
    /// * `GEMINI_API_KEY` - Gemini AI API key (optional)
    /// * `DEEPSEEK_API_KEY` - DeepSeek API key (optional)
    /// * `RUST_LOG` - Logging level (default: info)
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Load Qdrant URL
        if let Ok(qdrant_url) = env::var("QDRANT_URL") {
            if !qdrant_url.starts_with("http://") && !qdrant_url.starts_with("https://") {
                return Err(AuditError::configuration(
                    "QDRANT_URL",
                    "Must start with http:// or https://",
                ));
            }
            config.qdrant_url = qdrant_url;
        }

        // Load API keys (optional)
        config.openai_api_key = env::var("OPENAI_API_KEY").ok();
        config.anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();
        config.gemini_ai_api_key = env::var("GEMINI_API_KEY").ok();
        config.deepseek_api_key = env::var("DEEPSEEK_API_KEY").ok();

        // Validate at least one API key is provided
        if config.openai_api_key.is_none()
            && config.anthropic_api_key.is_none()
            && config.gemini_ai_api_key.is_none()
            && config.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        // Load logging level
        config.log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(config)
    }

    /// Validates the configuration and returns any errors found.
    pub fn validate(&self) -> Result<()> {
        // Validate qdrant_url
        if !self.qdrant_url.starts_with("http://") && !self.qdrant_url.starts_with("https://") {
            return Err(AuditError::configuration(
                "qdrant_url",
                "Must start with http:// or https://",
            ));
        }

        // Validate at least one API key is provided
        if self.openai_api_key.is_none()
            && self.anthropic_api_key.is_none()
            && self.gemini_ai_api_key.is_none()
            && self.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        // Stronger API key validation: reject placeholders and obviously invalid keys
        use crate::utils::env_security::is_placeholder_api_key;
        if let Some(k) = &self.openai_api_key {
            if is_placeholder_api_key(k) {
                return Err(AuditError::configuration(
                    "OPENAI_API_KEY",
                    "Appears to be a placeholder key (e.g., 'your-key-here'). Provide a real API key.",
                ));
            }
            // Basic OpenAI format check
            if !k.starts_with("sk-") {
                return Err(AuditError::configuration(
                    "OPENAI_API_KEY",
                    "OpenAI key should start with 'sk-'. Provide a real API key.",
                ));
            }
        }
        if let Some(k) = &self.anthropic_api_key
            && is_placeholder_api_key(k)
        {
            return Err(AuditError::configuration(
                "ANTHROPIC_API_KEY",
                "Appears to be a placeholder key",
            ));
        }
        if let Some(k) = &self.gemini_ai_api_key
            && is_placeholder_api_key(k)
        {
            return Err(AuditError::configuration(
                "GEMINI_API_KEY",
                "Appears to be a placeholder key",
            ));
        }
        if let Some(k) = &self.deepseek_api_key
            && is_placeholder_api_key(k)
        {
            return Err(AuditError::configuration(
                "DEEPSEEK_API_KEY",
                "Appears to be a placeholder key",
            ));
        }

        Ok(())
    }

    /// Returns true if OpenAI API key is configured.
    pub fn has_openai_key(&self) -> bool {
        self.openai_api_key.is_some()
    }

    /// Returns true if Anthropic API key is configured.
    pub fn has_anthropic_key(&self) -> bool {
        self.anthropic_api_key.is_some()
    }

    /// Returns true if Google AI API key is configured.
    pub fn has_google_ai_key(&self) -> bool {
        self.gemini_ai_api_key.is_some()
    }

    /// Returns true if DeepSeek API key is configured.
    pub fn has_deepseek_key(&self) -> bool {
        self.deepseek_api_key.is_some()
    }

    /// Returns a list of configured LLM providers.
    pub fn available_providers(&self) -> Vec<String> {
        let mut providers = Vec::new();
        if self.has_openai_key() {
            providers.push("OpenAI".to_string());
        }
        if self.has_anthropic_key() {
            providers.push("Anthropic".to_string());
        }
        if self.has_google_ai_key() {
            providers.push("Google AI".to_string());
        }
        if self.has_deepseek_key() {
            providers.push("DeepSeek".to_string());
        }
        providers
    }

    /// Creates a test configuration with minimal settings.
    #[cfg(test)]
    pub fn test_config() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: PATTERN_DISCOVERY_RUNS,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: Some("test-key".to_string()),
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "debug".to_string(),
            docker_volume: DOCKER_VOLUME.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
            vector_dimension: VECTOR_DIMENSION,
            context_chunks: CONTEXT_CHUNKS,
            similarity_threshold: SIMILARITY_THRESHOLD,
        }
    }
}

/// Global configuration instance.
use std::sync::OnceLock;
static CONFIG: OnceLock<AuditConfig> = OnceLock::new();

/// Initializes the global configuration from environment variables.
pub fn init_config() -> Result<()> {
    // Ensure .env is loaded even if the caller forgot; safe to call multiple times
    // Load .env and override any existing env vars to ensure repo-root .env wins in app runs
    dotenvy::dotenv_override().ok();

    let config = AuditConfig::from_env()?;
    config.validate()?;

    CONFIG.set(config).map_err(|_| {
        AuditError::configuration("global_config", "Configuration already initialized")
    })?;

    Ok(())
}

/// Returns a reference to the global configuration.
///
/// # Panics
/// Panics if the configuration has not been initialized with `init_config()`.
pub fn audit_config() -> &'static AuditConfig {
    CONFIG
        .get()
        .expect("Configuration not initialized. Call init_config() first.")
}

/// Returns a reference to the global configuration, or None if not initialized.
pub fn try_audit_config() -> Option<&'static AuditConfig> {
    CONFIG.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = AuditConfig::default();
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);
        assert_eq!(config.runs, PATTERN_DISCOVERY_RUNS);
        // Config validation will fail because no API keys are set
    }

    #[test]
    fn test_config_validation() {
        let mut config = AuditConfig {
            openai_api_key: Some("sk-valid-12345".to_string()),
            ..AuditConfig::default()
        };

        // Set a realistic OpenAI-style key to make validation pass
        assert!(config.validate().is_ok());

        // Test invalid URL
        config.qdrant_url = "invalid-url".to_string();
        assert!(config.validate().is_err());

        // Test no API keys
        config.qdrant_url = "http://localhost:6334".to_string();
        config.openai_api_key = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_available_providers() {
        let config = AuditConfig {
            openai_api_key: Some("test".to_string()),
            anthropic_api_key: Some("test".to_string()),
            ..AuditConfig::default()
        };

        let providers = config.available_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"OpenAI".to_string()));
        assert!(providers.contains(&"Anthropic".to_string()));
    }

    #[test]
    fn test_from_env() {
        unsafe {
            env::set_var("QDRANT_URL", "http://test:6334");
            env::set_var("OPENAI_API_KEY", "sk-valid-12345");
        }

        let config = AuditConfig::from_env().unwrap();
        assert_eq!(config.qdrant_url, "http://test:6334");
        assert!(config.has_openai_key());
        // Constants should be used for other values
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);

        // Clean up
        unsafe {
            env::remove_var("QDRANT_URL");
            env::remove_var("OPENAI_API_KEY");
        }
    }
}
