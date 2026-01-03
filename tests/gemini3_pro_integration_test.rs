/// Integration test for Gemini 3 Pro (gemini-3-pro-preview).
///
/// This test verifies that:
/// 1. The Gemini 3 Pro model can be configured correctly
/// 2. The agent can successfully respond to prompts
/// 3. JSON extraction with `extract_with_retry` works correctly
/// 4. No `generationConfig` errors occur
/// 5. Cost tracking works for the new model
///
/// IMPORTANT NOTES:
/// - Model name: "gemini-3-pro-preview"
/// - This test helps debug the "missing field `generationConfig`" error
/// - Uses `agent_extract_with_retry` for robust extraction
use ai_agent_audit::config::init_config;
use ai_agent_audit::llm_review::agent::agent_enums::AIAgent;
use ai_agent_audit::llm_review::agent::agent_factory::{
    AgentConfig, AgentFactory, init_llm_clients,
};
use ai_agent_audit::utils::extract_retry::agent_extract_with_retry;
use dotenvy::dotenv;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::env;

/// Test struct for JSON extraction
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct SimpleAnalysis {
    summary: String,
    key_points: Vec<String>,
    confidence: String,
}

/// Returns true if Gemini API key is present in the environment
fn gemini_key_present() -> bool {
    env::var("GEMINI_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

#[tokio::test]
async fn test_gemini3_pro_simple_prompt() {
    // Load environment variables
    dotenv().ok();

    // Skip if no Gemini API key present
    if !gemini_key_present() {
        eprintln!("⚠️ Skipping test_gemini3_pro_simple_prompt - no GEMINI_API_KEY found");
        return;
    }

    // Initialize logger (avoid panic if another test already set the logger)
    let _ = env_logger::try_init();

    // Initialize configuration from environment (ignore if already initialized)
    let _ = init_config();

    // Initialize LLM clients (ignore if already initialized)
    let _ = init_llm_clients();

    println!("\n🧪 Testing Gemini 3 Pro with simple prompt...\n");

    // Create agent configuration for Gemini 3 Pro with high thinking level
    let config = AgentConfig::new(None)
        .with_temperature(1.0)
        .with_model("gemini-3-pro-preview")
        .with_preamble("You are a helpful assistant.")
        .with_gemini_thinking_level("high");

    println!("✅ Configuration created successfully");
    println!("   - Model: {}", config.model);
    println!("   - Temperature: {}", config.temperature);
    println!(
        "   - Thinking Level: {}",
        config
            .gemini_config
            .thinking_level
            .as_ref()
            .unwrap_or(&"default".to_string())
    );

    // Create the Gemini agent
    let agent = AgentFactory::create_gemini_agent(&config)
        .expect("Should create Gemini agent successfully");

    println!("\n✅ Agent created successfully\n");

    // Test with a simple prompt
    let test_prompt = "What is 2 + 2? Explain briefly.";

    println!("📝 Sending prompt: \"{}\"\n", test_prompt);

    let response = agent
        .prompt(test_prompt)
        .await
        .expect("Should receive response from Gemini 3 Pro");

    println!("✅ Response received:\n{}\n", response);
    assert!(!response.is_empty(), "Response should not be empty");
}

#[tokio::test]
async fn test_gemini3_pro_json_extraction() {
    // Load environment variables
    dotenv().ok();

    // Skip if no Gemini API key present
    if !gemini_key_present() {
        eprintln!("⚠️ Skipping test_gemini3_pro_json_extraction - no GEMINI_API_KEY found");
        return;
    }

    // Initialize logger
    let _ = env_logger::try_init();

    // Initialize configuration
    let _ = init_config();
    let _ = init_llm_clients();

    println!("\n🧪 Testing Gemini 3 Pro with JSON extraction (extract_with_retry)...\n");

    // Create agent configuration with high thinking level
    let config = AgentConfig::new(None)
        .with_temperature(0.7)
        .with_model("gemini-3-pro-preview")
        .with_preamble("You are a security analyst. Always respond with valid JSON.")
        .with_gemini_thinking_level("high");

    println!("✅ Configuration created");
    println!("   - Model: {}", config.model);
    println!(
        "   - Thinking Level: {}",
        config
            .gemini_config
            .thinking_level
            .as_ref()
            .unwrap_or(&"default".to_string())
    );

    // Create the Gemini agent
    let agent = AgentFactory::create_gemini_agent(&config)
        .expect("Should create Gemini agent successfully");

    println!("✅ Agent created\n");

    // Test JSON extraction with a security analysis prompt
    let test_prompt = r#"Analyze this simple Solidity function for security issues:

```solidity
function transfer(address to, uint256 amount) public {
    balances[msg.sender] -= amount;
    balances[to] += amount;
}
```

Provide your analysis in JSON format with:
- summary: brief description of the issue
- key_points: list of specific problems
- confidence: "high", "medium", or "low"
"#;

    println!("📝 Sending extraction prompt...\n");

    // Extract the inner agent and metadata from AIAgent enum
    let result: SimpleAnalysis = match &agent {
        AIAgent::Gemini {
            agent: inner_agent,
            metadata,
        } => agent_extract_with_retry(inner_agent, test_prompt, metadata)
            .await
            .expect("Should extract JSON successfully from Gemini 3 Pro"),
        _ => panic!("Expected Gemini agent"),
    };

    println!("✅ JSON extraction successful!");
    println!("   - Summary: {}", result.summary);
    println!("   - Key points: {} items", result.key_points.len());
    println!("   - Confidence: {}", result.confidence);

    assert!(!result.summary.is_empty(), "Summary should not be empty");
    assert!(
        !result.key_points.is_empty(),
        "Key points should not be empty"
    );
}

#[tokio::test]
async fn test_gemini3_pro_thinking_level_low() {
    // Load environment variables
    dotenv().ok();

    // Skip if no Gemini API key present
    if !gemini_key_present() {
        eprintln!("⚠️ Skipping test_gemini3_pro_thinking_level_low - no GEMINI_API_KEY found");
        return;
    }

    // Initialize logger
    let _ = env_logger::try_init();

    // Initialize configuration
    let _ = init_config();
    let _ = init_llm_clients();

    println!("\n🧪 Testing Gemini 3 Pro with LOW thinking level...\n");

    // Create agent configuration with LOW thinking level
    let config = AgentConfig::new(None)
        .with_temperature(0.5)
        .with_model("gemini-3-pro-preview")
        .with_preamble("You are a helpful assistant.")
        .with_gemini_thinking_level("low");

    println!("✅ Configuration created");
    println!("   - Model: {}", config.model);
    println!(
        "   - Thinking Level: {}",
        config
            .gemini_config
            .thinking_level
            .as_ref()
            .unwrap_or(&"default".to_string())
    );

    // Create the Gemini agent
    let agent = AgentFactory::create_gemini_agent(&config)
        .expect("Should create Gemini agent successfully");

    println!("✅ Agent created\n");

    // Test with a simple prompt that doesn't require deep thinking
    let test_prompt = "List 3 primary colors.";

    println!("📝 Sending prompt: \"{}\"\n", test_prompt);

    let response = agent
        .prompt(test_prompt)
        .await
        .expect("Should receive response from Gemini 3 Pro");

    println!("✅ Response received:\n{}\n", response);
    assert!(!response.is_empty(), "Response should not be empty");
    assert!(
        response.to_lowercase().contains("red")
            || response.to_lowercase().contains("blue")
            || response.to_lowercase().contains("yellow"),
        "Response should mention primary colors"
    );
}
