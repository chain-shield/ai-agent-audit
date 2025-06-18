pub const POST_PROMPT: &str = r#"

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
