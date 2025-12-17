use std::collections::HashSet;

use crate::{
    config::AuditType,
    llm_review::{
        agent::agent_enums::{
            all_enum_variants, generate_enum_bulleted_list, generate_enum_list, EnumData,
        },
        findings::{
            finding_enums::{Severity, VulnerabilityType},
            findings::PrivilegeLevel,
        },
        prompt_support::severity_rubics::{
            CANTINA_SEVERITY_RUBRIC, CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use strum::IntoEnumIterator;

pub fn generate_findings_prompt<T: EnumData + std::fmt::Display>(
    issue_type: &str,
    issue_definition: &str,
    issue_full_spec: &str,
    issue_enum: &T,
    repo: &RepoPaths,
) -> String {
    let exploit_enums = issue_enum.to_types();
    let exploit_bullets = generate_enum_bulleted_list(exploit_enums); // "- Oracle\n- Reentrancy\n..."
    let severity_rubic = match repo.audit_type {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Cantina => CANTINA_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    };

    format!(
        r#"Your job is to take the previously discovered **{issue_type}** and turn them into **concrete, in-scope & valid findings**.

## **Persist until you've thoroughly anlyzed ALL possible exploits from provided pattern**
   - Do **not** stop at the first interesting exploit.
   - Your goal is **maximum coverage** – unearth every valid finding.

## Rules

- Only report exploits **directly tied** to the provided {issue_type}, **not** unrelated issues.
- Only analyze code **actually present** in the codebase. 
- Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
- Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.
- A valid finding must be:
  - In-scope,
  - Backed by a credible exploit path,
  - And clearly Valid finding according to the rubric.
- If nothing meets these criteria, return `{{"findings":[]}}`.

---

## Severity rubric

{severity_rubic}

---

## Exploit guidelines

- Severity priority: **Theft > DoS > accounting mismatch**.
- Bigger **blast radius** and simpler execution are more valuable.
- Assert conditions using `assertGt` / `assertEq`, not just logs.
- For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.

---

## {issue_type} Overview

- Type: {pattern_name}
- Definition: {issue_definition}

### Common Exploits

{exploit_bullets}

---

## {title_all_caps} TO ANALYZE

{issue_full_spec}

---
        "#,
        pattern_name = issue_enum.to_string(),
        title_all_caps = issue_type.to_uppercase()
    )
}

pub fn generate_findings_prompt_for_multiple_patterns<T: EnumData + std::fmt::Display>(
    issue_type: &str,
    full_spec_of_issues: &str,
    enum_issues: &[T],
    repo: &RepoPaths,
) -> String {
    let exploit_enums: Vec<VulnerabilityType> = enum_issues
        .iter()
        .flat_map(|e| e.to_types())
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let exploit_bullets = generate_enum_bulleted_list(&exploit_enums); // "- Oracle\n- Reentrancy\n..."
    let severity_rubic = match repo.audit_type {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Cantina => CANTINA_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    };

    let json = get_pre_json_requirement_for_multipattern(enum_issues, issue_type, repo);

    format!(
        r#"

        {json}

        Your job: analyze the main target contract **through the lens of the provided {pattern_type}** and enumerate the **top exploits/attack vectors** a hacker may deploy.

        ## **Persist until all patterns are considered**
        - Do **not** stop at the first interesting exploit.
        - Your goal is **maximum coverage** – find every valid finding.
        - Systematically go through **every** candidate block in "{title_all_caps} TO ANALYZE" and decide:
            - "Real in-scope vulnerability keep as a finding"

        ## Rules

        - Only analyze code **actually present** in the codebase. 
        - Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
        - Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.

        ## A valid finding must be:
        - In-scope,
        - Backed by a credible exploit path,
        - And clearly High or Medium severity according to the below rubric.
        - If nothing meets these criteria, return `{{"findings":[]}}`.

        ### In-Context Skill Primer: How to Find Semantic / Multi-Step Bugs - essential to MAXIMIZE Coverage

        **Goal:** Find issues that are *not* syntactically obvious. Avoid repeating common patterns unless uniquely exploitable.

        **Method (follow exactly):**

        1. **Model state transitions:** Identify critical state variables and who can change them *between* steps/txs.
        2. **Snapshot vs live reads:** For each multi-call flow, mark which values are snapshotted vs reread later.
        3. **Cross-contract edges:** List external calls + callbacks + hooks (ERC777, ERC4626, tokens, oracles, governance modules).
        4. **Probabilistic reasoning:** For any randomness/entropy, ask “what if inputs correlate / repeat / are controllable?”
        5. **Incentives & griefing:** Ask “who benefits if this fails or is delayed?” Include DoS-by-incentive.
        6. **Multi-step attack synthesis:** Write at least **2 candidate attack sequences** (3–6 steps each) before concluding “no issue”.

        **Hard constraint:** Produce **at least 3 findings candidates** that require ≥2 steps or cross-contract reasoning, even if tentative.

        ## {pattern_type} Overview

        ### Common Exploits

        {exploit_bullets}

        ---

        ## {title_all_caps} TO ANALYZE

        {full_spec}

        ## Severity Rubric

        {rubric}

        "#,
        pattern_type = issue_type,
        rubric = severity_rubic,
        exploit_bullets = exploit_bullets,
        full_spec = full_spec_of_issues,
        title_all_caps = issue_type.to_uppercase()
    )
}

pub fn get_post_findings_json_requirement<T>(
    pattern: &T,
    pattern_description: &str,
    repo: &RepoPaths,
) -> String
where
    T: std::fmt::Display + EnumData,
{
    let issue_list = generate_enum_list(pattern.to_types());
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let severity_enums_standard: Vec<Severity> = Severity::iter()
        .filter(|s| *s != Severity::Critical)
        .collect();
    let severity_enums_list_standard = generate_enum_list(severity_enums_standard.as_slice());
    let severity_list = match repo.audit_type {
        AuditType::Code4rena => severity_enums_list_standard,
        AuditType::Sherlock => severity_enums_list_standard,
        AuditType::Cantina => severity_enums_list_standard,
        _ => generate_enum_list(all_enum_variants::<Severity>().as_slice()),
    };

    format!(
        r#"

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {{
        "findings": [
            {{
            "derived_from": "{pattern_description}",
            "title": "200 chars or less audit report friendly title i.e. DOS due to unbounded loop in <contract_name>.<function_name> bricking withdrawals",
            "description": "Detailed explanation + vulnerable snippet",
            "exploit_type": "{issues}",
            "privilege": "{privileges}",
            "contract": "{{contract_name}}", 
            "function": "{{function_name}}", 
            "impact": "monetary/functional consequences",
            "proof_of_concept": "Step-by-step exploitation scenario",
            "proof_of_code": "compilable Foundry unit test",
            "severity": "{severity}",
            "mitigation": "concrete code fix"
            }}
        ]
        }}

        - Keep "derived_from" exactly as shown
        - *privilege* -> least privilege to trigger vulnerability
        - If no vulnerabilities are found, return: 

        {{
        "findings": []
        }}

        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
       "#,
        issues = issue_list,
        privileges = privilege_enum_list,
        severity = severity_list,
    )
}

pub fn get_post_json_requirement_for_multipattern<T>(
    patterns: &[T],
    pattern_type: &str,
    repo: &RepoPaths,
) -> String
where
    T: std::fmt::Display + EnumData,
{
    let json = get_json_requirement(patterns, pattern_type, repo);
    let vulnerabities: Vec<VulnerabilityType> = patterns
        .iter()
        .flat_map(|p| p.to_types())
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let issue_list = generate_enum_list(&vulnerabities);

    format!(
        r#"

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        - *privilege* -> least privilege to trigger vulnerability
        - for "exploit_type" please select from one of the listed types: {issue_list}
        - If no vulnerabilities are found, return: 

        {{
        "findings": []
        }}

        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
       "#
    )
}

pub fn get_pre_json_requirement_for_multipattern<T>(
    patterns: &[T],
    pattern_type: &str,
    repo: &RepoPaths,
) -> String
where
    T: std::fmt::Display + EnumData,
{
    let json = get_json_requirement(patterns, pattern_type, repo);

    format!(
        r#"

        Before instructions are provided on the task please note required output format:

        ## JSON Output Requirement

        **Output must be strictly valid JSON** with this structure (no extra text or code fencing):

        {json}

       "#
    )
}

pub fn get_json_requirement<T>(patterns: &[T], pattern_type: &str, repo: &RepoPaths) -> String
where
    T: std::fmt::Display + EnumData,
{
    let vulnerabities: Vec<VulnerabilityType> = patterns
        .iter()
        .flat_map(|p| p.to_types())
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let issue_list = generate_enum_list(&vulnerabities);
    let privilege_enum_list = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let severity_enums_standard: Vec<Severity> = Severity::iter()
        .filter(|s| *s != Severity::Critical)
        .collect();
    let severity_enums_list_standard = generate_enum_list(severity_enums_standard.as_slice());
    let severity_list = match repo.audit_type {
        AuditType::Code4rena => severity_enums_list_standard,
        AuditType::Sherlock => severity_enums_list_standard,
        AuditType::Cantina => severity_enums_list_standard,
        _ => generate_enum_list(all_enum_variants::<Severity>().as_slice()),
    };

    format!(
        r#"
        {{
        "findings": [
            {{
            "derived_from": "Insert title (or predicate) of most relevant {pattern_type} this finding derives from",
            "title": "200 chars or less audit report friendly title i.e. DOS due to unbounded loop in <contract_name>.<function_name> bricking withdrawals",
            "description": "Detailed explanation + vulnerable snippet",
            "exploit_type": "MUST be exactly one of: {issue_list}",
            "privilege": "{privilege_enum_list}",
            "contract": "{{contract_name}}", 
            "function": "{{function_name}}", 
            "impact": "monetary/functional consequences",
            "proof_of_concept": "Step-by-step exploitation scenario",
            "proof_of_code": "compilable Foundry unit test",
            "severity": "{severity_list}",
            "mitigation": "concrete code fix"
            }}
        ]
        }}
       "#
    )
}
