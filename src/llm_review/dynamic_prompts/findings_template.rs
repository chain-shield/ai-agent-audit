use crate::{
    config::{AuditType, AUDIT_TYPE},
    llm_review::{
        enums::{
            all_enum_variants, generate_enum_bulleted_list, generate_enum_list, EnumData,
            EnumString, Severity,
        },
        findings::PrivilegeLevel,
        prompt_support::severity_rubics::{
            self, CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
        },
    },
};

pub fn generate_findings_prompt<T: EnumData + EnumString>(
    issue_type: &str,
    issue_definition: &str,
    issue_full_spec: &str,
    issue_enum: &T,
) -> String {
    let exploit_enums = issue_enum.to_types();
    let exploit_bullets = generate_enum_bulleted_list(exploit_enums); // "- Oracle\n- Reentrancy\n..."
    let severity_rubic = match AUDIT_TYPE {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    };

    format!(
        r#"You are a top Code4rena security warden. Your job: analyze the main target contract **through the lens of the provided {pattern_type}** and enumerate the **top exploits/attack vectors** a hacker may deploy.

        ## Rules
        - Only report exploits tied to the below {pattern_name} {pattern_type}.
        - Prefer **unprivileged EOAs**; consider untrusted roles if in audit scope.
        - Present-state only (fixture state). No deployment/upgrade-only windows unless reopenable permissionlessly.
        - Valid exploit: High/Medium severity, reproducible Foundry test, clear profit or state break. No log-only PoCs.
        - If nothing qualifies, return: `{{"findings":[]}}`.

        ## Severity rubric
        {rubric}

        ## Exploit guidelines
        - Severity priority: Theft > DoS > accounting mismatch.
        - Bigger blast radius and simpler execution are more valuable.
        - Assert with `assertGt` / `assertEq`, not logs.
        - Proof must be a compilable Foundry test (`forge-std`, `vm.prank(attacker)`).

        ## {pattern_type} Overview
        - Type: {pattern_name}
        - Definition: {pattern_def}

        ### Common Exploits
        {exploit_bullets}

        ## {title_all_caps} TO ANALYZE
        {full_spec}
        "#,
        pattern_type = issue_type,
        rubric = severity_rubic,
        pattern_name = issue_enum.as_str(),
        pattern_def = issue_definition,
        exploit_bullets = exploit_bullets,
        full_spec = issue_full_spec,
        title_all_caps = issue_type.to_uppercase()
    )
}

pub fn get_findings_json_requirement<T>(pattern: &T, pattern_description: &str) -> String
where
    T: EnumString + EnumData,
{
    let issue_list = generate_enum_list(pattern.to_types());
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let severity_list = match AUDIT_TYPE {
        AuditType::Code4rena => "High|Medium|Low|Info".to_string(),
        _ => generate_enum_list(all_enum_variants::<Severity>().as_slice()),
    };

    format!(
        r#"

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {{
        "findings": [
            {{
            "derived_from": "{pattern_description}",
            "title": "200 chars or less audit report friendly title i.e. DOS due to unbounded loop in <contract_name>.<function_name> bricking withdrawals",
            "description": "Detailed explanation + vulnerable snippet",
            "exploit_type": "{issues}",
            "privilege": "{privileges}",
            "contract": "{{contract_name}}", 
            "function": "{{function_name}}", 
            "impact": "monetary/functional consequences",
            "proof_of_concept": "Step-by-step exploitation scenario",
            "proof_of_code": "compilable Foundry unit test",
            "severity": "{severity}",
            "mitigation": "concrete code fix"
            }}
        ]
        }}

        - Keep "derived_from" exactly as shown
        - *privilege* -> least privilege to trigger vulnerability
        - If no vulnerabilities are found, return: 

        {{
        "findings": []
        }}

        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
       "#,
        issues = issue_list,
        privileges = privilege_enum_list,
        severity = severity_list,
    )
}
