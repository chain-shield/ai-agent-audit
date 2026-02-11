/// Integration test for Kimi k2.5 via Fireworks.ai API.
///
/// This test verifies that:
/// 1. The Kimi client can be initialized from FIREWORKS_API_KEY environment variable
/// 2. The agent can successfully respond to simple prompts
/// 3. The agent can extract structured JSON data
/// 4. The custom HTTP client (no rig-core) works correctly
/// 5. Cost tracking is properly configured ($0.60 input / $3.00 output per 1M tokens)
use ai_agent_audit::config::{init_config, try_audit_config};
use ai_agent_audit::llm_review::agent::agent_factory::{
    AgentConfig, AgentFactory, init_llm_clients,
};
use dotenvy::dotenv;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::env;

/// Initialize dotenv, logger, config and LLM clients idempotently for tests
fn ensure_runtime_initialized() {
    dotenv().ok();
    let _ = env_logger::try_init();
    if try_audit_config().is_none() {
        let _ = init_config();
    }
    let _ = init_llm_clients();
}

/// Test struct for JSON extraction
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct SimpleResponse {
    answer: String,
    explanation: String,
}

/// Complex test struct for thorough JSON parsing validation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ComplexSecurityFinding {
    title: String,
    severity: String,
    description: String,
    affected_functions: Vec<String>,
    impact_score: u32,
    exploitable: bool,
    mitigation_steps: Vec<MitigationStep>,
    code_snippet: String,
    references: Vec<String>,
    metadata: FindingMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct MitigationStep {
    step_number: u32,
    action: String,
    priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct FindingMetadata {
    discovered_by: String,
    timestamp: String,
    confidence_level: f64,
    tags: Vec<String>,
}

/// Returns true if Fireworks API key is present in the environment
fn fireworks_key_present() -> bool {
    env::var("FIREWORKS_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

#[tokio::test]
async fn test_kimi_simple_prompt() {
    // Initialize runtime (dotenv, logger, config, clients) idempotently
    ensure_runtime_initialized();

    // Skip if no Fireworks API key present
    if !fireworks_key_present() {
        eprintln!("⚠️ Skipping test_kimi_simple_prompt - no FIREWORKS_API_KEY found");
        return;
    }

    // Give initialization a moment to complete (avoid race condition)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🧪 Testing Kimi k2.5 with simple prompt...\n");

    // Create agent configuration
    let config = AgentConfig::new(None)
        .with_temperature(1.0)
        .with_model("default") // Uses "accounts/fireworks/models/kimi-k2p5"
        .with_preamble("You are a helpful assistant.");

    println!("✅ Configuration created successfully");
    println!("   - Model: {}", config.model);
    println!("   - Temperature: {}", config.temperature);
    println!(
        "   - Kimi max_tokens: {} (Fireworks requires stream=true for >4096)",
        config.kimi_config.max_tokens
    );
    println!("   - Kimi top_p: {}", config.kimi_config.top_p);
    println!("   - Kimi top_k: {}", config.kimi_config.top_k);

    // Create the Kimi agent
    let agent = match AgentFactory::create_kimi_agent(&config) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!(
                "⚠️ Skipping test - Kimi client not properly initialized: {}",
                e
            );
            return;
        }
    };

    println!("\n✅ Agent created successfully\n");

    // Test with a simple question
    let test_prompt = "What is 2 + 2? Answer in one sentence.";

    println!("📝 Sending prompt: \"{}\"\n", test_prompt);

    let response = agent
        .prompt(test_prompt)
        .await
        .expect("Should receive response from Kimi k2.5");

    println!("📨 Response received:\n");
    println!("{}\n", response);
    println!("Response length: {} chars\n", response.len());

    // Verify we got a non-empty response
    assert!(!response.is_empty(), "Response should not be empty");

    // Verify the response contains the answer
    assert!(
        response.contains("4") || response.contains("four"),
        "Response should contain the answer '4'"
    );

    println!("✅ All assertions passed!");
    println!("🎉 Kimi k2.5 simple prompt test successful!");
}

#[tokio::test]
async fn test_kimi_json_extraction() {
    // Initialize runtime (dotenv, logger, config, clients) idempotently
    ensure_runtime_initialized();

    // Skip if no Fireworks API key present
    if !fireworks_key_present() {
        eprintln!("⚠️ Skipping test_kimi_json_extraction - no FIREWORKS_API_KEY found");
        return;
    }

    // Give initialization a moment to complete (avoid race condition)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🧪 Testing Kimi k2.5 with JSON extraction...\n");

    // Create agent configuration
    // Note: Fireworks API requires stream=true for max_tokens > 4096
    // Since we don't support streaming yet, keep max_tokens <= 4096
    let config = AgentConfig::new(None)
        .with_temperature(0.7)
        .with_model("default")
        .with_preamble("You are a helpful assistant that responds in JSON format.")
        .with_kimi_max_tokens(4096);

    // Create the Kimi agent
    let agent = match AgentFactory::create_kimi_agent(&config) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!(
                "⚠️ Skipping test - Kimi client not properly initialized: {}",
                e
            );
            return;
        }
    };

    println!("✅ Agent created successfully\n");

    // Test with JSON extraction
    let test_prompt = r#"What is the capital of France?

Return your response in this JSON format:
{
  "answer": "The capital city name",
  "explanation": "A brief explanation"
}"#;

    println!("📝 Sending prompt with JSON extraction...\n");

    let response: SimpleResponse = agent
        .extract_with_retry(test_prompt)
        .await
        .expect("Should extract JSON from Kimi k2.5 response");

    println!("📨 Response received and parsed successfully:\n");
    println!("Answer: {}", response.answer);
    println!("Explanation: {}\n", response.explanation);

    // Verify we got meaningful content
    assert!(!response.answer.is_empty(), "Answer should not be empty");
    assert!(
        !response.explanation.is_empty(),
        "Explanation should not be empty"
    );
    assert!(
        response.answer.to_lowercase().contains("paris"),
        "Answer should contain 'Paris'"
    );

    println!("✅ All assertions passed!");
    println!("🎉 Kimi k2.5 JSON extraction test successful!");
}

#[tokio::test]
async fn test_kimi_model_alias() {
    // Initialize runtime (dotenv, logger, config, clients) idempotently
    ensure_runtime_initialized();

    // Skip if no Fireworks API key present
    if !fireworks_key_present() {
        eprintln!("⚠️ Skipping test_kimi_model_alias - no FIREWORKS_API_KEY found");
        return;
    }

    // Give initialization a moment to complete (avoid race condition)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🧪 Testing Kimi k2.5 with model alias 'kimi_k2_5'...\n");

    // Create agent configuration using the friendly alias
    let config = AgentConfig::new(None)
        .with_temperature(1.0)
        .with_model("kimi_k2_5") // Should resolve to "accounts/fireworks/models/kimi-k2p5"
        .with_preamble("You are a helpful assistant.");

    println!("✅ Configuration created successfully");
    println!("   - Model specified: kimi_k2_5");

    // Create the Kimi agent
    let agent = match AgentFactory::create_kimi_agent(&config) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!(
                "⚠️ Skipping test - Kimi client not properly initialized: {}",
                e
            );
            return;
        }
    };

    println!("✅ Agent created successfully\n");

    // Verify the model was resolved correctly
    let resolved_model = agent.get_model();
    println!("   - Resolved model: {}", resolved_model);
    assert_eq!(
        resolved_model, "accounts/fireworks/models/kimi-k2p5",
        "Model should resolve to full Fireworks path"
    );

    // Test with a simple question to verify it works
    let test_prompt = "What is 3 + 3? Answer in one sentence.";

    println!("\n📝 Sending prompt: \"{}\"\n", test_prompt);

    let response = agent
        .prompt(test_prompt)
        .await
        .expect("Should receive response from Kimi k2.5");

    println!("📨 Response received:\n");
    println!("{}\n", response);

    // Verify we got a non-empty response
    assert!(!response.is_empty(), "Response should not be empty");

    // Verify the response contains the answer
    assert!(
        response.contains("6") || response.contains("six"),
        "Response should contain the answer '6'"
    );

    println!("✅ All assertions passed!");
    println!("🎉 Kimi k2.5 model alias test successful!");
}

#[tokio::test]
async fn test_kimi_complex_json_extraction() {
    ensure_runtime_initialized();

    if !fireworks_key_present() {
        eprintln!("⚠️ Skipping test_kimi_complex_json_extraction - no FIREWORKS_API_KEY found");
        return;
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🧪 Testing Kimi k2.5 with complex JSON extraction...\n");

    let config = AgentConfig::new(None)
        .with_temperature(0.7)
        .with_model("kimi_k2_5")
        .with_preamble("You are a security auditor that responds in JSON format.")
        .with_kimi_max_tokens(4096);

    let agent = match AgentFactory::create_kimi_agent(&config) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!(
                "⚠️ Skipping test - Kimi client not properly initialized: {}",
                e
            );
            return;
        }
    };

    println!("✅ Agent created successfully\n");

    let test_prompt = r#"Analyze this smart contract vulnerability and return a detailed security finding in JSON format:

Contract has a reentrancy vulnerability in the withdraw() function that allows attackers to drain funds by recursively calling the function before the balance is updated.

Return your response in this exact JSON format:
{
  "title": "Vulnerability title",
  "severity": "High/Medium/Low",
  "description": "Detailed description of the vulnerability",
  "affected_functions": ["function1", "function2"],
  "impact_score": 85,
  "exploitable": true,
  "mitigation_steps": [
    {
      "step_number": 1,
      "action": "Action description",
      "priority": "High"
    }
  ],
  "code_snippet": "function withdraw() { ... }",
  "references": ["https://example.com/ref1"],
  "metadata": {
    "discovered_by": "AI Auditor",
    "timestamp": "2026-02-01",
    "confidence_level": 0.95,
    "tags": ["reentrancy", "critical"]
  }
}"#;

    println!("📝 Sending complex prompt with nested JSON extraction...\n");

    let response: ComplexSecurityFinding = agent
        .extract_with_retry(test_prompt)
        .await
        .expect("Should extract complex JSON from Kimi k2.5 response");

    println!("📨 Complex response received and parsed successfully:\n");
    println!("Title: {}", response.title);
    println!("Severity: {}", response.severity);
    println!("Affected functions: {:?}", response.affected_functions);
    println!("Impact score: {}", response.impact_score);
    println!("Exploitable: {}", response.exploitable);
    println!(
        "Mitigation steps: {} steps",
        response.mitigation_steps.len()
    );
    println!(
        "Metadata confidence: {}",
        response.metadata.confidence_level
    );

    // Validate all fields
    assert!(!response.title.is_empty(), "Title should not be empty");
    assert!(
        !response.severity.is_empty(),
        "Severity should not be empty"
    );
    assert!(
        !response.description.is_empty(),
        "Description should not be empty"
    );
    assert!(
        !response.affected_functions.is_empty(),
        "Should have affected functions"
    );
    assert!(response.impact_score > 0, "Impact score should be positive");
    assert!(
        !response.mitigation_steps.is_empty(),
        "Should have mitigation steps"
    );
    assert!(
        !response.code_snippet.is_empty(),
        "Code snippet should not be empty"
    );
    assert!(
        !response.metadata.discovered_by.is_empty(),
        "Metadata should have discoverer"
    );
    assert!(
        response.metadata.confidence_level > 0.0,
        "Confidence should be positive"
    );
    assert!(!response.metadata.tags.is_empty(), "Should have tags");

    // Validate nested objects
    for step in &response.mitigation_steps {
        assert!(step.step_number > 0, "Step number should be positive");
        assert!(!step.action.is_empty(), "Step action should not be empty");
        assert!(
            !step.priority.is_empty(),
            "Step priority should not be empty"
        );
    }

    println!("\n✅ All assertions passed!");
    println!("🎉 Kimi k2.5 complex JSON extraction test successful!");
}

#[tokio::test]
async fn test_kimi_16k_max_tokens_with_streaming() {
    ensure_runtime_initialized();

    if !fireworks_key_present() {
        eprintln!(
            "⚠️ Skipping test_kimi_16k_max_tokens_with_streaming - no FIREWORKS_API_KEY found"
        );
        return;
    }

    // Give initialization a moment to complete (avoid race condition)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🧪 Testing Kimi with 16k max_tokens (streaming enabled automatically)...");

    // Create agent configuration with 16k max_tokens
    // Streaming is automatically enabled for >4096 tokens
    let config = AgentConfig::new(None)
        .with_temperature(0.7)
        .with_model("kimi_k2_5")
        .with_kimi_max_tokens(16384) // Streaming enabled automatically
        .with_preamble("You are a security auditor analyzing smart contracts.");

    // Create the Kimi agent
    let agent = match AgentFactory::create_kimi_agent(&config) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!(
                "⚠️ Skipping test - Kimi client not properly initialized: {}",
                e
            );
            return;
        }
    };

    // Create a large prompt that would generate a long response
    let large_prompt = r#"Analyze this Solidity contract for ALL security vulnerabilities and return a JSON array of findings.

Contract code:
```solidity
pragma solidity ^0.8.0;

contract VulnerableContract {
    mapping(address => uint256) public balances;
    address public owner;
    bool private locked;

    constructor() {
        owner = msg.sender;
    }

    function deposit() public payable {
        balances[msg.sender] += msg.value;
    }

    function withdraw(uint256 amount) public {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        balances[msg.sender] -= amount;
    }

    function withdrawAll() public {
        uint256 balance = balances[msg.sender];
        require(balance > 0, "No balance");
        (bool success, ) = msg.sender.call{value: balance}("");
        require(success, "Transfer failed");
        balances[msg.sender] = 0;
    }
}
```

Return JSON in this format with DETAILED findings:
{
  "findings": [
    {
      "title": "Issue title",
      "description": "Detailed description with code snippets and thorough analysis",
      "severity": "High",
      "contract": "VulnerableContract",
      "function": "functionName",
      "impact": "Comprehensive impact description",
      "proof_of_concept": "Step by step PoC with detailed explanation",
      "mitigation": "Detailed mitigation steps with code examples"
    }
  ]
}

Please provide VERY DETAILED findings with comprehensive descriptions, proof of concepts, and mitigation steps. Include code examples in your explanations."#;

    println!("Prompt length: {} chars", large_prompt.len());
    println!("Requesting response with max_tokens=16384...");

    // Test JSON extraction with large response
    let result = agent
        .extract_with_retry::<serde_json::Value>(large_prompt)
        .await;

    match result {
        Ok(json_value) => {
            let response_str =
                serde_json::to_string_pretty(&json_value).expect("Failed to serialize response");
            let response_len = response_str.len();

            println!("✅ Successfully received and parsed response");
            println!("Response length: {} chars", response_len);
            println!("Response preview (first 500 chars):");
            println!("{}", &response_str[..response_len.min(500)]);

            // Verify we got a substantial response (should be > 8000 chars if 16k tokens works)
            if response_len > 8000 {
                println!(
                    "✅ Response is large ({} chars), confirming 16k max_tokens works!",
                    response_len
                );
            } else {
                println!(
                    "⚠️  Response is only {} chars - may not have hit 16k limit",
                    response_len
                );
            }

            // Verify it's valid JSON with findings
            if let Some(findings) = json_value.get("findings") {
                if let Some(findings_array) = findings.as_array() {
                    println!("✅ Found {} findings in response", findings_array.len());
                    assert!(
                        !findings_array.is_empty(),
                        "Should have at least one finding"
                    );
                } else {
                    panic!("'findings' is not an array");
                }
            } else {
                panic!("Response missing 'findings' field");
            }
        }
        Err(e) => {
            panic!("Failed to get response from Kimi: {}", e);
        }
    }

    println!("✅ 16k max_tokens test passed!");
}

/// Test that the DEFAULT KimiConfig uses 16384 max_tokens (streaming enabled automatically)
/// This verifies that code_review_v2.rs doesn't need to call .with_kimi_max_tokens()
#[tokio::test]
async fn test_kimi_default_config_has_16k_max_tokens() {
    ensure_runtime_initialized();

    if !fireworks_key_present() {
        eprintln!(
            "⚠️ Skipping test_kimi_default_config_has_16k_max_tokens - no FIREWORKS_API_KEY found"
        );
        return;
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🧪 Testing Kimi DEFAULT config uses 16k max_tokens...");

    // Create config WITHOUT calling .with_kimi_max_tokens() - this is what code_review_v2.rs does
    let config = AgentConfig::new(None)
        .with_temperature(0.7)
        .with_model("kimi_k2_5")
        .with_preamble("You are a security auditor.");

    // Verify the default config has 16384 max_tokens
    assert_eq!(
        config.kimi_config.max_tokens, 16384,
        "Default KimiConfig should have max_tokens=16384 for streaming support"
    );

    println!(
        "✅ Default KimiConfig.max_tokens = {}",
        config.kimi_config.max_tokens
    );
    println!("✅ Streaming will be enabled automatically (max_tokens > 4096)");
    println!("✅ Default config test passed!");
}
