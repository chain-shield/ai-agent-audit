pub const PRE_QUALIFY: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

"#;
