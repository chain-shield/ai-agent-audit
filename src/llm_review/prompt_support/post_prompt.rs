pub const POST_PROMPT: &str = r#"

 **For Every VIOLATION** please return:
1. **Title**: Format as "<Name of Security vulnerability> in <Contract>::<Function>"
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontRunAttack
3. **Description**: Detailed explanation including vulnerable code snippet 
4. **Impact**: Financial and security consequences 
5. **Proof of Concept**: Step-by-step exploitation scenario 
6. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
7. **Severity**: High/Medium/Low/Info based on Impact on Protocol AND Likelihood of Exploitation 
8. **Mitigation**: Suggested Mitigation with code example of fix

"#;

pub const POST_PROMPT_V2: &str = r#"

 **For Every VIOLATION** return:
1. **Title**: Format as "<Name of Security vulnerability> in <Contract>::<Function>"
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontRunAttack
3. **Description**: Detailed explanation including vulnerable code snippet 
4. **Impact**: Financial and security consequences 
5. **Proof of Concept**: Step-by-step exploitation scenario 
6. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
7. **Severity**: High/Medium/Low/Info based on Impact on Protocol AND Likelihood of Exploitation 
8. **Mitigation**: Suggested Mitigation with code example of fix

*Please respond with ONLY valid JSON in the following exact format:*

{
  "findings": [
    {
      "title": "<Issue Type> in {contract_name}::<Function>",
      "description": "Detailed explanation including vulnerable code snippet",
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

- If no vulnerabilities are found, return: 

{
  "findings": []
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON

Now analyze the provided smart contract code below systematically:


"#;
