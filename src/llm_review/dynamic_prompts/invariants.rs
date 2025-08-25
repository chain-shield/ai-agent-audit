use crate::llm_review::{
    enums::{all_enum_variants, generate_enum_list, EnumString},
    invariants::{
        InvariantFinding, InvariantSpec, InvariantStatus, InvariantType, INVARIANT_LIBRARY,
    },
    utils::prompt_context::generate_formatted_invariant_finding,
};

pub fn generate_invariant_prompt(inv: &[InvariantType]) -> String {
    let invariant_type_list = generate_enum_list(inv);
    let invariant_json = get_invariant_json(inv);
    let invariant_categories = generate_formated_list_from_invariant_data(inv);

    format!(
        r#"Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}

        You are a senior smart-contract security auditor. Your task is to propose AND evaluate high-value,
        machine-checkable invariants for ONE target contract.

        ## Preparation
        Carefully review the target contract plus docs/scope/summaries to understand how the protocol operates.

        ## STEPS
        1) Summarize the intended behaviour of THIS contract (bullet list; max 6 bullets).

        2) Propose at least 3, ideally 3 to 7 invariants. For each include:
        - "inv_type": "{types}"
        - "contract": exact contract name where the invariant applies
        - "function": exact function name most relevant to the invariant; use "NA" if not applicable
        - "predicate": one-line predicate over real symbols (e.g., "totalSupply() == sum(balances[*])")
        - "desc": short description; include a short code snippet (3–8 lines) if relevant
        - "checks": array of where to assert (e.g., ["after deposit","after withdraw","after claim"])

        3) Evaluate each invariant using IR/call-flow reasoning:
        - "status": "Holds" if the invariant is demonstrably enforced in all paths; otherwise "PossibleViolation"
        - For "PossibleViolation", include "pre_state" (minimal setup), "post_state" (state/value change), and "impact".

        ## INVARIANT TYPES TO FOCUS ON
        {invariants}

        STRICT JSON ONLY (no markdown, no comments):

        {json}

        - If no vulnerabilities are found, return: 

        {{
        "invariants": []
        }}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        **Please double-check opening and closing brakets: `}}` and `]`, make sure 
        they match up correctly.
    "#,
        types = invariant_type_list,
        json = invariant_json,
        invariants = invariant_categories
    )
}

pub fn generate_invariant_verify_prompt(inv: &InvariantFinding) -> String {
    let verify_json = get_invariant_verify_json();
    let inv_finding_report = generate_formatted_invariant_finding(inv);

    format!(
        r#"Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}

        ## Your task: decide if the reported Invariant is legit.
        
        You should return `"true"` if invariant is legit and description, predicate, and status all check out.
        Otherwise return `"false"`.

        ## OUTPUT REQUIREMENTS 

        1. **is_legit_invariant**: true|false 
        • `true`   → invariant is legit
        • `false`  → invariant is NOT legit
        *NOTE* : this is boolean value, NO "" around it
        2. **why_its_not_legit**: IF above is false (OMIT this field if above true), provide brief explanation why invariant is not legit

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

        ## INVARIANT TO VERIFY
        {report} 
        "#,
        json = verify_json,
        report = inv_finding_report
    )
}
pub fn get_invariant_json(inv: &[InvariantType]) -> String {
    let status_enum_list = generate_enum_list(all_enum_variants::<InvariantStatus>().as_slice());
    let invariant_type_list = generate_enum_list(inv);

    format!(
        r#"{{
        
        "invariants": [
            {{
            "inv_type": "{types}",
            "contract": "string",
            "function": "string",
            "predicate": "vault.totalAssets() == asset.balanceOf(address(vault)) + strategyDebt",
            "desc": "string (include short code snippet if relevant)",
            "checks": ["after deposit","after withdraw","after harvest"],
            "status": "{status}",
 lksdjfskljj           "pre_state": "string (omit if Holds)",
            "post_state": "string (omit if Holds)",
            "impact": "string (omit if Holds)"
            }}
        ]
        }}
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
