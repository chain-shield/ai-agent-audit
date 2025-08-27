use crate::{
    config::{AuditType, AUDIT_TYPE},
    llm_review::{
        enums::{
            all_enum_variants, generate_enum_bulleted_list, generate_enum_list, EnumData,
            EnumString, Severity,
        },
        findings::PrivilegeLevel,
        prompt_support::severity_rubics::CODE4RENA_SEVERITY_RUBRIC,
    },
};

/// TODO - add C4 severity rubric
pub fn generate_findings_prompt<T: EnumData + EnumString>(
    issue_type: &str,
    issue_definition: &str,
    issue_full_spec: &str,
    issue_enum: &T,
    issue_desc: &str,
) -> String {
    let exploit_enums = issue_enum.to_types();
    let exploit_bullets = generate_enum_bulleted_list(exploit_enums); // "- Oracle\n- Reentrancy\n..."
    let exploit_types = generate_enum_list(exploit_enums); // "Oracle|Reentrancy|..."
    let privilege_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let json = get_findings_json(issue_enum, issue_desc);
    format!(
        r#"Before we begin, note the required output format:

        ## JSON Output Requirement
        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}

        You are a top Code4rena security warden. Your job: analyze the target contract **through the lens of the provided {pattern_type}** and enumerate the **top exploits/attack vectors** a hacker may deploy.

        ## Criteria for a Top Exploit/Attack
        - Severity (High/Medium) per C4: **permissionless (or untrusted role), present-state, financial path**.
        - Profit magnitude (attacker net gain) > payout denial (DoS) > stale/logic mismatch.
        - Fewer calls & lower complexity is better.
        - Larger blast radius (affects many users/epochs) is better.
        - Reproducibility: **Foundry asserts on balances/totals, not logs**.

        ## ATTACKER MODEL & SCOPE (MANDATORY)
        - Attacker: an **unprivileged EOA** (or arbitrary contract) with **no roles** is preferred over privileged role attacks.
        - Untrusted Roles: do check vectors from **untrusted roles** if defined in scope — but always attempt an **unprivileged** path first.
        - Time: **present-state only** (the deployed/fixture state for this contest).
        - Focus: ONLY report attacks and exploits **stemming from** the {pattern_type} below.
        - Scope: If scope is provided, **only** report vulnerabilities **within scope**.

        ## Produce PoC + Foundry test
        - Use forge-std. Show attacker EOA (`vm.prank(attacker)`), arrange/act/assert.
        - Assert profit/state break with `assertGt`, `assertEq`, etc. **No logs-only**.
        - Keep imports complete; test must compile with standard `forge` setup.

        ## {pattern_type} Overview
        - Type: **{pattern_name}**
        - Definition: {pattern_def}
        - The {pattern_name} commonly maps to:

        ### Exploits
        {exploit_bullets}

        ## {title_all_caps} TO ANALYZE - FIND TOP EXPLOITS/ATTACKS FOR BELOW
        {full_spec}


        ## QUALITY BAR (reject if not met)
        - Exploit/Attack MUST be tied to the {pattern_type} above.
        - No privileged calls UNLESS listed as untrusted in scope.
        - No deployment/upgrade-only windows unless opened permissionlessly first.
        - Assertions MUST show profit or invariant break (not just logs).
        - If zero Highs/Mediums pass this bar, output **{{\"findings\": []}}**.

        ## OUTPUT REQUIREMENTS (for each finding)
        - title: 200 chars or less competitive audit report friendly title i.e. DOS due to unbounded loop in <contract_name>.<function_name> bricking withdrawals
        - description: Detailed explanation + exact vulnerable snippet.
        - exploit_type: {exploit_types} (choose **one**)
        - privilege: least privilege that can trigger vulnerability: {privileges}
        - contract: Exact contract name.
        - function: Exact function name (or "multiple" if truly necessary).
        - impact: Monetary/functional consequence quantified where possible.
        - proof_of_concept: Step-by-step exploitation scenario.
        - proof_of_code: A COMPLETE Foundry test (compilable) that asserts impact.
        - severity: "High|Medium" (choose **one**)
        - mitigation: Concrete code-level change; include a short diff or snippet.

        ## Rubric to Follow to Classify Severity (ignore finding if below Medium)
        {rubric}

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

        "#,
        pattern_type = issue_type,
        rubric = CODE4RENA_SEVERITY_RUBRIC,
        pattern_name = issue_enum.as_str(),
        pattern_def = issue_definition,
        exploit_bullets = exploit_bullets,
        exploit_types = exploit_types,
        privileges = privilege_list,
        full_spec = issue_full_spec,
        title_all_caps = issue_type.to_uppercase()
    )
}

pub fn get_findings_json<T>(pattern: &T, pattern_description: &str) -> String
where
    T: EnumString + EnumData,
{
    let issue_list = generate_enum_list(pattern.to_types());
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let severity_list = match AUDIT_TYPE {
        AuditType::Code4rena => "High|Medium|Low|Info".to_string(),
        _ => generate_enum_list(all_enum_variants::<Severity>().as_slice()),
    };

    format!(
        r#"{{
        "findings": [
            {{
            "derived_from": "{pattern_description}",
            "title": "200 chars or less audit report friendly title",
            "description": "Detailed explanation if vulnerability including vulnerable code snippet",
            "exploit_type": "{issues}",
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
        issues = issue_list,
        privileges = privilege_enum_list,
        severity = severity_list,
    )
}
