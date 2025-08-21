use crate::llm_review::enums::{
    all_enum_variants, generate_enum_list, InvariantStatus, InvariantType,
};

pub fn generate_invariant_prompt() -> String {
    let invariant_type_list = generate_enum_list(all_enum_variants::<InvariantType>().as_slice());
    let invariant_json = get_invariant_json();

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

        ## INVARIANT TYPES
        1. Arithmetic   values, sums, ratios must match expectations  
        2. Balance      token/ETH balances and supply monotonicity  
        3. Permission    only-owner / only-role / re-entrancy locks  
        4. Temporal      timeouts, epochs, can’t rewind clock  
        5. Referential   mappings/arrays stay in sync (index→value)  
        6. StateMachine only allowed state transitions

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
        json = invariant_json
    )
}

pub fn get_invariant_json() -> String {
    let status_enum_list = generate_enum_list(all_enum_variants::<InvariantStatus>().as_slice());
    let invariant_type_list = generate_enum_list(all_enum_variants::<InvariantType>().as_slice());

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
            "pre_state": "string (omit if Holds)",
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
