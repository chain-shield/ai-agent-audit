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

use super::enums::{AIAgent, AIExtractor};

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

    AIAgent::Anthropic(agent)
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

    match context {
        Some(added_context) => AIAgent::Openai(builder.context(added_context).build()),
        None => AIAgent::Openai(builder.build()),
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

    match context {
        Some(added_context) => AIAgent::Gemini(builder.context(added_context).build()),
        None => AIAgent::Gemini(builder.build()),
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

    match context {
        Some(added_context) => AIAgent::Deepseek(builder.context(added_context).build()),
        None => AIAgent::Deepseek(builder.build()),
    }
}
