/// Test parsing JSON with escaped newlines (\\n) as they appear in raw Gemini responses
use ai_agent_audit::llm_review::findings::findings::{Findings, FromLLMJson};

#[test]
fn test_escaped_newline_broken_string() {
    // This simulates what Gemini actually sends - escaped newlines with escaped quotes
    // The raw string from Gemini looks like: "value\\n    \\\"nextfield\\\": \\\"value\\\""
    let json_with_escaped_newlines = r#"{
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
    "proof_of_code": "function test() { code here }\n    \"severity": "High",
    "mitigation": "Test mitigation"
    }
]
}"#;

    println!("Testing JSON with escaped newline pattern...");

    let result = Findings::parse_from_json(json_with_escaped_newlines);

    match result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 1);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);

            // Debug: Show what the cleaning function produces
            let cleaned = Findings::clean_json_string(json_with_escaped_newlines);
            println!("\n=== CLEANED JSON ===");
            println!("{}", cleaned);
            println!("=== END ===\n");

            panic!("Failed to parse escaped newline pattern: {}", e);
        }
    }
}

#[test]
fn test_valid_json_not_broken_by_cleaning() {
    // Make sure valid JSON with curly braces and quotes doesn't get broken by our cleaning
    let valid_json = r#"{
"findings": [
    {
    "derived_from": "Reentrancy",
    "title": "Reentrancy vulnerability",
    "description": "The refund function has issues",
    "exploit_type": "Reentrancy",
    "privilege": "Permissionless",
    "contract": "PuppyRaffle",
    "function": "refund",
    "impact": "Theft of funds",
    "proof_of_concept": "Attack steps",
    "proof_of_code": "contract Attack { function attack() external payable { raffle.enterRaffle{value: 1 ether}(p); raffle.refund(idx); } receive() external payable { if (address(raffle).balance >= 1 ether) { raffle.refund(idx); } } }",
    "severity": "High",
    "mitigation": "Use CEI pattern"
    }
]
}"#;

    println!("Testing that valid JSON is not broken by cleaning...");

    let result = Findings::parse_from_json(valid_json);

    match result {
        Ok(findings) => {
            println!(
                "✅ SUCCESS: Valid JSON still works - {} findings",
                findings.findings.len()
            );
            assert_eq!(findings.findings.len(), 1);
            assert_eq!(findings.findings[0].contract, "PuppyRaffle");
        }
        Err(e) => {
            println!("❌ FAILED: Valid JSON was broken by cleaning: {}", e);
            panic!("Valid JSON should not be broken: {}", e);
        }
    }
}

#[test]
fn test_raw_json_fast_path() {
    // Test that valid JSON takes the fast path (no cleaning)
    let valid_json = r#"{"findings": [{"derived_from": "Test","title": "Test","description": "Test","exploit_type": "Reentrancy","privilege": "Permissionless","contract": "Test","function": "test","impact": "Test","proof_of_concept": "Test","proof_of_code": "Test","severity": "High","mitigation": "Test"}]}"#;

    println!("Testing fast path for valid JSON...");

    let result = Findings::parse_from_json(valid_json);

    match result {
        Ok(findings) => {
            println!(
                "✅ SUCCESS: Fast path works - {} findings",
                findings.findings.len()
            );
            assert_eq!(findings.findings.len(), 1);
        }
        Err(e) => {
            panic!("Fast path failed: {}", e);
        }
    }
}
