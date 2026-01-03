use crate::llm_review::{
    agent::agent_enums::{all_enum_variants, generate_enum_list},
    findings::findings::PrivilegeLevel,
    threat_models::{
        pattern_category::{get_category_library_spec, PatternCategory},
        patterns::{
            Pattern, VulnerabilityPattern, VulnerabilityPatternSpec, VULNERABILITY_PATTERN_LIBRARY,
        },
    },
    utils::prompt_context::generate_formatted_pattern,
};
use rand::seq::SliceRandom;

pub fn generate_pattern_category_prompt(category: &PatternCategory) -> String {
    let category_spec = get_category_library_spec(&category).expect("could not find category");
    let pattern_categories = generate_formated_list_from_pattern_data(&category_spec.issues);

    // NOTE: SHORT INSTRUCTION
    // format!(
    //     r#"
    //  You are a top Code4rena Security Warden. In this phase, your job is to identify **potential {title} vulnerability PATTERNS** in the target contract.
    //
    //  You must be systematic and persistent in your internal reasoning:
    //
    //  - Do **not** stop early just because the first few patterns look clean.
    //  - Mentally iterate through **every** pattern in the list below.
    //  - For each pattern, either:
    //    - find at least one plausible code location where it might apply, **or**
    //    - conclude (in your own reasoning) that it is unlikely to appear in this contract and why.
    //  - Even if some patterns ultimately do **not** appear in the JSON output, you must still check them carefully in your internal analysis.
    //
    //  Please analyse the main target contract below for
    //  *each* {title} security vulnerability pattern listed below:
    //
    //  ## {title_all_caps} VULNERABILITY PATTERNS TO LOOK FOR
    //  {categories}
    //
    //
    //  ## Governance / Admin Assumptions
    //
    //  - Assume admin / owner / multisig / governance is **trusted by default**, however:
    //    - If a security vulnerability pattern manifests when an admin or privileged function operates **exactly according to the intended specification**, tag it as a spec-aligned admin logic flaw (implementation-level).
    //    - On the other hand, if a pattern only manifests when an admin behaves maliciously or recklessly, please exclude this pattern as it falls under governance risk, and it not a true security vulnerability.
    //
    //
    //  ## Rules
    //
    //  - **ONLY LOOK FOR {title_all_caps} VULNERABILITY PATTERNS** - ignore unrelated categories.
    //  - This is a **pattern discovery** phase, not final exploit or severity evaluation.
    //  - It is acceptable to include candidates that may later be triaged out, and you must:
    //    - Avoid purely stylistic or QA-only observations.
    //    - Provide clear reasoning for why each candidate matches (or nearly matches) one of the listed patterns.
    //  - In your final response:
    //    - Output only the patterns you believe are plausible vulnerability patterns in the exact JSON structure described in the OUTPUT REQUIREMENTS section that follows.
    //    - If you find no plausible vulnerability patterns, return an empty patterns list as specified in the OUTPUT REQUIREMENTS section.
    //  "#,
    //     title = category_spec.title,
    //     title_all_caps = category_spec.title.to_uppercase(),
    //     categories = pattern_categories,
    // )

    // NOTE: LARGE INSTRUCTIONS SET
    format!(
        r#"
     You are a top Code4rena Security Warden. In this phase, your job is to identify **potential {title} vulnerability PATTERNS** in the target contract.

     Your primary objective is **high recall of realistically exploitable patterns**, while still allowing later phases to discard false positives.

     You must be systematic and persistent in your internal reasoning:

     - Do **not** stop early just because the first few patterns look clean.
     - Mentally iterate through **every** pattern in the list below.
     - For each pattern, either:
       - find at least one plausible code location where it might apply, **or**
       - conclude (in your own reasoning) that it is unlikely to appear in this contract and why.
     - Even if some patterns ultimately do **not** appear in the JSON output, you must still check them carefully in your internal analysis.

     Please analyse the main target contract below for
     *each* {title} security vulnerability pattern listed below:

     ## {title_all_caps} VULNERABILITY PATTERNS TO LOOK FOR
     {categories}

     ---

     ## Systems-Level Mindset (internal plan - do NOT echo this section)

     When reasoning, silently follow this plan:

     1. **Build a mental model of the contract (system view)**
        - Identify the contract's role (vault, router, token, oracle adapter, governance, proxy, bridge, signature validator, etc.).
        - Identify critical state:
     - balances, shares, debts, limits, indices, epochs, flags, roles, configuration parameters, checkpoints, nonces.
        - Identify external dependencies:
     - tokens, routers, factories, oracles, multicall, proxies, libraries, bridges, external configs, middleware modules.
        - Sketch the lifecycle:
     - how assets, permissions, and configuration flow through this contract over time
       (for example: deposit -> accrue -> withdraw; open -> modify -> close; submit -> execute -> settle; sign -> validate -> execute).

     2. **Derive key invariants and assumptions (including from docs/metadata)**
        Treat comments, metadata files, and protocol documentation as the **intended specification**:

        - Safety invariants (what must always hold), for example:
     - accounting relationships (total assets vs. shares/debt/reserves),
     - role and permission boundaries,
     - monotonic or one-way state transitions (indices, epochs, nonces, checkpoints, initialization),
     - upgrade / delegatecall / storage-layout assumptions.
        - Integration assumptions:
     - decimals, rounding behavior, return types, expected behavior of external tokens/libraries/oracles,
     - assumptions about multicall, bridges, cross-chain behavior, middlewares/checkpointers.
        - **Spec vs implementation mismatches:**
     For each invariant or assumption stated in docs/metadata/comments
     (for example: signatures must always enforce a particular condition, checkpoints must not be bypassed,
     or once a signer is evicted they must never be able to act under the old configuration),
     check whether the implementation can violate it through any realistic sequence of calls or flag/parameter choices.
     Any such mismatch that enables a realistic exploit is a valid security vulnerability pattern.

     3. **For EACH vulnerability pattern (internal checklist)**
        For each pattern in the list above:

        - Locate all functions and code regions that could realistically exhibit that pattern.
        - For each candidate location:
     - trace preconditions (modifiers, `require` checks),
     - trace storage reads and writes (how state evolves across calls and over time),
     - trace external calls (including `call`, `delegatecall`, multicall, token transfers, oracle reads, middlewares),
     - connect this to the invariants and assumptions from step 2.
        - Consider:
     - single-call behavior,
     - multi-step / multi-transaction sequences (call A then B then C, possibly across different users or roles),
     - cross-contract and cross-library interactions (for example: router <-> vault, adapter <-> AMM, signature library <-> auth module).

        Pay special attention to:
        - **Flags / mode bits / "ignore" booleans / optional middlewares** that can disable checks
     (for example: checkpoint/nonce usage flags, toggles that skip validation, optional modules).
     Ask whether an attacker or evicted signer can choose a mode that bypasses intended validation,
     reuses stale configuration, or skips a checkpoint/nonce/snapshot.
        - **Chained or nested flows** (for example: chained signatures, batched operations, multicalls)
     where each step looks safe in isolation but the composition breaks an invariant.

     4. **Scenario-based reasoning (edges of the state space)**
        For each relevant pattern, imagine at least one **realistic scenario** (2-4 calls over time) where:
        - boundary conditions are hit (first/last depositor, zero/non-zero balances, max/min values),
        - donations, fee changes, rebases, or emergency functions are involved,
        - ordering is non-trivial (withdraw before claim, emergency mode between operations, admin config change between user calls),
        - for signatures/auth: various combinations of flags, nonces, checkpoints, and signer revocations.

        Ask whether this scenario plausibly breaks an invariant or assumption identified earlier,
        or creates a clear profit or state-corruption opportunity for some actor.

     5. **Cross-module / cross-library composition**
        Pay close attention to how **different pieces combine**:

        - This contract's logic plus math/token/signature libraries,
        - This contract plus external routers/oracles/multicall/middleware,
        - Storage and delegatecall interactions between this contract and its caller or proxy.

        Look for "safe + safe = unsafe" patterns, for example:
        - rounding in one module plus truncation in another,
        - different decimal assumptions between modules,
        - a generic multicall/delegatecall primitive used with a wrong or weakly-controlled address,
        - optional middleware (for example: checkpointing, rate limits) that can be switched off by the attacker's choice of parameters or flags.

     6. **Record pattern candidates (do not over-filter)**
        - For each pattern:
     - if you see plausible matching code, treat it as a candidate and be explicit in your reasoning (internally) about why it matches or nearly matches,
     - if you believe the pattern does **not** apply, be clear in your own reasoning why (for example: no external calls of this form, no mutable privileged state of this kind, no signature/nonce/checkpoint usage here).
        - It is acceptable to surface potential false positives in the JSON, as long as your reasoning is explicit and the scenario is realistic. Later phases will confirm or reject them.

     ---

     ## Guidance for finding Hard to Detect Patterns

     - Prefer **contract-specific, security-relevant** manifestations of these patterns over purely textbook or cosmetic issues, as long as they are still **realistically satisfiable** for this contract (under normal configurations and expected user/admin flows).
     - Look for **context-dependent** breakages where the pattern only becomes dangerous because of how THIS contract implements its state, math, integrations, signature modes, or lifecycle.
     - Check **multi-step / multi-transaction / cross-contract** flows, not just single function bodies.
     - Examine **interactions with imported libraries and external contracts** (for example: math or token helpers, signature/auth libraries, routers, oracles, multicall, proxies, factories). Two individually safe components can still combine into a dangerous pattern.
     - Still note obvious instances,  but give extra attention and detail to subtle, protocol-specific, and compositional ones.

     ---

     ## Governance / Admin Assumptions

     - Assume admin / owner / multisig / governance is **trusted by default**, however:
       - If a security vulnerability pattern manifests when an admin or privileged function operates **exactly according to the intended specification**, tag it as a spec-aligned admin logic flaw (implementation-level).
       - On the other hand, if a pattern only manifests when an admin behaves maliciously or recklessly, please exclude this pattern as it falls under governance risk, and it not a true security vulnerability.

     ---

     ## Rules

     - **ONLY LOOK FOR {title_all_caps} VULNERABILITY PATTERNS** - ignore unrelated categories.
     - This is a **pattern discovery** phase, not final exploit or severity evaluation.
     - It is acceptable to include candidates that may later be triaged out, and you must:
       - Avoid purely stylistic or QA-only observations.
       - Provide clear reasoning for why each candidate matches (or nearly matches) one of the listed patterns.
     - In your final response:
       - Output only the patterns you believe are plausible vulnerability patterns in the exact JSON structure described in the OUTPUT REQUIREMENTS section that follows.
       - If you find no plausible vulnerability patterns, return an empty patterns list as specified in the OUTPUT REQUIREMENTS section.
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
    let mut top_patterns_spec: Vec<VulnerabilityPatternSpec> = VULNERABILITY_PATTERN_LIBRARY
        .iter()
        .filter(|v| patterns_to_use.contains(&v.key))
        .map(|v| v.to_owned())
        .collect();

    // Randomize the order of patterns
    let mut rng = rand::rng();
    top_patterns_spec.shuffle(&mut rng);

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
