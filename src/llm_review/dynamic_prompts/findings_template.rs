use std::collections::HashSet;

use crate::{
    config::{AuditType, JSON_REQUIREMENT_SECTION},
    llm_review::{
        agent::agent_enums::{all_enum_variants, generate_enum_list, EnumData},
        dynamic_prompts::prompt_index,
        findings::{
            finding_enums::{Severity, VulnerabilityType},
            findings::PrivilegeLevel,
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use strum::IntoEnumIterator;

pub fn get_post_json_requirement_for_multipattern<T>(
    patterns: &[T],
    pattern_type: &str,
    repo: &RepoPaths,
) -> String
where
    T: std::fmt::Display + EnumData,
{
    let json = get_json_requirement(patterns, pattern_type, repo);
    let vulnerabities: Vec<VulnerabilityType> = patterns
        .iter()
        .flat_map(|p| p.to_types())
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let issue_list = generate_enum_list(&vulnerabities);
    let section_11_header = prompt_index::generated_section_header(
        "JSON OUTPUT REQUIREMENTS",
        JSON_REQUIREMENT_SECTION,
    );

    format!(
        r#"

{section_11_header}

*Please respond with ONLY valid JSON in the following exact format:*

{json}

- *privilege* -> least privilege to trigger vulnerability
- for "exploit_type" please select from one of the listed types: {issue_list}
- If no vulnerabilities are found, return: 

{{
"findings": []
}}

**Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
**Please double-check opening and closing brackets: `}}` and `]`, make sure 
they match up correctly.
"#
    )
}

pub fn get_pre_json_requirement_for_multipattern() -> String
where
{
    let section_1_header = prompt_index::generated_section_header("OUTPUT FORMAT REQUIREMENTS", 1);
    let section_1_1_header = prompt_index::generated_sub_header("JSON OUTPUT REQUIREMENT", 1, 1);

    format!(
        r#"

{section_1_header}

Before instructions are provided on the task please note required output format:

{section_1_1_header}

- **Output must be strictly valid JSON**.
- No markdown, no code fences.
- Must validate against schema below in SECTION 11; use only allowed enums.
- If no vulnerabilities are found, return: 

{{
"findings": []
}}

       "#
    )
}

pub fn get_json_requirement<T>(patterns: &[T], pattern_type: &str, repo: &RepoPaths) -> String
where
    T: std::fmt::Display + EnumData,
{
    let vulnerabities: Vec<VulnerabilityType> = patterns
        .iter()
        .flat_map(|p| p.to_types())
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let issue_list = generate_enum_list(&vulnerabities);
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let severity_enums_standard: Vec<Severity> = Severity::iter()
        .filter(|s| *s != Severity::Critical)
        .collect();
    let severity_enums_list_standard = generate_enum_list(severity_enums_standard.as_slice());
    let severity_list = match repo.audit_type {
        AuditType::Code4rena => severity_enums_list_standard,
        AuditType::Sherlock => severity_enums_list_standard,
        AuditType::Cantina => severity_enums_list_standard,
        _ => generate_enum_list(all_enum_variants::<Severity>().as_slice()),
    };

    format!(
        r#"
        {{
        "findings": [
            {{
            "derived_from": "Insert title (or predicate) of most relevant {pattern_type} this finding derives from",
            "title": "200 chars or less audit report friendly title i.e. DOS due to unbounded loop in <contract_name>.<function_name> bricking withdrawals",
            "description": "Detailed explanation + vulnerable snippet",
            "exploit_type": "MUST be exactly one of: {issue_list}",
            "privilege": "{privilege_enum_list}",
            "contract": "{{contract_name}}", 
            "function": "{{function_name}}", 
            "impact": "monetary/functional consequences",
            "proof_of_concept": "Step-by-step exploitation scenario",
            "proof_of_code": "compilable Foundry unit test",
            "severity": "{severity_list}",
            "mitigation": "concrete code fix"
            }}
        ]
        }}
       "#
    )
}
