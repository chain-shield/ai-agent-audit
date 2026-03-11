use super::agent_enums::{AIAgent, AgentMetadata};
/// AI Agent Factory for centralized agent creation across LLM providers.
///
/// This module provides a unified interface for creating AI agents from different
/// LLM providers (OpenAI, Anthropic, Gemini, DeepSeek) with consistent configuration
/// and error handling.
use crate::config::{OPENAI_MODEL, audit_config};
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use rig::{
    agent::AgentBuilderSimple,
    client::CompletionClient,
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self},
    },
};
use serde_json::json;

// use rig_qdrant::QdrantVectorStore;  // Temporarily disabled due to version conflicts
use std::{str::FromStr, sync::OnceLock};

/// Antrophic thinking
const VALID_THINKING_SETTING: &[&str] = &["enabled", "disabled"];

/// Default OpenAI model for agents
const DEFAULT_OPENAI_MODEL: &str = OPENAI_MODEL;

/// Valid OpenAI service tiers (for most accounts)
/// - "auto": Let OpenAI choose automatically
/// - "default": Standard rates and speed
/// - "flex": Half the cost, slower responses
///
/// Note: "priority" tier requires special account approval
/// rig-core 0.17.0 supports only: auto, default, flex
const VALID_SERVICE_TIERS: &[&str] = &["auto", "default", "flex"];

/// Valid OpenAI reasoning effort levels
/// OpenAI docs for reasoning models use "none", "low", "medium", "high".
/// Internally we treat "none" as a request for the lowest non-zero effort and
/// map it to "low" when calling the API.
const VALID_REASONING_EFFORTS: &[&str] = &["none", "low", "medium", "high"];

/// Valid Gemini thinking levels (for Gemini 3 Pro models)
const VALID_THINKING_LEVELS: &[&str] = &["low", "high"];

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
}

impl FromStr for LlmProvider {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" => Ok(LlmProvider::OpenAI),
            "anthropic" | "claude" => Ok(LlmProvider::Anthropic),
            "gemini" | "google" => Ok(LlmProvider::Gemini),
            "deepseek" => Ok(LlmProvider::DeepSeek),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    pub thinking: Option<String>,
    pub thinking_token_budget: u32,
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self {
            thinking: Some("disabled".to_string()),
            thinking_token_budget: 30000,
        }
    }
}

impl AnthropicConfig {
    /// Validates the service tier value
    pub fn validate_thinking(thinking_setting: &str) -> Result<()> {
        if VALID_THINKING_SETTING.contains(&thinking_setting) {
            Ok(())
        } else {
            Err(AuditError::configuration(
                "anthropic_thinking_setting",
                format!(
                    "Invalid thinking setting '{}'. Valid options: {}",
                    thinking_setting,
                    VALID_THINKING_SETTING.join(", ")
                ),
            ))
        }
    }
}

/// OpenAI-specific configuration options
#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    /// Service tier for OpenAI API calls ("default", "flex", "priority")
    pub service_tier: Option<String>,
    /// Reasoning effort for OpenAI models ("none", "minimal", "low", "medium", "high")
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
                format!(
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
                format!(
                    "Invalid reasoning effort '{}'. Valid options: {}",
                    effort,
                    VALID_REASONING_EFFORTS.join(", ")
                ),
            ))
        }
    }
}

/// Gemini-specific configuration options
#[derive(Debug, Clone)]
pub struct GeminiConfig {
    /// Thinking level for Gemini 3 Pro models ("low" or "high")
    pub thinking_level: Option<String>,
    /// Top-p (nucleus sampling) parameter (0.0-1.0, default: 0.95)
    pub top_p: Option<f64>,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            thinking_level: Some("high".to_string()),
            top_p: Some(0.95),
        }
    }
}

impl GeminiConfig {
    /// Validates the thinking level value
    pub fn validate_thinking_level(level: &str) -> Result<()> {
        if VALID_THINKING_LEVELS.contains(&level) {
            Ok(())
        } else {
            Err(AuditError::configuration(
                "gemini_thinking_level",
                format!(
                    "Invalid thinking level '{}'. Valid options: {}",
                    level,
                    VALID_THINKING_LEVELS.join(", ")
                ),
            ))
        }
    }

    /// Validates the top_p value (must be between 0.0 and 1.0)
    pub fn validate_top_p(top_p: f64) -> Result<()> {
        if (0.0..=1.0).contains(&top_p) {
            Ok(())
        } else {
            Err(AuditError::configuration(
                "gemini_top_p",
                format!(
                    "Invalid top_p value '{}'. Must be between 0.0 and 1.0",
                    top_p
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
    pub repo_paths: Option<RepoPaths>,
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
    /// Anthropic-specific configuration (thinking)
    pub anthropic_config: AnthropicConfig,
    /// Gemini-specific configuration (thinking level)
    pub gemini_config: GeminiConfig,
}

impl AgentConfig {
    /// Creates a new agent configuration with required repository paths.
    pub fn new(repo_paths: Option<RepoPaths>) -> Self {
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
            anthropic_config: AnthropicConfig::default(),
            gemini_config: GeminiConfig::default(),
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

    /// Sets the OpenAI reasoning effort ("none", "minimal", "low", "medium", "high").
    /// Validates the input and panics on invalid values during development.
    pub fn with_openai_reasoning_effort(mut self, reasoning_effort: impl Into<String>) -> Self {
        let effort = reasoning_effort.into();
        if let Err(e) = OpenAIConfig::validate_reasoning_effort(&effort) {
            panic!("Invalid reasoning effort in config builder: {}", e);
        }
        self.openai_config.reasoning_effort = Some(effort);
        self
    }

    /// Sets the anthropic thinking effort ("disabled", "enabled").
    /// Validates the input and panics on invalid values during development.
    pub fn with_anthropic_thinking(
        mut self,
        thinking: impl Into<String>,
        budget: impl Into<u32>,
    ) -> Self {
        let think = thinking.into();
        let token_budget = budget.into();
        if let Err(e) = AnthropicConfig::validate_thinking(&think) {
            panic!("Invalid thinking in config builder: {}", e);
        }
        self.anthropic_config.thinking = Some(think);
        self.anthropic_config.thinking_token_budget = token_budget;
        self
    }

    /// Sets the Gemini thinking level ("low" or "high").
    /// Validates the input and panics on invalid values during development.
    pub fn with_gemini_thinking_level(mut self, thinking_level: impl Into<String>) -> Self {
        let level = thinking_level.into();
        if let Err(e) = GeminiConfig::validate_thinking_level(&level) {
            panic!("Invalid thinking level in config builder: {}", e);
        }
        self.gemini_config.thinking_level = Some(level);
        self
    }

    /// Sets the Gemini top_p (nucleus sampling) parameter (0.0-1.0, default: 0.95).
    /// Validates the input and panics on invalid values during development.
    pub fn with_top_p(mut self, top_p: f64) -> Self {
        if let Err(e) = GeminiConfig::validate_top_p(top_p) {
            panic!("Invalid top_p in config builder: {}", e);
        }
        self.gemini_config.top_p = Some(top_p);
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
        Self::new(Some(repo_paths))
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
    if audit_config().has_openai_key() && OPENAI_CLIENT.get().is_none() {
        log::info!("Initializing OpenAI client...");
        let client = openai::Client::from_env();
        OPENAI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("openai_client", "OpenAI client already initialized")
        })?;
        log::info!("OpenAI client initialized successfully");
    }

    // Initialize Anthropic client if API key is available
    if audit_config().has_anthropic_key() && ANTHROPIC_CLIENT.get().is_none() {
        let client = anthropic::Client::from_env();
        ANTHROPIC_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("anthropic_client", "Anthropic client already initialized")
        })?;
    }

    // Initialize Gemini client if API key is available
    if audit_config().has_google_ai_key() && GEMINI_CLIENT.get().is_none() {
        log::info!("Initializing Gemini client...");
        let client = gemini::Client::from_env();
        GEMINI_CLIENT.set(client).map_err(|_| {
            AuditError::configuration("gemini_client", "Gemini client already initialized")
        })?;
        log::info!("Gemini client initialized successfully");
    }

    // Initialize DeepSeek client if API key is available
    if audit_config().has_deepseek_key() && DEEPSEEK_CLIENT.get().is_none() {
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

/// Returns the Gemini client instance, initializing it if necessary.
fn gemini_client() -> Result<&'static gemini::Client> {
    GEMINI_CLIENT.get_or_init(|| {
        if !audit_config().has_google_ai_key() {
            panic!("Gemini API key not configured");
        }
        log::info!("Lazy-initializing Gemini client...");
        gemini::Client::from_env()
    });

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

        let mut builder = AgentBuilderSimple::new(client.completion_model(model))
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
                // Docs expose: "none" | "low" | "medium" | "high". To keep behavior simple
                // across the app, we treat "none" as an alias for the lowest non-zero setting
                // and send "low" on the wire.
                let effective_effort = if reasoning_effort == "none" {
                    "low"
                } else {
                    reasoning_effort.as_str()
                };
                additional_params.insert(
                    "reasoning".to_string(),
                    json!({ "effort": effective_effort }),
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

        // Create metadata for pricing calculations
        let metadata = AgentMetadata {
            model: config.model.clone(),
            temperature: config.temperature,
            service_tier: config.openai_config.service_tier.clone(),
            reasoning_effort: config.openai_config.reasoning_effort.clone(),
            file_picker_enabled: config.enable_file_picker,
            file_retrieval_enabled: config.enable_file_retrieval,
            dynamic_context_enabled: config.enable_dynamic_context,
        };

        Ok(AIAgent::Openai {
            agent: builder.build(),
            metadata,
        })
    }

    /// Creates an Anthropic agent with the specified configuration.
    pub fn create_anthropic_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = anthropic_client()?;
        let model = if config.model == "default" {
            CLAUDE_3_7_SONNET
        } else {
            &config.model
        };

        let mut builder = AgentBuilderSimple::new(client.completion_model(model))
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

        // Add Anthropic-specific parameters using additional_params
        // Only send non-default values to avoid unnecessary API overhead
        let mut additional_params = serde_json::Map::new();

        if let Some(thinking) = &config.anthropic_config.thinking {
            let token_budget = config.anthropic_config.thinking_token_budget;
            if thinking != "disabled" {
                additional_params.insert(
                    "thinking".to_string(),
                    json!({ "type": thinking, "budget_tokens": token_budget }),
                );
            }
        }

        if !additional_params.is_empty() {
            builder = builder.additional_params(serde_json::Value::Object(additional_params));
        }

        // Create metadata for pricing calculations
        let metadata = AgentMetadata {
            model: config.model.clone(),
            temperature: config.temperature,
            service_tier: None,     // Anthropic doesn't have service tiers
            reasoning_effort: None, // Anthropic doesn't have reasoning effort
            file_picker_enabled: config.enable_file_picker,
            file_retrieval_enabled: config.enable_file_retrieval,
            dynamic_context_enabled: config.enable_dynamic_context,
        };

        Ok(AIAgent::Anthropic {
            agent: builder.build(),
            metadata,
        })
    }

    /// Creates a Gemini agent with the specified configuration.
    pub fn create_gemini_agent(config: &AgentConfig) -> Result<AIAgent> {
        use rig::providers::gemini::completion::gemini_api_types::{
            AdditionalParameters, GenerationConfig, HarmBlockThreshold, HarmCategory,
            SafetySetting, ThinkingConfig,
        };

        let client = gemini_client()?;
        let model = if config.model == "default" {
            "gemini-2.5-pro"
        } else {
            &config.model
        };

        // Validate Gemini-specific configuration at runtime
        if let Some(ref level) = config.gemini_config.thinking_level {
            GeminiConfig::validate_thinking_level(level)?;
        }
        if let Some(top_p) = config.gemini_config.top_p {
            GeminiConfig::validate_top_p(top_p)?;
        }

        // Disable safety filters for security research (analyzing vulnerabilities)
        let safety_settings = vec![
            SafetySetting {
                category: HarmCategory::HarmCategoryDangerousContent,
                threshold: HarmBlockThreshold::BlockNone,
            },
            SafetySetting {
                category: HarmCategory::HarmCategoryHarassment,
                threshold: HarmBlockThreshold::BlockNone,
            },
            SafetySetting {
                category: HarmCategory::HarmCategoryHateSpeech,
                threshold: HarmBlockThreshold::BlockNone,
            },
            SafetySetting {
                category: HarmCategory::HarmCategorySexuallyExplicit,
                threshold: HarmBlockThreshold::BlockNone,
            },
        ];

        // Set max output tokens to prevent truncation
        // Gemini 3 Pro supports up to 65,536 output tokens
        // Configure thinking level and top_p for Gemini 3 Pro models
        let mut generation_config = GenerationConfig {
            max_output_tokens: Some(64_000),
            temperature: Some(config.temperature),
            top_p: config.gemini_config.top_p,
            ..Default::default()
        };

        // Add thinking configuration if specified
        if let Some(ref thinking_level) = config.gemini_config.thinking_level {
            generation_config.thinking_config = Some(ThinkingConfig {
                thinking_budget: None,
                include_thoughts: None,
                thinking_level: Some(thinking_level.clone()),
            });
        }

        let additional_params = AdditionalParameters::default()
            .with_safety_settings(safety_settings)
            .with_config(generation_config);

        let mut builder = AgentBuilderSimple::new(client.completion_model(model))
            .preamble(&config.preamble)
            .temperature(config.temperature)
            .additional_params(serde_json::to_value(additional_params)?);

        if let Some(context) = &config.context {
            builder = builder.context(context);
        }

        // Add dynamic context if enabled
        // Temporarily disabled due to rig-qdrant version conflicts
        // if config.enable_dynamic_context {
        //     let vector_store = create_vector_store(&config.repo_paths)?;
        //     builder = builder.dynamic_context(config.dynamic_context_chunks, vector_store);
        // }

        // Create metadata for pricing calculations
        let metadata = AgentMetadata {
            model: config.model.clone(),
            temperature: config.temperature,
            service_tier: None,     // Gemini doesn't have service tiers
            reasoning_effort: None, // Gemini doesn't have reasoning effort
            file_picker_enabled: config.enable_file_picker,
            file_retrieval_enabled: config.enable_file_retrieval,
            dynamic_context_enabled: config.enable_dynamic_context,
        };

        Ok(AIAgent::Gemini {
            agent: builder.build(),
            metadata,
        })
    }

    /// Creates a DeepSeek agent with the specified configuration.
    pub fn create_deepseek_agent(config: &AgentConfig) -> Result<AIAgent> {
        let client = deepseek_client()?;
        let model = if config.model == "default" {
            DEEPSEEK_CHAT
        } else {
            &config.model
        };

        let mut builder = AgentBuilderSimple::new(client.completion_model(model))
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

        // Create metadata for pricing calculations
        let metadata = AgentMetadata {
            model: config.model.clone(),
            temperature: config.temperature,
            service_tier: None,     // DeepSeek doesn't have service tiers
            reasoning_effort: None, // DeepSeek doesn't have reasoning effort
            file_picker_enabled: config.enable_file_picker,
            file_retrieval_enabled: config.enable_file_retrieval,
            dynamic_context_enabled: config.enable_dynamic_context,
        };

        Ok(AIAgent::Deepseek {
            agent: builder.build(),
            metadata,
        })
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
        let provider_enum = provider.parse::<LlmProvider>().map_err(|_| {
            AuditError::configuration(
                "llm_provider",
                format!("Unsupported LLM provider: {}", provider),
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
    fn test_anthropic_thinking_config() {
        // Test default config
        let config = AnthropicConfig::default();
        assert_eq!(config.thinking, Some("disabled".to_string()));
        assert_eq!(config.thinking_token_budget, 30000);

        // Test validation
        assert!(AnthropicConfig::validate_thinking("enabled").is_ok());
        assert!(AnthropicConfig::validate_thinking("disabled").is_ok());
        assert!(AnthropicConfig::validate_thinking("invalid").is_err());
    }

    #[test]
    fn test_agent_config_with_anthropic_thinking() {
        use crate::config::init_config;

        // Initialize config (required for AgentConfig::new)
        let _ = init_config();

        let config = AgentConfig::new(None).with_anthropic_thinking("enabled", 10000u32);

        assert_eq!(
            config.anthropic_config.thinking,
            Some("enabled".to_string())
        );
        assert_eq!(config.anthropic_config.thinking_token_budget, 10000);
    }

    #[test]
    #[should_panic(expected = "Invalid thinking")]
    fn test_agent_config_with_invalid_thinking() {
        use crate::config::init_config;

        // Initialize config (required for AgentConfig::new)
        let _ = init_config();

        let _config = AgentConfig::new(None).with_anthropic_thinking("invalid", 10000u32);
    }

    #[test]
    fn test_gemini_thinking_level_config() {
        // Test default config
        let config = GeminiConfig::default();
        assert_eq!(config.thinking_level, Some("high".to_string()));

        // Test validation
        assert!(GeminiConfig::validate_thinking_level("low").is_ok());
        assert!(GeminiConfig::validate_thinking_level("high").is_ok());
        assert!(GeminiConfig::validate_thinking_level("invalid").is_err());
    }

    #[test]
    fn test_agent_config_with_gemini_thinking_level() {
        use crate::config::init_config;

        // Initialize config (required for AgentConfig::new)
        let _ = init_config();

        let config = AgentConfig::new(None).with_gemini_thinking_level("low");

        assert_eq!(config.gemini_config.thinking_level, Some("low".to_string()));
    }

    #[test]
    #[should_panic(expected = "Invalid thinking level")]
    fn test_agent_config_with_invalid_thinking_level() {
        use crate::config::init_config;

        // Initialize config (required for AgentConfig::new)
        let _ = init_config();

        let _config = AgentConfig::new(None).with_gemini_thinking_level("invalid");
    }

    #[test]
    fn test_gemini_top_p_config() {
        // Test default config
        let config = GeminiConfig::default();
        assert_eq!(config.top_p, Some(0.95));

        // Test validation - valid values
        assert!(GeminiConfig::validate_top_p(0.0).is_ok());
        assert!(GeminiConfig::validate_top_p(0.5).is_ok());
        assert!(GeminiConfig::validate_top_p(0.95).is_ok());
        assert!(GeminiConfig::validate_top_p(1.0).is_ok());

        // Test validation - invalid values
        assert!(GeminiConfig::validate_top_p(-0.1).is_err());
        assert!(GeminiConfig::validate_top_p(1.1).is_err());
        assert!(GeminiConfig::validate_top_p(2.0).is_err());
    }

    #[test]
    fn test_agent_config_with_top_p() {
        use crate::config::init_config;

        // Initialize config (required for AgentConfig::new)
        let _ = init_config();

        let config = AgentConfig::new(None).with_top_p(0.8);

        assert_eq!(config.gemini_config.top_p, Some(0.8));
    }

    #[test]
    #[should_panic(expected = "Invalid top_p")]
    fn test_agent_config_with_invalid_top_p() {
        use crate::config::init_config;

        // Initialize config (required for AgentConfig::new)
        let _ = init_config();

        let _config = AgentConfig::new(None).with_top_p(1.5);
    }
}
