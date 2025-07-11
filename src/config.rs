/// Configuration management for the AI Agent Audit application.
///
/// This module provides centralized configuration handling, replacing hardcoded
/// constants with environment-based configuration that can be easily modified
/// for different deployment scenarios and testing environments.

use crate::error::{AuditError, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Main configuration structure for the AI Agent Audit application.
///
/// This struct contains all configurable parameters for the audit process,
/// including analysis settings, LLM provider configurations, and system limits.
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
    pub google_ai_api_key: Option<String>,
    
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
    
    /// Maximum number of concurrent LLM requests
    pub max_concurrent_requests: usize,
    
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
            max_depth: 3,
            token_budget: 150_000,
            runs: 3,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
            google_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "info".to_string(),
            docker_volume: "/tmp/audit-analysis".to_string(),
            max_repo_url_length: 2048,
            llm_timeout_seconds: 120,
            max_concurrent_requests: 10,
            default_temperature: 0.7,
            max_response_tokens: 4096,
            vector_dimension: 1536,
            context_chunks: 5,
            similarity_threshold: 0.7,
        }
    }
}

impl AuditConfig {
    /// Creates a new configuration from environment variables.
    ///
    /// This function reads configuration values from environment variables,
    /// falling back to sensible defaults when variables are not set.
    ///
    /// # Returns
    /// * `Result<AuditConfig>` - Configuration loaded from environment
    ///
    /// # Environment Variables
    /// * `AUDIT_MAX_DEPTH` - Call graph traversal depth (default: 3)
    /// * `AUDIT_TOKEN_BUDGET` - Token budget per code block (default: 150000)
    /// * `AUDIT_RUNS` - Number of analysis runs (default: 3)
    /// * `QDRANT_URL` - Vector database URL (default: http://localhost:6334)
    /// * `OPENAI_API_KEY` - OpenAI API key (optional)
    /// * `ANTHROPIC_API_KEY` - Anthropic API key (optional)
    /// * `GOOGLE_AI_API_KEY` - Google AI API key (optional)
    /// * `DEEPSEEK_API_KEY` - DeepSeek API key (optional)
    /// * `RUST_LOG` - Logging level (default: info)
    /// * `DOCKER_VOLUME` - Docker volume path (default: /tmp/audit-analysis)
    /// * `MAX_REPO_URL_LENGTH` - Maximum repo URL length (default: 2048)
    /// * `LLM_TIMEOUT_SECONDS` - LLM request timeout (default: 120)
    /// * `MAX_CONCURRENT_REQUESTS` - Max concurrent requests (default: 10)
    /// * `DEFAULT_TEMPERATURE` - Default LLM temperature (default: 0.7)
    /// * `MAX_RESPONSE_TOKENS` - Max response tokens (default: 4096)
    /// * `VECTOR_DIMENSION` - Vector dimension (default: 1536)
    /// * `CONTEXT_CHUNKS` - Context chunks to retrieve (default: 5)
    /// * `SIMILARITY_THRESHOLD` - Similarity threshold (default: 0.7)
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Load numeric configurations with validation
        if let Ok(max_depth) = env::var("AUDIT_MAX_DEPTH") {
            config.max_depth = max_depth.parse().map_err(|_| {
                AuditError::configuration("AUDIT_MAX_DEPTH", "Invalid number format")
            })?;
            if config.max_depth == 0 || config.max_depth > 10 {
                return Err(AuditError::configuration(
                    "AUDIT_MAX_DEPTH",
                    "Must be between 1 and 10",
                ));
            }
        }

        if let Ok(token_budget) = env::var("AUDIT_TOKEN_BUDGET") {
            config.token_budget = token_budget.parse().map_err(|_| {
                AuditError::configuration("AUDIT_TOKEN_BUDGET", "Invalid number format")
            })?;
            if config.token_budget < 1000 || config.token_budget > 1_000_000 {
                return Err(AuditError::configuration(
                    "AUDIT_TOKEN_BUDGET",
                    "Must be between 1000 and 1000000",
                ));
            }
        }

        if let Ok(runs) = env::var("AUDIT_RUNS") {
            config.runs = runs.parse().map_err(|_| {
                AuditError::configuration("AUDIT_RUNS", "Invalid number format")
            })?;
            if config.runs == 0 || config.runs > 10 {
                return Err(AuditError::configuration(
                    "AUDIT_RUNS",
                    "Must be between 1 and 10",
                ));
            }
        }

        // Load string configurations
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
        config.google_ai_api_key = env::var("GOOGLE_AI_API_KEY").ok();
        config.deepseek_api_key = env::var("DEEPSEEK_API_KEY").ok();

        // Validate at least one API key is provided
        if config.openai_api_key.is_none()
            && config.anthropic_api_key.is_none()
            && config.google_ai_api_key.is_none()
            && config.deepseek_api_key.is_none()
        {
            return Err(AuditError::configuration(
                "API_KEYS",
                "At least one LLM API key must be provided",
            ));
        }

        // Load other configurations
        config.log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        config.docker_volume = env::var("DOCKER_VOLUME").unwrap_or_else(|_| "/tmp/audit-analysis".to_string());

        if let Ok(max_url_len) = env::var("MAX_REPO_URL_LENGTH") {
            config.max_repo_url_length = max_url_len.parse().map_err(|_| {
                AuditError::configuration("MAX_REPO_URL_LENGTH", "Invalid number format")
            })?;
        }

        if let Ok(timeout) = env::var("LLM_TIMEOUT_SECONDS") {
            config.llm_timeout_seconds = timeout.parse().map_err(|_| {
                AuditError::configuration("LLM_TIMEOUT_SECONDS", "Invalid number format")
            })?;
        }

        if let Ok(max_concurrent) = env::var("MAX_CONCURRENT_REQUESTS") {
            config.max_concurrent_requests = max_concurrent.parse().map_err(|_| {
                AuditError::configuration("MAX_CONCURRENT_REQUESTS", "Invalid number format")
            })?;
        }

        if let Ok(temperature) = env::var("DEFAULT_TEMPERATURE") {
            config.default_temperature = temperature.parse().map_err(|_| {
                AuditError::configuration("DEFAULT_TEMPERATURE", "Invalid number format")
            })?;
            if config.default_temperature < 0.0 || config.default_temperature > 2.0 {
                return Err(AuditError::configuration(
                    "DEFAULT_TEMPERATURE",
                    "Must be between 0.0 and 2.0",
                ));
            }
        }

        if let Ok(max_tokens) = env::var("MAX_RESPONSE_TOKENS") {
            config.max_response_tokens = max_tokens.parse().map_err(|_| {
                AuditError::configuration("MAX_RESPONSE_TOKENS", "Invalid number format")
            })?;
        }

        if let Ok(vector_dim) = env::var("VECTOR_DIMENSION") {
            config.vector_dimension = vector_dim.parse().map_err(|_| {
                AuditError::configuration("VECTOR_DIMENSION", "Invalid number format")
            })?;
        }

        if let Ok(context_chunks) = env::var("CONTEXT_CHUNKS") {
            config.context_chunks = context_chunks.parse().map_err(|_| {
                AuditError::configuration("CONTEXT_CHUNKS", "Invalid number format")
            })?;
        }

        if let Ok(similarity) = env::var("SIMILARITY_THRESHOLD") {
            config.similarity_threshold = similarity.parse().map_err(|_| {
                AuditError::configuration("SIMILARITY_THRESHOLD", "Invalid number format")
            })?;
            if config.similarity_threshold < 0.0 || config.similarity_threshold > 1.0 {
                return Err(AuditError::configuration(
                    "SIMILARITY_THRESHOLD",
                    "Must be between 0.0 and 1.0",
                ));
            }
        }

        Ok(config)
    }

    /// Validates the configuration and returns any errors found.
    pub fn validate(&self) -> Result<()> {
        // Validate max_depth
        if self.max_depth == 0 || self.max_depth > 10 {
            return Err(AuditError::configuration(
                "max_depth",
                "Must be between 1 and 10",
            ));
        }

        // Validate token_budget
        if self.token_budget < 1000 || self.token_budget > 1_000_000 {
            return Err(AuditError::configuration(
                "token_budget",
                "Must be between 1000 and 1000000",
            ));
        }

        // Validate runs
        if self.runs == 0 || self.runs > 10 {
            return Err(AuditError::configuration(
                "runs",
                "Must be between 1 and 10",
            ));
        }

        // Validate qdrant_url
        if !self.qdrant_url.starts_with("http://") && !self.qdrant_url.starts_with("https://") {
            return Err(AuditError::configuration(
                "qdrant_url",
                "Must start with http:// or https://",
            ));
        }

        // Validate temperature
        if self.default_temperature < 0.0 || self.default_temperature > 2.0 {
            return Err(AuditError::configuration(
                "default_temperature",
                "Must be between 0.0 and 2.0",
            ));
        }

        // Validate similarity threshold
        if self.similarity_threshold < 0.0 || self.similarity_threshold > 1.0 {
            return Err(AuditError::configuration(
                "similarity_threshold",
                "Must be between 0.0 and 1.0",
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
        self.google_ai_api_key.is_some()
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
            max_depth: 2,
            token_budget: 50_000,
            runs: 1,
            qdrant_url: "http://localhost:6334".to_string(),
            openai_api_key: Some("test-key".to_string()),
            anthropic_api_key: None,
            google_ai_api_key: None,
            deepseek_api_key: None,
            log_level: "debug".to_string(),
            docker_volume: "/tmp/test-audit".to_string(),
            max_repo_url_length: 1024,
            llm_timeout_seconds: 30,
            max_concurrent_requests: 5,
            default_temperature: 0.5,
            max_response_tokens: 2048,
            vector_dimension: 1536,
            context_chunks: 3,
            similarity_threshold: 0.8,
        }
    }
}

/// Global configuration instance.
use std::sync::OnceLock;
static CONFIG: OnceLock<AuditConfig> = OnceLock::new();

/// Initializes the global configuration from environment variables.
pub fn init_config() -> Result<()> {
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
pub fn config() -> &'static AuditConfig {
    CONFIG.get().expect("Configuration not initialized. Call init_config() first.")
}

/// Returns a reference to the global configuration, or None if not initialized.
pub fn try_config() -> Option<&'static AuditConfig> {
    CONFIG.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = AuditConfig::default();
        assert_eq!(config.max_depth, 3);
        assert_eq!(config.token_budget, 150_000);
        assert_eq!(config.runs, 3);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = AuditConfig::default();
        
        // Test invalid max_depth
        config.max_depth = 0;
        assert!(config.validate().is_err());
        
        config.max_depth = 11;
        assert!(config.validate().is_err());
        
        // Test invalid token_budget
        config.max_depth = 3;
        config.token_budget = 500;
        assert!(config.validate().is_err());
        
        // Test invalid temperature
        config.token_budget = 150_000;
        config.default_temperature = 2.5;
        assert!(config.validate().is_err());
        
        // Test valid config
        config.default_temperature = 0.7;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_available_providers() {
        let mut config = AuditConfig::default();
        config.openai_api_key = Some("test".to_string());
        config.anthropic_api_key = Some("test".to_string());
        
        let providers = config.available_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"OpenAI".to_string()));
        assert!(providers.contains(&"Anthropic".to_string()));
    }

    #[test]
    fn test_from_env() {
        env::set_var("AUDIT_MAX_DEPTH", "5");
        env::set_var("AUDIT_TOKEN_BUDGET", "200000");
        env::set_var("OPENAI_API_KEY", "test-key");
        
        let config = AuditConfig::from_env().unwrap();
        assert_eq!(config.max_depth, 5);
        assert_eq!(config.token_budget, 200000);
        assert!(config.has_openai_key());
        
        // Clean up
        env::remove_var("AUDIT_MAX_DEPTH");
        env::remove_var("AUDIT_TOKEN_BUDGET");
        env::remove_var("OPENAI_API_KEY");
    }
}