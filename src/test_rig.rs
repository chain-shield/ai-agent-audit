// Test to understand rig-core 0.15.1 API and verify what works
use anyhow::Result;
use rig::providers::openai;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TestSummary {
    summary: String,
}

pub async fn test_rig_basic() -> Result<()> {
    println!("🧪 Testing rig-core 0.15.1 API behavior...");
    
    let client = openai::Client::new(&std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));
    
    // Test 1: What happens with GPT-4O agent?
    println!("\n=== Test 1: GPT-4O Agent ===");
    let gpt4_agent = client
        .agent("gpt-4o")
        .preamble("You are a test assistant.")
        .build();
    
    println!("✅ GPT-4O agent created!");
    
    // Test 2: What happens with GPT-4O extractor?
    println!("\n=== Test 2: GPT-4O Extractor ===");
    let gpt4_extractor = client
        .extractor::<TestSummary>("gpt-4o")
        .preamble("Return only JSON: {\"summary\": \"your_text\"}")
        .build();
    
    println!("✅ GPT-4O extractor created!");
    
    // Test 3: Try actual prompt with GPT-4O agent
    println!("\n=== Test 3: GPT-4O Agent Prompt ===");
    match gpt4_agent.prompt("Say 'Hello test' in exactly 2 words").await {
        Ok(response) => println!("✅ GPT-4O agent response: {}", response),
        Err(e) => println!("❌ GPT-4O agent failed: {}", e),
    }
    
    // Test 4: Try actual extraction with GPT-4O 
    println!("\n=== Test 4: GPT-4O Extractor Test ===");
    match gpt4_extractor.extract("Create a test summary").await {
        Ok(result) => println!("✅ GPT-4O extraction result: {:?}", result),
        Err(e) => println!("❌ GPT-4O extraction failed: {}", e),
    }
    
    // Test 5: O3 Agent Creation
    println!("\n=== Test 5: O3 Agent ===");
    let o3_agent = client
        .agent(openai::O3)
        .preamble("You are a test assistant.")
        .build();
    
    println!("✅ O3 agent created!");
    
    // Test 6: O3 Agent Prompt 
    println!("\n=== Test 6: O3 Agent Prompt ===");
    match o3_agent.prompt("Say 'Hello O3' in exactly 2 words").await {
        Ok(response) => println!("✅ O3 agent response: {}", response),
        Err(e) => println!("❌ O3 agent failed: {}", e),
    }
    
    // Test 7: O3 Extractor (expected to fail)
    println!("\n=== Test 7: O3 Extractor (Expected to Fail) ===");
    let o3_extractor = client
        .extractor::<TestSummary>(openai::O3)
        .preamble("Return only JSON: {\"summary\": \"your_text\"}")
        .build();
    
    println!("✅ O3 extractor created (surprisingly!)");
    
    match o3_extractor.extract("Create a test summary").await {
        Ok(result) => println!("✅ O3 extraction result: {:?}", result),
        Err(e) => println!("❌ O3 extraction failed (expected): {}", e),
    }
    
    println!("\n🎯 Test Summary:");
    println!("- If GPT-4O works but O3 extraction fails, the issue is O3 + responses API");
    println!("- If both fail with same error, the issue is broader responses API problem");
    println!("- If all work, our enum changes fixed the issue!");
    
    Ok(())
}