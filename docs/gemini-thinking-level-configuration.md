# Gemini Thinking Level Configuration

## Overview

This document describes the implementation of Gemini 3 Pro's `thinking_level` parameter in the AI Agent Audit tool. The thinking level parameter allows you to control the reasoning depth of Gemini 3 Pro models, balancing between latency, cost, and quality.

## Background

According to the [Gemini API documentation](https://ai.google.dev/gemini-api/docs/thinking), Gemini 3 Pro models support a `thinkingLevel` parameter that can be set to:
- `"low"` - Faster responses with less reasoning depth
- `"high"` - Deeper reasoning with higher quality (default for Gemini 3 Pro Preview)

This parameter is part of the `generationConfig.thinkingConfig` object in the Gemini API.

## Implementation

### 1. Updated `ThinkingConfig` Struct

Modified the vendored `rig-core` library to add the `thinking_level` field:

```rust
// vendor/rig-core/src/providers/gemini/completion.rs
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_thoughts: Option<bool>,
    /// Thinking level for Gemini 3 Pro models ("low" or "high")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_level: Option<String>,
}
```

### 2. Added `GeminiConfig` Struct

Created a new configuration struct for Gemini-specific settings:

```rust
// src/llm_review/agent/agent_factory.rs
#[derive(Debug, Clone)]
pub struct GeminiConfig {
    /// Thinking level for Gemini 3 Pro models ("low" or "high")
    pub thinking_level: Option<String>,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            thinking_level: Some("high".to_string()),
        }
    }
}
```

### 3. Updated `AgentConfig`

Added `gemini_config` field to the main `AgentConfig` struct:

```rust
pub struct AgentConfig {
    // ... existing fields ...
    /// Gemini-specific configuration (thinking level)
    pub gemini_config: GeminiConfig,
}
```

### 4. Added Builder Method

Created a fluent builder method for setting the thinking level:

```rust
impl AgentConfig {
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
}
```

### 5. Updated `create_gemini_agent`

Modified the agent creation to apply the thinking level configuration:

```rust
// Add thinking configuration if specified
if let Some(ref thinking_level) = config.gemini_config.thinking_level {
    generation_config.thinking_config = Some(ThinkingConfig {
        thinking_budget: None,
        include_thoughts: None,
        thinking_level: Some(thinking_level.clone()),
    });
}
```

## Usage

### Basic Usage

```rust
use ai_agent_audit::llm_review::agent::agent_factory::{AgentConfig, AgentFactory};

// Create agent with high thinking level (default)
let config = AgentConfig::new(None)
    .with_model("gemini-3-pro-preview")
    .with_gemini_thinking_level("high");

let agent = AgentFactory::create_gemini_agent(&config)?;

// Create agent with low thinking level for faster responses
let config = AgentConfig::new(None)
    .with_model("gemini-3-pro-preview")
    .with_gemini_thinking_level("low");

let agent = AgentFactory::create_gemini_agent(&config)?;
```

### Integration Tests

Three integration tests demonstrate the functionality:

1. `test_gemini3_pro_simple_prompt` - Tests basic prompting with high thinking level
2. `test_gemini3_pro_json_extraction` - Tests JSON extraction with high thinking level
3. `test_gemini3_pro_thinking_level_low` - Tests low thinking level for simple tasks

Run tests with:
```bash
cargo test --test gemini3_pro_integration_test -- --nocapture
```

## Validation

The implementation includes validation to ensure only valid thinking levels are used:

```rust
const VALID_THINKING_LEVELS: &[&str] = &["low", "high"];

impl GeminiConfig {
    pub fn validate_thinking_level(level: &str) -> Result<()> {
        if VALID_THINKING_LEVELS.contains(&level) {
            Ok(())
        } else {
            Err(AuditError::configuration(
                "gemini_thinking_level",
                &format!(
                    "Invalid thinking level '{}'. Valid options: {}",
                    level,
                    VALID_THINKING_LEVELS.join(", ")
                ),
            ))
        }
    }
}
```

## Notes

- The default thinking level is `"high"` for Gemini 3 Pro Preview
- The thinking level parameter is only supported on Gemini 3 Pro models
- For Gemini 2.5 models, use `thinking_budget` instead (already supported)
- Invalid thinking levels will cause a panic during configuration building

## References

- [Gemini Thinking Documentation](https://ai.google.dev/gemini-api/docs/thinking)
- [Gemini 3 Developer Guide](https://ai.google.dev/gemini-api/docs/gemini-3)

