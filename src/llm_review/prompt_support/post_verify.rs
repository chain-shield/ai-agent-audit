pub const POST_VERIFY: &str = r#"

### OUTPUT REQUIREMENTS 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "is_legit_vulnerability": true|false, 
    "why_its_not_legit": "in 40 words less explain why NOT legit (OMIT if legit)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

pub const POST_IN_SCOPE_VERIFY: &str = r#"
### OUTPUT REQUIREMENTS 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "is_vulnerability_in_scope": true|false,
    "why_its_not_in_scope": "explain why NOT in scope (OMIT if in scope)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
"#;
