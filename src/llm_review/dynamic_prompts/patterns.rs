use crate::llm_review::{
    enums::{all_enum_variants, generate_enum_list},
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
You are a top Code4rena Security Warden. In this phase, your job is to identify **potential {title} vulnerability PATTERNS** in the target contract.
You are NOT deciding final severity or scope yet. You are building a rich map of plausible weak spots that later stages will triage.

You must be systematic and persistent:
- Do **not** stop early just because the first few patterns look clean.
- For **every** pattern in the list below, you must either:
  - find at least one plausible code location where it might apply, **or**
  - clearly explain why this pattern is unlikely to appear in this contract.
- If information seems missing or ambiguous, state your assumptions and continue with your best effort rather than giving up.

Please analyse the main target contract below for
*each* {title} security vulnerability pattern listed below:

## {title_all_caps} VULNERABILITY PATTERNS TO LOOK FOR
{categories}

---

## Systems-Level Mindset (internal plan – do NOT echo this section)

When reasoning, silently follow this plan:

1. **Build a mental model of the contract (system view)**
   - Identify the contract's role (vault, router, token, oracle adapter, governance, proxy, bridge, etc.).
   - Identify critical state:
     - balances, shares, debts, limits, indices, epochs, flags, roles, configuration parameters.
   - Identify external dependencies:
     - tokens, routers, factories, oracles, multicall, proxies, libraries, bridges, external configs.
   - Sketch the lifecycle:
     - how assets and permissions flow through this contract over time (deposit → accrue → withdraw; open → modify → close; submit → execute → settle).

2. **Derive key invariants and assumptions**
   - Safety invariants (what must always hold):
     - accounting relationships (e.g. total assets vs. shares, debt, or reserves),
     - role and permission boundaries,
     - monotonic or one-way state transitions (indices, epochs, nonces, initialization),
     - upgrade / delegatecall / storage-layout assumptions.
   - Integration assumptions:
     - decimals, rounding behavior, return types, expected behavior of external libraries and tokens,
     - assumptions about oracles, multicall, and cross-chain behavior.

3. **For EACH vulnerability pattern**
   - Locate all functions and code regions that could realistically exhibit that pattern.
   - For each candidate location:
     - trace preconditions (modifiers, `require` checks),
     - trace storage reads and writes (how state evolves),
     - trace external calls (including delegatecall, multicall, token transfers, oracle reads),
     - connect this to the invariants and assumptions from step 2.
   - Consider:
     - single-call behavior,
     - multi-step / multi-transaction sequences (call A then B then C, possibly across different users or roles),
     - cross-contract and cross-library interactions (router ↔ vault, adapter ↔ external AMM, math lib ↔ accounting logic).

4. **Scenario-based reasoning (edges of the state space)**
   - For each relevant pattern, imagine at least one **realistic scenario** (2–4 calls over time) where:
     - boundary conditions are hit (first/last depositor, zero/non-zero balances, max/min values),
     - donations, fee changes, rebases, or emergency functions are involved,
     - ordering is non-trivial (withdraw before claim, emergency mode between operations, admin config change between user calls).
   - Ask: does this scenario plausibly break an invariant or assumption identified earlier?

5. **Cross-module / cross-library composition**
   - Pay close attention to how **different pieces combine**:
     - this contract’s logic + math/token libraries,
     - this contract + external routers/oracles/multicall,
     - storage / delegatecall interactions between this contract and its caller.
   - Look for “safe + safe = unsafe” patterns:
     - rounding in one module + truncation in another,
     - one module assuming 18 decimals while another returns 6,
     - a generic multicall/delegatecall primitive used with a wrong or unverified address.

6. **Record pattern candidates (do not over-filter)**
   - For each pattern:
     - if you see plausible matching code, record it and explain **why** it matches or nearly matches,
     - if you believe the pattern does **not** apply, briefly justify why (e.g. “no external calls of this form”, “no privileged state mutation”, “no oracle usage”).
   - It is acceptable to include potential false positives, as long as your reasoning is explicit. Later phases will confirm or reject them.

---

## Generic Solo / Low-Duplicate Guidance (for pattern discovery)

- Prefer **less obvious, protocol-specific** manifestations of these patterns over trivial textbook ones, as long as they are still **realistically satisfiable** for this contract (current configuration space, normal user/admin flows).
- Look for **context-dependent** breakages where the pattern only becomes dangerous because of how THIS contract implements its state, math, integrations, or lifecycle.
- Check **multi-step / multi-transaction / cross-contract** flows, not just single-function bodies.
- Examine **interactions with imported libraries and external contracts** (math/token helpers, routers, oracles, multicall, proxies, factories): two individually safe components can combine into a dangerous pattern.
- Consider **HIGH-impact edge cases** that are rare but clearly realistic and impactful (e.g. first/last depositor, donation before withdrawal, extreme but valid parameter values) while still consistent with how integrators and users are expected to use the contract.
- Please still note obvious instances, but give extra attention and detail to subtle, protocol-specific, and compositional ones.

---


## Governance / Admin Assumptions

- **Exclude** vulnerabilities that rely on an admin behaving maliciously, making configuration mistakes, or neglecting duties — these are governance risks and out of scope.
- **Include** vulnerabilities where the admin or privileged function operates **exactly according to the specification**, but the implementation itself introduces a vulnerability.

---

## Rules

- **ONLY LOOK FOR {title_all_caps} VULNERABILITY PATTERNS** — ignore unrelated categories.
- This is a **pattern discovery** phase, not final exploit or severity evaluation.
- It is acceptable to include candidates that may later be triaged out, but you must:
  - Avoid purely stylistic / QA-only observations.
  - Provide clear reasoning for why each candidate matches (or nearly matches) one of the listed patterns.
- For **every pattern** in the list above:
  - either identify at least one plausible matching location and explain why,
  - or explicitly explain why that pattern is unlikely to appear in this contract.
  - output all plausible vulnerability patterns, that appear in the cotract, in json format provided below 
"#,
        title = category_spec.title,
        title_all_caps = category_spec.title.to_uppercase(),
        categories = pattern_categories,
    )
}

// pub fn generate_pattern_category_prompt(category: &PatternCategory) -> String {
//     let category_spec = get_category_library_spec(&category).expect("could not find category");
//     let pattern_categories = generate_formated_list_from_pattern_data(&category_spec.issues);
//
//     format!(
//         r#"
//         Please Analyse the main target contract below for
//         *each* {title} security vulnerability patterns listed below:
//
//         ## {title_all_caps} VULNERABILITY PATTERNS TO LOOK FOR
//         {categories}
//
//         ## Governance / Admin Assumptions
//         - **Exclude** vulnerabilities that rely on an admin behaving maliciously, making configuration mistakes, or neglecting duties — these are governance risks and out of scope.
//         - **Include** vulnerabilities where the admin or privileged function operates **exactly according to the specification**, but the implementation itself introduces a vulnerability.
//
//         ## Rules
//         - **ONLY LOOK FOR {title_all_caps} VULNERABILITY PATTERNS listed above** - disregard everything else
//     "#,
//         title = category_spec.title,
//         title_all_caps = category_spec.title.to_uppercase(),
//         categories = pattern_categories,
//     )
// }

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
            "description": "Detailed explanation + vulnerable code snippets",
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
        top_patterns_list.push_str(&pattern.key.to_string());
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
        top_patterns_list.push_str(&pattern.impact_hint.to_string());
        top_patterns_list.push_str("\n\n");
    }

    top_patterns_list
}
