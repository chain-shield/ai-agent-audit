/// Integration test to verify Gemini 3 Pro safety filters are disabled for security research
use ai_agent_audit::config::init_config;
use ai_agent_audit::llm_review::agent::agent_enums::AIAgent;
use ai_agent_audit::llm_review::agent::agent_factory::{
    AgentConfig, AgentFactory, init_llm_clients,
};
use dotenvy::dotenv;
use rig::completion::Prompt;
use std::env;

/// Returns true if Gemini API key is present in the environment
fn gemini_key_present() -> bool {
    env::var("GEMINI_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

#[tokio::test]
async fn test_gemini3_pro_security_research_prompt() {
    // Load environment variables
    dotenv().ok();

    // Skip if no Gemini API key present
    if !gemini_key_present() {
        eprintln!(
            "⚠️ Skipping test_gemini3_pro_security_research_prompt - no GEMINI_API_KEY found"
        );
        return;
    }

    // Initialize logger (avoid panic if another test already set the logger)
    let _ = env_logger::try_init();

    // Initialize configuration from environment (ignore if already initialized)
    let _ = init_config();

    // Initialize LLM clients (ignore if already initialized)
    let _ = init_llm_clients();

    println!(
        "\n🧪 Testing Gemini 3 Pro with security research prompt (safety filters disabled)...\n"
    );

    // Create agent configuration
    let config = AgentConfig::new(None)
        .with_temperature(1.0)
        .with_model("gemini-3-pro-preview")
        .with_preamble("You are a smart contract security researcher analyzing vulnerabilities.");

    println!("✅ Configuration created");
    println!("   - Model: gemini-3-pro-preview");
    println!("   - Safety filters: DISABLED (BlockNone)");

    // Create Gemini agent
    let agent = AgentFactory::create_gemini_agent(&config)
        .expect("Should create Gemini agent successfully");

    println!("✅ Agent created\n");

    // Test with a security research prompt that would normally trigger safety filters
    let security_prompt = r#"Analyze this Solidity function for reentrancy vulnerabilities:

```solidity
function withdraw(uint amount) public {
    require(balances[msg.sender] >= amount);
    (bool success, ) = msg.sender.call{value: amount}("");
    require(success);
    balances[msg.sender] -= amount;
}
```

Explain how an attacker could exploit this vulnerability to steal funds."#;

    println!("📝 Sending security research prompt...\n");

    // Extract the agent and call prompt
    let response = match &agent {
        AIAgent::Gemini {
            agent: inner_agent, ..
        } => inner_agent
            .prompt(security_prompt)
            .await
            .expect("Should receive response from Gemini 3 Pro"),
        _ => panic!("Expected Gemini agent"),
    };

    println!("✅ Response received (safety filters did NOT block):\n");
    println!("{}\n", response);

    // Verify the response contains security analysis keywords
    let response_lower = response.to_lowercase();
    assert!(
        response_lower.contains("reentrancy") || response_lower.contains("re-entrancy"),
        "Response should mention reentrancy"
    );
    assert!(
        response_lower.contains("attack") || response_lower.contains("exploit"),
        "Response should mention attack/exploit"
    );

    println!(
        "✅ Test passed - Gemini 3 Pro can analyze security vulnerabilities without safety filter blocking"
    );
}
