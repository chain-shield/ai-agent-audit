pub const DEDUP_PROMPT: &str = r#"

Below is a security audit of a solidity code base, submitted in the following.
Format:

[
    {
      "title": "[Severity-1] - <Issue Type> in {contract_name}::<Function>",
      "description": "Detailed explanation including vulnerable code snippet",
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High"
    }
  ]

Please carefully review all security vulnerabilities return the **EXACT title** of 
each duplicate vulnerability found (if any), except for 1 (the one we should keep - 
should be best written one). 

If no duplicates found, then return: { titles: [] }

Note: the title, description, etc can be different but could be same vulnerability.
So need to review carefully.

EXAMPLE: 

If title:A, title:B, title:C, all refer to same vulnerabilities, and title:C is best
written, then return { titles: [A,B] }

And lets say we also have in the same set: title: D, title: E which are same (but different
vulnerability then A, B, and C), and D is best written, then your final return value will be 
{ titles: [A,B,E] } - this is comprised of list of all duplicate vulnerabilities 
that need to be remove from origin set of security vulnerabilities, leaving C, D. 
"#;
