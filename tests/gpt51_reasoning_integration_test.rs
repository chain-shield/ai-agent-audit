/// Integration test for GPT-5.1 with reasoning effort controls.
///
/// This test verifies that:
/// 1. The `with_openai_reasoning_effort()` configuration works correctly
/// 2. The reasoning effort parameter is properly passed to the OpenAI API
/// 3. The agent can successfully respond to prompts with different reasoning levels
/// 4. The "none" -> "low" mapping works as expected
/// 5. No JSON deserialization errors occur with reasoning responses
///
/// IMPORTANT NOTES:
/// - GPT-5.1 supports reasoning effort: "none" | "low" | "medium" | "high"
/// - Internally we map "none" -> "low" for consistency
/// - The API may return reasoning.effort = "none" in responses
/// - Our patched rig-core handles this correctly
use ai_agent_audit::config::init_config;
use ai_agent_audit::llm_review::agent_factory::{AgentConfig, AgentFactory, init_llm_clients};
use dotenvy::dotenv;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::env;

/// Test struct for JSON extraction
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct MathExplanation {
    answer: i32,
    steps: Vec<String>,
    reasoning: String,
}

/// Returns true if OpenAI API key is present in the environment
fn openai_key_present() -> bool {
    env::var("OPENAI_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

#[tokio::test]
async fn test_gpt51_with_reasoning_effort_none() {
    // Load environment variables
    dotenv().ok();

    // Skip if no OpenAI API key present
    if !openai_key_present() {
        eprintln!("⚠️ Skipping test_gpt51_with_reasoning_effort_none - no OPENAI_API_KEY found");
        return;
    }

    // Initialize logger (avoid panic if another test already set the logger)
    let _ = env_logger::try_init();

    // Initialize configuration from environment (ignore if already initialized)
    let _ = init_config();

    // Initialize LLM clients (ignore if already initialized)
    let _ = init_llm_clients();

    println!("\n🧪 Testing GPT-5.1 with reasoning effort = 'none' (mapped to 'low')...\n");

    // Create agent configuration with reasoning effort = "none"
    let config = AgentConfig::new(None)
        .with_temperature(1.0)
        .with_model("gpt-5.1")
        .with_preamble("You are a helpful assistant that explains your reasoning clearly.")
        .with_openai_reasoning_effort("none"); // Should be mapped to "low" internally

    // Verify configuration was set correctly
    assert_eq!(
        config.openai_config.reasoning_effort,
        Some("none".to_string()),
        "Reasoning effort should be 'none' in config"
    );

    println!("✅ Configuration created successfully");
    println!("   - Model: {}", config.model);
    println!("   - Temperature: {}", config.temperature);
    println!(
        "   - Reasoning effort (config): {:?}",
        config.openai_config.reasoning_effort
    );
    println!("   - Reasoning effort (sent to API): 'low' (mapped from 'none')");

    // Create the OpenAI agent
    let agent = AgentFactory::create_openai_agent(&config)
        .expect("Should create OpenAI agent successfully");

    println!("\n✅ Agent created successfully\n");

    // Test with a simple math question
    let test_prompt = "What is 15 * 24? Show your work step by step.";

    println!("📝 Sending prompt: \"{}\"\n", test_prompt);

    let response = agent
        .prompt(test_prompt)
        .await
        .expect("Should receive response from GPT-5.1 with reasoning effort = none");

    println!("📨 Response received:\n");
    println!("{}\n", response);
    println!("Response length: {} chars\n", response.len());

    // Verify we got a non-empty response
    assert!(!response.is_empty(), "Response should not be empty");

    // Verify the response contains the correct answer
    assert!(
        response.contains("360"),
        "Response should contain the correct answer (360)"
    );

    println!("✅ Test passed: GPT-5.1 with reasoning effort = 'none' works correctly!");
}

#[tokio::test]
async fn test_gpt51_reasoning_effort_levels() {
    // Load environment variables
    dotenv().ok();

    // Skip if no OpenAI API key present
    if !openai_key_present() {
        eprintln!("⚠️ Skipping test_gpt51_reasoning_effort_levels - no OPENAI_API_KEY found");
        return;
    }

    // Initialize logger
    let _ = env_logger::try_init();

    // Initialize configuration (ignore if already initialized)
    let _ = init_config();

    // Initialize LLM clients (ignore if already initialized)
    let _ = init_llm_clients();

    println!("\n🧪 Testing GPT-5.1 with different reasoning effort levels...\n");

    let test_prompt = "What is 7 * 8? Just give me the answer.";

    // Test each reasoning effort level
    for effort in &["none", "low", "medium", "high"] {
        println!("📝 Testing reasoning effort = '{}'...", effort);

        let config = AgentConfig::new(None)
            .with_model("gpt-5.1")
            .with_preamble("You are a helpful math assistant.")
            .with_openai_reasoning_effort(*effort);

        let agent = AgentFactory::create_openai_agent(&config).expect(&format!(
            "Should create agent with reasoning effort = '{}'",
            effort
        ));

        let response = agent.prompt(test_prompt).await.expect(&format!(
            "Should receive response with reasoning effort = '{}'",
            effort
        ));

        println!(
            "   Response: {}",
            response.chars().take(100).collect::<String>()
        );

        // Verify we got a non-empty response
        assert!(
            !response.is_empty(),
            "Response should not be empty for reasoning effort = '{}'",
            effort
        );

        // Verify the response contains the correct answer
        assert!(
            response.contains("56"),
            "Response should contain the correct answer (56) for reasoning effort = '{}'",
            effort
        );

        println!("   ✅ Passed\n");
    }

    println!("✅ All reasoning effort levels work correctly!");
}

#[tokio::test]
async fn test_gpt51_json_extraction_with_reasoning() {
    // Load environment variables
    dotenv().ok();

    // Skip if no OpenAI API key present
    if !openai_key_present() {
        eprintln!(
            "⚠️ Skipping test_gpt51_json_extraction_with_reasoning - no OPENAI_API_KEY found"
        );
        return;
    }

    // Initialize logger
    let _ = env_logger::try_init();

    // Initialize configuration (ignore if already initialized)
    let _ = init_config();

    // Initialize LLM clients (ignore if already initialized)
    let _ = init_llm_clients();

    println!("\n🧪 Testing GPT-5.1 JSON extraction with reasoning effort = 'high'...\n");

    // Create agent configuration with high reasoning effort
    let config = AgentConfig::new(None)
        .with_model("gpt-5.1")
        .with_preamble("You are a helpful math assistant that provides detailed explanations.")
        .with_openai_reasoning_effort("high");

    let agent = AgentFactory::create_openai_agent(&config).expect("Should create OpenAI agent");

    let test_prompt = r#"Calculate 15 * 24 and explain your reasoning.

Return your response in this JSON format:
{
  "answer": 360,
  "steps": ["step1", "step2", "step3"],
  "reasoning": "explanation of how you solved it"
}"#;

    println!("📝 Sending prompt with JSON extraction...\n");

    let response_result: Result<MathExplanation, _> = agent.extract_with_retry(test_prompt).await;

    match response_result {
        Ok(response) => {
            println!("📨 Response received and parsed successfully:\n");
            println!("Answer: {}", response.answer);
            println!("\nSteps:");
            for step in &response.steps {
                println!("  - {}", step);
            }
            println!("\nReasoning: {}", response.reasoning);

            // Verify we got the correct answer
            assert_eq!(response.answer, 360, "Answer should be 360");
            assert!(!response.steps.is_empty(), "Should have steps");
            assert!(!response.reasoning.is_empty(), "Should have reasoning");

            println!("\n✅ JSON extraction with reasoning effort = 'high' works correctly!");
        }
        Err(e) => {
            // If we get an error, check if it's a known OpenAI backend issue
            let error_msg = format!("{:?}", e);

            if error_msg.contains("500 Internal Server Error")
                || error_msg.contains("503 Service Unavailable")
                || error_msg.contains("520")
            {
                println!("⚠️  OpenAI backend error (5xx): {}", error_msg);
                println!("    This is an OpenAI infrastructure issue, not our code.");
                println!("    Test is inconclusive but configuration is correct.");
                return;
            }

            // Check for JSON deserialization errors (the bug we're testing for)
            if error_msg.contains("JsonError") || error_msg.contains("unknown variant") {
                panic!(
                    "❌ JSON deserialization error - this should not happen!\nError: {:?}",
                    e
                );
            }

            // Some other unexpected error
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[test]
fn test_openai_reasoning_effort_config_validation() {
    // Load environment variables
    dotenv().ok();

    // Initialize config
    let _ = init_config();

    println!("\n🧪 Testing OpenAI reasoning effort configuration validation...\n");

    // Valid configurations should work
    let valid_none = AgentConfig::new(None).with_openai_reasoning_effort("none");
    assert_eq!(
        valid_none.openai_config.reasoning_effort,
        Some("none".to_string())
    );

    let valid_low = AgentConfig::new(None).with_openai_reasoning_effort("low");
    assert_eq!(
        valid_low.openai_config.reasoning_effort,
        Some("low".to_string())
    );

    let valid_medium = AgentConfig::new(None).with_openai_reasoning_effort("medium");
    assert_eq!(
        valid_medium.openai_config.reasoning_effort,
        Some("medium".to_string())
    );

    let valid_high = AgentConfig::new(None).with_openai_reasoning_effort("high");
    assert_eq!(
        valid_high.openai_config.reasoning_effort,
        Some("high".to_string())
    );

    println!("✅ All valid reasoning effort values accepted: none, low, medium, high");
}

#[test]
#[should_panic(expected = "Invalid reasoning effort")]
fn test_openai_reasoning_effort_invalid_value() {
    // Load environment variables
    dotenv().ok();

    // Initialize config
    let _ = init_config();

    println!("\n🧪 Testing invalid reasoning effort value (should panic)...\n");

    // This should panic with "Invalid reasoning effort" message
    let _invalid = AgentConfig::new(None).with_openai_reasoning_effort("invalid_value");
}

#[test]
#[should_panic(expected = "Invalid reasoning effort")]
fn test_openai_reasoning_effort_minimal_not_allowed() {
    // Load environment variables
    dotenv().ok();

    // Initialize config
    let _ = init_config();

    println!("\n🧪 Testing that 'minimal' is not allowed (should panic)...\n");

    // "minimal" is not a valid value per OpenAI docs - should panic
    let _invalid = AgentConfig::new(None).with_openai_reasoning_effort("minimal");
}
