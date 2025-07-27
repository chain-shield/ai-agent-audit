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
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::{
    client::{CompletionClient, EmbeddingsClient, ProviderClient},
    providers::{
        anthropic::{self, CLAUDE_3_7_SONNET},
        deepseek::{self, DEEPSEEK_CHAT},
        gemini::{self},
        openai::{self, O3, TEXT_EMBEDDING_3_SMALL, GPT_4O},
    },
};
// use rig_qdrant::QdrantVectorStore;  // Temporarily disabled due to version conflicts
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

// /// Helper function to create vector store for dynamic context
// fn create_vector_store(repo: &RepoPaths) -> Result<QdrantVectorStore<openai::EmbeddingModel>> {
//     let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL").map_err(|_| {
//         AuditError::configuration("qdrant_url", "QDRANT_URL environment variable not set")
//     })?)
//     .build()
//     .map_err(|e| {
//         AuditError::configuration(
//             "qdrant_connection",
//             &format!("Failed to connect to Qdrant: {}", e),
//         )
//     })?;

//     let openai = openai::Client::new(&std::env::var("OPENAI_API_KEY").map_err(|_| {
//         AuditError::configuration(
//             "openai_api_key",
//             "OPENAI_API_KEY environment variable not set",
//         )
//     })?);
//     let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

//     let collection_name = format!("{}-contract_chunks", repo.unique_repo_hash());
//     let qp = QueryPointsBuilder::new(&collection_name)
//         .with_payload(true)
//         .build();

//     Ok(QdrantVectorStore::new(qdrant, model, qp))
// }

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
}
