/// Test parsing of broken JSON from Gemini where strings are split across lines
use ai_agent_audit::llm_review::findings::findings::{Findings, FromLLMJson};

#[test]
fn test_broken_json_string_continuation() {
    // This is the exact pattern from the error log where Gemini breaks a string mid-value
    let broken_json = r#"{
    "findings": [
        {
        "derived_from": "Test",
        "title": "Test Finding",
        "description": "Test description",
        "exploit_type": "Reentrancy",
        "privilege": "Permissionless",
        "contract": "TestContract",
        "function": "testFunction",
        "impact": "Test impact",
        "proof_of_concept": "Test PoC",
        "proof_of_code": "function testFeeOverflow() public { address[] memory players = new address[](4); for(uint i=0; i<4; i++) players[i] = address(i+1);
    \"severity": "High",
        "mitigation": "Test mitigation"
        }
    ]
    }"#;

    println!("Testing broken JSON with string continuation...");

    let result = Findings::parse_from_json(broken_json);

    match result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 1);
            println!("Finding title: {}", findings.findings[0].title);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse broken JSON: {}", e);
        }
    }
}

#[test]
fn test_multiple_broken_strings() {
    // Test multiple broken strings in one JSON
    let broken_json = r#"{
    "findings": [
        {
        "derived_from": "First broken
    \"title": "Finding 1",
        "description": "Description 1",
        "exploit_type": "Reentrancy",
        "privilege": "Permissionless",
        "contract": "Contract1",
        "function": "function1",
        "impact": "Impact 1",
        "proof_of_concept": "PoC 1",
        "proof_of_code": "Code 1
    \"severity": "High",
        "mitigation": "Mitigation 1"
        },
        {
        "derived_from": "Second broken
    \"title": "Finding 2",
        "description": "Description 2",
        "exploit_type": "AccessControl",
        "privilege": "RequiresRole",
        "contract": "Contract2",
        "function": "function2",
        "impact": "Impact 2",
        "proof_of_concept": "PoC 2",
        "proof_of_code": "Code 2
    \"severity": "Medium",
        "mitigation": "Mitigation 2"
        }
    ]
    }"#;

    println!("Testing multiple broken strings...");

    let result = Findings::parse_from_json(broken_json);

    match result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 2);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse multiple broken strings: {}", e);
        }
    }
}

#[test]
fn test_normal_json_still_works() {
    // Make sure we didn't break normal JSON parsing
    let normal_json = r#"{
    "findings": [
        {
        "derived_from": "Normal finding",
        "title": "Test Finding",
        "description": "Test description",
        "exploit_type": "Reentrancy",
        "privilege": "Permissionless",
        "contract": "TestContract",
        "function": "testFunction",
        "impact": "Test impact",
        "proof_of_concept": "Test PoC",
        "proof_of_code": "Normal code",
        "severity": "High",
        "mitigation": "Test mitigation"
        }
    ]
    }"#;

    let result = Findings::parse_from_json(normal_json);

    match result {
        Ok(findings) => {
            println!(
                "✅ Normal JSON still works: {} findings",
                findings.findings.len()
            );
            assert_eq!(findings.findings.len(), 1);
        }
        Err(e) => {
            panic!("Normal JSON parsing broke: {}", e);
        }
    }
}
