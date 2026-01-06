use crate::llm_review::{
    agent::agent_enums::{all_enum_variants, generate_enum_list},
    dynamic_prompts::prompt_index,
    threat_models::invariants::{
        ContractInvariants, InvariantFinding, InvariantSpec, InvariantStatus, InvariantType,
        INVARIANT_LIBRARY,
    },
};
use rand::seq::SliceRandom;

pub fn generate_invariant_prompt(inv: &[InvariantType]) -> String {
    let invariant_categories = generate_formated_list_from_invariant_data(inv);
    let toc = prompt_index::generate_invariant_discovery_toc();
    let section_1_header = prompt_index::generated_section_header("CORE INSTRUCTIONS", 1);
    let section_1_1_header = prompt_index::generated_sub_header("YOUR GOALS", 1, 1);
    let section_1_2_header = prompt_index::generated_sub_header("SOURCES OF TRUTH", 1, 2);
    let section_1_3_header =
        prompt_index::generated_sub_header("INVARIANT TYPES TO FOCUS ON", 1, 3);
    let section_1_4_header = prompt_index::generated_sub_header("HOW TO THINK", 1, 4);

    format!(
        r#"
{toc}

{section_1_header}

You are a senior smart-contract security auditor. In this phase, your job is to **propose and evaluate high-value, machine-checkable invariants** for the target contract and its role in the wider protocol.

You are not writing tests; you are designing the properties those tests would enforce, and checking whether the current implementation appears to uphold them.

{section_1_1_header}

1. Propose **3–7 strong invariants** that:
   - Capture critical safety, correctness, accounting, or authorization properties of the system.
   - Are concrete enough to be checked automatically (given state and events).
2. For each invariant, mentally try to falsify it using the implementation below:
   - Look for realistic flows, flag combinations, edge cases, or multi-step sequences that might break it.
   - If you find a plausible violation path, treat the invariant as "PossibleViolation" and describe the scenario.
   - If you see no realistic way to violate it, treat it as "Holds" (or the closest status from the allowed enum).

You may include both "obviously critical" invariants and simpler ones, as long as they are precise, checkable, and relevant to security or correctness. The goal is to surface as many meaningful High and Medium risk issues as possible.

{section_1_2_header}

Use all of the following information in your reasoning:

- The Solidity code and storage layout of the main contract (and any directly related modules) shown below.
- Any protocol documentation, audit scope, "areas of concern", and stated invariants included in the context.
- The Invariant Types list and their definitions below.

Treat documentation and scope text as the intended specification, and the code as the implementation that may or may not satisfy it.

{section_1_3_header}

You must base your invariants on the following invariant types and their descriptions, signals, and examples:

{invariants}

You are not required to use every type, but you should prefer types that clearly match the contract's role (e.g. Balance, Permission, Temporal, StateMachine, Referential, Arithmetic, etc.).

{section_1_4_header}

When designing each invariant:

1. Model the contract and its role
   - Identify what the contract is for (e.g. signature validation, vault, permissions, recovery module, oracle, router).
   - Identify who the key actors are (owners, signers, admins, modules, external protocols).

2. Extract candidate invariants from the spec and context
   - Translate any stated invariants or assumptions in the docs/scope into precise, checkable properties.
   - Think about:
     - Access control and privilege boundaries.
     - Balance and accounting relationships.
     - Nonces, counters, and sequencing.
     - Configuration / image hash / checkpointer behavior.
     - Cross-contract or cross-chain relationships if referenced.

3. Make them machine-checkable
   - Express each invariant as a clear predicate over contract state and/or events.
   - Use concrete conditions like:
     - Relationships between balances and totals.
     - Relationships between stored configuration and computed hashes.
     - Conditions on who is allowed to perform which actions under which flags/modes.
     - Temporal properties across function calls (e.g. nonces, cooldowns, checkpoints).

4. Actively search for violations
   - For each invariant, scan the code for:
     - Branches that skip checks (e.g. flag bits, mode switches, early returns).
     - Edge cases in loops, array indexing, or boundary conditions.
     - Multi-step flows (chained signatures, batched calls, upgradable configs) where state may drift from the intended invariant.
   - If you find a credible way the invariant could be broken, mark it as PossibleViolation (or equivalent status) and describe:
     - The pre-state (relevant configuration / storage / role assumptions).
     - The actions or sequence of calls.
     - The post-state and why it violates the invariant.
     - The likely impact.

5. Coverage vs signal
   - It is acceptable to include some simpler invariants if they help cover more potential High/Medium issues.
   - Still avoid vague or purely stylistic "invariants"; each one should correspond to a concrete, checkable property whose violation could matter in practice.
    "#,
        invariants = invariant_categories
    )
}

pub fn generate_all_invariants_verify_prompt(invs: &ContractInvariants) -> String {
    let verify_json = get_pre_all_invariants_verify_json();
    let section_2_header = prompt_index::generated_section_header("CORE INSTRUCTIONS", 2);
    let section_3_header = prompt_index::generated_section_header("INVARIANTS TO VERIFY", 3);
    let inv_findings_report = {
        let mut invariant_findings = String::new();
        for invariant in &invs.invariants {
            let finding = generate_formatted_invariant_finding(invariant);
            invariant_findings.push_str(&finding);
        }
        invariant_findings
    };

    format!(
        r#"
{json}

{section_2_header}

Your task: decide if EACH reported Invariant is valid and should be respected, and if so, does it hold in the code or is it violated?

You should return `"true"` for `is_invariant_valid` if invariant is valid and description, predicate, and status all check out.
Otherwise return `"false"`.

Please continue until you have carefully evaluated ALL invariants.

Based on your assessment please provided the following for EACH invariant:

*invariant id*: insert invariant id (from 'id' field)
*is invariant valid*: true | false
*is invariant violated*: true | false (OMIT if invariant is invalid)

{section_3_header}

{report}

        "#,
        json = verify_json,
        report = inv_findings_report
    )
}

pub fn generate_invariant_verify_prompt(inv: &InvariantFinding) -> String {
    let verify_json = get_invariant_verify_json();
    let inv_finding_report = generate_formatted_invariant_finding(inv);

    format!(
        r#"
        ## Your task: decide if the reported Invariant is valid and should be respected, and if so, does it hold in the code or is it violated? 
        
        You should return `"true"` if invariant is valid and description, predicate, and status all check out.
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
    let section_1_header = prompt_index::generated_section_header("OUTPUT FORMAT REQUIREMENTS", 1);
    let section_1_1_header = prompt_index::generated_sub_header("JSON OUTPUT REQUIREMENT", 1, 1);

    format!(
        r#"

{section_1_header}

Before instructions are provided on the task please note required output format:

{section_1_1_header}

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):
        
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

pub fn get_post_all_invariants_verify_json() -> String {
    let json = get_all_invariants_verify_json();

    format!(
        r#"

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
    "#
    )
}

pub fn get_pre_all_invariants_verify_json() -> String {
    format!(
        r#"

        Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        - **Output must be strictly valid JSON** 
        - No markdown, no code fences
        - Must validate against schema below in SECTION 11.1

    "#
    )
}

pub fn get_all_invariants_verify_json() -> String {
    format!(
        r#"
        {{
            "findings": [
                {{
                    "invariant_id": "'id' field from finding",
                    "is_invariant_valid": true|false,
                    "is_invariant_violated": true|false, (OMIT this field if invariant is not valid)
                    "why_its_not_valid": "in 40 words less explain why NOT valid (OMIT if valid)"
                }}
            ]
        }}
        "#
    )
}

pub fn get_invariant_verify_json() -> String {
    format!(
        r#"
        {{
            "is_invariant_valid": true|false,
            "why_its_not_valid": "in 40 words less explain why NOT legit (OMIT if legit)"
        }}
        "#
    )
}

pub fn generate_formated_list_from_invariant_data(patterns_to_use: &[InvariantType]) -> String {
    let mut top_invariant_spec: Vec<InvariantSpec> = INVARIANT_LIBRARY
        .iter()
        .filter(|inv| patterns_to_use.contains(&inv.key))
        .map(|inv| inv.to_owned())
        .collect();

    let mut rng = rand::rng();
    top_invariant_spec.shuffle(&mut rng);

    let mut top_invariant_list = String::new();

    for pattern in top_invariant_spec {
        top_invariant_list.push_str("\n\n");
        top_invariant_list.push_str("### Invariant Type\n");
        top_invariant_list.push_str(&pattern.key.to_string());
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
        top_invariant_list.push_str(&pattern.impact_hint.to_string());
        top_invariant_list.push_str("\n\n");
    }

    top_invariant_list
}

pub fn generate_full_list_of_invariant_findings(
    co_invariants: &ContractInvariants,
    section_num: u8,
) -> (String, String) {
    let invariant_title = "Contract Invariants to Consider";
    let mut invariant_findings = format!(
        r#"

{}

**NOTE**: The invariants below are pertinent to the codebase where vulnerability were found, please incorporate them in your analysis.
Also, this is NOT a complete list of invariants, other may exist in codebase.

                "#,
        prompt_index::generated_section_header(&invariant_title.to_uppercase(), 7)
    );

    let mut invariant_index =
        prompt_index::generated_table_of_context_header(invariant_title, section_num);

    // Randomize the order of invariant
    let mut invariants = co_invariants.invariants.clone();
    let mut rng = rand::rng();
    invariants.shuffle(&mut rng);

    for (section, invariant) in invariants.iter().enumerate() {
        // update table of contents with new entry
        invariant_index.push_str(&format!(
            "-{}.{} {}\n",
            section_num,
            section + 1,
            invariant.predicate
        ));

        // add new invariant to list
        invariant_findings.push_str(&prompt_index::generated_sub_header(
            &invariant.predicate,
            7,
            section + 1,
        ));
        let finding = generate_formatted_invariant_finding(invariant);
        invariant_findings.push_str(&finding);
    }
    invariant_findings.push_str("\n\n");

    (invariant_findings, invariant_index)
}

pub fn generate_formatted_invariant_finding(invariant: &InvariantFinding) -> String {
    let mut invariant_finding = String::new();

    invariant_finding.push_str(&format!(
        "\n\n ### Invariant Type: {}\n",
        &invariant.inv_type.to_string()
    ));

    if let Some(id) = &invariant.id {
        invariant_finding.push_str(&format!("\n\n ### Invariant Id: {}\n", id));
    }

    invariant_finding.push_str(&format!(
        "\n ### Relevant Function/Location: {}.{}\n",
        invariant.contract, invariant.function
    ));

    invariant_finding.push_str("\n ### Predicate\n");
    invariant_finding.push_str(&invariant.predicate);

    invariant_finding.push_str("\n ### Description/Code Snippet\n");
    invariant_finding.push_str(&invariant.desc.to_string());

    invariant_finding.push_str("\n ### Checks\n");
    invariant_finding.push_str(&invariant.checks.join(", "));

    invariant_finding.push_str("\n ### Pre-State\n");
    invariant_finding.push_str(&invariant.pre_state.clone().unwrap_or_default());

    invariant_finding.push_str("\n ### Post-State\n");
    invariant_finding.push_str(&invariant.post_state.clone().unwrap_or_default());
    invariant_finding.push_str("\n");

    invariant_finding
}
