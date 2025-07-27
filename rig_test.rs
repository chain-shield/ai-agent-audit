#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rig-core = "0.13.0"
//! anyhow = "1.0"
//! tokio = { version = "1.0", features = ["full"] }
//! serde = { version = "1.0", features = ["derive"] }
//! schemars = "0.8"
//! ```

// Standalone test for rig-core 0.13.0 API
use anyhow::Result;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use rig::providers::openai;
use rig::client::CompletionClient;
use rig::completion::Prompt;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TestSummary {
    summary: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing rig-core 0.13.0 API behavior...");
    
    let client = openai::Client::new(
        &std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
    );
    
    // Test 1: GPT-4O Agent
    println!("\n=== Test 1: GPT-4O Agent ===");
    let gpt4_agent = client
        .agent("gpt-4o")
        .preamble("You are a test assistant.")
        .build();
    
    println!("✅ GPT-4O agent created!");
    
    match gpt4_agent.prompt("Say 'Hello test'").await {
        Ok(response) => println!("✅ GPT-4O agent response: {}", response),
        Err(e) => println!("❌ GPT-4O agent failed: {}", e),
    }
    
    // Test 2: GPT-4O Extractor
    println!("\n=== Test 2: GPT-4O Extractor ===");
    let gpt4_extractor = client
        .extractor::<TestSummary>("gpt-4o")
        .preamble("Return only JSON: {\"summary\": \"your_text\"}")
        .build();
    
    match gpt4_extractor.extract("Create a test summary").await {
        Ok(result) => println!("✅ GPT-4O extraction result: {:?}", result),
        Err(e) => println!("❌ GPT-4O extraction failed: {}", e),
    }
    
    // Test 3: O3 Agent
    println!("\n=== Test 3: O3 Agent ===");
    let o3_agent = client
        .agent(openai::O3)
        .preamble("You are a test assistant.")
        .build();
    
    match o3_agent.prompt("Say 'Hello O3'").await {
        Ok(response) => println!("✅ O3 agent response: {}", response),
        Err(e) => println!("❌ O3 agent failed: {}", e),
    }
    
    // Test 4: O3 Extractor (expected to fail)
    println!("\n=== Test 4: O3 Extractor ===");
    let o3_extractor = client
        .extractor::<TestSummary>(openai::O3)
        .preamble("Return only JSON: {\"summary\": \"your_text\"}")
        .build();
    
    match o3_extractor.extract("Create a test summary").await {
        Ok(result) => println!("✅ O3 extraction result: {:?}", result),
        Err(e) => println!("❌ O3 extraction failed: {}", e),
    }
    
    println!("\n🎯 rig-core 0.13.0 Analysis:");
    println!("- If 0.14.0 extractors work but 0.15.1 failed: downgrade is the solution!");
    println!("- If O3 works in 0.14.0: the reasoning format issue is 0.15.1 specific");
    println!("- If both work perfectly: we should downgrade rig-core in the project");
    
    Ok(())
}