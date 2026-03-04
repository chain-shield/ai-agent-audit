use ai_agent_audit::llm_review::findings::findings::{Finding, Findings, FromLLMJson};

#[test]
fn test_kimi_response_with_leading_space() {
    // Simulate actual Kimi response with leading space
    let response = r#" {"findings":[{"derived_from":"Reentrancy","title":"Test Finding","description":"Test description","exploit_type":"Reentrancy","privilege":"Permissionless","contract":"TestContract","function":"testFunction","impact":"High impact","proof_of_concept":"PoC here","proof_of_code":"Code here","severity":"High","mitigation":"Fix it"}]}"#;

    let result = Findings::parse_from_llm_response(response);
    assert!(
        result.is_ok(),
        "Should parse Kimi response with leading space: {:?}",
        result.err()
    );

    let findings = result.unwrap();
    assert_eq!(findings.findings.len(), 1);
    assert_eq!(findings.findings[0].title, "Test Finding");
}

#[test]
fn test_kimi_response_truncated_mid_json() {
    // Simulate truncated response (incomplete JSON)
    let response = r#" {"findings":[{"derived_from":"Reentrancy","title":"Test Finding","description":"Test desc"#;

    let result = Findings::parse_from_llm_response(response);
    assert!(result.is_err(), "Should fail on truncated JSON");
}

#[test]
fn test_kimi_response_with_analysis_before_json() {
    // Simulate response with analysis text before JSON
    let response = r#"The user wants me to audit the contract. Let me analyze:

1. First issue is reentrancy
2. Second issue is access control

Here is the JSON:

{"findings":[{"derived_from":"Reentrancy","title":"Test Finding","description":"Test description","exploit_type":"Reentrancy","privilege":"Permissionless","contract":"TestContract","function":"testFunction","impact":"High impact","proof_of_concept":"PoC here","proof_of_code":"Code here","severity":"High","mitigation":"Fix it"}]}"#;

    let result = Findings::parse_from_llm_response(response);
    assert!(
        result.is_ok(),
        "Should extract JSON from response with analysis: {:?}",
        result.err()
    );

    let findings = result.unwrap();
    assert_eq!(findings.findings.len(), 1);
}

#[test]
fn test_kimi_response_with_payload_example_and_real_json() {
    // Simulate response with example payload JSON before real findings JSON
    let response = r#"Analysis text here.

payload = {"model":"accounts/fireworks/models/kimi-k2p5","temperature":0.1}

Now the real findings:
{"findings":[{"derived_from":"Reentrancy","title":"Real Finding","description":"Test description","exploit_type":"Reentrancy","privilege":"Permissionless","contract":"TestContract","function":"testFunction","impact":"High impact","proof_of_concept":"PoC here","proof_of_code":"Code here","severity":"High","mitigation":"Fix it"}]}"#;

    let result = Findings::parse_from_llm_response(response);
    assert!(
        result.is_ok(),
        "Should extract correct JSON when multiple JSON objects present: {:?}",
        result.err()
    );

    let findings = result.unwrap();
    assert_eq!(findings.findings.len(), 1);
    assert_eq!(findings.findings[0].title, "Real Finding");
}

#[test]
fn test_large_kimi_response_16k_chars() {
    // Simulate large response similar to actual Kimi output (16k+ chars)
    let mut large_findings = Vec::new();
    for i in 0..10 {
        large_findings.push(format!(
            r#"{{"derived_from":"Reentrancy","title":"Finding {}","description":"Long description that simulates real Kimi output with detailed analysis and code snippets. This is a very detailed finding that includes multiple paragraphs of explanation, code examples, and thorough analysis of the vulnerability. The description continues for several lines to simulate realistic output from the LLM.","exploit_type":"Reentrancy","privilege":"Permissionless","contract":"TestContract","function":"testFunction{}","impact":"High impact with detailed explanation of consequences","proof_of_concept":"Detailed PoC with multiple steps: 1. Step one 2. Step two 3. Step three 4. Step four 5. Step five","proof_of_code":"// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n\ncontract Test {{\n    function exploit() public {{\n        // Exploit code here\n        // More code\n        // Even more code\n    }}\n}}","severity":"High","mitigation":"Detailed mitigation steps with code examples"}}"#,
            i, i
        ));
    }

    let response = format!(" {{\"findings\":[{}]}}", large_findings.join(","));

    println!("Test response length: {} chars", response.len());

    let result = Findings::parse_from_llm_response(&response);
    assert!(
        result.is_ok(),
        "Should parse large response: {:?}",
        result.err()
    );

    let findings = result.unwrap();
    assert_eq!(findings.findings.len(), 10);
}

#[test]
fn test_debug_logging_works() {
    // Initialize logger for test
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    let response = r#" {"findings":[]}"#;

    let result = Findings::parse_from_llm_response(response);
    assert!(result.is_ok());

    // If debug logging works, we should see output in test output with --nocapture
    println!("✅ If you see debug logs above, logging is working!");
}

#[test]
fn test_actual_kimi_response_pattern() {
    // This is the EXACT pattern from your actual Kimi response
    // Response starts with space, has complete JSON with nested objects and long strings
    let response = r#" {"findings":[{"derived_from":"BlockVarsAsPrimaryRandomnessSource","title":"Miner-manipulable winner selection via block.timestamp and block.difficulty in selectWinner","description":"The `selectWinner()` function derives winnerIndex and NFT rarity using `keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))`. Both `block.timestamp` and `block.difficulty` are miner/validator-influencable within protocol-permissible ranges (timestamp can vary ~12s, post-merge difficulty is constant/beacon chain derived). This allows block producers to choose favorable outcomes or sell private-orderflow bundles that guarantee winning.","exploit_type":"Randomness","privilege":"Permissionless","contract":"PuppyRaffle","function":"selectWinner","impact":"Winner selection and NFT rarity can be biased by miners/validators or MEV searchers","proof_of_concept":"1. Miner observes mempool\n2. Validator chooses favorable timing\n3. MEV searcher uses private mempool","proof_of_code":"// SPDX-License-Identifier: MIT\npragma solidity ^0.7.6;\n\ncontract Test {}","severity":"Medium","mitigation":"Use Chainlink VRF"}]}"#;

    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Debug)
        .try_init();

    println!("\n🔍 Testing actual Kimi response pattern...");
    println!("Response length: {} chars", response.len());
    println!("First 200 chars: {}", &response[..response.len().min(200)]);
    println!(
        "Last 100 chars: {}",
        &response[response.len().saturating_sub(100)..]
    );

    let result = Findings::parse_from_llm_response(response);

    match &result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 1);
            assert_eq!(
                findings.findings[0].title,
                "Miner-manipulable winner selection via block.timestamp and block.difficulty in selectWinner"
            );
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Should parse actual Kimi response pattern: {}", e);
        }
    }
}
