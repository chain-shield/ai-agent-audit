use crate::{
    config::MAX_PATTERNS_FOR_PROMPT,
    llm_review::{
        agent::agent_enums::{all_enum_variants, generate_enum_list},
        threat_models::invariants::{
            ContractInvariants, InvariantFinding, InvariantSpec, InvariantStatus, InvariantType,
            INVARIANT_LIBRARY,
        },
    },
};
use rand::seq::SliceRandom;

pub fn generate_invariant_prompt(inv: &[InvariantType]) -> String {
    let invariant_categories = generate_formated_list_from_invariant_data(inv);

    format!(
        r#"
You are a senior smart-contract security auditor. In this phase, your job is to **propose and evaluate high-value, machine-checkable invariants** for the target contract and its role in the wider protocol.

You are not writing tests; you are designing the properties those tests would enforce, and checking whether the current implementation appears to uphold them.

---

## Your goals

1. Propose **3–7 strong invariants** that:
   - Capture critical safety, correctness, accounting, or authorization properties of the system.
   - Are concrete enough to be checked automatically (given state and events).
2. For each invariant, mentally try to falsify it using the implementation below:
   - Look for realistic flows, flag combinations, edge cases, or multi-step sequences that might break it.
   - If you find a plausible violation path, treat the invariant as "PossibleViolation" and describe the scenario.
   - If you see no realistic way to violate it, treat it as "Holds" (or the closest status from the allowed enum).

You may include both "obviously critical" invariants and simpler ones, as long as they are precise, checkable, and relevant to security or correctness. The goal is to surface as many meaningful High and Medium risk issues as possible.

---

## Sources of truth

Use all of the following information in your reasoning:

- The Solidity code and storage layout of the main contract (and any directly related modules) shown below.
- Any protocol documentation, audit scope, "areas of concern", and stated invariants included in the context.
- The Invariant Types list and their definitions below.

Treat documentation and scope text as the intended specification, and the code as the implementation that may or may not satisfy it.

---

## Invariant types to focus on

You must base your invariants on the following invariant types and their descriptions, signals, and examples:

{invariants}

You are not required to use every type, but you should prefer types that clearly match the contract's role (e.g. Balance, Permission, Temporal, StateMachine, Referential, Arithmetic, etc.).

---

## How to think

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
    let inv_findings_report = generate_full_list_of_invariant_findings(invs);

    format!(
        r#"
        {json} 

        ## Your task: decide if EACH reported Invariant is valid and should be respected, and if so, does it hold in the code or is it violated? 

        You should return `"true"` for `is_invariant_valid` if invariant is valid and description, predicate, and status all check out.
        Otherwise return `"false"`.

        Please continue until you have carefully evaluated ALL invariants.

        Based on your assessment please provided the following for EACH invariant:

        *invariant id*: insert invariant id (from 'id' field)
        *is invariant valid*: true | false
        *is invariant violated*: true | false (OMIT if invariant is invalid) 

        ## INVARIANTS TO VERIFY
        {report} 

        "#,
        json = verify_json,
        report = inv_findings_report
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
    let json = get_all_invariants_verify_json();

    format!(
        r#"

        Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}
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

pub fn generate_formated_list_from_invariant_data(patterns_to_use: &[InvariantType]) -> String {
    let mut top_invariant_spec: Vec<InvariantSpec> = INVARIANT_LIBRARY
        .iter()
        .filter(|inv| patterns_to_use.contains(&inv.key))
        .map(|inv| inv.to_owned())
        .collect();

    let mut top_invariant_list = String::new();

    let mut rng = rand::rng();
    top_invariant_spec.shuffle(&mut rng);

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

pub fn generate_full_list_of_invariant_findings(co_invariants: &ContractInvariants) -> String {
    let mut invariant_findings = String::new();

    // Randomize the order of invariant
    let mut invariants = co_invariants.invariants.clone();
    let mut rng = rand::rng();
    invariants.shuffle(&mut rng);

    invariants = invariants
        .into_iter()
        .take(MAX_PATTERNS_FOR_PROMPT)
        .collect();

    // log::info!("printing {} actors in prompt", invariants.len());

    for invariant in &invariants {
        let finding = generate_formatted_invariant_finding(invariant);
        invariant_findings.push_str(&finding);
    }
    invariant_findings.push_str("\n\n");

    invariant_findings
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

    invariant_finding
}
