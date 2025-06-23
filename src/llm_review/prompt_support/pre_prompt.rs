pub const PRE_PROMPT: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "findings": [
    {
      "description": "Detailed explanation including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontRunAttack",
      "contract": "{contract_name}", // the exact contract name where vulnerability is found
      "function": "<Function>", // exact function name where vulnerability is found, if not applicable set to "NA"
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High|Medium|Low|Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

"#;
