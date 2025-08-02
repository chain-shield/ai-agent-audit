use crate::{
    config::{AuditType, AUDIT_TYPE},
    llm_review::prompt_support::severity_rubics::{
        CODE4RENA_SEVERITY_RUBRIC, DEFAULT_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
    },
};

pub const POST_QUALIFY_STATIC: &str = r#"

### OUTPUT REQUIREMENTS 

1. **is_quality_check_passed**: true|false 
   • `true`   → if nothing has to be updated
   • `false`  → at least one field (impact, POC, proof of code, severity, mitigation) requires an update 
   *NOTE* : this is boolean value, NO "" around it
2. **where_quality_lacks**: Brief summary of issues found with vulnerability write up (omit this field if quality check passed)
3. **impact**: provide updated impact statement (ONLY IF current one is not adequately addressing impact)
4. **proof_of_concept**: provide an updated proof of concept ONLY IF NEEDED
5. **proof_of_code**: provide an updated proof of code ONLY IF NEEDED
6. **severity**: provid an updated severity (High|Medium|Low|Info), ONLY IF current severity is not accurate
    Judge severity based on below table

    | Severity     | Typical impact examples                                                                                                                                                                                                            | What it signals to the team               |
    | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
    | **Critical** | - Direct theft of any funds - Permanent, **total** loss or control of all user or protocol funds - Arbitrary code execution                                                                                                  | “Drop everything—patch immediately.”      |
    | **High**     | - **Permanent freezing** or bricking of user or protocol funds (can’t be reversed without privileged migration) - Loss of governance control - Logic that lets an attacker mint/ burn / drain but under specific constraints | “Must fix before next release / upgrade.” |
    | **Medium**   | - Temporary loss (funds stuck until admin action) - Convincing grief / DoS that makes the protocol unusable - Oracle or math bugs that skew accounting but don’t directly drain value                                        | “Important, schedule a patch.”            |
    | **Low**      | - Minor economic grief (extra gas, incorrect event data) - Edge-case DoS that requires unusual conditions - Best-practice deviations with limited real-world impact                                                          | “Fix in regular development cycle.”       |
    | **Insight**  | Code cleanliness, documentation issues, minor style or test suggestions                                                                                                                                                            | “Nice-to-have, no security impact.”       |

7. **mitigation**: provide updated mitigation, ONLY IF current one is inadequate

*Please respond with ONLY valid JSON in the following exact format:*

{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "Critical | High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#;

pub fn generate_post_qualify() -> String {
    let security_rubric = match AUDIT_TYPE {
        AuditType::Code4rena => CODE4RENA_SEVERITY_RUBRIC,
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Client => DEFAULT_SEVERITY_RUBRIC,
    };

    format!(
        r#"

### OUTPUT REQUIREMENTS 

1. **is_quality_check_passed**: true|false 
   • `true`   → if nothing has to be updated
   • `false`  → at least one field (impact, POC, proof of code, severity, mitigation) requires an update 
   *NOTE* : this is boolean value, NO "" around it
2. **where_quality_lacks**: Brief summary of issues found with vulnerability write up (omit this field if quality check passed)
3. **impact**: provide updated impact statement (ONLY IF current one is not adequately addressing impact)
4. **proof_of_concept**: provide an updated proof of concept ONLY IF NEEDED
5. **proof_of_code**: provide an updated proof of code ONLY IF NEEDED
6. **severity**: provid an updated severity (High|Medium|Low|Info), ONLY IF current severity is not accurate
    Judge severity based on below table

{security_rubric}

7. **mitigation**: provide updated mitigation, ONLY IF current one is inadequate

*Please respond with ONLY valid JSON in the following exact format:*

{{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "Critical | High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#
    )
}
