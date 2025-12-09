/// Test with the ACTUAL broken JSON from the error log
use ai_agent_audit::llm_review::findings::findings::{Findings, FromLLMJson};

#[test]
fn test_actual_gemini_broken_json() {
    // This is a simplified version of the actual broken JSON from the error log
    // The key issue is: "proof_of_code": "function testFeeOverflow() ...\n    \"severity": "High"
    let broken_json = r#"{
    "findings": [
        {
        "derived_from": "Unsafe Casting to uint64 in selectWinner() causes Fee Accounting Drift",
        "title": "Unsafe downcasting and integer overflow in fee calculation",
        "description": "In selectWinner, the contract calculates the fee and casts to uint64",
        "exploit_type": "IntegerMath",
        "privilege": "Permissionless",
        "contract": "PuppyRaffle",
        "function": "selectWinner",
        "impact": "Protocol permanently loses access to fees",
        "proof_of_concept": "1. Large number of players enter. 2. Fee exceeds uint64.max",
        "proof_of_code": "function testFeeOverflow() public { address[] memory players = new address[](4); for(uint i=0; i<4; i++) players[i] = address(i+1);
    \"severity": "High",
        "mitigation": "Use uint256 for totalFees"
        }
    ]
    }"#;

    println!("Testing actual Gemini broken JSON...");
    
    let result = Findings::parse_from_json(broken_json);
    
    match result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 1);
            assert_eq!(findings.findings[0].contract, "PuppyRaffle");
            assert_eq!(findings.findings[0].function, "selectWinner");
            println!("✅ All fields parsed correctly!");
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse actual Gemini broken JSON: {}", e);
        }
    }
}

#[test]
fn test_actual_gemini_pattern_multiple_breaks() {
    // Test with multiple findings that have the broken pattern
    let broken_json = r#"{
    "findings": [
        {
        "derived_from": "Reentrancy in refund",
        "title": "Reentrancy vulnerability",
        "description": "The refund function violates CEI pattern",
        "exploit_type": "Reentrancy",
        "privilege": "Permissionless",
        "contract": "PuppyRaffle",
        "function": "refund",
        "impact": "Theft of all entrance fees",
        "proof_of_concept": "1. Attacker deploys malicious contract. 2. Calls refund",
        "proof_of_code": "contract ReentrancyAttacker { function attack() external payable { raffle.refund(myIndex); }
    \"severity": "High",
        "mitigation": "Apply CEI pattern"
        },
        {
        "derived_from": "Weak Randomness",
        "title": "Predictable randomness in selectWinner",
        "description": "Uses block.timestamp for randomness",
        "exploit_type": "Randomness",
        "privilege": "Permissionless",
        "contract": "PuppyRaffle",
        "function": "selectWinner",
        "impact": "Winner manipulation",
        "proof_of_concept": "1. Attacker monitors mempool. 2. Simulates selectWinner",
        "proof_of_code": "function testPredictableRandomness() public { vm.warp(12345); vm.roll(100);
    \"severity": "High",
        "mitigation": "Use Chainlink VRF"
        }
    ]
    }"#;

    println!("Testing multiple broken findings...");
    
    let result = Findings::parse_from_json(broken_json);
    
    match result {
        Ok(findings) => {
            println!("✅ SUCCESS: Parsed {} findings", findings.findings.len());
            assert_eq!(findings.findings.len(), 2);
            println!("Finding 1: {}", findings.findings[0].title);
            println!("Finding 2: {}", findings.findings[1].title);
        }
        Err(e) => {
            println!("❌ FAILED: {}", e);
            panic!("Failed to parse multiple broken findings: {}", e);
        }
    }
}

