use rig::{
    client::CompletionClient,
    providers::{
        anthropic::{self},
        deepseek,
        gemini::{self},
        openai::{self},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::llm_review::agent::agent_enums::{AIAgent, AIExtractor, AgentMetadata};

pub fn build_anthropic_agent(
    client: &anthropic::Client,
    temperature: f64,
    model: &str,
    max_tokens: u64,
) -> AIAgent {
    let agent = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .max_tokens(max_tokens)
        .temperature(temperature)
        .build();

    // Create metadata for pricing calculations
    let metadata = AgentMetadata {
        model: model.to_string(),
        temperature,
        service_tier: None,     // Anthropic doesn't have service tiers
        reasoning_effort: None, // Anthropic doesn't have reasoning effort
        file_picker_enabled: false,
        file_retrieval_enabled: false,
        dynamic_context_enabled: false,
    };

    AIAgent::Anthropic { agent, metadata }
}

pub fn build_openai_extractor<T>(
    client: &openai::Client,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIExtractor<T>
where
    T: 'static + JsonSchema + Serialize + for<'a> Deserialize<'a> + Send + Sync,
{
    let builder = client.extractor::<T>(model).preamble(preamble);

    match context {
        Some(added_context) => AIExtractor::Openai(builder.context(added_context).build()),
        None => AIExtractor::Openai(builder.build()),
    }
}

pub fn build_openai_agent(
    client: &openai::Client,
    temperature: f64,
    model: &str,
    preamble: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(preamble)
        .temperature(temperature);

    // Create metadata for pricing calculations
    let metadata = AgentMetadata {
        model: model.to_string(),
        temperature,
        service_tier: None,     // Default service tier for legacy functions
        reasoning_effort: None, // Default reasoning effort for legacy functions
        file_picker_enabled: false,
        file_retrieval_enabled: false,
        dynamic_context_enabled: false,
    };

    match context {
        Some(added_context) => AIAgent::Openai {
            agent: builder.context(added_context).build(),
            metadata,
        },
        None => AIAgent::Openai {
            agent: builder.build(),
            metadata,
        },
    }
}

pub fn build_gemini_agent(
    client: &gemini::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    // Create metadata for pricing calculations
    let metadata = AgentMetadata {
        model: model.to_string(),
        temperature,
        service_tier: None,     // Gemini doesn't have service tiers
        reasoning_effort: None, // Gemini doesn't have reasoning effort
        file_picker_enabled: false,
        file_retrieval_enabled: false,
        dynamic_context_enabled: false,
    };

    match context {
        Some(added_context) => AIAgent::Gemini {
            agent: builder.context(added_context).build(),
            metadata,
        },
        None => AIAgent::Gemini {
            agent: builder.build(),
            metadata,
        },
    }
}

pub fn build_deepseek_agent(
    client: &deepseek::Client,
    temperature: f64,
    model: &str,
    context: Option<&str>,
) -> AIAgent {
    let builder = client
        .agent(model)
        .preamble(
            "You are a world renowned expert in smart-contract security auditing, 
            known for your uncanny ability to find all security bugs in a protocol, 
            even the obscure ones.",
        )
        .temperature(temperature);

    // Create metadata for pricing calculations
    let metadata = AgentMetadata {
        model: model.to_string(),
        temperature,
        service_tier: None,     // DeepSeek doesn't have service tiers
        reasoning_effort: None, // DeepSeek doesn't have reasoning effort
        file_picker_enabled: false,
        file_retrieval_enabled: false,
        dynamic_context_enabled: false,
    };

    match context {
        Some(added_context) => AIAgent::Deepseek {
            agent: builder.context(added_context).build(),
            metadata,
        },
        None => AIAgent::Deepseek {
            agent: builder.build(),
            metadata,
        },
    }
}
