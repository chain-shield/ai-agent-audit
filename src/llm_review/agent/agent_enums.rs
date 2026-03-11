/// AI agent and vulnerability type enumerations.
///
/// This module defines the core enums for multi-LLM support and vulnerability
/// categorization, providing unified interfaces for different AI providers
/// and systematic vulnerability detection across 19+ security categories.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::utils::extract_retry::agent_extract_with_retry;
use rig::{
    agent::Agent,
    extractor::Extractor,
    providers::{
        anthropic, deepseek, gemini,
        openai::{self},
    },
};

use serde::de::DeserializeOwned;

/// Configuration metadata for AI agents.
/// Stores the original configuration used to create the agent for pricing calculations.
#[derive(Debug, Clone, Default)]
pub struct AgentMetadata {
    pub model: String,
    pub temperature: f64,
    pub service_tier: Option<String>,
    pub reasoning_effort: Option<String>,
    pub file_picker_enabled: bool,
    pub file_retrieval_enabled: bool,
    pub dynamic_context_enabled: bool,
}

/// Unified AI agent enum supporting multiple LLM providers.
///
/// Provides a common interface for different AI providers while maintaining
/// provider-specific optimizations and cost tracking capabilities.
pub enum AIAgent {
    /// Anthropic Claude models (3.7 Sonnet, 4.0 Sonnet)
    Anthropic {
        agent: Agent<anthropic::completion::CompletionModel>,
        metadata: AgentMetadata,
    },
    /// OpenAI models (GPT-4o, O3)
    Openai {
        agent: Agent<openai::responses_api::ResponsesCompletionModel>,
        metadata: AgentMetadata,
    },
    /// Google Gemini models
    Gemini {
        agent: Agent<gemini::completion::CompletionModel>,
        metadata: AgentMetadata,
    },
    /// DeepSeek models (cost-effective option)
    Deepseek {
        agent: Agent<deepseek::CompletionModel>,
        metadata: AgentMetadata,
    },
}

/// Unified AI extractor enum for structured data extraction.
///
/// Provides type-safe extraction capabilities across different LLM providers
/// with automatic retry logic and error handling.
pub enum AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    Anthropic(Extractor<anthropic::completion::CompletionModel, T>),
    Openai(Extractor<openai::responses_api::ResponsesCompletionModel, T>),
    Gemini(Extractor<gemini::completion::CompletionModel, T>),
    Deepseek(Extractor<deepseek::CompletionModel, T>),
}

// EnumString trait removed - use strum's Display trait (to_string()) instead!

pub trait EnumData {
    type Spec;
    fn to_types(&self) -> &'static [crate::llm_review::findings::finding_enums::VulnerabilityType];
    fn get_spec(&self) -> Self::Spec;
    // get predicate or defintion of patten
}

use std::fmt::Write;

pub fn generate_enum_list<T: std::fmt::Display>(patterns: &[T]) -> String {
    let mut enum_list = String::new();
    let top_pattern_count = patterns.len();
    for (i, pattern) in patterns.iter().enumerate() {
        let _ = write!(enum_list, "{pattern}");
        if i < top_pattern_count - 1 {
            enum_list.push('|');
        }
    }
    enum_list
}

pub fn generate_enum_bulleted_list<T: std::fmt::Display>(patterns: &[T]) -> String {
    let mut enum_list = String::new();
    enum_list.push('\n');
    for pattern in patterns {
        let _ = writeln!(enum_list, "- {pattern}");
    }
    enum_list
}

pub fn all_enum_variants<T: IntoEnumIterator>() -> Vec<T> {
    T::iter().collect()
}

impl AIAgent {
    /// Simple prompt method for text generation
    pub async fn prompt(&self, prompt: &str) -> anyhow::Result<String> {
        use rig::completion::Prompt;

        let out = match self {
            AIAgent::Anthropic { agent, .. } => agent.prompt(prompt).await?,
            AIAgent::Openai { agent, .. } => agent.prompt(prompt).await?,
            AIAgent::Gemini { agent, .. } => agent.prompt(prompt).await?,
            AIAgent::Deepseek { agent, .. } => agent.prompt(prompt).await?,
        };
        Ok(out)
    }

    //
    pub async fn extract_with_retry<T>(&self, prompt: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            AIAgent::Anthropic { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
            AIAgent::Openai { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
            AIAgent::Gemini { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
            AIAgent::Deepseek { agent, metadata } => {
                Ok(agent_extract_with_retry::<_, T>(agent, prompt, metadata).await?)
            }
        }
    }

    // Getter methods for pricing calculations and configuration inspection

    /// Gets the model name.
    pub fn get_model(&self) -> &str {
        match self {
            AIAgent::Anthropic { metadata, .. } => &metadata.model,
            AIAgent::Openai { metadata, .. } => &metadata.model,
            AIAgent::Gemini { metadata, .. } => &metadata.model,
            AIAgent::Deepseek { metadata, .. } => &metadata.model,
        }
    }

    /// Gets the OpenAI service tier.
    /// Returns "default" if not explicitly set or not applicable.
    pub fn get_service_tier(&self) -> &str {
        match self {
            AIAgent::Openai { metadata, .. } => {
                metadata.service_tier.as_deref().unwrap_or("default")
            }
            _ => "default", // Non-OpenAI providers don't have service tiers
        }
    }

    /// Gets the OpenAI reasoning effort level.
    /// Returns "medium" if not explicitly set or not applicable.
    pub fn get_reasoning_effort(&self) -> &str {
        match self {
            AIAgent::Openai { metadata, .. } => {
                metadata.reasoning_effort.as_deref().unwrap_or("medium")
            }
            _ => "medium", // Non-OpenAI providers don't have reasoning effort
        }
    }

    /// Gets the temperature setting.
    pub fn get_temperature(&self) -> f64 {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.temperature,
            AIAgent::Openai { metadata, .. } => metadata.temperature,
            AIAgent::Gemini { metadata, .. } => metadata.temperature,
            AIAgent::Deepseek { metadata, .. } => metadata.temperature,
        }
    }

    /// Checks if file picker is enabled.
    pub fn is_file_picker_enabled(&self) -> bool {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.file_picker_enabled,
            AIAgent::Openai { metadata, .. } => metadata.file_picker_enabled,
            AIAgent::Gemini { metadata, .. } => metadata.file_picker_enabled,
            AIAgent::Deepseek { metadata, .. } => metadata.file_picker_enabled,
        }
    }

    /// Checks if file retrieval is enabled.
    pub fn is_file_retrieval_enabled(&self) -> bool {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.file_retrieval_enabled,
            AIAgent::Openai { metadata, .. } => metadata.file_retrieval_enabled,
            AIAgent::Gemini { metadata, .. } => metadata.file_retrieval_enabled,
            AIAgent::Deepseek { metadata, .. } => metadata.file_retrieval_enabled,
        }
    }

    /// Checks if dynamic context is enabled.
    pub fn is_dynamic_context_enabled(&self) -> bool {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata.dynamic_context_enabled,
            AIAgent::Openai { metadata, .. } => metadata.dynamic_context_enabled,
            AIAgent::Gemini { metadata, .. } => metadata.dynamic_context_enabled,
            AIAgent::Deepseek { metadata, .. } => metadata.dynamic_context_enabled,
        }
    }

    /// Gets the provider name.
    pub fn get_provider(&self) -> &'static str {
        match self {
            AIAgent::Anthropic { .. } => "anthropic",
            AIAgent::Openai { .. } => "openai",
            AIAgent::Gemini { .. } => "gemini",
            AIAgent::Deepseek { .. } => "deepseek",
        }
    }

    /// Gets the complete metadata for pricing calculations and configuration inspection.
    /// This is the easiest way to get metadata for cost tracking functions.
    pub fn get_metadata(&self) -> &AgentMetadata {
        match self {
            AIAgent::Anthropic { metadata, .. } => metadata,
            AIAgent::Openai { metadata, .. } => metadata,
            AIAgent::Gemini { metadata, .. } => metadata,
            AIAgent::Deepseek { metadata, .. } => metadata,
        }
    }
}
