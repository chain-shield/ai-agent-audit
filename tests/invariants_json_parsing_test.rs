/// Test that ContractInvariants uses the FromLLMJson trait properly
/// and benefits from the improved clean_json_string() implementation
use ai_agent_audit::llm_review::{
    findings::findings::FromLLMJson,
    threat_models::invariants::{ContractInvariants, InvariantStatus, InvariantType},
};

#[test]
fn test_invariants_parse_with_unescaped_quotes() {
    // Test JSON with unescaped quotes inside string values (like Gemini outputs)
    let json_with_unescaped_quotes = r#"{
        "invariants": [
            {
                "inv_type": "Balance",
                "contract": "Vault",
                "function": "withdraw",
                "predicate": "vault.totalAssets() == asset.balanceOf(address(vault))",
                "desc": "The withdraw function uses `vault.call{value: amount}("")` which could fail",
                "checks": ["after withdraw", "after deposit"],
                "status": "PossibleViolation",
                "pre_state": "vault has 100 ETH",
                "post_state": "vault has 50 ETH",
                "impact": "Users cannot withdraw funds"
            }
        ]
    }"#;

    // This should work now with the improved clean_json_string()
    let result = ContractInvariants::parse_from_json(json_with_unescaped_quotes);

    match result {
        Ok(invariants) => {
            println!("✅ SUCCESS: Parsed {} invariants", invariants.invariants.len());
            assert_eq!(invariants.invariants.len(), 1);
            assert_eq!(invariants.invariants[0].inv_type, InvariantType::Balance);
            assert_eq!(invariants.invariants[0].contract, "Vault");
            assert_eq!(invariants.invariants[0].status, InvariantStatus::PossibleViolation);
            assert!(invariants.invariants[0].desc.contains("call{value: amount}"));
        }
        Err(e) => {
            panic!("❌ Failed to parse invariants JSON: {}", e);
        }
    }
}

#[test]
fn test_invariants_parse_from_llm_response() {
    // Test parsing from raw LLM response with extra text
    let llm_response = r#"
    Here are the invariants I found:
    
    {
        "invariants": [
            {
                "inv_type": "Arithmetic",
                "contract": "Token",
                "function": "transfer",
                "predicate": "balanceOf[sender] + balanceOf[receiver] == constant",
                "desc": "Balance conservation must hold",
                "checks": ["after transfer"],
                "status": "Holds"
            }
        ]
    }
    
    That's all I found.
    "#;

    let result = ContractInvariants::parse_from_llm_response(llm_response);

    match result {
        Ok(invariants) => {
            println!("✅ SUCCESS: Parsed {} invariants from LLM response", invariants.invariants.len());
            assert_eq!(invariants.invariants.len(), 1);
            assert_eq!(invariants.invariants[0].inv_type, InvariantType::Arithmetic);
            assert_eq!(invariants.invariants[0].status, InvariantStatus::Holds);
        }
        Err(e) => {
            panic!("❌ Failed to parse invariants from LLM response: {}", e);
        }
    }
}

#[test]
fn test_invariants_parse_with_markdown_code_blocks() {
    // Test parsing JSON wrapped in markdown code blocks
    let markdown_json = r#"```json
{
    "invariants": [
        {
            "inv_type": "Permission",
            "contract": "Ownable",
            "function": "transferOwnership",
            "predicate": "onlyOwner modifier is present",
            "desc": "Only owner can transfer ownership",
            "checks": ["before call"],
            "status": "Holds"
        }
    ]
}
```"#;

    let result = ContractInvariants::parse_from_json(markdown_json);

    match result {
        Ok(invariants) => {
            println!("✅ SUCCESS: Parsed {} invariants from markdown", invariants.invariants.len());
            assert_eq!(invariants.invariants.len(), 1);
            assert_eq!(invariants.invariants[0].inv_type, InvariantType::Permission);
        }
        Err(e) => {
            panic!("❌ Failed to parse invariants from markdown: {}", e);
        }
    }
}

#[test]
fn test_invariants_empty_list() {
    let empty_json = r#"{"invariants": []}"#;

    let result = ContractInvariants::parse_from_json(empty_json);

    match result {
        Ok(invariants) => {
            println!("✅ SUCCESS: Parsed empty invariants list");
            assert_eq!(invariants.invariants.len(), 0);
        }
        Err(e) => {
            panic!("❌ Failed to parse empty invariants: {}", e);
        }
    }
}

