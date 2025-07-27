// Simple test to understand rig-core 0.15.1 API

pub async fn test_rig_api() -> anyhow::Result<()> {
    use rig::{providers::openai};
    use serde::{Deserialize, Serialize};
    use schemars::JsonSchema;
    
    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct TestSummary {
        summary: String,
    }
    println!("Testing rig-core 0.15.1 API...");
    
    let client = openai::Client::from_env();
    
    // Test 1: Simple agent with regular completion
    println!("\n=== Test 1: Simple Agent ===");
    let agent = client
        .agent("gpt-4o")
        .preamble("You are a helpful assistant.")
        .build();
    
    let response = agent.prompt("Say hello in JSON format").await?;
    println!("Agent response: {}", response);
    
    // Test 2: Extractor with structured output
    println!("\n=== Test 2: Extractor ===");
    let extractor = client
        .extractor::<TestSummary>("gpt-4o")
        .preamble("You are a JSON generator. Always respond with valid JSON.")
        .build();
    
    let result = extractor.extract("Create a summary of 'Hello World'").await?;
    println!("Extractor result: {:?}", result);
    
    // Test 3: Try with O3 model
    println!("\n=== Test 3: O3 Agent ===");
    let o3_agent = client
        .agent(openai::O3)
        .preamble("You are a helpful assistant.")
        .build();
    
    let o3_response = o3_agent.prompt("Say hello").await?;
    println!("O3 Agent response: {}", o3_response);
    
    // Test 4: Try O3 with extractor
    println!("\n=== Test 4: O3 Extractor ===");
    let o3_extractor = client
        .extractor::<TestSummary>(openai::O3)
        .preamble("You are a JSON generator. Always respond with valid JSON.")
        .build();
    
    match o3_extractor.extract("Create a summary of 'Hello World'").await {
        Ok(result) => println!("O3 Extractor result: {:?}", result),
        Err(e) => println!("O3 Extractor failed: {}", e),
    }
    
    println!("\nTest completed!");
    Ok(())
}