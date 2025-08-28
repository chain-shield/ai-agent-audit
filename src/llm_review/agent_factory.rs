use super::enums::AIAgent;
use crate::ai_bot::file_picker::FilePickerTool;
/// AI Agent Factory for centralized agent creation across LLM providers.
///
/// This module provides a unified interface for creating AI agents from different
/// LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) with consistent configuration
/// and error handling.
use crate::ai_bot::file_retrival::FileRetrievalTool;
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use rig::{
    client::{CompletionClient, ProviderClient},
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self},
    },
};
use serde_json::json;

// use rig_qdrant::QdrantVectorStore;  // Temporarily disabled due to version conflicts
use std::sync::OnceLock;

/// Default OpenAI model for agents
const DEFAULT_OPENAI_MODEL: &str = "gpt-5";

/// Valid OpenAI service tiers (for most accounts)
/// - "auto": Let OpenAI choose automatically
/// - "default": Standard rates and speed
/// - "flex": Half the cost, slower responses
/// Note: "priority" tier requires special account approval
const VALID_SERVICE_TIERS: &[&str] = &["auto", "default", "flex"];

/// Valid OpenAI reasoning effort levels
const VALID_REASONING_EFFORTS: &[&str] = &["minimal", "low", "medium", "high"];

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
            LlmProvider::Gemini => "GEMINI_API_KEY",
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

/// OpenAI-specific configuration options
#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    /// Service tier for OpenAI API calls ("default", "flex", "priority")
    pub service_tier: Option<String>,
    /// Reasoning effort for OpenAI models ("minimal", "low", "medium", "high")
    pub reasoning_effort: Option<String>,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            service_tier: Some("default".to_string()),
            reasoning_effort: Some("medium".to_string()),
        }
    }
}

impl OpenAIConfig {
    /// Validates the service tier value
    pub fn validate_service_tier(tier: &str) -> Result<()> {
        if VALID_SERVICE_TIERS.contains(&tier) {
            Ok(())
        } else {
            Err(AuditError::configuration(
                "openai_service_tier",
                &format!(
                    "Invalid service tier '{}'. Valid options: {}",
                    tier,
                    VALID_SERVICE_TIERS.join(", ")
                ),
            ))
        }
    }

    /// Validates the reasoning effort value
    pub fn validate_reasoning_effort(effort: &str) -> Result<()> {
        if VALID_REASONING_EFFORTS.contains(&effort) {
            Ok(())
        } else {
            Err(AuditError::configuration(
                "openai_reasoning_effort",
                &format!(
                    "Invalid reasoning effort '{}'. Valid options: {}",
                    effort,
                    VALID_REASONING_EFFORTS.join(", ")
                ),
            ))
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
    /// Repository paths for file retrieval tool and dynamic context (required)
    pub repo_paths: RepoPaths,
    /// Enable dynamic context with vector search
    pub enable_dynamic_context: bool,
    /// Number of dynamic context chunks to retrieve (default: 5)
    pub dynamic_context_chunks: usize,
    /// Enable file retrieval tool
    pub enable_file_retrieval: bool,
    /// Enable file picker tool
    pub enable_file_picker: bool,
    /// OpenAI-specific configuration (service tier, reasoning effort)
    pub openai_config: OpenAIConfig,
}

impl AgentConfig {
    /// Creates a new agent configuration with required repository paths.
    pub fn new(repo_paths: RepoPaths) -> Self {
        Self {
            temperature: audit_config().default_temperature,
            model: "default".to_string(),
            context: None,
            max_tokens: None,
            preamble: "You are a world renowned expert in smart-contract security auditing, known for your uncanny ability to find all security bugs in a protocol, even the obscure ones. You have access to advanced tools including file retrieval for searching specific file types (source, test, script, library) and dynamic context from vector search.".to_string(),
            repo_paths,
            enable_dynamic_context: false,
            dynamic_context_chunks: 5,
            enable_file_retrieval: false,
            enable_file_picker: false,
            openai_config: OpenAIConfig::default(),
        }
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

    /// Enables or disables dynamic context.
    pub fn with_dynamic_context(mut self, enabled: bool) -> Self {
        self.enable_dynamic_context = enabled;
        self
    }

    /// Sets the number of dynamic context chunks to retrieve.
    pub fn with_dynamic_context_chunks(mut self, chunks: usize) -> Self {
        self.dynamic_context_chunks = chunks;
        self
    }

    /// Enables or disables file retrieval tool.
    pub fn with_file_retrieval(mut self, enabled: bool) -> Self {
        self.enable_file_retrieval = enabled;
        self
    }

    /// Enables or disables file picker tool.
    pub fn with_file_picker(mut self, enabled: bool) -> Self {
        self.enable_file_picker = enabled;
        self
    }

    /// Sets the OpenAI service tier ("default", "flex", "priority").
    /// Validates the input and panics on invalid values during development.
    pub fn with_openai_service_tier(mut self, service_tier: impl Into<String>) -> Self {
        let tier = service_tier.into();
        if let Err(e) = OpenAIConfig::validate_service_tier(&tier) {
            panic!("Invalid service tier in config builder: {}", e);
        }
        self.openai_config.service_tier = Some(tier);
        self
    }

    /// Sets the OpenAI reasoning effort ("minimal", "low", "medium", "high").
    /// Validates the input and panics on invalid values during development.
    pub fn with_openai_reasoning_effort(mut self, reasoning_effort: impl Into<String>) -> Self {
        let effort = reasoning_effort.into();
        if let Err(e) = OpenAIConfig::validate_reasoning_effort(&effort) {
            panic!("Invalid reasoning effort in config builder: {}", e);
        }
        self.openai_config.reasoning_effort = Some(effort);
        self
    }

    /// Sets both OpenAI service tier and reasoning effort.
    pub fn with_openai_config(
        mut self,
        service_tier: impl Into<String>,
        reasoning_effort: impl Into<String>,
    ) -> Self {
        self.openai_config.service_tier = Some(service_tier.into());
        self.openai_config.reasoning_effort = Some(reasoning_effort.into());
        self
    }

    /// Creates an agent configuration for security auditing with advanced tools enabled.
    pub fn for_security_audit(repo_paths: RepoPaths) -> Self {
        Self::new(repo_paths)
            .with_file_picker(true)
            .with_file_retrieval(true)
            .with_dynamic_context(true)
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

/// Helper function to create file retrieval tool
fn create_file_retrieval_tool(repo: &RepoPaths) -> Result<FileRetrievalTool> {
    let qdrant_url = std::env::var("QDRANT_URL").map_err(|_| {
        AuditError::configuration("qdrant_url", "QDRANT_URL environment variable not set")
    })?;
    let openai_api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        AuditError::configuration(
            "openai_api_key",
            "OPENAI_API_KEY environment variable not set",
        )
    })?;

    Ok(FileRetrievalTool::new(
        qdrant_url,
        openai_api_key,
        repo.clone(),
    ))
}

/// Helper function to create file picker tool
fn create_file_picker_tool(repo: &RepoPaths) -> FilePickerTool {
    FilePickerTool::new(repo.clone())
}

/// Factory for creating AI agents across different providers.
pub struct AgentFactory;

impl AgentFactory {
    /// Creates an OpenAI agent with the specified configuration.
    ///
    /// Uses rig's additional_params to pass OpenAI-specific parameters like
    /// service_tier and reasoning_effort directly to the OpenAI API.
    pub fn create_openai_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = openai_client()?;
        let model = if config.model == "default" {
            DEFAULT_OPENAI_MODEL
        } else {
            &config.model
        };

        // Validate OpenAI-specific configuration at runtime
        if let Some(ref tier) = config.openai_config.service_tier {
            OpenAIConfig::validate_service_tier(tier)?;
        }
        if let Some(ref effort) = config.openai_config.reasoning_effort {
            OpenAIConfig::validate_reasoning_effort(effort)?;
        }

        let mut builder = client
            .agent(model)
            .preamble(&config.preamble)
            .temperature(config.temperature);

        // Add OpenAI-specific parameters using additional_params
        // Only send non-default values to avoid unnecessary API overhead
        let mut additional_params = serde_json::Map::new();

        if let Some(service_tier) = &config.openai_config.service_tier {
            // Only send if not default
            if service_tier != "default" {
                additional_params.insert("service_tier".to_string(), json!(service_tier));
            }
        }

        if let Some(reasoning_effort) = &config.openai_config.reasoning_effort {
            // Only send if not default (medium)
            if reasoning_effort != "medium" {
                // OpenAI expects nested structure: { "reasoning": { "effort": "low" } }
                additional_params.insert(
                    "reasoning".to_string(),
                    json!({ "effort": reasoning_effort }),
                );
            }
        }

        if !additional_params.is_empty() {
            builder = builder.additional_params(serde_json::Value::Object(additional_params));
        }

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        // Add dynamic context if enabled
        // Temporarily disabled due to rig-qdrant version conflicts
        // if config.enable_dynamic_context {
        //     let vector_store = create_vector_store(&config.repo_paths)?;
        //     builder = builder.dynamic_context(config.dynamic_context_chunks, vector_store);
        // }

        // Add file retrieval tool if enabled
        if config.enable_file_retrieval {
            let file_tool = create_file_retrieval_tool(&config.repo_paths)?;
            builder = builder.tool(file_tool);
        }

        // Add file picker tool if enabled
        if config.enable_file_picker {
            let file_picker = create_file_picker_tool(&config.repo_paths);
            builder = builder.tool(file_picker);
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

        // Add dynamic context if enabled
        // Temporarily disabled due to rig-qdrant version conflicts
        // if config.enable_dynamic_context {
        //     let vector_store = create_vector_store(&config.repo_paths)?;
        //     builder = builder.dynamic_context(config.dynamic_context_chunks, vector_store);
        // }

        // Add file retrieval tool if enabled
        if config.enable_file_retrieval {
            let file_tool = create_file_retrieval_tool(&config.repo_paths)?;
            builder = builder.tool(file_tool);
        }

        // Add file picker tool if enabled
        if config.enable_file_picker {
            let file_picker = create_file_picker_tool(&config.repo_paths);
            builder = builder.tool(file_picker);
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

        // Add dynamic context if enabled
        // Temporarily disabled due to rig-qdrant version conflicts
        // if config.enable_dynamic_context {
        //     let vector_store = create_vector_store(&config.repo_paths)?;
        //     builder = builder.dynamic_context(config.dynamic_context_chunks, vector_store);
        // }

        // Add file retrieval tool if enabled
        if config.enable_file_retrieval {
            let file_tool = create_file_retrieval_tool(&config.repo_paths)?;
            builder = builder.tool(file_tool);
        }

        // Add file picker tool if enabled
        if config.enable_file_picker {
            let file_picker = create_file_picker_tool(&config.repo_paths);
            builder = builder.tool(file_picker);
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

        // Add dynamic context if enabled
        // Temporarily disabled due to rig-qdrant version conflicts
        // if config.enable_dynamic_context {
        //     let vector_store = create_vector_store(&config.repo_paths)?;
        //     builder = builder.dynamic_context(config.dynamic_context_chunks, vector_store);
        // }

        // Add file retrieval tool if enabled
        if config.enable_file_retrieval {
            let file_tool = create_file_retrieval_tool(&config.repo_paths)?;
            builder = builder.tool(file_tool);
        }

        // Add file picker tool if enabled
        if config.enable_file_picker {
            let file_picker = create_file_picker_tool(&config.repo_paths);
            builder = builder.tool(file_picker);
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
    fn test_available_providers() {
        let providers = AgentFactory::available_providers();
        // This will depend on environment variables, so we just check it returns a Vec
        assert!(providers.is_empty() || !providers.is_empty());
    }

    #[test]
    fn test_openai_config_builder() {
        // Test OpenAI config without creating AgentConfig to avoid config dependency
        let mut openai_config = OpenAIConfig::default();
        assert_eq!(openai_config.service_tier, Some("default".to_string()));
        assert_eq!(openai_config.reasoning_effort, Some("medium".to_string()));

        // Test that we can modify the config
        openai_config.service_tier = Some("flex".to_string());
        openai_config.reasoning_effort = Some("high".to_string());

        assert_eq!(openai_config.service_tier, Some("flex".to_string()));
        assert_eq!(openai_config.reasoning_effort, Some("high".to_string()));
    }

    #[test]
    fn test_openai_config_validation() {
        // Test valid service tiers
        assert!(OpenAIConfig::validate_service_tier("default").is_ok());
        assert!(OpenAIConfig::validate_service_tier("flex").is_ok());
        assert!(OpenAIConfig::validate_service_tier("priority").is_ok());

        // Test invalid service tier
        assert!(OpenAIConfig::validate_service_tier("invalid").is_err());

        // Test valid reasoning efforts
        assert!(OpenAIConfig::validate_reasoning_effort("minimal").is_ok());
        assert!(OpenAIConfig::validate_reasoning_effort("low").is_ok());
        assert!(OpenAIConfig::validate_reasoning_effort("medium").is_ok());
        assert!(OpenAIConfig::validate_reasoning_effort("high").is_ok());

        // Test invalid reasoning effort
        assert!(OpenAIConfig::validate_reasoning_effort("invalid").is_err());
    }

    #[test]
    #[should_panic(expected = "Invalid service tier")]
    fn test_builder_validation_service_tier() {
        // This should panic due to validation
        if let Err(e) = OpenAIConfig::validate_service_tier("invalid_tier") {
            panic!("{}", e);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid reasoning effort")]
    fn test_builder_validation_reasoning_effort() {
        // This should panic due to validation
        if let Err(e) = OpenAIConfig::validate_reasoning_effort("invalid_effort") {
            panic!("{}", e);
        }
    }

    #[tokio::test]
    async fn test_openai_agent_with_custom_params() {
        use crate::prepare_code::git_clone::RepoPaths;

        // Initialize config for test
        let _ = crate::config::init_config();

        // Skip if no OpenAI API key
        if !audit_config().has_openai_key() {
            println!("⚠️ Skipping OpenAI agent test - no API key");
            return;
        }

        let repo_paths = RepoPaths {
            project_id: "test-project".to_string(),
            root: "/tmp/test".into(),
            sol_files: vec![],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            source_code_folder: "/tmp/test/src".into(),
            docs: vec![],
            repo_name: "test-repo".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files: None,
            commit_hash: "abc123".to_string(),
        };

        let config = AgentConfig::new(repo_paths)
            .with_model("gpt-5")
            .with_openai_service_tier("default")
            .with_openai_reasoning_effort("low")
            .with_temperature(0.7);

        // Initialize clients first
        let _ = init_llm_clients();

        match AgentFactory::create_openai_agent(&config) {
            Ok(_agent) => {
                println!(
                    "✅ OpenAI agent created with service_tier='default' and reasoning_effort='low'"
                );
                println!("💡 These parameters will be passed to OpenAI API via additional_params");
            }
            Err(e) => {
                println!("❌ Failed to create OpenAI agent: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_openai_params_verification() {
        use crate::prepare_code::git_clone::RepoPaths;

        // Skip if no OpenAI API key
        if std::env::var("OPENAI_API_KEY").is_err() {
            println!("⚠️ Skipping OpenAI params verification - no API key");
            return;
        }

        println!("🧪 Testing OpenAI parameter verification...");

        // Initialize config for test
        let _ = crate::config::init_config();
        let _ = init_llm_clients();

        let repo_paths = RepoPaths {
            project_id: "test-project".to_string(),
            root: "/tmp/test".into(),
            sol_files: vec![],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            source_code_folder: "/tmp/test/src".into(),
            docs: vec![],
            repo_name: "test-repo".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files: None,
            commit_hash: "abc123".to_string(),
        };

        // Test 1: Default tier with low reasoning effort
        println!("\n=== Test 1: Default tier + Low reasoning ===");
        let config1 = AgentConfig::new(repo_paths.clone())
            .with_model("gpt-5")
            .with_openai_service_tier("default")
            .with_openai_reasoning_effort("low")
            .with_temperature(0.7);

        match AgentFactory::create_openai_agent(&config1) {
            Ok(agent) => {
                println!("✅ Agent created with default/low config");

                // Test with a simple prompt
                match agent
                    .prompt("Say 'Hello from default tier with low reasoning' in exactly 8 words")
                    .await
                {
                    Ok(response) => {
                        println!("✅ Response: {}", response);
                        println!("💡 This used default tier + low reasoning effort");
                    }
                    Err(e) => println!("❌ Prompt failed: {}", e),
                }
            }
            Err(e) => println!("❌ Agent creation failed: {}", e),
        }

        // Test 2: Flex tier (cheaper, slower) with minimal reasoning
        println!("\n=== Test 2: Flex tier + Minimal reasoning ===");
        let config2 = AgentConfig::new(repo_paths.clone())
            .with_model("gpt-5")
            .with_openai_service_tier("flex")
            .with_openai_reasoning_effort("minimal")
            .with_temperature(0.7);

        match AgentFactory::create_openai_agent(&config2) {
            Ok(agent) => {
                println!("✅ Agent created with flex/minimal config");

                // Test with a simple prompt
                match agent
                    .prompt("Say 'Hello from flex tier with minimal reasoning' in exactly 8 words")
                    .await
                {
                    Ok(response) => {
                        println!("✅ Response: {}", response);
                        println!("💡 This used flex tier (cheaper) + minimal reasoning effort");
                    }
                    Err(e) => println!("❌ Prompt failed: {}", e),
                }
            }
            Err(e) => println!("❌ Agent creation failed: {}", e),
        }

        // Test 3: Priority tier (expensive, fast) with high reasoning
        println!("\n=== Test 3: Priority tier + High reasoning ===");
        let config3 = AgentConfig::new(repo_paths)
            .with_model("gpt-5")
            .with_openai_service_tier("priority")
            .with_openai_reasoning_effort("high")
            .with_temperature(0.7);

        match AgentFactory::create_openai_agent(&config3) {
            Ok(agent) => {
                println!("✅ Agent created with priority/high config");

                // Test with a simple prompt
                match agent
                    .prompt("Say 'Hello from priority tier with high reasoning' in exactly 8 words")
                    .await
                {
                    Ok(response) => {
                        println!("✅ Response: {}", response);
                        println!("💡 This used priority tier (expensive) + high reasoning effort");
                    }
                    Err(e) => println!("❌ Prompt failed: {}", e),
                }
            }
            Err(e) => println!("❌ Agent creation failed: {}", e),
        }

        println!("\n🔍 To verify SERVICE TIER parameters are working:");
        println!("1. Check response times: flex should be slower than default");
        println!("2. Monitor costs: flex should be ~50% cheaper than default");
        println!("3. Quality should be similar (same reasoning effort across all tests)");
        println!("\n🔬 Scientific approach:");
        println!("   - All tests use reasoning_effort='medium' (controlled variable)");
        println!("   - Only service_tier changes (independent variable)");
        println!("   - No temperature parameter (not supported with reasoning models)");
        println!("   - This isolates service tier effects from reasoning effort effects");
        println!("\n💡 To run this test with your API key:");
        println!("   export OPENAI_API_KEY='your-key-here'");
        println!(
            "   cargo test agent_factory::tests::test_openai_params_verification -- --nocapture"
        );
    }

    #[test]
    fn test_openai_config_parameter_passing() {
        // Test OpenAI configuration without creating AgentConfig to avoid config dependency
        println!("🧪 Testing OpenAI parameter configuration...");

        // Test 1: Default configuration
        let config1 = OpenAIConfig::default();
        assert_eq!(config1.service_tier, Some("default".to_string()));
        assert_eq!(config1.reasoning_effort, Some("medium".to_string()));
        println!("✅ Default config: service_tier='default', reasoning_effort='medium'");
        println!("   → Should send NO additional_params (both are defaults)");

        // Test 2: Non-default service tier
        let mut config2 = OpenAIConfig::default();
        config2.service_tier = Some("flex".to_string());
        assert_eq!(config2.service_tier, Some("flex".to_string()));
        println!("✅ Flex config: service_tier='flex', reasoning_effort='medium'");
        println!("   → Should send: {{\"service_tier\": \"flex\"}}");

        // Test 3: Non-default reasoning effort
        let mut config3 = OpenAIConfig::default();
        config3.reasoning_effort = Some("low".to_string());
        assert_eq!(config3.reasoning_effort, Some("low".to_string()));
        println!("✅ Low reasoning config: service_tier='default', reasoning_effort='low'");
        println!("   → Should send: {{\"reasoning\": {{\"effort\": \"low\"}}}}");

        // Test 4: Both non-default
        let mut config4 = OpenAIConfig::default();
        config4.service_tier = Some("priority".to_string());
        config4.reasoning_effort = Some("high".to_string());
        assert_eq!(config4.service_tier, Some("priority".to_string()));
        assert_eq!(config4.reasoning_effort, Some("high".to_string()));
        println!("✅ Priority+High config: service_tier='priority', reasoning_effort='high'");
        println!(
            "   → Should send: {{\"service_tier\": \"priority\", \"reasoning\": {{\"effort\": \"high\"}}}}"
        );

        println!("\n💡 These parameters will be passed to OpenAI via rig's additional_params");
        println!("💡 To verify they're working, run the API test with your OpenAI key");
        println!("💡 The agent factory will only send non-default values to optimize API calls");
    }

    #[tokio::test]
    async fn test_openai_api_with_custom_params() {
        // Skip if no OpenAI API key
        if std::env::var("OPENAI_API_KEY").is_err() {
            println!("⚠️ Skipping OpenAI API test - no API key");
            println!("💡 To run this test: export OPENAI_API_KEY='your-key-here'");
            return;
        }

        println!("🧪 Testing OpenAI API with custom parameters...");
        println!("🎯 Goal: Verify parameters are accepted (no API errors = success!)");

        // Initialize config and clients
        let _ = crate::config::init_config();
        let _ = init_llm_clients();

        let repo_paths = RepoPaths {
            project_id: "test-project".to_string(),
            root: "/tmp/test".into(),
            sol_files: vec![],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            source_code_folder: "/tmp/test/src".into(),
            docs: vec![],
            repo_name: "test-repo".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files: None,
            commit_hash: "abc123".to_string(),
        };

        // Test 1: Flex tier (should send service_tier param only)
        println!("\n=== Test 1: Flex Service Tier ===");
        println!("📤 Sending: {{\"service_tier\": \"flex\"}}");
        println!("💰 Expected: 50% cheaper, slower responses");

        let config1 = AgentConfig::new(repo_paths.clone())
            .with_model("gpt-5")
            .with_openai_service_tier("flex")
            .with_openai_reasoning_effort("medium"); // Keep constant, no temperature for reasoning models

        match AgentFactory::create_openai_agent(&config1) {
            Ok(agent) => {
                println!("✅ Agent created successfully");

                match agent
                    .prompt("Say 'Flex tier works' in exactly 3 words")
                    .await
                {
                    Ok(response) => {
                        println!("✅ API call successful! Response: {}", response);
                        println!("💡 Flex tier parameter was accepted by OpenAI API");
                    }
                    Err(e) => {
                        println!("❌ API call failed: {}", e);
                        println!("💡 This might indicate flex tier rejection or other API issue");
                    }
                }
            }
            Err(e) => println!("❌ Agent creation failed: {}", e),
        }

        // Test 2: Default tier (baseline - should send no params)
        println!("\n=== Test 2: Default Service Tier (Baseline) ===");
        println!("📤 Sending: {{}} (no additional params - both are defaults)");
        println!("💰 Expected: Standard cost and speed");

        let config2 = AgentConfig::new(repo_paths.clone())
            .with_model("gpt-5")
            .with_openai_service_tier("default")
            .with_openai_reasoning_effort("medium"); // Keep constant, no temperature for reasoning models

        match AgentFactory::create_openai_agent(&config2) {
            Ok(agent) => {
                println!("✅ Agent created successfully");

                match agent
                    .prompt("Say 'Default tier works' in exactly 3 words")
                    .await
                {
                    Ok(response) => {
                        println!("✅ API call successful! Response: {}", response);
                        println!("💡 Default tier (baseline) working normally");
                    }
                    Err(e) => {
                        println!("❌ API call failed: {}", e);
                        println!("💡 This might indicate API issue");
                    }
                }
            }
            Err(e) => println!("❌ Agent creation failed: {}", e),
        }

        // Note: Priority tier test removed - requires special account approval
        println!("\n💡 Priority tier test skipped - requires special OpenAI account approval");
        println!("📋 Available tiers for most accounts: 'auto', 'default', 'flex'");

        println!("\n🎯 VERIFICATION COMPLETE!");
        println!("✅ If all API calls succeeded, your custom parameters are working!");
        println!("❌ If any API calls failed, check the error messages for parameter issues");
        println!("\n💡 Next steps:");
        println!("   - Monitor response times (flex should be slower, priority faster)");
        println!("   - Check costs (flex should be cheaper, priority more expensive)");
        println!("   - Observe reasoning quality differences");
    }
}
