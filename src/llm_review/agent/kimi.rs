//! Kimi k2.5 integration via Fireworks.ai API.
//!
//! This module provides a custom HTTP client for the Fireworks.ai API,
//! specifically for the Kimi k2.5 model. It does NOT use rig-core.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Maximum retry attempts for failed requests
const MAX_ATTEMPTS: usize = 10;

/// Fireworks API endpoint
const FIREWORKS_API_URL: &str = "https://api.fireworks.ai/inference/v1/chat/completions";

/// Kimi k2.5 model identifier on Fireworks (full path)
pub const KIMI_K2P5_MODEL: &str = "accounts/fireworks/models/kimi-k2p5";

/// Kimi k2.5 model identifier (friendly alias)
pub const KIMI_K2_5: &str = "kimi_k2_5";

/// Chat message for Fireworks API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Request body for Fireworks chat completions API
#[derive(Debug, Clone, Serialize)]
pub struct FireworksChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<FireworksResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Fireworks structured output format.
///
/// See: https://docs.fireworks.ai/structured-responses/structured-response-formatting
#[derive(Debug, Clone, Serialize)]
pub struct FireworksResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String,
}

impl FireworksResponseFormat {
    /// Force the model to output valid JSON (no schema enforcement).
    pub fn json_object() -> Self {
        Self {
            format_type: "json_object".to_string(),
        }
    }
}

/// Choice in Fireworks API response
#[derive(Debug, Clone, Deserialize)]
pub struct FireworksChoice {
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

/// Usage information from Fireworks API
#[derive(Debug, Clone, Deserialize)]
pub struct FireworksUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Response from Fireworks chat completions API
#[derive(Debug, Clone, Deserialize)]
pub struct FireworksChatResponse {
    pub id: String,
    pub choices: Vec<FireworksChoice>,
    pub usage: Option<FireworksUsage>,
}

/// Error response from Fireworks API
#[derive(Debug, Clone, Deserialize)]
pub struct FireworksError {
    pub error: FireworksErrorDetail,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FireworksErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}

/// Kimi client for Fireworks API
#[derive(Clone)]
pub struct KimiClient {
    client: Client,
    api_key: String,
}

impl KimiClient {
    /// Creates a new KimiClient with the given API key
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minute timeout for long responses
            .build()
            .expect("Failed to create HTTP client");

        Self { client, api_key }
    }

    /// Creates a KimiClient from the FIREWORKS_API_KEY environment variable
    pub fn from_env() -> Option<Self> {
        std::env::var("FIREWORKS_API_KEY")
            .ok()
            .map(|key| Self::new(key))
    }

    /// Sends a chat completion request to Fireworks API
    pub async fn chat_completion(
        &self,
        request: &FireworksChatRequest,
    ) -> Result<FireworksChatResponse, KimiError> {
        // Check if streaming is enabled
        if request.stream == Some(true) {
            return self.chat_completion_streaming(request).await;
        }

        let response = self
            .client
            .post(FIREWORKS_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(request)
            .send()
            .await
            .map_err(|e| KimiError::Network(e.to_string()))?;

        let status = response.status();

        if status.is_success() {
            response
                .json::<FireworksChatResponse>()
                .await
                .map_err(|e| KimiError::Parse(e.to_string()))
        } else {
            let error_text = response.text().await.unwrap_or_default();
            if status.as_u16() == 429 {
                Err(KimiError::RateLimit(error_text))
            } else if status.is_server_error() {
                Err(KimiError::Server(status.as_u16(), error_text))
            } else {
                Err(KimiError::Api(status.as_u16(), error_text))
            }
        }
    }

    /// Handles streaming chat completion requests
    async fn chat_completion_streaming(
        &self,
        request: &FireworksChatRequest,
    ) -> Result<FireworksChatResponse, KimiError> {
        use futures_util::StreamExt;

        let response = self
            .client
            .post(FIREWORKS_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(request)
            .send()
            .await
            .map_err(|e| KimiError::Network(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return if status.as_u16() == 429 {
                Err(KimiError::RateLimit(error_text))
            } else if status.is_server_error() {
                Err(KimiError::Server(status.as_u16(), error_text))
            } else {
                Err(KimiError::Api(status.as_u16(), error_text))
            };
        }

        // Collect streaming response
        let mut stream = response.bytes_stream();
        let mut accumulated_content = String::new();
        let mut prompt_tokens = 0;
        let mut completion_tokens = 0;
        let mut finish_reason = None;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| KimiError::Network(e.to_string()))?;
            let chunk_str = String::from_utf8_lossy(&chunk);

            // Parse SSE format: "data: {...}\n\n"
            for line in chunk_str.lines() {
                if let Some(json_str) = line.strip_prefix("data: ") {
                    if json_str.trim() == "[DONE]" {
                        break;
                    }

                    // Parse the streaming chunk
                    if let Ok(chunk_data) = serde_json::from_str::<serde_json::Value>(json_str) {
                        // Extract content delta
                        if let Some(choices) = chunk_data.get("choices").and_then(|c| c.as_array())
                        {
                            if let Some(choice) = choices.first() {
                                if let Some(delta) = choice.get("delta") {
                                    if let Some(content) =
                                        delta.get("content").and_then(|c| c.as_str())
                                    {
                                        accumulated_content.push_str(content);
                                    }
                                }
                                if let Some(reason) =
                                    choice.get("finish_reason").and_then(|r| r.as_str())
                                {
                                    finish_reason = Some(reason.to_string());
                                }
                            }
                        }

                        // Extract usage info (usually in the last chunk)
                        if let Some(usage) = chunk_data.get("usage") {
                            if let Some(p) = usage.get("prompt_tokens").and_then(|t| t.as_u64()) {
                                prompt_tokens = p as u32;
                            }
                            if let Some(c) = usage.get("completion_tokens").and_then(|t| t.as_u64())
                            {
                                completion_tokens = c as u32;
                            }
                        }
                    }
                }
            }
        }

        // Construct a non-streaming response format
        Ok(FireworksChatResponse {
            id: "streaming-response".to_string(), // Streaming responses don't have IDs
            choices: vec![FireworksChoice {
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content: accumulated_content,
                },
                finish_reason,
            }],
            usage: Some(FireworksUsage {
                prompt_tokens,
                completion_tokens,
                total_tokens: prompt_tokens + completion_tokens,
            }),
        })
    }
}

/// Kimi-specific error types
#[derive(Debug)]
pub enum KimiError {
    Network(String),
    RateLimit(String),
    Server(u16, String),
    Api(u16, String),
    Parse(String),
}

impl std::fmt::Display for KimiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KimiError::Network(msg) => write!(f, "Network error: {}", msg),
            KimiError::RateLimit(msg) => write!(f, "Rate limit exceeded: {}", msg),
            KimiError::Server(code, msg) => write!(f, "Server error {}: {}", code, msg),
            KimiError::Api(code, msg) => write!(f, "API error {}: {}", code, msg),
            KimiError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for KimiError {}

impl KimiError {
    /// Returns true if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            KimiError::RateLimit(_) | KimiError::Server(_, _) | KimiError::Network(_)
        )
    }
}

/// Configuration for KimiAgent
#[derive(Debug, Clone)]
pub struct KimiAgentConfig {
    pub model: String,
    pub preamble: String,
    pub context: Option<String>,
    pub temperature: f64,
    pub max_tokens: u32,
    pub top_p: f64,
    pub top_k: u32,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
}

impl Default for KimiAgentConfig {
    fn default() -> Self {
        Self {
            model: KIMI_K2P5_MODEL.to_string(),
            preamble: String::new(),
            context: None,
            temperature: 1.0,
            max_tokens: 16384, // Streaming enabled automatically for >4096 tokens
            top_p: 1.0,
            top_k: 40,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        }
    }
}

/// Kimi agent for text generation
#[derive(Clone)]
pub struct KimiAgent {
    client: KimiClient,
    config: KimiAgentConfig,
}

impl KimiAgent {
    /// Creates a new KimiAgent with the given client and configuration
    pub fn new(client: KimiClient, config: KimiAgentConfig) -> Self {
        Self { client, config }
    }

    /// Builds the messages array for the API request
    fn build_messages(&self, user_prompt: &str) -> Vec<ChatMessage> {
        let mut messages = Vec::new();

        // Add preamble as system message
        if !self.config.preamble.is_empty() {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: self.config.preamble.clone(),
            });
        }

        // Add context as system message if present
        if let Some(ref context) = self.config.context {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: context.clone(),
            });
        }

        // Add user prompt
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_prompt.to_string(),
        });

        messages
    }

    /// Builds messages for structured extraction.
    ///
    /// Even when using `response_format`, Fireworks recommends explicitly instructing the
    /// model to output JSON in the prompt.
    fn build_messages_for_json(&self, user_prompt: &str) -> Vec<ChatMessage> {
        let mut messages = Vec::new();

        if !self.config.preamble.is_empty() {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: self.config.preamble.clone(),
            });
        }

        if let Some(ref context) = self.config.context {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: context.clone(),
            });
        }

        // Hard guardrail: JSON only.
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: "Return ONLY valid JSON. Do not include any analysis, markdown, code fences, or extra text.".to_string(),
        });

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_prompt.to_string(),
        });

        messages
    }

    /// Sends a prompt and returns the response text.
    ///
    /// This method does NOT retry on errors. Use `extract_with_retry()` for
    /// structured JSON extraction with automatic retry logic.
    pub async fn prompt(&self, user_prompt: &str) -> Result<String, KimiError> {
        let messages = self.build_messages(user_prompt);

        let request = FireworksChatRequest {
            model: self.config.model.clone(),
            messages,
            response_format: None,
            max_tokens: Some(self.config.max_tokens),
            temperature: Some(self.config.temperature),
            top_p: Some(self.config.top_p),
            top_k: Some(self.config.top_k),
            presence_penalty: Some(self.config.presence_penalty),
            frequency_penalty: Some(self.config.frequency_penalty),
            stream: Some(self.config.max_tokens > 4096), // Enable streaming for >4096 tokens
        };

        let response = self.client.chat_completion(&request).await?;

        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(KimiError::Parse("No choices in response".to_string()))
        }
    }

    /// Sends a prompt and extracts structured data from the JSON response with retry logic.
    ///
    /// This method prompts the LLM, parses the response as JSON, and returns the
    /// deserialized type. It retries on transient errors and parse failures.
    ///
    /// Cost tracking is performed for both input and output tokens.
    pub async fn extract_with_retry<T>(
        &self,
        user_prompt: &str,
        metadata: &crate::llm_review::agent::agent_enums::AgentMetadata,
    ) -> Result<T, KimiError>
    where
        T: serde::de::DeserializeOwned,
    {
        use crate::cost::cost_data::{TokenType, add_to_inference_cost_by_type};
        use crate::llm_review::findings::findings::FromLLMJson;

        let mut last_error = None;

        for attempt in 1..=MAX_ATTEMPTS {
            // Track input token cost
            add_to_inference_cost_by_type(user_prompt, metadata, TokenType::Input).await;

            // Get the raw response (forced JSON mode)
            let messages = self.build_messages_for_json(user_prompt);
            let request = FireworksChatRequest {
                model: self.config.model.clone(),
                messages,
                response_format: Some(FireworksResponseFormat::json_object()),
                max_tokens: Some(self.config.max_tokens),
                temperature: Some(self.config.temperature),
                top_p: Some(self.config.top_p),
                top_k: Some(self.config.top_k),
                presence_penalty: Some(self.config.presence_penalty),
                frequency_penalty: Some(self.config.frequency_penalty),
                stream: Some(self.config.max_tokens > 4096), // Enable streaming for >4096 tokens
            };

            if attempt == 1 {
                // Important: don't log the full prompt/context. We just confirm JSON mode is enabled.
                log::debug!(
                    "Kimi Fireworks request: model={}, response_format={:?}, max_tokens={:?}, temperature={:?}, stream={:?}",
                    request.model,
                    request
                        .response_format
                        .as_ref()
                        .map(|f| f.format_type.as_str()),
                    request.max_tokens,
                    request.temperature,
                    request.stream
                );
            }

            let raw = match self.client.chat_completion(&request).await {
                Ok(resp) => {
                    // Log finish_reason to detect truncation
                    if let Some(choice) = resp.choices.first() {
                        if let Some(ref reason) = choice.finish_reason {
                            if reason == "length" {
                                log::warn!(
                                    "Kimi response truncated due to max_tokens limit (finish_reason=length)"
                                );
                            } else {
                                log::debug!("Kimi finish_reason: {}", reason);
                            }
                        }
                    }
                    resp.choices
                        .first()
                        .map(|c| c.message.content.clone())
                        .ok_or_else(|| KimiError::Parse("No choices in response".to_string()))?
                }
                Err(e) if e.is_retryable() && attempt < MAX_ATTEMPTS => {
                    log::warn!(
                        "Kimi extract error (attempt {}/{}): {} - retrying...",
                        attempt,
                        MAX_ATTEMPTS,
                        e
                    );
                    last_error = Some(e);
                    continue;
                }
                Err(e) => return Err(e),
            };

            // Track output token cost
            add_to_inference_cost_by_type(&raw, metadata, TokenType::Output).await;

            // Try to parse the response as JSON
            match T::parse_from_llm_response(&raw) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let msg = e.to_string();
                    if attempt < MAX_ATTEMPTS && Self::should_retry_parse_error(&msg) {
                        log::warn!(
                            "Kimi JSON parse error (attempt {}/{}): {} - retrying...",
                            attempt,
                            MAX_ATTEMPTS,
                            msg
                        );
                        // Log the actual response for debugging
                        log::warn!(
                            "Kimi FULL raw response (total {} chars): {}",
                            raw.len(),
                            raw // &raw[..raw.len().min(5000)]
                        );
                        last_error = Some(KimiError::Parse(msg));
                        continue;
                    }
                    // Log the final failed response
                    log::error!(
                        "Kimi final parse failure. Raw response (first 1000 chars): {}",
                        &raw[..raw.len().min(1000)]
                    );
                    return Err(KimiError::Parse(msg));
                }
            }
        }

        Err(last_error.unwrap_or(KimiError::Parse(
            "Max retries exceeded for JSON extraction".to_string(),
        )))
    }

    /// Determines if a parse error is retryable
    fn should_retry_parse_error(error_msg: &str) -> bool {
        let lower = error_msg.to_lowercase();
        const RETRYABLE_PATTERNS: &[&str] = &[
            "unexpected",
            "invalid",
            "syntax",
            "parse",
            "json",
            "deserialize",
            "missing field",
            "expected",
        ];
        RETRYABLE_PATTERNS.iter().any(|p| lower.contains(p))
    }
}
