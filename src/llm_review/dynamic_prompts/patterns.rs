use crate::llm_review::{
    config::PrivilegeLevel,
    enums::{all_enum_variants, generate_enum_list, EnumString},
    patterns::{VulnerabilityPattern, VulnerabilityPatternSpec, VULNERABILITY_PATTERN_LIBRARY},
};

pub fn generate_pattern_prompt(p: &[VulnerabilityPattern]) -> String {
    let pattern_categories = generate_formated_list_from_pattern_data(p);
    let issue_enum_list = generate_enum_list(p);
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let pattern_json = get_pattern_json(p);

    format!(
        r#"
        Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json} 

        Please Analyse the *entire* Solidity source below for 
        *each category* of the security vulnerability patterns listed below:

        CATEGORIES  
        {categories}

        ## 🔍 ANALYSIS REQUIREMENTS

        ### DEPTH OF ANALYSIS
        - **Read every line** of the contract code - pay extra attention to external/public functions
        - **Consider edge cases** for each category
        - **Look for subtle vulnerabilities** that may not be immediately obvious
        - **Consider interactions** between different parts of the contract

        ### CLASSIFICATION CRITERIA
        For **each category** decide one of:
        • VIOLATION – bug exists in this contract
        • SAFE      – relevant but properly handled
        • N/A       – category not applicable to this code

        ### REASONING PROCESS
        Before providing your final JSON output, you must:
        1. **Silently analyze each category** in order 
        2. **Consider all relevant code sections** for each category
        3. **Make evidence-based classifications** 
        4. **Double-check** that no category was skipped

        ## ⚠️ CRITICAL REMINDERS
        - **ANALYZE ALL CATEGORIES** - No exceptions
        - **Be thorough** - Don't rush through categories
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
        **Please double-check opening and closing brakets: `}}` and `]`, make sure 
        they match up correctly.
    "#,
        categories = pattern_categories,
        enums = issue_enum_list,
        privileges = privilege_enum_list,
        json = pattern_json
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
