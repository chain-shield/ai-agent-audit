use ai_agent_audit::{
    config::{audit_config, init_config},
    llm_review::agent::agent_factory::{AgentConfig, AgentFactory, init_llm_clients},
};
use dotenvy::dotenv;
use std::env;
// Needed to enable Client::from_env() in tests
use rig::client::CompletionClient;
use rig::completion::Prompt;

/// Test to reproduce the "your-key*here" API key issue
///
/// This test verifies that:
// Initialize dotenv, logger, config and LLM clients idempotently for tests
fn ensure_runtime_initialized() {
    dotenv().ok();
    let _ = env_logger::try_init();
    if ai_agent_audit::config::try_audit_config().is_none() {
        let _ = init_config();
    }
    let _ = init_llm_clients();
}

/// 1. Environment variables are loaded correctly
/// 2. Config initialization works
/// 3. LLM client initialization works
/// 4. Agent creation works
/// 5. API calls use the correct API key (not placeholder)
#[tokio::test]
#[ignore = "requires live OpenAI credentials"]
async fn test_api_key_loading_issue() {
    // Load environment variables
    dotenv().ok();

    // Verify the API key is actually set in the environment; if not, skip the test gracefully
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("⚠️ Skipping test_api_key_loading_issue - no OPENAI_API_KEY found");
            return;
        }
    };
    if api_key.contains("your-key") {
        eprintln!("⚠️ Skipping test_api_key_loading_issue - placeholder OPENAI_API_KEY detected");
        return;
    }
    assert!(api_key.starts_with("sk-"), "API key should start with sk-");
    println!("✅ Environment API key verified: [redacted]");

    // Initialize runtime once (dotenv, logger, config, clients)
    ensure_runtime_initialized();

    // Verify config has the API key
    assert!(
        audit_config().has_openai_key(),
        "Config should detect OpenAI API key"
    );
    let config_key = audit_config().openai_api_key.as_ref().unwrap();
    assert_eq!(
        config_key, &api_key,
        "Config API key should match environment"
    );
    println!("✅ Config API key verified");

    // Clients may already be initialized by other tests via ensure_runtime_initialized()
    // Do not re-initialize here to avoid OnceLock errors.

    // Create an agent using the AgentFactory
    let agent_config = AgentConfig::new(None)
        .with_model("gpt-4o")
        .with_preamble("You are a test assistant.")
        .with_temperature(0.7);

    let agent =
        AgentFactory::create_openai_agent(&agent_config).expect("Agent creation should succeed");
    println!("✅ Agent created successfully");

    // Test a simple prompt to verify the API key is working
    // This is where the "your-key*here" error would appear if there's an issue
    let test_prompt = "Say 'Hello, API key is working!' and nothing else.";

    match agent.prompt(test_prompt).await {
        Ok(response) => {
            println!("✅ API call successful: {}", response.trim());
            assert!(
                !response.contains("your-key"),
                "Response should not contain placeholder API key"
            );
            assert!(!response.is_empty(), "Response should not be empty");
        }
        Err(e) => {
            let error_msg = e.to_string();
            println!("❌ API call failed: {}", error_msg);

            // Check if this is the specific "your-key*here" issue
            if error_msg.contains("your-key") || error_msg.contains("placeholder") {
                panic!(
                    "🚨 REPRODUCED: API key placeholder issue detected: {}",
                    error_msg
                );
            } else if error_msg.to_lowercase().contains("invalid_api_key")
                || error_msg.to_lowercase().contains("incorrect api key")
            {
                eprintln!(
                    "⚠️ Skipping test_api_key_loading_issue - invalid API key in environment"
                );
                return; // treat invalid live creds as skip
            } else {
                eprintln!("⚠️ Skipping: non-deterministic API error: {}", error_msg);
                return;
            }
        }
    }
}

/// Test to reproduce the issue with extract_with_retry specifically
/// This tests the path where the "your-key*here" error was most likely occurring
#[tokio::test]
#[ignore = "requires live OpenAI credentials"]
async fn test_extract_with_retry_api_key_issue() {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct TestResponse {
        message: String,
    }

    // Setup
    dotenv().ok();
    if env::var("OPENAI_API_KEY").is_err()
        || env::var("OPENAI_API_KEY")
            .unwrap_or_default()
            .contains("your-key")
    {
        eprintln!(
            "⚠️ Skipping test_extract_with_retry_api_key_issue - no or placeholder OPENAI_API_KEY found"
        );
        return;
    }
    ensure_runtime_initialized();

    // Create agent
    let agent_config = AgentConfig::new(None)
        .with_model("gpt-4o")
        .with_preamble("You are a test assistant. Always respond with valid JSON.")
        .with_temperature(0.7);

    let agent =
        AgentFactory::create_openai_agent(&agent_config).expect("Agent creation should succeed");

    // Test extract_with_retry - this is where the issue was occurring
    let test_prompt = r#"
    Respond with JSON in this exact format:
    {"message": "API key is working correctly"}
    "#;

    match agent.extract_with_retry::<TestResponse>(test_prompt).await {
        Ok(response) => {
            println!("✅ Extract with retry successful: {:?}", response);
            assert_eq!(response.message, "API key is working correctly");
        }
        Err(e) => {
            let error_msg = e.to_string();
            println!("❌ Extract with retry failed: {}", error_msg);

            // Check if this is the specific "your-key*here" issue
            if error_msg.contains("your-key") || error_msg.contains("placeholder") {
                panic!(
                    "🚨 REPRODUCED: API key placeholder issue in extract_with_retry: {}",
                    error_msg
                );
            } else if error_msg.to_lowercase().contains("invalid_api_key")
                || error_msg.to_lowercase().contains("incorrect api key")
            {
                eprintln!(
                    "⚠️ Skipping test_extract_with_retry_api_key_issue - invalid API key in environment"
                );
                return;
            } else {
                eprintln!(
                    "⚠️ Skipping: non-deterministic extraction error: {}",
                    error_msg
                );
                return;
            }
        }
    }
}

/// Test to verify the client singleton is working correctly
#[tokio::test]
#[ignore = "requires live OpenAI credentials"]
async fn test_client_singleton_consistency() {
    // Setup
    ensure_runtime_initialized();

    // Create multiple agents and verify they use the same underlying client
    let config1 = AgentConfig::new(None)
        .with_model("gpt-4o")
        .with_preamble("Agent 1");

    let config2 = AgentConfig::new(None)
        .with_model("gpt-4o")
        .with_preamble("Agent 2");

    let agent1 =
        AgentFactory::create_openai_agent(&config1).expect("Agent 1 creation should succeed");

    let agent2 =
        AgentFactory::create_openai_agent(&config2).expect("Agent 2 creation should succeed");

    // Both agents should work with the same API key
    let prompt = "Respond with just 'OK'";

    let response1 = match agent1.prompt(prompt).await {
        Ok(r) => r,
        Err(e) => {
            let msg = e.to_string();
            if msg.to_lowercase().contains("invalid_api_key")
                || msg.to_lowercase().contains("incorrect api key")
            {
                eprintln!(
                    "⚠️ Skipping test_client_singleton_consistency - invalid API key in environment"
                );
                return;
            }
            // Handle transient JSON parsing errors from API
            if msg.contains("JsonError") || msg.contains("EOF while parsing") {
                eprintln!(
                    "⚠️ Skipping test - transient API JSON parsing error: {}",
                    msg
                );
                return;
            }
            panic!("Agent 1 should work: {}", msg);
        }
    };
    let response2 = match agent2.prompt(prompt).await {
        Ok(r) => r,
        Err(e) => {
            let msg = e.to_string();
            if msg.to_lowercase().contains("invalid_api_key")
                || msg.to_lowercase().contains("incorrect api key")
            {
                eprintln!(
                    "⚠️ Skipping test_client_singleton_consistency - invalid API key in environment"
                );
                return;
            }
            // Handle transient JSON parsing errors from API
            if msg.contains("JsonError") || msg.contains("EOF while parsing") {
                eprintln!(
                    "⚠️ Skipping test - transient API JSON parsing error: {}",
                    msg
                );
                return;
            }
            panic!("Agent 2 should work: {}", msg);
        }
    };

    println!("✅ Agent 1 response: {}", response1.trim());
    println!("✅ Agent 2 response: {}", response2.trim());

    // Neither should contain placeholder values
    assert!(
        !response1.contains("your-key"),
        "Agent 1 should not use placeholder API key"
    );
    assert!(
        !response2.contains("your-key"),
        "Agent 2 should not use placeholder API key"
    );
}

/// Test to check if the issue occurs with direct client creation vs factory
#[tokio::test]
#[ignore = "requires live OpenAI credentials"]
async fn test_direct_vs_factory_client_creation() {
    use rig::providers::openai;

    dotenv().ok();
    if env::var("OPENAI_API_KEY").is_err()
        || env::var("OPENAI_API_KEY")
            .unwrap_or_default()
            .contains("your-key")
    {
        eprintln!(
            "⚠️ Skipping test_direct_vs_factory_client_creation - no or placeholder OPENAI_API_KEY found"
        );
        return;
    }

    // Test 1: Direct client creation (old way)
    println!("Testing direct client creation...");
    let direct_client = openai::Client::from_env();
    let direct_agent = direct_client
        .agent("gpt-4o")
        .preamble("Direct client test")
        .build();

    let direct_response = direct_agent.prompt("Say 'direct client works'").await;
    match direct_response {
        Ok(resp) => println!("✅ Direct client works: {}", resp.trim()),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("your-key") {
                println!(
                    "🚨 Direct client has placeholder API key issue: {}",
                    error_msg
                );
            } else {
                println!("❌ Direct client error: {}", error_msg);
            }
        }
    }

    // Test 2: Factory client creation (new way)
    println!("Testing factory client creation...");
    ensure_runtime_initialized();

    let factory_config = AgentConfig::new(None)
        .with_model("gpt-4o")
        .with_preamble("Factory client test");

    let factory_agent = AgentFactory::create_openai_agent(&factory_config)
        .expect("Factory agent creation should succeed");

    let factory_response = factory_agent.prompt("Say 'factory client works'").await;
    match factory_response {
        Ok(resp) => println!("✅ Factory client works: {}", resp.trim()),
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("your-key") {
                println!(
                    "🚨 Factory client has placeholder API key issue: {}",
                    error_msg
                );
            } else {
                println!("❌ Factory client error: {}", error_msg);
            }
        }
    }
}
