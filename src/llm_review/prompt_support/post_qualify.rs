pub const POST_QUALIFY: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_quality_check_passed**: true|false 
   • `true`   → if nothing has to be updated
   • `false`  → at least one field (impact, POC, proof of code, severity, mitigation) requires an update 
   *NOTE* : this is boolean value, NO "" around it
2. **where_quality_lacks**: Brief summary of issues found with vulnerability write up (omit this field if quality check passed)
3. **impact**: provide updated impact statement (ONLY IF current one is not adequately addressing impact)
4. **proof of concept**: provide an updated proof of concept ONLY IF NEEDED
5. **proof of code**: provide an updated proof of code ONLY IF NEEDED
6. **severity**: provid an updated severity (High|Medium|Low|Info), ONLY IF current severity is not accurate
7. **mitigation**: provide updated mitigation, ONLY IF current one is inadequate

*Please respond with ONLY valid JSON in the following exact format:*

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;
