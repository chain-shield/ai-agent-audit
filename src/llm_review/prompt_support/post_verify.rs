pub const POST_VERIFY: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_legit_vulnerability**: true|false 
   • `true`   → issue is real
   • `false`  → issue is clearly harmless, irrelevant, or deliberate with no risk or confusion
   *NOTE* : this is boolean value, NO "" around it
2. **why_its_not_legit**: IF above is false (OMIT this field if above true), provide brief explanation why issue is NOT real 

*Please respond with ONLY valid JSON in the following exact format:*

{
    "is_legit_vulnerability": true|false, 
    "why_its_not_legit": " in 40 words less explain why NOT legit (OMIT if legit)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

pub const POST_IN_SCOPE_VERIFY: &str = r#"
### OUTPUT REQUIREMENTS 

1. **is_vulnerability_in_scope**: true|false 
   • `true`   → issue is in scope
   • `false`  → issue is out of scope 
   *NOTE* : this is boolean value, NO "" around it
2. **why_its_not_in_scope**: IF above is false (OMIT this field if above true), provide brief explanation why issue is NOT in scope.

*Please respond with ONLY valid JSON in the following exact format:*

{
    "is_vulnerability_in_scope": true|false,
    "why_its_not_in_scope": "explain why NOT in scope (OMIT if in scope)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
"#;
