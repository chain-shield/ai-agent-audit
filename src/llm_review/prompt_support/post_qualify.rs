use crate::{
    config::{AuditType, AUDIT_TYPE},
    llm_review::prompt_support::severity_rubics::{
        CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
    },
};

pub fn generate_post_qualify() -> String {
    let severity_rubic = match AUDIT_TYPE {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    };
    format!(
        r#"

## Severity Rubric 
{severity_rubic}

### OUTPUT REQUIREMENTS 
*Please respond with ONLY valid JSON in the following exact format:*

{{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "High | Medium | Low | Info (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#
    )
}
