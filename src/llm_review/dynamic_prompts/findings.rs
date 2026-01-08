use crate::{
    config::AuditType,
    llm_review::{
        dynamic_prompts::findings_template::get_pre_json_requirement_for_multipattern,
        prompt_support::severity_rubics::{
            CANTINA_SEVERITY_RUBRIC, CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
        },
        threat_models::{
            pattern_category,
            patterns::{
                VulnerabilityPattern, VulnerabilityPatternSpec, VULNERABILITY_PATTERN_LIBRARY,
            },
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use rand::seq::SliceRandom;

fn severity_rubric_for_repo(repo: &RepoPaths) -> &'static str {
    match repo.audit_type {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Cantina => CANTINA_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    }
}

fn generate_shared_findings_prompt_body(
    pre_json: &str,
    title: &str,
    vulnerabilities_block: &str,
    severity_rubic: &str,
) -> String {
    let title_all_caps = title.to_uppercase();

    format!(
        r#"

		{pre_json} 

	     ## **Persist until you've thoroughly analyzed ALL possible exploits from provided patterns**
	     - Your goal is **maximum coverage** – unearth **EVERY** valid security finding.
	     - **Persist** until you unearth every valid finding

	     Please analyse the main target contract below for
	     *each* {title} security vulnerability pattern listed below:

	     ## {title_all_caps} VULNERABILITIES TO LOOK FOR
	     {vulnerabilities_block}

	    ## Exploit guidelines

	    - Severity priority: **Theft > DoS > accounting mismatch**.
	    - Bigger **blast radius** and simpler execution are more valuable.
	    - Assert conditions using `assertGt` / `assertEq`, not just logs.
	    - For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.

	    ## Rules

	    - Only report exploits **directly tied** to the provided list of security vulnerability patterns, **not** unrelated issues.
	    - Only analyze code **actually present** in the codebase. 
	    - Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
	    - Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.
	    - A valid finding must be:
	    - In-scope,
	    - Backed by a credible exploit path,
	    - And clearly Valid finding according to the rubric.
	    - If nothing meets these criteria, return `{{"findings":[]}}`.

	    ### Semantic / multi-step hunting checklist (apply to EACH critical flow): 
	    1. Identify state vars + who can change them between txs. 
	    2. Mark snapshot vs live reads (values cached vs reread later). 
	    3. Enumerate cross-contract edges (external calls, hooks, callbacks, token/oracle/governance modules). 
	    4. For randomness/entropy: test “repeat/correlate/control inputs” scenarios. 
	    5. Incentives/griefing: who profits from delay/failure/DoS? 
	    6. Synthesize 2+ attack sequences (3–6 steps) before concluding “no issue”.

	    ## Example: Incentives / Game Theory (Governance / Griefing DoS)

	    **Pattern:** “Attack is rational because attacker profits from delay/failure; no code bug needed besides an incentive misalignment.”

	    **Setup (typical):**

	    * Protocol has an on-chain action `executeProposal()` / `finalizeEpoch()` / `distributeRewards()` / `rebalance()` that must run for system health.
	    * Anyone *can* call it, but it is **unprofitable** or **costly** to call (gas-heavy, or caller gets slashed / pays).
	    * Meanwhile a subset of users **benefit if it does NOT execute** (e.g., avoid liquidation, keep emissions flowing, block parameter decrease).

	    **How to hunt it:**

	    1. Identify an **“eventual progress” dependency** (something must happen for safety or fairness).
	    2. Check **who pays** to trigger it (gas/penalty) vs **who benefits** if it’s delayed.
	    3. If the beneficiaries can rationally outbid helpers / keepers, it becomes a **credible DoS-by-incentive**.

	    **Exploit hypothesis:**

	    1. Attacker takes a position that becomes bad if `finalize()/execute()` runs (e.g., undercollateralized loan, governance parameter about to tighten).
	    2. Attacker ensures the only way to progress is calling a gas-heavy function with no reward.
	    3. Rational actors don’t call it; attacker can also grief callers (e.g., via MEV/backrun if relevant).
	    4. Protocol remains in stale state; attacker avoids liquidation / keeps emissions / blocks risk reduction.
	    5. Attacker exits position once favorable, or repeats each epoch.

	    **Impact phrasing:**

	    * “Functionally permanent DoS because rational actors are disincentivized to execute the necessary transition.”

	    **Mitigation:**

	    * Add **keeper incentive** (caller reward funded from protocol fees) or make execution **cheap/batched**, and/or add a **fallback** path (permissioned keeper / bounded work per call).

	    ## Example: Semantic (Snapshot vs Live Read / Mid-Flow Parameter Change)

	    **Pattern:** value is read twice across calls/txs; attacker changes it in between.

	    **Exploit hypothesis:**

	    1. User starts flow that assumes `feeRate` / `oraclePrice` / `config` stays constant.
	    2. Before finalization, attacker/admin/MEV changes config or price.
	    3. Final step rereads “live” value and applies it inconsistently → user underpays / over-withdraws / bypasses checks.

	    **Mitigation:** snapshot the value once and reuse, or enforce bounds/time validity.

	    ## Example: Probabilistic (Same Seed / Correlated “Randomness”)

	    **Pattern:** randomness looks fine syntactically but is **correlated/reused**.

	    **Exploit hypothesis:**

	    1. Protocol uses `seed = keccak(blockhash, user, timestamp)` in multiple places or multiple draws.
	    2. Attacker chooses inputs / timing so draws become correlated (or repeats seed).
	    3. Outcomes become predictable / biased → attacker wins raffle/selection more than expected.

	    **Mitigation:** domain-separate draws, use commit-reveal / VRF / include unique nonces per draw.

	    ## Severity rubric each finding should adhere to

	    {severity_rubic}

	     "#,
    )
}

pub fn generate_pattern_category_to_findings_prompt(
    category: &pattern_category::PatternCategory,
    repo: &RepoPaths,
) -> String {
    let category_spec =
        pattern_category::get_category_library_spec(&category).expect("could not find category");
    let pattern_categories = generate_formated_list_from_pattern_data(&category_spec.issues);
    let severity_rubic = severity_rubric_for_repo(repo);

    let pre_json = get_pre_json_requirement_for_multipattern(
        &category_spec.issues,
        "security vulnerability pattern",
        repo,
    );

    generate_shared_findings_prompt_body(
        &pre_json,
        &category_spec.title,
        &pattern_categories,
        severity_rubic,
    )
}

fn generate_formated_list_from_pattern_data(patterns_to_use: &[VulnerabilityPattern]) -> String {
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
