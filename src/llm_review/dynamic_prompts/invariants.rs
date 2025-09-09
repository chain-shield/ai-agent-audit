use crate::llm_review::{
    enums::{all_enum_variants, generate_enum_list, EnumString},
    invariants::{
        InvariantFinding, InvariantSpec, InvariantStatus, InvariantType, INVARIANT_LIBRARY,
    },
    utils::prompt_context::generate_formatted_invariant_finding,
};

pub fn generate_invariant_prompt(inv: &[InvariantType]) -> String {
    let invariant_categories = generate_formated_list_from_invariant_data(inv);

    format!(
        r#"
        You are a senior smart-contract security auditor. Your task is to propose AND evaluate high-value,
        machine-checkable invariants for ONE target contract.

        ## Task
        - Propose 3–7 invariants

        ## Invariant Types to Focus On
        {invariants}

    "#,
        invariants = invariant_categories
    )
}

pub fn generate_invariant_verify_prompt(inv: &InvariantFinding) -> String {
    let verify_json = get_invariant_verify_json();
    let inv_finding_report = generate_formatted_invariant_finding(inv);

    format!(
        r#"
        ## Your task: decide if the reported Invariant is legit or not.
        
        You should return `"true"` if invariant is legit and description, predicate, and status all check out.
        Otherwise return `"false"`.

        ## INVARIANT TO VERIFY
        {report} 

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        "#,
        json = verify_json,
        report = inv_finding_report
    )
}

pub fn get_invariant_json(inv: &[InvariantType]) -> String {
    let status_enum_list = generate_enum_list(all_enum_variants::<InvariantStatus>().as_slice());
    let invariant_type_list = generate_enum_list(inv);

    format!(
        r#"

        ## OUTPUT REQUIREMENTS
        
        - STRICT JSON ONLY (no markdown, no comments):

        {{
        "invariants": [
            {{
            "inv_type": "{types}",
            "contract": "{{contract_name}}",
            "function": "{{function_name}}",
            "predicate": "vault.totalAssets() == asset.balanceOf(address(vault)) + strategyDebt",
            "desc": "description + code snippet if relevant",
            "checks": ["after deposit","after withdraw","after harvest","..."],
            "status": "{status}",
            "pre_state": "string (omit if Holds)",
            "post_state": "string (omit if Holds)",
            "impact": "string (omit if Holds)"
            }}
        ]
        }}

        - If no vulnerabilities are found, return: 

        {{
        "invariants": []
        }}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        **Please double-check opening and closing brakets: `}}` and `]`, make sure 
        they match up correctly.
    "#,
        types = invariant_type_list,
        status = status_enum_list
    )
}

pub fn get_invariant_verify_json() -> String {
    format!(
        r#"
        {{
            "is_legit_invariant": true|false,
            "why_its_not_legit": "in 40 words less explain why NOT legit (OMIT if legit)"
        }}
        "#
    )
}

pub fn generate_formated_list_from_invariant_data(patterns_to_use: &[InvariantType]) -> String {
    let top_invariant_spec: Vec<InvariantSpec> = INVARIANT_LIBRARY
        .iter()
        .filter(|inv| patterns_to_use.contains(&inv.key))
        .map(|inv| inv.to_owned())
        .collect();

    let mut top_invariant_list = String::new();

    for pattern in top_invariant_spec {
        top_invariant_list.push_str("\n\n");
        top_invariant_list.push_str("### Invariant Type\n");
        top_invariant_list.push_str(&pattern.key.as_str());
        top_invariant_list.push_str("\n\n");

        top_invariant_list.push_str("### Definition\n");
        top_invariant_list.push_str(pattern.definition);
        top_invariant_list.push_str("\n\n");

        top_invariant_list.push_str("### Static Signals\n");
        top_invariant_list.push_str(&pattern.static_signals.join("\n"));
        top_invariant_list.push_str("\n\n");

        top_invariant_list.push_str("### Examples\n");
        top_invariant_list.push_str(&pattern.examples.join("\n"));
        top_invariant_list.push_str("\n\n");

        top_invariant_list.push_str("### Impact Hint\n");
        top_invariant_list.push_str(&pattern.impact_hint.as_str());
        top_invariant_list.push_str("\n\n");
    }

    top_invariant_list
}
