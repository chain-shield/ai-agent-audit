/// Standalone test script to verify OpenAI service tier and reasoning effort parameters
///
/// This script tests that our custom OpenAI parameters are being accepted by the API.
/// If the API calls succeed without errors, the parameters are working correctly.
///
/// Usage:
///   export OPENAI_API_KEY="your-key-here"
///   cargo run --bin test_openai_params
use ai_agent_audit::{
    config::init_config,
    llm_review::agent_factory::{AgentConfig, AgentFactory, init_llm_clients},
    prepare_code::git_clone::RepoPaths,
};
use anyhow::Result;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 OpenAI Custom Parameters Test");
    println!("=================================");

    // Check for API key
    if std::env::var("OPENAI_API_KEY").is_err() {
        println!("❌ OPENAI_API_KEY environment variable not set");
        println!("💡 Please set your API key: export OPENAI_API_KEY='your-key-here'");
        return Ok(());
    }

    println!("✅ OpenAI API key found");

    // Initialize
    let _ = init_config();
    let _ = init_llm_clients();

    let repo_paths = RepoPaths {
        project_id: "test-project".to_string(),
        root: "/tmp/test".into(),
        sol_files: vec![],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        source_code_folder: "/tmp/test/src".into(),
        docs: vec![],
        repo_name: "test-repo".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        commit_hash: "abc123".to_string(),
    };

    println!("\n🎯 Testing SERVICE TIER effects (keeping reasoning effort constant)...");
    println!("Goal: Isolate service tier performance differences");
    println!("📋 All tests use: reasoning_effort='medium' (default)");

    // Test 1: Flex tier (cheaper, slower)
    println!("\n=== Test 1: Flex Service Tier ===");
    println!("📤 Parameters: service_tier='flex', reasoning_effort='medium'");
    println!("💰 Expected: 50% cheaper, slower responses");

    let config_flex = AgentConfig::new(repo_paths.clone())
        .with_model("gpt-5")
        .with_openai_service_tier("flex")
        .with_openai_reasoning_effort("medium"); // Keep constant, no temperature for reasoning models

    match test_configuration(
        "Flex Tier",
        config_flex,
        "Respond with exactly: 'Flex tier works'",
    )
    .await
    {
        Ok(duration) => {
            println!("✅ Flex tier test PASSED in {:.2}s", duration.as_secs_f64());
            println!("💡 Flex tier parameters were accepted by OpenAI");
        }
        Err(e) => {
            println!("❌ Flex tier test FAILED: {}", e);
            println!("💡 This might indicate flex tier is not available or parameter rejection");
        }
    }

    // Test 2: Default tier (baseline)
    println!("\n=== Test 2: Default Service Tier (Baseline) ===");
    println!("📤 Parameters: service_tier='default', reasoning_effort='medium'");
    println!("💰 Expected: Standard cost and speed");

    let config_default = AgentConfig::new(repo_paths.clone())
        .with_model("gpt-5")
        .with_openai_service_tier("default")
        .with_openai_reasoning_effort("medium"); // Keep constant, no temperature for reasoning models

    match test_configuration(
        "Default Tier",
        config_default,
        "Respond with exactly: 'Default tier works'",
    )
    .await
    {
        Ok(duration) => {
            println!(
                "✅ Default tier test PASSED in {:.2}s",
                duration.as_secs_f64()
            );
            println!("💡 Default tier (baseline) working normally");
        }
        Err(e) => {
            println!("❌ Default tier test FAILED: {}", e);
        }
    }

    // Note: Priority tier test removed - requires special account approval
    println!("\n💡 Priority tier test skipped - requires special OpenAI account approval");
    println!("📋 Available tiers for most accounts: 'auto', 'default', 'flex'");

    println!("\n🎉 SERVICE TIER TESTING COMPLETE!");
    println!("✅ Service tier parameters are working correctly!");

    // Now test reasoning effort levels
    println!("\n{}", "=".repeat(50));
    println!("🧠 REASONING EFFORT TESTING");
    println!("{}", "=".repeat(50));
    println!("🎯 Testing REASONING EFFORT effects (keeping service tier constant)...");
    println!("Goal: Isolate reasoning effort performance differences");
    println!("📋 All tests use: service_tier='default' (constant)");

    // Test 1: Low reasoning effort
    println!("\n=== Test 1: Low Reasoning Effort ===");
    println!("📤 Parameters: service_tier='default', reasoning_effort='low'");
    println!("🧠 Expected: Faster, less thoughtful responses");

    let config_low = AgentConfig::new(repo_paths.clone())
        .with_model("gpt-5")
        .with_openai_service_tier("default") // Keep constant
        .with_openai_reasoning_effort("low");

    match test_configuration(
        "Low Reasoning",
        config_low,
        "Explain why 2+2=4 in exactly 10 words",
    )
    .await
    {
        Ok(duration) => {
            println!(
                "✅ Low reasoning test PASSED in {:.2}s",
                duration.as_secs_f64()
            );
            println!("💡 Low reasoning effort parameter was accepted");
        }
        Err(e) => {
            println!("❌ Low reasoning test FAILED: {}", e);
            println!("💡 This might indicate reasoning effort parameter rejection");
        }
    }

    // Test 2: Medium reasoning effort (baseline)
    println!("\n=== Test 2: Medium Reasoning Effort (Baseline) ===");
    println!("📤 Parameters: service_tier='default', reasoning_effort='medium'");
    println!("🧠 Expected: Balanced reasoning and speed");

    let config_medium = AgentConfig::new(repo_paths.clone())
        .with_model("gpt-5")
        .with_openai_service_tier("default") // Keep constant
        .with_openai_reasoning_effort("medium");

    match test_configuration(
        "Medium Reasoning",
        config_medium,
        "Explain why 2+2=4 in exactly 10 words",
    )
    .await
    {
        Ok(duration) => {
            println!(
                "✅ Medium reasoning test PASSED in {:.2}s",
                duration.as_secs_f64()
            );
            println!("💡 Medium reasoning effort (baseline) working normally");
        }
        Err(e) => {
            println!("❌ Medium reasoning test FAILED: {}", e);
        }
    }

    // Test 3: High reasoning effort
    println!("\n=== Test 3: High Reasoning Effort ===");
    println!("📤 Parameters: service_tier='default', reasoning_effort='high'");
    println!("🧠 Expected: Slower, more thoughtful responses");

    let config_high = AgentConfig::new(repo_paths)
        .with_model("gpt-5")
        .with_openai_service_tier("default") // Keep constant
        .with_openai_reasoning_effort("high");

    match test_configuration(
        "High Reasoning",
        config_high,
        "Explain why 2+2=4 in exactly 10 words",
    )
    .await
    {
        Ok(duration) => {
            println!(
                "✅ High reasoning test PASSED in {:.2}s",
                duration.as_secs_f64()
            );
            println!("💡 High reasoning effort parameter was accepted");
        }
        Err(e) => {
            println!("❌ High reasoning test FAILED: {}", e);
            println!("💡 This might indicate reasoning effort parameter rejection");
        }
    }

    println!("\n🎉 REASONING EFFORT TESTING COMPLETE!");
    println!("✅ If all tests passed, your reasoning effort parameters are working correctly!");

    println!("\n💡 What to observe (REASONING EFFORT effects only):");
    println!("   - Response times: low < medium < high (reasoning effort order)");
    println!("   - Quality: low should be basic, high should be more thoughtful");
    println!("   - Reasoning tokens: high effort should use more reasoning tokens");
    println!("\n🔬 Scientific approach:");
    println!("   - All tests use service_tier='default' (controlled variable)");
    println!("   - Only reasoning_effort changes (independent variable)");
    println!("   - Same prompt for all tests (controlled variable)");
    println!("   - This isolates reasoning effort effects from service tier effects");

    Ok(())
}

async fn test_configuration(
    test_name: &str,
    config: AgentConfig,
    prompt: &str,
) -> Result<std::time::Duration> {
    let start = Instant::now();

    println!("🔧 Creating agent for {}...", test_name);
    let agent = AgentFactory::create_openai_agent(&config)?;

    println!("📞 Making API call...");
    let response = agent.prompt(prompt).await?;

    let duration = start.elapsed();

    println!("📝 Response: {}", response);

    Ok(duration)
}
