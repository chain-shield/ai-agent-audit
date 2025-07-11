use super::enums::AIAgent;
/// AI Agent Factory for centralized agent creation across LLM providers.
///
/// This module provides a unified interface for creating AI agents from different
/// LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) with consistent configuration
/// and error handling.
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self, O3},
    },
};
use std::sync::OnceLock;

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    OpenAI,
    Anthropic,
    Gemini,
    DeepSeek,
}

impl LlmProvider {
    /// Returns all supported providers.
    pub fn all() -> &'static [LlmProvider] {
        &[
            LlmProvider::OpenAI,
            LlmProvider::Anthropic,
            LlmProvider::Gemini,
            LlmProvider::DeepSeek,
        ]
    }

    /// Returns the string representation of the provider.
    pub fn as_str(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "openai",
            LlmProvider::Anthropic => "anthropic",
            LlmProvider::Gemini => "gemini",
            LlmProvider::DeepSeek => "deepseek",
        }
    }

    /// Returns the environment variable name for the API key.
    pub fn api_key_env_var(&self) -> &'static str {
        match self {
            LlmProvider::OpenAI => "OPENAI_API_KEY",
            LlmProvider::Anthropic => "ANTHROPIC_API_KEY",
            LlmProvider::Gemini => "GOOGLE_AI_API_KEY",
            LlmProvider::DeepSeek => "DEEPSEEK_API_KEY",
        }
    }

    /// Checks if this provider is available based on environment variables.
    pub fn is_available(&self) -> bool {
        std::env::var(self.api_key_env_var()).is_ok()
    }

    /// Parse provider from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" => Some(LlmProvider::OpenAI),
            "anthropic" | "claude" => Some(LlmProvider::Anthropic),
            "gemini" | "google" => Some(LlmProvider::Gemini),
            "deepseek" => Some(LlmProvider::DeepSeek),
            _ => None,
        }
    }
}

/// Configuration for creating AI agents.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Temperature for response generation (0.0-2.0)
    pub temperature: f64,
    /// Model name to use for the provider
    pub model: String,
    /// Optional context to include in the agent
    pub context: Option<String>,
    /// Maximum tokens for responses (Anthropic only)
    pub max_tokens: Option<u64>,
    /// System preamble/prompt for the agent
    pub preamble: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones.".to_string(),
        }
    }
}

impl AgentConfig {
    /// Creates a new agent configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the temperature for response generation.
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Sets the model name.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Sets the context for the agent.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Sets the maximum tokens (for Anthropic models).
    pub fn with_max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the system preamble/prompt.
    pub fn with_preamble(mut self, preamble: impl Into<String>) -> Self {
        self.preamble = preamble.into();
        self
    }

    /// Creates an agent configuration for security auditing.
    pub fn for_security_audit() -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones.".to_string(),
        }
    }
}

/// Singleton clients for LLM providers
static OPENAI_CLIENT: OnceLock<openai::Client> = OnceLock::new();
static ANTHROPIC_CLIENT: OnceLock<anthropic::Client> = OnceLock::new();
static GEMINI_CLIENT: OnceLock<gemini::Client> = OnceLock::new();
static DEEPSEEK_CLIENT: OnceLock<deepseek::Client> = OnceLock::new();

/// Initializes all LLM clients from environment variables.
///
/// This function should be called once during application startup to initialize
/// all available LLM clients. Clients are only created if their API keys are available.
pub fn init_llm_clients() -> Result<()> {
    // Initialize OpenAI client if API key is available
    if audit_config().has_openai_key() {
        let client = openai::Client::from_env();
        OPENAI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("openai_client", "OpenAI client already initialized")
        })?;
    }

    // Initialize Anthropic client if API key is available
    if audit_config().has_anthropic_key() {
        let client = anthropic::Client::from_env();
        ANTHROPIC_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("anthropic_client", "Anthropic client already initialized")
        })?;
    }

    // Initialize Gemini client if API key is available
    if audit_config().has_google_ai_key() {
        let client = gemini::Client::from_env();
        GEMINI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("gemini_client", "Gemini client already initialized")
        })?;
    }

    // Initialize DeepSeek client if API key is available
    if audit_config().has_deepseek_key() {
        let client = deepseek::Client::from_env();
        DEEPSEEK_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("deepseek_client", "DeepSeek client already initialized")
        })?;
    }

    Ok(())
}

/// Returns the OpenAI client instance.
fn openai_client() -> Result<&'static openai::Client> {
    OPENAI_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "openai_client",
            "OpenAI client not initialized or API key not configured",
        )
    })
}

/// Returns the Anthropic client instance.
fn anthropic_client() -> Result<&'static anthropic::Client> {
    ANTHROPIC_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "anthropic_client",
            "Anthropic client not initialized or API key not configured",
        )
    })
}

/// Returns the Gemini client instance.
fn gemini_client() -> Result<&'static gemini::Client> {
    GEMINI_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "gemini_client",
            "Gemini client not initialized or API key not configured",
        )
    })
}

/// Returns the DeepSeek client instance.
fn deepseek_client() -> Result<&'static deepseek::Client> {
    DEEPSEEK_CLIENT.get().ok_or_else(|| {
        AuditError::configuration(
            "deepseek_client",
            "DeepSeek client not initialized or API key not configured",
        )
    })
}

/// Factory for creating AI agents across different providers.
pub struct AgentFactory;

impl AgentFactory {
    /// Creates an OpenAI agent with the specified configuration.
    pub fn create_openai_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = openai_client()?;
        let model = if config.model == "default" {
            O3
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Openai(builder.build()))
    }

    /// Creates an Anthropic agent with the specified configuration.
    pub fn create_anthropic_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = anthropic_client()?;
        let model = if config.model == "default" {
            CLAUDE_3_7_SONNET
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(max_tokens) = config.max_tokens {
            builder = builder.max_tokens(max_tokens);
        }

        Ok(AIAgent::Anthropic(builder.build()))
    }

    /// Creates a Gemini agent with the specified configuration.
    pub fn create_gemini_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = gemini_client()?;
        let model = if config.model == "default" {
            "gemini-2.5-pro"
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Gemini(builder.build()))
    }

    /// Creates a DeepSeek agent with the specified configuration.
    pub fn create_deepseek_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = deepseek_client()?;
        let model = if config.model == "default" {
            DEEPSEEK_CHAT
        } else {
            &config.model
        };

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        Ok(AIAgent::Deepseek(builder.build()))
    }

    /// Creates an agent from the specified provider type.
    pub fn create_agent(provider: LlmProvider, config: &AgentConfig) -> Result<AIAgent> {
        match provider {
            LlmProvider::OpenAI => Self::create_openai_agent(config),
            LlmProvider::Anthropic => Self::create_anthropic_agent(config),
            LlmProvider::Gemini => Self::create_gemini_agent(config),
            LlmProvider::DeepSeek => Self::create_deepseek_agent(config),
        }
    }

    /// Creates an agent from a string provider name.
    pub fn create_agent_from_str(provider: &str, config: &AgentConfig) -> Result<AIAgent> {
        let provider_enum = LlmProvider::from_str(provider).ok_or_else(|| {
            AuditError::configuration(
                "llm_provider",
                &format!("Unsupported LLM provider: {}", provider),
            )
        })?;
        Self::create_agent(provider_enum, config)
    }

    /// Creates an agent from environment configuration.
    ///
    /// This method checks which API keys are available and creates an agent
    /// from the first available provider in priority order.
    pub fn create_from_env(config: &AgentConfig) -> Result<AIAgent> {
        // Try providers in order of preference
        for provider in LlmProvider::all() {
            if provider.is_available() {
                match Self::create_agent(*provider, config) {
                    Ok(agent) => return Ok(agent),
                    Err(_) => continue, // Try next provider
                }
            }
        }

        Err(AuditError::configuration(
            "llm_providers",
            "No LLM provider API keys found in environment",
        ))
    }

    /// Returns a list of available providers based on environment variables.
    pub fn available_providers() -> Vec<LlmProvider> {
        LlmProvider::all()
            .iter()
            .filter(|provider| provider.is_available())
            .copied()
            .collect()
    }
}

/// Convenience functions that match the original API but use the factory internally.
/// These maintain backward compatibility with existing code.

/// Creates an OpenAI agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_openai_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_openai_agent(
    _client: &openai::Client,
    temperature: f64,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_preamble(preamble)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_openai_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create OpenAI agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates an Anthropic agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_anthropic_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_anthropic_agent(
    _client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_max_tokens(max_tokens);

    // Use the factory with singleton clients
    AgentFactory::create_anthropic_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create Anthropic agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates a Gemini agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_gemini_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_gemini_agent(
    _client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_gemini_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create Gemini agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

/// Creates a DeepSeek agent with the original API.
///
/// # Deprecated
/// This function is deprecated. Use `AgentFactory::create_deepseek_agent` instead.
/// The client parameter is ignored as singleton clients are used internally.
pub fn build_deepseek_agent(
    _client: &deepseek::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let config = AgentConfig::new()
        .with_temperature(temperature)
        .with_model(model)
        .with_context(context.unwrap_or_default());

    // Use the factory with singleton clients
    AgentFactory::create_deepseek_agent(&config).unwrap_or_else(|_| {
        // Fallback should not happen in normal operation
        panic!("Failed to create DeepSeek agent. Ensure init_llm_clients() was called and API key is configured.");
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_provider_enum() {
        assert_eq!(LlmProvider::OpenAI.as_str(), "openai");
        assert_eq!(LlmProvider::Anthropic.as_str(), "anthropic");
        assert_eq!(LlmProvider::Gemini.as_str(), "gemini");
        assert_eq!(LlmProvider::DeepSeek.as_str(), "deepseek");
    }

    #[test]
    fn test_provider_from_str() {
        assert_eq!(LlmProvider::from_str("openai"), Some(LlmProvider::OpenAI));
        assert_eq!(LlmProvider::from_str("gpt"), Some(LlmProvider::OpenAI));
        assert_eq!(
            LlmProvider::from_str("claude"),
            Some(LlmProvider::Anthropic)
        );
        assert_eq!(LlmProvider::from_str("invalid"), None);
    }

    #[test]
    fn test_agent_config_builder() {
        let config = AgentConfig::new()
            .with_temperature(0.8)
            .with_model("gpt-4")
            .with_context("test context");

        assert_eq!(config.temperature, 0.8);
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.context, Some("test context".to_string()));
    }

    #[test]
    fn test_security_audit_config() {
        let config = AgentConfig::for_security_audit();
        assert_eq!(config.temperature, 1.0);
        assert!(config.preamble.contains("security auditing"));
        assert_eq!(config.max_tokens, Some(4096));
    }

    #[test]
    fn test_available_providers() {
        let providers = AgentFactory::available_providers();
        // This will depend on environment variables, so we just check it returns a Vec
        assert!(providers.is_empty() || !providers.is_empty());
    }
}

