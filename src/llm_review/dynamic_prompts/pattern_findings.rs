use crate::llm_review::{
    enums::{
        all_enum_variants, generate_enum_bulleted_list, generate_enum_list, EnumString, Severity,
    },
    findings::PrivilegeLevel,
    patterns::{Pattern, VulnerabilityPattern},
    utils::prompt_context::generate_formatted_pattern,
};

pub fn generate_pattern_to_findings_prompt(pattern: &Pattern) -> String {
    let exploit_enums = pattern.issue_type.pattern_to_types();
    let exploit_bullets = generate_enum_bulleted_list(exploit_enums); // "- Oracle\n- Reentrancy\n..."
    let exploit_types = generate_enum_list(exploit_enums); // "Oracle|Reentrancy|..."
    let pattern_data = pattern.issue_type.get_vulnerability_spec();
    let privilege_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let json = get_findings_json(&pattern.issue_type);
    let pattern_full_spec = generate_formatted_pattern(pattern);

    format!(
        r#"Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement
        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}

        You are a top Code4rena security warden. Your job: analyze the target contract **through the lens of the provided Security Vulnerability Pattern** and enumerate the **top exploits/attack vectors** a hacker may deploy.

        ## Criteria for a Top Exploit/Attack
        - Severity (High/Medium) per C4: **permissionless (or untrusted role), present-state, financial path**.
        - Profit magnitude (attacker net gain) > payout denial (DoS) > stale/logic mismatch.
        - Fewer calls & lower complexity is better.
        - Larger blast radius (affects many users/epochs) is better.
        - Reproducibility: **Foundry asserts on balances/totals, not logs**.

        ## ATTACKER MODEL & SCOPE (MANDATORY)
        - Attacker: an **unprivileged EOA** (or arbitrary contract) with **no roles** (prefer this over privileged role attacks).
        - Untrusted Roles: do check vectors from **untrusted roles** if defined in scope — but always attempt an **unprivileged** path first.
        - Time: **present-state only** (the deployed/fixture state for this contest).
        - Focus: ONLY report attacks and exploits **stemming from** the Security Vulnerability Pattern below.
        - Scope: If scope is provided, **only** report vulnerabilities **within scope**.

        ## Produce PoC + Foundry test
        - Use forge-std. Show attacker EOA (`vm.prank(attacker)`), arrange/act/assert.
        - Assert profit/state break with `assertGt`, `assertEq`, etc. **No logs-only**.
        - Keep imports complete; test must compile with standard `forge` setup.

        ## Security Vulnerability Pattern
        - Pattern: **{pattern_name}**
        - Definition: {pattern_def}
        - The {pattern_name} commonly maps to:
        ### Exploits
        {exploit_bullets}

        ## QUALITY BAR (reject if not met)
        - Exploit/Attack MUST be tied to the Security Vulnerability Pattern above.
        - No privileged calls UNLESS listed as untrusted in scope.
        - No deployment/upgrade-only windows unless opened permissionlessly first.
        - Assertions MUST show profit or invariant break (not just logs).
        - If zero Highs/Mediums pass this bar, output **{{\"findings\": []}}**.

        ## OUTPUT REQUIREMENTS (for each finding)
        - description: Detailed explanation + exact vulnerable snippet.
        - issue_type: {exploit_types} (choose **one**)
        - privilege: least privilege that can trigger vulnerability: {privileges}
        - contract: Exact contract name.
        - function: Exact function name (or "multiple" if truly necessary).
        - impact: Monetary/functional consequence quantified where possible.
        - proof_of_concept: Step-by-step exploitation scenario.
        - proof_of_code: A COMPLETE Foundry test (compilable) that asserts impact.
        - severity: "High|Medium" (choose **one**)
        - mitigation: Concrete code-level change; include a short diff or snippet.

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        - Keep "derived_from" exactly as shown
        - For "issue_type" choose the best match. If nothing fits after careful review, you may use "Custom".
        - If no vulnerabilities are found, return: 

        {{
        "findings": []
        }}

        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.

        ## SECURITY VULNERABILITY PATTERN TO ANALYZE - FIND TOP EXPLOITS/ATTACKS FOR BELOW
        {full_spec}

        "#,
        pattern_name = pattern.issue_type.as_str(),
        pattern_def = pattern_data.definition,
        exploit_bullets = exploit_bullets,
        exploit_types = exploit_types,
        privileges = privilege_list,
        full_spec = pattern_full_spec
    )
}

pub fn get_findings_json(pattern: &VulnerabilityPattern) -> String {
    let issue_list = generate_enum_list(pattern.pattern_to_types());
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let severity_list = generate_enum_list(all_enum_variants::<Severity>().as_slice());

    format!(
        r#"{{
        "findings": [
            {{
            "derived_from": "{pattern_enum}",
            "description": "Detailed explanation if vulnerability including vulnerable code snippet",
            "issue_type": "{issues}",
            "privilege": "{privileges}",
            "contract": "{{contract_name}}", 
            "function": "{{function_name}}", 
            "impact": "Business and security consequences of the vulnerability",
            "proof_of_concept": "Step-by-step exploitation scenario",
            "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
            "severity": "{severity}",
            "mitigation": "suggested mitigation with code example for the fix"
            }}
        ]
        }}
       "#,
        pattern_enum = pattern.as_str(),
        issues = issue_list,
        privileges = privilege_enum_list,
        severity = severity_list,
    )
}
