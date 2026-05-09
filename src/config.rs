use crate::error::{AuditError, Result};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

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
    Code4renaBounty,
    ImmunefiBugBounty,
    Sherlock,
    Cantina,
    Client,
}

// Application constants - these don't need to be configurable via environment
/// Maximum call graph traversal depth for code slice generation
pub const MAX_DEPTH: usize = 3;

/// Maximum token budget per codeblock + context to stay within LLM context limits
pub const TOKEN_BUDGET: usize = 200_000;
pub const NICHE_PATTERN_ANALYSIS_MODE: bool = true;

// if true set DISCOVERY_RUNS accordingly
pub const PATTERN_DISCOVERY_RUNS: usize = 5; // old value 10
pub const INVARIANT_DISCOVERY_RUNS: usize = 5; // old value 5
pub const ACTOR_DISCOVERY_RUNS: usize = 5; // old value 10

pub const OPENAI_MODEL: &str = "gpt-5.5";
pub const OPENAI_REASONING_EFFORT: &str = "high";
pub const OPENAI_SUMMARY_MODEL: &str = "gpt-5.4";
pub const OPENAI_SUMMARY_REASONING_EFFORT: &str = "low";
pub const OPENAI_DEDUP_MODEL: &str = "gpt-5.4";
pub const OPENAI_DEDUP_REASONING_EFFORT: &str = "low";
pub const DISCOVERY_PROVIDER: &str = "openai";
pub const GEMINI_DISCOVERY_MODEL: &str = "gemini-3.1-pro-preview";
pub const DISCOVERY_GEMINI_THINKING_LEVEL: &str = "high";
pub const SKIP_LIBRARIES: bool = true;
pub const SKIP_INVARIANT_RUNS: bool = false;

// SKIP or RUN MAIN PATTERN RUNS
pub const SKIP_ACTOR_PATTERN_RUNS: bool = false;
// RUNS R1 (basic) and R2 (complex) patterns
pub const R1_RUNS: usize = 10; // default: 10 , testing: 5
pub const R2_RUNS: usize = 10; // default: 10 , testing: 5

// NOTE: for large protocols consider reducing scale, skip libs
/// Number of discovery rounds per contract during analysis
pub const INVARIANT_RUNS: usize = 3; // default: 3
pub const ACTOR_RUNS: usize = 2; // default: 2
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

/// Codex/OpenAI review concurrency caps.
pub const MAX_CONCURRENTS_VERIFY: usize = 1;
pub const MAX_CONCURRENTS_REVIEW: usize = 2;
pub const MAX_CONCURRENTS_POC: usize = 1;
pub const MAX_CONCURRENTS_GENERAL: usize = 20;

/// File summarization fanout. Keep this low because Codex-backed summary jobs
/// can retain large prompt/context strings and become memory-heavy.
pub const SUMMARY_MAX_PARALLEL: usize = 50;

/// Recycle pooled Codex app-server sessions after a small number of completed
/// turns so helper subprocesses cannot accumulate unboundedly in one process.
pub const MAX_CODEX_TURNS_PER_SESSION: usize = 10;

/// Default local workspace root for cloned audit targets.
pub const DEFAULT_WORKSPACE_ROOT: &str = "~/Desktop/Audit";

/// Default local directory for SQLite caches and analysis state.
pub const DEFAULT_APP_DATA_DIR: &str = ".ai-agent-audit";
pub const REPO_DATA_DB: &str = "repo_data.db";
pub const SUMMARY_DB: &str = "summary.db";
pub const SEMANTIC_DB: &str = "semantic.db";
pub const CODEBLOCK_DB: &str = "codeblock.db";
pub const FINDINGS_DB: &str = "findings.db";

/// Returns the local application data directory.
///
/// The location can be overridden with `AI_AGENT_AUDIT_DATA_DIR`. By default,
/// the tool stores local cache state in a repo-local `.ai-agent-audit/` folder.
pub fn app_data_dir() -> PathBuf {
    env::var("AI_AGENT_AUDIT_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_APP_DATA_DIR))
}

/// Returns a path inside the application data directory.
pub fn app_db_path(filename: &str) -> PathBuf {
    app_data_dir().join(filename)
}

/// Expands a leading `~/` in a user-facing path.
pub fn expand_home_path(path: &str) -> PathBuf {
    if path == "~" {
        return env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(path));
    }

    if let Some(rest) = path.strip_prefix("~/") {
        return env::var_os("HOME")
            .map(|home| PathBuf::from(home).join(rest))
            .unwrap_or_else(|| PathBuf::from(path));
    }

    PathBuf::from(path)
}

/// Maximum repository URL length for security validation
pub const MAX_REPO_URL_LENGTH: usize = 2048;

/// Timeout for LLM requests in seconds
pub const LLM_TIMEOUT_SECONDS: u64 = 120;

/// Default temperature for LLM models
pub const DEFAULT_TEMPERATURE: f64 = 1.0;

/// Maximum tokens for LLM responses
pub const MAX_RESPONSE_TOKENS: u64 = 100_000;

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

    /// OpenAI API key for GPT models
    pub openai_api_key: Option<String>,

    /// Anthropic API key for Claude models
    pub anthropic_api_key: Option<String>,

    /// Google AI API key for Gemini models
    pub gemini_ai_api_key: Option<String>,

    /// DeepSeek API key for DeepSeek models
    pub deepseek_api_key: Option<String>,

    /// Provider to use for discovery-style runs (patterns, actors, invariants)
    pub discovery_provider: String,

    /// Effective model used for the configured discovery provider
    pub discovery_model: String,

    /// Gemini thinking level for discovery when `discovery_provider=gemini`
    pub discovery_gemini_thinking_level: String,

    /// Logging level for the application
    pub log_level: String,

    /// Local workspace root for cloned audit targets
    pub workspace_root: String,

    /// Maximum repository URL length for security validation
    pub max_repo_url_length: usize,

    /// Timeout for LLM requests in seconds
    pub llm_timeout_seconds: u64,

    /// Default temperature for LLM models
    pub default_temperature: f64,

    /// Maximum tokens for LLM responses
    pub max_response_tokens: u64,
}

impl Default for AuditConfig {
    fn default() -> Self {
        let discovery_provider = DISCOVERY_PROVIDER.to_string();
        let discovery_model = if discovery_provider.eq_ignore_ascii_case("gemini") {
            GEMINI_DISCOVERY_MODEL.to_string()
        } else {
            OPENAI_MODEL.to_string()
        };

        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: PATTERN_DISCOVERY_RUNS,
            openai_api_key: None,
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            discovery_provider,
            discovery_model,
            discovery_gemini_thinking_level: DISCOVERY_GEMINI_THINKING_LEVEL.to_string(),
            log_level: "info".to_string(),
            workspace_root: DEFAULT_WORKSPACE_ROOT.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
        }
    }
}

impl AuditConfig {
    /// Creates a new configuration from environment variables.
    ///
    /// This function reads runtime-sensitive configuration from environment variables
    /// (API keys, logging, filesystem overrides) and uses code constants for model/provider selection.
    ///
    /// # Returns
    /// * `Result<AuditConfig>` - Configuration loaded from environment
    ///
    /// # Environment Variables
    /// * `OPENAI_API_KEY` - Legacy OpenAI API key (optional, not used by the default Codex path)
    /// * `ANTHROPIC_API_KEY` - Anthropic API key (optional)
    /// * `GEMINI_API_KEY` - Gemini AI API key (optional)
    /// * `GOOGLE_AI_API_KEY` - Legacy Gemini env var alias (optional)
    /// * `DEEPSEEK_API_KEY` - DeepSeek API key (optional)
    /// * `RUST_LOG` - Logging level (default: info)
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Load API keys (optional)
        config.openai_api_key = env::var("OPENAI_API_KEY").ok();
        config.anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok();
        config.gemini_ai_api_key = env::var("GEMINI_API_KEY")
            .or_else(|_| env::var("GOOGLE_AI_API_KEY"))
            .ok();
        config.deepseek_api_key = env::var("DEEPSEEK_API_KEY").ok();
        // Load logging level
        config.log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        config.workspace_root =
            env::var("AI_AGENT_AUDIT_WORKSPACE_ROOT").unwrap_or(config.workspace_root);

        Ok(config)
    }

    /// Validates the configuration and returns any errors found.
    pub fn validate(&self) -> Result<()> {
        // Stronger API key validation: reject placeholders and obviously invalid keys
        // for active API-key based providers. OPENAI_API_KEY is legacy-only, so an
        // invalid leftover value should not block the default OAuth startup path.
        use crate::utils::env_security::is_placeholder_api_key;
        if let Some(k) = &self.openai_api_key {
            if is_placeholder_api_key(k) || !k.starts_with("sk-") {
                log::warn!(
                    "Ignoring legacy OPENAI_API_KEY because it does not look like a real API key; default OpenAI path uses ChatGPT/Codex OAuth."
                );
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

        let discovery_provider = self.discovery_provider.to_ascii_lowercase();
        if !matches!(discovery_provider.as_str(), "openai" | "gemini") {
            return Err(AuditError::configuration(
                "DISCOVERY_PROVIDER",
                format!(
                    "Unsupported discovery provider '{}'. Valid options: openai, gemini",
                    self.discovery_provider
                ),
            ));
        }

        let gemini_thinking = self.discovery_gemini_thinking_level.to_ascii_lowercase();
        if !matches!(gemini_thinking.as_str(), "low" | "high") {
            return Err(AuditError::configuration(
                "DISCOVERY_GEMINI_THINKING_LEVEL",
                format!(
                    "Unsupported Gemini discovery thinking level '{}'. Valid options: low, high",
                    self.discovery_gemini_thinking_level
                ),
            ));
        }

        if discovery_provider == "gemini" && !self.has_google_ai_key() {
            log::warn!(
                "DISCOVERY_PROVIDER is set to gemini, but GEMINI_API_KEY / GOOGLE_AI_API_KEY is not configured. Discovery runs will fail until a Gemini key is provided."
            );
        }

        Ok(())
    }

    /// Returns true if OpenAI API key is configured.
    pub fn has_openai_key(&self) -> bool {
        use crate::utils::env_security::is_placeholder_api_key;

        self.openai_api_key
            .as_deref()
            .is_some_and(|key| key.starts_with("sk-") && !is_placeholder_api_key(key))
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

    /// Returns the normalized provider used for discovery-style runs.
    pub fn discovery_provider_name(&self) -> &str {
        &self.discovery_provider
    }

    /// Returns the effective model name used for discovery-style runs.
    pub fn discovery_model_name(&self) -> &str {
        &self.discovery_model
    }

    /// Returns a list of configured LLM providers.
    pub fn available_providers(&self) -> Vec<String> {
        let mut providers = vec!["OpenAI".to_string()];
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

    /// Returns the expanded local workspace root.
    pub fn workspace_root_path(&self) -> PathBuf {
        expand_home_path(&self.workspace_root)
    }

    /// Creates a test configuration with minimal settings.
    #[cfg(test)]
    pub fn test_config() -> Self {
        Self {
            max_depth: MAX_DEPTH,
            token_budget: TOKEN_BUDGET,
            runs: PATTERN_DISCOVERY_RUNS,
            openai_api_key: Some("sk-test-key".to_string()),
            anthropic_api_key: None,
            gemini_ai_api_key: None,
            deepseek_api_key: None,
            discovery_provider: DISCOVERY_PROVIDER.to_string(),
            discovery_model: if DISCOVERY_PROVIDER.eq_ignore_ascii_case("gemini") {
                GEMINI_DISCOVERY_MODEL.to_string()
            } else {
                OPENAI_MODEL.to_string()
            },
            discovery_gemini_thinking_level: DISCOVERY_GEMINI_THINKING_LEVEL.to_string(),
            log_level: "debug".to_string(),
            workspace_root: DEFAULT_WORKSPACE_ROOT.to_string(),
            max_repo_url_length: MAX_REPO_URL_LENGTH,
            llm_timeout_seconds: LLM_TIMEOUT_SECONDS,
            default_temperature: DEFAULT_TEMPERATURE,
            max_response_tokens: MAX_RESPONSE_TOKENS,
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
        assert_eq!(OPENAI_MODEL, "gpt-5.5");
        assert_eq!(OPENAI_SUMMARY_MODEL, "gpt-5.4");
        assert_eq!(OPENAI_DEDUP_MODEL, "gpt-5.4");
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

        // Test no API keys
        config.openai_api_key = None;
        assert!(config.validate().is_ok());

        // Legacy placeholder/invalid OpenAI keys should not block OAuth startup.
        config.openai_api_key = Some("your_openai_api_key".to_string());
        assert!(config.validate().is_ok());
        assert!(!config.has_openai_key());

        config.openai_api_key = Some("not-a-real-openai-key".to_string());
        assert!(config.validate().is_ok());
        assert!(!config.has_openai_key());
    }

    #[test]
    fn test_available_providers() {
        let config = AuditConfig {
            anthropic_api_key: Some("test".to_string()),
            ..AuditConfig::default()
        };

        let providers = config.available_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&"OpenAI".to_string()));
        assert!(providers.contains(&"Anthropic".to_string()));
    }

    #[test]
    fn test_workspace_root_expands_home() {
        let config = AuditConfig {
            workspace_root: "~/Desktop/Audit".to_string(),
            ..AuditConfig::default()
        };

        if let Some(home) = env::var_os("HOME") {
            assert_eq!(
                config.workspace_root_path(),
                PathBuf::from(home).join("Desktop/Audit")
            );
        }
    }

    #[test]
    fn test_discovery_model_follows_selected_provider() {
        let mut config = AuditConfig {
            discovery_provider: "openai".to_string(),
            discovery_model: GEMINI_DISCOVERY_MODEL.to_string(),
            ..AuditConfig::default()
        };

        assert_eq!(config.discovery_provider_name(), "openai");
        assert_eq!(config.discovery_model_name(), GEMINI_DISCOVERY_MODEL);

        config.discovery_provider = "gemini".to_string();
        config.gemini_ai_api_key = Some("test-gemini-key".to_string());
        config.discovery_model = GEMINI_DISCOVERY_MODEL.to_string();
        assert_eq!(config.discovery_model_name(), GEMINI_DISCOVERY_MODEL);
    }

    #[test]
    fn test_from_env() {
        unsafe {
            env::set_var("OPENAI_API_KEY", "sk-valid-12345");
            env::set_var("GEMINI_API_KEY", "test-gemini-key");
        }

        let config = AuditConfig::from_env().unwrap();
        assert!(config.has_openai_key());
        assert!(config.has_google_ai_key());
        assert_eq!(config.discovery_provider_name(), DISCOVERY_PROVIDER);
        assert_eq!(
            config.discovery_model_name(),
            if DISCOVERY_PROVIDER.eq_ignore_ascii_case("gemini") {
                GEMINI_DISCOVERY_MODEL
            } else {
                OPENAI_MODEL
            }
        );
        assert_eq!(
            config.discovery_gemini_thinking_level,
            DISCOVERY_GEMINI_THINKING_LEVEL
        );
        // Constants should be used for other values
        assert_eq!(config.max_depth, MAX_DEPTH);
        assert_eq!(config.token_budget, TOKEN_BUDGET);

        // Clean up
        unsafe {
            env::remove_var("OPENAI_API_KEY");
            env::remove_var("GEMINI_API_KEY");
        }
    }
}
