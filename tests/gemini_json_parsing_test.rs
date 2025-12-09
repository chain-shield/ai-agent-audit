/// Integration test for Gemini JSON parsing with Puppy Raffle assets
///
/// This test verifies:
/// 1. Gemini can generate findings from code
/// 2. JSON cleaning logic handles Gemini's output format
/// 3. Control characters and malformed JSON are properly handled
///
/// Uses real Puppy Raffle code from 4-puppy-raffle-audit/
use ai_agent_audit::{
    config::init_config,
    llm_review::{
        agent::agent_factory::{AgentConfig, AgentFactory},
        findings::findings::{Findings, FromLLMJson},
    },
};
use dotenvy::dotenv;
use std::{env, fs, path::PathBuf};

/// Returns true if Gemini API key is present in the environment
fn gemini_key_present() -> bool {
    env::var("GEMINI_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

/// Test JSON cleaning with various malformed inputs
#[test]
fn test_json_cleaning_control_characters() {
    // Test 1: JSON with unescaped newline in string
    let json_with_newline = r#"{
        "findings": [{
            "title": "Test
Finding",
            "exploit_type": "Reentrancy",
            "privilege": "Permissionless",
            "contract": "TestContract",
            "function": "testFunction",
            "severity": "High",
            "description": "Has newline"
        }]
    }"#;

    println!("\n🧪 Test 1: JSON with unescaped newline");
    match Findings::parse_from_json(json_with_newline) {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings[0].title, "Test\nFinding");
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse JSON with newline: {}", e);
        }
    }

    // Test 2: JSON with tab character
    let json_with_tab = r#"{
        "findings": [{
            "title": "Test	Finding",
            "exploit_type": "Reentrancy",
            "privilege": "Permissionless",
            "contract": "TestContract",
            "function": "testFunction",
            "severity": "High",
            "description": "Has tab"
        }]
    }"#;

    println!("\n🧪 Test 2: JSON with tab character");
    match Findings::parse_from_json(json_with_tab) {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings[0].title, "Test\tFinding");
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse JSON with tab: {}", e);
        }
    }

    // Test 3: JSON with // comment (from our template)
    let json_with_comment = r#"{
        "findings": [{
            "title": "Test Finding",
            "exploit_type": "Reentrancy",
            "privilege": "Permissionless",
            "contract": "TestContract",
            "function": "testFunction",
            "severity": "High",
            "description": "Test",
            "finding_complexity": 5  // Number 1-10
        }]
    }"#;

    println!("\n🧪 Test 3: JSON with // comment");
    match Findings::parse_from_json(json_with_comment) {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings[0].finding_complexity, Some(5));
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse JSON with comment: {}", e);
        }
    }

    println!("\n✅ All JSON cleaning tests passed!");
}

#[test]
fn test_json_with_curly_braces_in_strings() {
    use ai_agent_audit::llm_review::findings::findings::{Findings, FromLLMJson};

    // Test: JSON with curly braces in strings (like Solidity code)
    let json_with_braces = r#"{
        "findings": [
            {
                "title": "Test Finding",
                "description": "The function uses `winner.call{value: prizePool}(\"\")` to send ETH",
                "exploit_type": "Reentrancy",
                "privilege": "Permissionless",
                "contract": "TestContract",
                "function": "testFunction",
                "severity": "High"
            }
        ]
    }"#;

    println!("\n🧪 Test: JSON with curly braces in strings (Solidity code)");
    match Findings::parse_from_json(json_with_braces) {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            println!(
                "   Description: {}",
                findings.findings[0].description.as_ref().unwrap()
            );
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse JSON with curly braces in string: {}", e);
        }
    }

    // Test with parse_from_llm_response (which calls clean_json_string)
    println!("\n🧪 Test: Same JSON through parse_from_llm_response");
    match Findings::parse_from_llm_response(json_with_braces) {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            println!(
                "   Description: {}",
                findings.findings[0].description.as_ref().unwrap()
            );
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!(
                "Failed to parse JSON with curly braces after cleaning: {}",
                e
            );
        }
    }

    println!("\n✅ Curly braces test passed!");
}

#[tokio::test]
#[ignore] // Run with: cargo test --test gemini_json_parsing_test -- --ignored --nocapture
async fn test_gemini_raw_output_puppy_raffle() {
    // Load .env file
    dotenv().ok();

    // Skip if no Gemini API key
    if !gemini_key_present() {
        println!("⏭️  Skipping test: GEMINI_API_KEY not set in .env file");
        println!("   Please add GEMINI_API_KEY=your_key_here to your .env file");
        return;
    }

    println!("✅ GEMINI_API_KEY found in environment\n");

    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    println!("\n🧪 Testing Gemini raw JSON output with Puppy Raffle code...\n");

    // Initialize config
    let _ = init_config();

    // Load Puppy Raffle code
    let code_path =
        PathBuf::from("4-puppy-raffle-audit/Contract-PuppyRaffle-NFTCollection-size-3700.md");

    assert!(code_path.exists(), "Code file not found: {:?}", code_path);

    let code_content = fs::read_to_string(&code_path).expect("Failed to read code");

    println!(
        "✅ Loaded Puppy Raffle code: {} chars\n",
        code_content.len()
    );

    // Create Gemini agent
    println!("🤖 Creating Gemini 3 Pro Preview agent...");
    let agent_config = AgentConfig::new(None)
        .with_model("gemini-3-pro-preview")
        .with_temperature(0.0);

    let agent =
        AgentFactory::create_gemini_agent(&agent_config).expect("Failed to create Gemini agent");

    println!("✅ Gemini 3 Pro Preview agent created\n");

    // Create a simple prompt to extract findings
    let prompt = format!(
        r#"Analyze the following Solidity contract for security vulnerabilities.

Return your findings in this exact JSON format:
{{
    "findings": [
        {{
            "title": "Brief title of the vulnerability",
            "description": "Detailed description of the issue",
            "exploit_type": "Reentrancy",
            "privilege": "Permissionless",
            "contract": "ContractName",
            "function": "functionName",
            "severity": "High"
        }}
    ]
}}

Required fields:
- title: Brief description of the vulnerability
- description: Detailed explanation
- exploit_type: One of [Reentrancy, AccessControl, Dos, IntegerMath, Oracle, Randomness, UnexpectedEth, FrontrunMev, Custom]
- privilege: One of [Permissionless, RequiresRole, RequiresAdminRole]
- contract: Exact contract name where issue appears
- function: Exact function name where issue appears
- severity: One of [High, Medium, Low, Info]

CONTRACT CODE:
{}

Return ONLY the JSON, no other text."#,
        code_content
    );

    println!("🔍 Sending prompt to Gemini...\n");

    // Get raw response from Gemini
    let raw_response = agent
        .prompt(&prompt)
        .await
        .expect("Failed to get response from Gemini");

    println!("\n📝 RAW GEMINI RESPONSE:");
    println!("{}", "=".repeat(80));
    println!("{}", raw_response);
    println!("{}", "=".repeat(80));

    // Save raw response to file
    let raw_output_path = PathBuf::from("test-output-gemini-raw-response.txt");
    fs::write(&raw_output_path, &raw_response).expect("Failed to write raw response");
    println!("\n✅ Saved raw response to: {:?}", raw_output_path);

    // Try to parse it
    println!("\n🔍 Attempting to parse JSON...");

    // Debug: Show what parse_from_llm_response extracts and cleans
    let json_start = raw_response.find('{');
    let json_end = raw_response.rfind('}');
    if let (Some(start), Some(end)) = (json_start, json_end) {
        let extracted_json = &raw_response[start..=end];
        println!("📋 Extracted JSON (first 500 chars):");
        println!("{}", &extracted_json[..extracted_json.len().min(500)]);
        println!("...\n");

        // Now show what clean_json_string does to it
        use ai_agent_audit::llm_review::findings::findings::FromLLMJson;
        println!("🧹 Testing clean_json_string on extracted JSON...");
    }

    match Findings::parse_from_llm_response(&raw_response) {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());

            // Save parsed findings
            let output_path = PathBuf::from("test-output-gemini-parsed-findings.json");
            let findings_json =
                serde_json::to_string_pretty(&findings).expect("Failed to serialize findings");
            fs::write(&output_path, findings_json).expect("Failed to write findings");
            println!("✅ Saved parsed findings to: {:?}", output_path);
        }
        Err(e) => {
            println!("❌ PARSE FAILED: {}", e);
            println!("\n💡 This shows what Gemini is outputting that breaks JSON parsing");
            println!("   Check test-output-gemini-raw-response.txt to see the exact output");
            panic!("Failed to parse Gemini output: {}", e);
        }
    }
}
