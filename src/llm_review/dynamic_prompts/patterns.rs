use crate::llm_review::{
    enums::{all_enum_variants, generate_enum_list, EnumString},
    findings::PrivilegeLevel,
    pattern_category::{get_category_spec, PatternCategory},
    patterns::{
        Pattern, VulnerabilityPattern, VulnerabilityPatternSpec, VULNERABILITY_PATTERN_LIBRARY,
    },
    utils::prompt_context::generate_formatted_pattern,
};

pub fn generate_pattern_category_prompt(category: &PatternCategory) -> String {
    let category_spec = get_category_spec(&category).expect("could not find category");
    let pattern_categories = generate_formated_list_from_pattern_data(&category_spec.issues);
    let issue_enum_list = generate_enum_list(&category_spec.issues);
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let pattern_json = get_pattern_json(&category_spec.issues);

    format!(
        r#"
        Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json} 

        You are a top C4 Security Warden specializing in finding {title} vulnerability.

        Please Analyse the *entire* Solidity source below for 
        *each* {title} security vulnerability patterns listed below:

        {title_all_caps} VULNERABILITY PATTERNS TO LOOK FOR
        {categories}

        ## 🔍 ANALYSIS REQUIREMENTS

        ### DEPTH OF ANALYSIS
        - **Read every line** of the contract code - pay extra attention to external/public functions
        - **Consider edge cases** for each vulnerability pattern
        - **Look for subtle vulnerabilities** that may not be immediately obvious
        - **Consider interactions** between different parts of the contract

        ### CLASSIFICATION CRITERIA
        For **each vulnerability pattern** decide one of:
        • VIOLATION – bug exists in this contract
        • SAFE      – relevant but properly handled
        • N/A       – pattern not applicable to this code

        ### REASONING PROCESS
        Before providing your final JSON output, you must:
        1. **Silently analyze each vulnerability pattern** in order 
        2. **Consider all relevant code sections** for each pattern
        3. **Make evidence-based classifications** 
        4. **Double-check** that no vulnerability pattern was skipped

        ## ⚠️ CRITICAL REMINDERS
        - **ONLY LOOK FOR {title_all_caps} VULNERABILITY** - disregard everything else
        - **Be thorough** - Don't rush through list of vulnerability patterns
        - **Be precise** - Use exact classification criteria
        - **Scope** - if scope is provided below, then only report vulnerability that are in scope
        - **Provide only the JSON** - No additional commentary in final output

        ### OUTPUT REQUIREMENTS 

        **For Every VIOLATION** return:
        1. **Description**: Detailed explanation including vulnerable code snippet 
        2. **Issue Type**: {enums}
        3. **Contract**: The exact contract name where vulnerability is found 
        4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
        6. **Static Signals**: evidence of vulnerability as array of strings, i.e. ["missing onlyOwner","state update after external call","amountOutMin=0",...] 
        7. **Assets at Risk**: list assets at risk as array of strings i.e. ["treasury", "rewards",...]
        8. **Privilege Level Required**: what is least privilege that can trigger vulnerability: {privileges}

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        - If no vulnerabilities are found, return: 

        {{
        "patterns": []
        }}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
    "#,
        title = category_spec.title,
        title_all_caps = category_spec.title.to_uppercase(),
        categories = pattern_categories,
        enums = issue_enum_list,
        privileges = privilege_enum_list,
        json = pattern_json
    )
}
pub fn generate_pattern_verify_prompt(pattern: &Pattern) -> String {
    let verify_json = get_pattern_verify_json();
    let pattern_finding_report = generate_formatted_pattern(pattern);

    format!(
        r#"Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}

        ## Your task: decide if the reported Security Vulnerability Pattern is legit.
        
        You should return `"true"` if security vulnerability pattern is legit and contract/fuction, description, static_signals, assets_at_risk,
        and privilege (minimum privilege required to exploit vulnerability) all check out.
        Otherwise return `"false"`.

        ## OUTPUT REQUIREMENTS 

        1. **is_pattern_legit**: true|false 
        • `true`   → pattern is legit
        • `false`  → pattern is NOT legit
        *NOTE* : this is boolean value, NO "" around it
        2. **why_its_not_legit**: IF above is false (OMIT this field if above true), provide brief explanation why pattern is not legit

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

pub fn get_pattern_json(patterns: &[VulnerabilityPattern]) -> String {
    let issue_list = generate_enum_list(patterns);
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());

    format!(
        r#"{{
        "patterns": [
            {{  
            "description": "Detailed explanation of vulnerability including vulnerable code snippet",
            "issue_type": "{issues}",
            "contract": "{{contract_name}}", 
            "function": "{{function_name}}", 
            "static_signals": ["amountOutMin=0","no onlyOwner",...],
            "assets_at_risk": ["treasury", "rewards", ...],
            "privilege": {privileges}
            }}
         ]
        }}
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
