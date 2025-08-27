use crate::llm_review::enums::AIAgent;
/// Cost tracking and calculation for LLM inference across multiple providers.
///
/// This module provides real-time cost tracking for AI agent operations,
/// supporting OpenAI, Anthropic, Gemini, and DeepSeek providers with
/// accurate token-based pricing calculations.
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

/// LLM provider cost types with specific model variants
#[derive(Clone, Copy, Debug)]
pub enum LlmCostType {
    /// OpenAI GPT-5 input tokens
    Openai5Input,
    /// OpenAI GPT-5 input tokens
    Openai5Output,
    /// OpenAI GPT-4o input tokens
    Openai4oInput,
    /// OpenAI GPT-4o output tokens
    Openai4oOutput,
    /// OpenAI O3 input tokens
    OpenaiO3Input,
    /// OpenAI O3 output tokens
    OpenaiO3Output,
    /// Anthropic Claude input tokens
    AnthropicClaudeInput,
    /// Anthropic Claude output tokens
    AnthropicClaudeOutput,
    /// Google Gemini input tokens
    GeminiInput,
    /// Google Gemini output tokens
    GeminiOutput,
    /// DeepSeek input tokens
    DeepseekInput,
    /// DeepSeek output tokens
    DeepseekOutput,
}

/// Token direction for cost calculation
#[derive(PartialEq, Eq)]
pub enum TokenType {
    /// Input tokens (prompt)
    Input,
    /// Output tokens (response)
    Output,
}

impl LlmCostType {
    /// Returns the cost per million tokens for each LLM provider and model.
    /// Prices are based on current provider pricing as of 2024.
    pub fn get_cost_per_million_tokens(self) -> f64 {
        match self {
            LlmCostType::Openai4oInput => 2.50,
            LlmCostType::Openai4oOutput => 10.00,
            LlmCostType::Openai5Input => 1.25,
            LlmCostType::Openai5Output => 10.00,
            LlmCostType::OpenaiO3Input => 2.00,
            LlmCostType::OpenaiO3Output => 8.00,
            LlmCostType::AnthropicClaudeInput => 3.00,
            LlmCostType::AnthropicClaudeOutput => 15.00,
            LlmCostType::GeminiInput => 1.25,
            LlmCostType::GeminiOutput => 10.00,
            LlmCostType::DeepseekInput => 0.07,
            LlmCostType::DeepseekOutput => 1.10,
        }
    }
}

impl AIAgent {
    pub fn get_cost_per_million_tokens(&self, token_type: TokenType) -> f64 {
        match self {
            // assume highest cost
            AIAgent::Openai(_) if token_type == TokenType::Input => 2.00,
            AIAgent::Openai(_) => 8.00,
            AIAgent::Anthropic(_) if token_type == TokenType::Input => 3.00,
            AIAgent::Anthropic(_) => 15.00,
            AIAgent::Gemini(_) if token_type == TokenType::Input => 1.25,
            AIAgent::Gemini(_) => 10.00,
            AIAgent::Deepseek(_) if token_type == TokenType::Input => 0.07,
            AIAgent::Deepseek(_) => 1.10,
        }
    }
}
static INFERENCE_COST_DATA: Lazy<Arc<Mutex<f64>>> = Lazy::new(|| Arc::new(Mutex::new(0.0)));

pub async fn add_to_inference_cost_by_type(content: &str, cost_type: LlmCostType) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;
    let tokens = get_token_count(content);

    *inference_cost = *inference_cost + tokens as f64 * cost_type.get_cost_per_million_tokens();
}

pub async fn add_to_inference_cost_by_agent(
    content: &str,
    agent: &Arc<AIAgent>,
    token_type: TokenType,
) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;
    let tokens = get_token_count(content);

    *inference_cost =
        *inference_cost + tokens as f64 * agent.get_cost_per_million_tokens(token_type);
}

pub async fn get_total_inference_cost() -> String {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let inference_cost = cost_data.lock().await;

    format!("{:.2}", *inference_cost / 1_000_000_f64)
}

/// Estimates tokens with additional handling for whitespace and special cases
pub fn get_token_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    // Remove extra whitespace and count characters
    let cleaned = text.trim();
    let char_count = cleaned.chars().count();

    // Use ceiling division: (n + divisor - 1) / divisor
    (char_count + 3) / 4
}
