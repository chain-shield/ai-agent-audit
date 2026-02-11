use crate::{
    llm_review::agent::agent_enums::{AIAgent, AgentMetadata},
    utils::bpe::get_bpe,
};
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
    /// Kimi/Fireworks input tokens
    KimiInput,
    /// Kimi/Fireworks output tokens
    KimiOutput,
}

/// Token direction for cost calculation
#[derive(PartialEq, Eq, Copy, Clone)]
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
            LlmCostType::Openai5Output => 10.00, // triple book cost to account for reasoning tokens
            LlmCostType::OpenaiO3Input => 2.00,
            LlmCostType::OpenaiO3Output => 8.00,
            LlmCostType::AnthropicClaudeInput => 3.00,
            LlmCostType::AnthropicClaudeOutput => 15.00,
            LlmCostType::GeminiInput => 1.25,
            LlmCostType::GeminiOutput => 10.00,
            LlmCostType::DeepseekInput => 0.07,
            LlmCostType::DeepseekOutput => 1.10,
            LlmCostType::KimiInput => 0.60,
            LlmCostType::KimiOutput => 3.00,
        }
    }
}

pub fn get_cost_per_million_tokens_by_model(model: &str, token_type: TokenType) -> f64 {
    match token_type {
        TokenType::Input => match model {
            // OpenAI models
            "gpt-4o" => 2.50,
            "gpt-5" => 1.25,
            "gpt-5-mini" => 0.25,
            "gpt-5.1" => 1.25,
            "gpt-5.2" => 1.75,
            "o3" => 2.00,

            // Anthropic models
            "claude-3.5-sonnet" | "claude-sonnet-3-5" => 3.00,
            "claude-3.7-sonnet" | "claude-sonnet-3-7" => 3.00,
            "claude-4.0-sonnet" | "claude-sonnet-4-0" => 3.00,
            "claude-4.5-sonnet" | "claude-sonnet-4-5" => 3.00,
            "claude-4" => 3.00,

            // Gemini models
            "gemini-2.5-pro" | "gemini-2-5-pro" => 1.25,
            "gemini-pro" => 1.25,
            "gemini-3-pro-preview" => 2.00,

            // DeepSeek models
            "deepseek-chat" => 0.07,
            "deepseek-coder" => 0.07,

            // Kimi/Fireworks models
            "accounts/fireworks/models/kimi-k2p5" | "kimi-k2p5" | "kimi-k2.5" => 0.60,

            // Default fallback for unknown models
            _ => {
                log::warn!(
                    "Unknown model '{}' for input tokens, using default rate",
                    model
                );
                1.25 // Default to GPT-5 input rate
            }
        },
        TokenType::Output => match model {
            // OpenAI models
            "gpt-4o" => 10.00,
            "gpt-5" => 25.00, // set to 2.5X actual value to account for reasoning tokens
            "gpt-5-mini" => 2.00,
            "gpt-5.1" => 25.00,
            "gpt-5.2" => 30.00,
            "o3" => 8.00,

            // Anthropic models
            "claude-3.5-sonnet" | "claude-sonnet-3-5" => 15.00,
            "claude-3.7-sonnet" | "claude-sonnet-3-7" => 15.00,
            "claude-4.0-sonnet" | "claude-sonnet-4-0" => 15.00,
            "claude-4.5-sonnet" | "claude-sonnet-4-5" => 15.00,
            "claude-4" => 15.00,

            // Gemini models
            "gemini-2.5-pro" | "gemini-2-5-pro" => 10.00,
            "gemini-pro" => 10.00,
            "gemini-3-pro-preview" => 12.00,

            // DeepSeek models
            "deepseek-chat" => 1.10,
            "deepseek-coder" => 1.10,

            // Kimi/Fireworks models
            "accounts/fireworks/models/kimi-k2p5" | "kimi-k2p5" | "kimi-k2.5" => 3.00,

            // Default fallback for unknown models
            _ => {
                log::warn!(
                    "Unknown model '{}' for output tokens, using default rate",
                    model
                );
                10.00 // Default to GPT-5 output rate
            }
        },
    }
}

impl AIAgent {
    pub fn get_cost_per_million_tokens(&self, token_type: TokenType) -> f64 {
        match self {
            // Use correct GPT-5 rates: $1.25 input, $10.00 output
            AIAgent::Openai { .. } if token_type == TokenType::Input => 1.25,
            AIAgent::Openai { .. } => 10.00,
            AIAgent::Anthropic { .. } if token_type == TokenType::Input => 3.00,
            AIAgent::Anthropic { .. } => 15.00,
            AIAgent::Gemini { .. } if token_type == TokenType::Input => 1.25,
            AIAgent::Gemini { .. } => 10.00,
            AIAgent::Deepseek { .. } if token_type == TokenType::Input => 0.07,
            AIAgent::Deepseek { .. } => 1.10,
            AIAgent::Kimi { .. } if token_type == TokenType::Input => 0.60,
            AIAgent::Kimi { .. } => 3.00,
        }
    }
}
static INFERENCE_COST_DATA: Lazy<Arc<Mutex<f64>>> = Lazy::new(|| Arc::new(Mutex::new(0.0)));

// main method to update cost
pub async fn add_to_inference_cost_by_type(
    content: &str,
    metadata: &AgentMetadata,
    token_type: TokenType,
) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;

    // Use improved token counting for input tokens
    let tokens = if metadata.model == "gpt-5" {
        estimate_chat_completion_tokens(content)
    } else {
        get_token_count(content)
    };

    // Convert tokens to cost: (tokens * cost_per_million) / 1M for better precision
    let cost_per_million = get_cost_per_million_tokens_by_model(&metadata.model, token_type);
    let mut cost = (tokens as f64 * cost_per_million) / 1_000_000.0;

    // reduce cost if flex mode
    cost = if metadata.service_tier.as_deref().unwrap_or("default") == "flex" {
        cost / 2.0
    } else {
        cost
    };

    // cost update
    *inference_cost = *inference_cost + cost;
}

pub async fn add_to_inference_cost_by_agent(
    content: &str,
    agent: &Arc<AIAgent>,
    token_type: TokenType,
) {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let mut inference_cost = cost_data.lock().await;

    // Use improved token counting for input tokens
    let tokens = match (agent.as_ref(), token_type) {
        (AIAgent::Openai { .. }, TokenType::Input) => estimate_chat_completion_tokens(content),
        _ => get_token_count(content),
    };

    // Convert tokens to cost: (tokens * cost_per_million) / 1M for better precision
    let cost = (tokens as f64 * agent.get_cost_per_million_tokens(token_type)) / 1_000_000.0;
    *inference_cost = *inference_cost + cost;
}

pub async fn get_total_inference_cost() -> String {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let inference_cost = cost_data.lock().await;

    // Cost is already in dollars, use more precision for small amounts
    format!("{:.6}", *inference_cost)
}

/// Get total inference cost formatted for user display
pub async fn get_total_inference_cost_display() -> String {
    let cost_data = Arc::clone(&INFERENCE_COST_DATA);
    let inference_cost = cost_data.lock().await;

    // Format appropriately based on cost magnitude
    if *inference_cost >= 1.0 {
        format!("${:.2}", *inference_cost)
    } else if *inference_cost >= 0.01 {
        format!("${:.4}", *inference_cost)
    } else {
        format!("${:.6}", *inference_cost)
    }
}

/// Accurately counts tokens using OpenAI's BPE tokenizer
pub fn get_token_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    // NOTE testing different count
    // Use the actual BPE tokenizer for accurate token counting
    let bpe = get_bpe();
    bpe.encode_with_special_tokens(text).len()
}

/// Estimates input tokens for chat completion format
/// This accounts for the overhead of chat completion structure
pub fn estimate_chat_completion_tokens(user_message: &str) -> usize {
    // OpenAI chat completion adds overhead for:
    // - Message structure
    // - Role markers
    // - System formatting
    // Based on empirical testing, add ~6 tokens overhead
    let base_tokens = get_token_count(user_message);
    base_tokens + 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cost_calculation_accuracy() {
        // Reset cost tracking
        let cost_data = Arc::clone(&INFERENCE_COST_DATA);
        {
            let mut inference_cost = cost_data.lock().await;
            *inference_cost = 0.0;
        }

        // Test with a known string
        let test_text = "Hello world, this is a test prompt for GPT-5 cost calculation.";

        // Create test metadata for GPT-5
        let test_metadata = AgentMetadata {
            model: "gpt-5".to_string(),
            temperature: 0.7,
            service_tier: None,
            reasoning_effort: None,
            file_picker_enabled: false,
            file_retrieval_enabled: false,
            dynamic_context_enabled: false,
        };

        // Add input cost
        add_to_inference_cost_by_type(test_text, &test_metadata, TokenType::Input).await;

        // Get token count and verify calculation
        let tokens = estimate_chat_completion_tokens(test_text); // Use improved counting
        let expected_cost = (tokens as f64 * 1.25) / 1_000_000.0; // $1.25 per 1M tokens

        let total_cost_str = get_total_inference_cost().await;
        let total_cost: f64 = total_cost_str.parse().unwrap();

        println!("Test text: '{}'", test_text);
        println!("Token count: {}", tokens);
        println!("Expected cost: ${:.6}", expected_cost);
        println!("Actual cost: ${:.6}", total_cost);

        // Should be very close (within rounding error)
        assert!(
            (total_cost - expected_cost).abs() < 0.000001,
            "Cost calculation mismatch: expected ${:.6}, got ${:.6}",
            expected_cost,
            total_cost
        );
    }

    #[tokio::test]
    async fn test_cost_calculation_vs_openai_api() {
        use reqwest::Client;
        use serde_json::{Value, json};

        // Skip test if no API key
        let api_key = match std::env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("Skipping OpenAI API test - no OPENAI_API_KEY found");
                return;
            }
        };

        // Reset cost tracking
        let cost_data = Arc::clone(&INFERENCE_COST_DATA);
        {
            let mut inference_cost = cost_data.lock().await;
            *inference_cost = 0.0;
        }

        let test_prompt = "Hello, this is a test prompt to verify cost calculation accuracy.";

        // Create test metadata for GPT-5
        let test_metadata = AgentMetadata {
            model: "gpt-5".to_string(),
            temperature: 0.7,
            service_tier: None,
            reasoning_effort: None,
            file_picker_enabled: false,
            file_retrieval_enabled: false,
            dynamic_context_enabled: false,
        };

        // Track input cost with our system
        add_to_inference_cost_by_type(test_prompt, &test_metadata, TokenType::Input).await;

        // Make direct OpenAI API call
        let client = Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-5",
                "messages": [
                    {
                        "role": "user",
                        "content": test_prompt
                    }
                ],
                "max_completion_tokens": 50,
                "service_tier": "default"
            }))
            .send()
            .await;

        let response = match response {
            Ok(resp) => resp,
            Err(e) => {
                println!("API call failed: {}", e);
                return;
            }
        };

        if !response.status().is_success() {
            println!("API returned error: {}", response.status());
            println!("Response: {}", response.text().await.unwrap_or_default());
            return;
        }

        let response_json: Value = response.json().await.expect("Failed to parse JSON");

        // Extract usage information
        let usage = &response_json["usage"];
        let prompt_tokens = usage["prompt_tokens"]
            .as_u64()
            .expect("Missing prompt_tokens") as f64;
        let completion_tokens = usage["completion_tokens"]
            .as_u64()
            .expect("Missing completion_tokens") as f64;

        // Check if service tier information is available
        let service_tier = response_json
            .get("service_tier")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        println!("🔍 Service Tier: {}", service_tier);
        println!(
            "📋 Full response structure: {}",
            serde_json::to_string_pretty(&response_json).unwrap_or_default()
        );

        // Track output cost with our system
        let response_text = response_json["choices"][0]["message"]["content"]
            .as_str()
            .expect("Missing response content");

        // Use the same test metadata as for input
        add_to_inference_cost_by_type(response_text, &test_metadata, TokenType::Output).await;

        // Calculate expected cost using OpenAI's token counts and service tier
        let (input_rate, output_rate) = match service_tier {
            "auto" | "default" => (1.25, 10.00), // Standard rates
            "flex" => (1.875, 15.00),            // 50% more than standard
            "priority" => (2.50, 20.00),         // 2x more than standard
            _ => {
                println!(
                    "⚠️  Unknown service tier '{}', assuming standard rates",
                    service_tier
                );
                (1.25, 10.00)
            }
        };

        let expected_input_cost = (prompt_tokens * input_rate) / 1_000_000.0;
        let expected_output_cost = (completion_tokens * output_rate) / 1_000_000.0;
        let expected_total_cost = expected_input_cost + expected_output_cost;

        // Get our calculated cost
        let our_cost_str = get_total_inference_cost().await;
        let our_cost: f64 = our_cost_str.parse().unwrap();

        // Get our token counts
        let our_input_tokens = get_token_count(test_prompt);
        let our_output_tokens = get_token_count(response_text);

        // Also try counting the full chat completion format
        let chat_format = format!(r#"{{"role": "user", "content": "{}"}}"#, test_prompt);
        let our_chat_tokens = get_token_count(&chat_format);

        println!("=== OpenAI Token Count Verification ===");
        println!("Test prompt: '{}'", test_prompt);
        println!("Response: '{}'", response_text);
        println!("");
        println!("📊 TOKEN COMPARISON:");
        println!("  OpenAI input tokens:     {}", prompt_tokens);
        println!("  Our input tokens (raw):  {}", our_input_tokens);
        println!("  Our input tokens (chat): {}", our_chat_tokens);
        println!(
            "  Input difference (raw):  {}",
            (prompt_tokens as i64 - our_input_tokens as i64).abs()
        );
        println!(
            "  Input difference (chat): {}",
            (prompt_tokens as i64 - our_chat_tokens as i64).abs()
        );
        println!("");
        println!("  OpenAI output tokens: {}", completion_tokens);
        println!("  Our output tokens:    {}", our_output_tokens);
        println!(
            "  Output difference:    {}",
            (completion_tokens as i64 - our_output_tokens as i64).abs()
        );

        // Check if there are reasoning tokens
        if let Some(reasoning_tokens) =
            usage["completion_tokens_details"]["reasoning_tokens"].as_u64()
        {
            println!("  🧠 Reasoning tokens:     {}", reasoning_tokens);
            println!(
                "  📝 Actual content tokens: {}",
                completion_tokens - reasoning_tokens as f64
            );
        }
        println!("");
        println!("💰 COST COMPARISON:");
        println!(
            "  Service tier: {} (${:.2} input, ${:.2} output per 1M tokens)",
            service_tier, input_rate, output_rate
        );
        println!(
            "  Expected cost (OpenAI tokens): ${:.6}",
            expected_total_cost
        );
        println!("  Our calculated cost:           ${:.6}", our_cost);
        println!(
            "  Cost difference:               ${:.6}",
            (our_cost - expected_total_cost).abs()
        );

        // Analyze token count differences
        let input_token_diff = (prompt_tokens as i64 - our_input_tokens as i64).abs();
        let input_chat_diff = (prompt_tokens as i64 - our_chat_tokens as i64).abs();
        let output_token_diff = (completion_tokens as i64 - our_output_tokens as i64).abs();

        println!("🔍 ANALYSIS:");
        if input_token_diff > 5 {
            println!(
                "❌ Our raw tokenizer is significantly off (diff: {})",
                input_token_diff
            );
            println!("💡 This explains why our cost calculation is inaccurate");
        }

        if input_chat_diff < input_token_diff {
            println!(
                "✅ Chat format tokenization is closer (diff: {})",
                input_chat_diff
            );
        }

        if output_token_diff > 10 {
            println!("❌ Output token counting is problematic");
            println!("💡 This might be due to reasoning tokens or empty responses");
        }

        // Don't fail the test, just report the findings
        println!(
            "📊 CONCLUSION: Token counting discrepancies found - this explains cost calculation errors"
        );

        // 🚨 IMPORTANT: Check if rig is using a different service tier!
        if service_tier == "flex" || service_tier == "priority" {
            println!(
                "🚨 WARNING: OpenAI API used '{}' tier, not 'default'!",
                service_tier
            );
            println!("🚨 This means rig library might be defaulting to a more expensive tier!");
            if service_tier == "priority" {
                println!("🚨 Priority tier costs 2x more than default rates!");
            } else {
                println!("🚨 Flex tier costs 50% more than default rates!");
            }
        }
    }

    #[tokio::test]
    async fn test_token_overhead_patterns() {
        // Test different prompt sizes to understand overhead patterns
        let test_cases = vec![
            "Hi",
            "Hello world",
            "Hello, this is a test prompt to verify cost calculation accuracy.",
            "This is a much longer prompt that contains significantly more text to see how the token overhead scales with larger inputs. We want to understand if the 6 token overhead is fixed or if it grows proportionally with the prompt size.",
        ];

        println!("🔍 TESTING TOKEN OVERHEAD PATTERNS:");
        println!("Checking if overhead is fixed (6 tokens) or scales with prompt size\n");

        for (i, prompt) in test_cases.iter().enumerate() {
            let raw_tokens = get_token_count(prompt);
            let estimated_tokens = estimate_chat_completion_tokens(prompt);
            let overhead = estimated_tokens - raw_tokens;

            println!("Test {}: \"{}...\"", i + 1, &prompt[..prompt.len().min(30)]);
            println!("  Raw tokens: {}", raw_tokens);
            println!("  Estimated tokens: {}", estimated_tokens);
            println!("  Overhead: {} tokens", overhead);
            println!();
        }

        println!("💡 If overhead is always 6, it's fixed");
        println!("💡 If overhead varies, we need a better estimation formula");
    }
}
