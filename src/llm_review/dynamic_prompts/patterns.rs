use crate::llm_review::{
    enums::{all_enum_variants, generate_enum_list, EnumString},
    findings::PrivilegeLevel,
    pattern_category::{get_category_library_spec, PatternCategory},
    patterns::{
        Pattern, VulnerabilityPattern, VulnerabilityPatternSpec, VULNERABILITY_PATTERN_LIBRARY,
    },
    utils::prompt_context::generate_formatted_pattern,
};

pub fn generate_pattern_category_prompt(category: &PatternCategory) -> String {
    let category_spec = get_category_library_spec(&category).expect("could not find category");
    let pattern_categories = generate_formated_list_from_pattern_data(&category_spec.issues);

    format!(
        r#"
        You are a top C4 Security Warden specializing in finding {title} vulnerabilities.

        Please Analyse the main target contract below for 
        *each* {title} security vulnerability patterns listed below:

        ## {title_all_caps} VULNERABILITY PATTERNS TO LOOK FOR
        {categories}

        ## Rules
        - **ONLY LOOK FOR {title_all_caps} VULNERABILITY** - disregard everything else
        - **Scope** - if scope is provided below, then only report vulnerability that are in scope
    "#,
        title = category_spec.title,
        title_all_caps = category_spec.title.to_uppercase(),
        categories = pattern_categories,
    )
}
pub fn generate_pattern_verify_prompt(pattern: &Pattern) -> String {
    let verify_json = get_pattern_verify_json();
    let pattern_finding_report = generate_formatted_pattern(pattern);

    format!(
        r#"
        ## Your task: decide if the reported Security Vulnerability Pattern is legit.
        
        You should return `"true"` if security vulnerability pattern is legit and contract/fuction, description, static_signals, assets_at_risk,
        and privilege (minimum privilege required to exploit vulnerability) all check out.
        Otherwise return `"false"`.

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

        ## SECURITY VULNERABILITY PATTERN TO VERIFY
        {report} 
        "#,
        json = verify_json,
        report = pattern_finding_report
    )
}

pub fn get_pattern_json_requirement(patterns: &[VulnerabilityPattern]) -> String {
    let issue_list = generate_enum_list(patterns);
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());

    format!(
        r#"

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {{
        "patterns": [
            {{
            "title": "100 chars or less audit report friendly title",
            "description": "Detailed explanation + vulnerable code snippet",
            "issue_type": "{issues}",
            "contract": "{{contract_name}}",
            "function": "{{function_name}}",
            "static_signals": ["amountOutMin=0","no onlyOwner","..."],
            "assets_at_risk": ["treasury", "rewards", "..."],
            "privilege": "{privileges}" 
            }}
         ]
        }}

        - **privilege** -> least privilege to trigger vulnerability
        - If no vulnerabilities are found, return: 

        {{
        "patterns": []
        }}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
    "#,
        issues = issue_list,
        privileges = privilege_enum_list
    )
}

pub fn get_pattern_verify_json() -> String {
    format!(
        r#"
        {{
            "is_legit_pattern": true|false,
            "why_its_not_legit": "in 40 words less explain why NOT legit (OMIT if legit)"
        }}
        "#
    )
}

pub fn generate_formated_list_from_pattern_data(
    patterns_to_use: &[VulnerabilityPattern],
) -> String {
    let top_patterns_spec: Vec<VulnerabilityPatternSpec> = VULNERABILITY_PATTERN_LIBRARY
        .iter()
        .filter(|v| patterns_to_use.contains(&v.key))
        .map(|v| v.to_owned())
        .collect();

    let mut top_patterns_list = String::new();

    for pattern in top_patterns_spec {
        top_patterns_list.push_str("\n\n");
        top_patterns_list.push_str("### Vulnerability Pattern\n");
        top_patterns_list.push_str(&pattern.key.as_str());
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Definition\n");
        top_patterns_list.push_str(pattern.definition);
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Static Signals\n");
        top_patterns_list.push_str(&pattern.static_signals.join("\n"));
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Examples\n");
        top_patterns_list.push_str(&pattern.examples.join("\n"));
        top_patterns_list.push_str("\n\n");

        top_patterns_list.push_str("### Impact Hint\n");
        top_patterns_list.push_str(&pattern.impact_hint.as_str());
        top_patterns_list.push_str("\n\n");
    }

    top_patterns_list
}
