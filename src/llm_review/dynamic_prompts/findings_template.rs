use std::collections::HashSet;

use crate::{
    config::AuditType,
    llm_review::{
        enums::{
            all_enum_variants, generate_enum_bulleted_list, generate_enum_list, EnumData, Severity,
            VulnerabilityType,
        },
        findings::PrivilegeLevel,
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
        r#"Your job is to take the previously discovered **{pattern_type}** and turn them into **concrete, in-scope & valid findings**.

## **Persist until you've thoroughly anlyzed ALL possible exploits from provided pattern**
   - Do **not** stop at the first interesting exploit.
   - Your goal is **maximum coverage** – unearth every valid finding.

## Rules

- Only report exploits **directly tied** to the provided {pattern_type} (patterns or invariants), **not** unrelated issues.
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

{rubric}

---

## Exploit guidelines

- Severity priority: **Theft > DoS > accounting mismatch**.
- Bigger **blast radius** and simpler execution are more valuable.
- Assert conditions using `assertGt` / `assertEq`, not just logs.
- For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.

---

## {pattern_type} Overview

- Type: {pattern_name}
- Definition: {pattern_def}

### Common Exploits

{exploit_bullets}

---

## {title_all_caps} TO ANALYZE

{full_spec}

---
        "#,
        pattern_type = issue_type,
        rubric = severity_rubic,
        pattern_name = issue_enum.to_string(),
        pattern_def = issue_definition,
        exploit_bullets = exploit_bullets,
        full_spec = issue_full_spec,
        title_all_caps = issue_type.to_uppercase()
    )

    //    format!(
    //         r#"Your job is to take the previously discovered **{pattern_type}** and turn them into **concrete, in-scope High/Medium severity findings**.
    //
    // You are **not** searching for new categories of issues now. Instead, you must:
    //
    // - Decide **which** of the provided {pattern_type} actually correspond to real, exploitable vulnerabilities in the code below, and
    // - For each real vulnerability, produce a finding with clear impact, exploit path(s), and mitigation.
    //
    // ---
    //
    // ## Your goals
    //
    // 1. Use the {pattern_type} definition and **common exploit modes** as your primary lens.
    // 2. Only emit findings that:
    //    - Are **within the security audit scope** (contracts, features, roles, chains) provided elsewhere in this prompt.
    //    - Are **realistically exploitable** (not purely theoretical or contrived).
    //    - Justify a **High or Medium severity** under the severity rubric provided below.
    // 3. If no such findings exist, return exactly:
    //    - `{{"findings":[]}}`
    //
    // ---
    //
    // ## How to think (internal plan – do NOT echo this section)
    //
    // 1. **Re-read the context**
    //    - Study the **severity rubric** provided below.
    //    - Review the **{pattern_type} overview**:
    //      - Type: `{pattern_name}`
    //      - Definition: `{pattern_def}`
    //      - Common Exploits: `{exploit_bullets}`
    //    - Carefully read the **"{title_all_caps} TO ANALYZE"** section, which summarizes the candidate patterns or invariants (contracts, functions, descriptions).
    //
    // 2. **For each candidate pattern or invariant** in the "{title_all_caps} TO ANALYZE" section:
    //    - Locate the referenced **contract** and **function(s)** in the code.
    //    - Trace the **full execution path**, including:
    //      - Modifiers and preconditions (`require` checks, role gating, pause switches),
    //      - Storage reads/writes and how state evolves across calls and over time,
    //      - External calls (`call`, `delegatecall`, token transfers, routers, middlewares, oracles),
    //      - Cross-transaction or cross-chain interactions if relevant.
    //    - Consider different attacker roles and identify the **least privilege** that can realistically trigger the exploit (permissionless user, specific role, admin, etc.).
    //
    // 3. **Attempt to construct a concrete exploit**
    //    - Define clear **initial state assumptions**:
    //      - balances, configuration flags, checkpoints, nonces, thresholds, signer sets, etc.
    //    - Define one or more **transactions/operations** the attacker executes, in order.
    //    - Derive the **post-state** and explain why it:
    //      - Violates an important invariant, or
    //      - Causes clear monetary or functional loss.
    //
    //    - If, after careful search, you cannot find a **credible, realistic exploit**, or the impact is only Low/Informational, treat that candidate as **“no finding”**.
    //
    // 4. **Persist until all candidates are considered**
    //    - Do **not** stop at the first interesting exploit.
    //    - Your goal is **maximum H/M coverage** – find every valid High/Medium finding.
    //    - Systematically go through **every** candidate in "{title_all_caps} TO ANALYZE" and decide:
    //      - “Real in-scope H/M vulnerability → keep as a finding”, or
    //      - “No credible in-scope exploit or only Low/Info impact → ignore”.
    //
    // Your actual response must follow the **exact JSON output instructions** that will be provided after this section.
    //
    // ---
    //
    // ## Rules
    //
    // - Only report exploits **directly tied** to the provided {pattern_type} (patterns or invariants), **not** unrelated issues.
    // - Only analyze code **actually present** in the codebase.
    // - Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
    // - Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.
    // - A valid finding must be:
    //   - In-scope,
    //   - Backed by a credible exploit path,
    //   - And clearly **High** or **Medium** severity according to the rubric.
    // - If nothing meets these criteria, return `{{"findings":[]}}`.
    //
    // ---
    //
    // ## Severity rubric
    //
    // {rubric}
    //
    // ---
    //
    // ## Exploit guidelines
    //
    // - Severity priority: **Theft > DoS > accounting mismatch**.
    // - Bigger **blast radius** and simpler execution are more valuable.
    // - Assert conditions using `assertGt` / `assertEq`, not just logs.
    // - For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.
    //
    // ---
    //
    // ## {pattern_type} Overview
    //
    // - Type: {pattern_name}
    // - Definition: {pattern_def}
    //
    // ### Common Exploits
    //
    // {exploit_bullets}
    //
    // ---
    //
    // ## {title_all_caps} TO ANALYZE
    //
    // {full_spec}
    //
    // ---
    //         "#,
    //         pattern_type = issue_type,
    //         rubric = severity_rubic,
    //         pattern_name = issue_enum.to_string(),
    //         pattern_def = issue_definition,
    //         exploit_bullets = exploit_bullets,
    //         full_spec = issue_full_spec,
    //         title_all_caps = issue_type.to_uppercase()
    //     )
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

    format!(
        r#"Your job: analyze the main target contract **through the lens of the provided {pattern_type}** and enumerate the **top exploits/attack vectors** a hacker may deploy.

## **Persist until all patterns are considered**
   - Do **not** stop at the first interesting exploit.
   - Your goal is **maximum coverage** – find every valid finding.
   - Systematically go through **every** candidate block in "{title_all_caps} TO ANALYZE" and decide:
     - "Real in-scope vulnerability keep as a finding"

## Rules

- Only report exploits **directly tied** to the provided {pattern_type} (patterns or invariants), **not** unrelated issues.
- Only analyze code **actually present** in the codebase. 
- Prefer exploits accessible to **unprivileged EOAs**; if an exploit requires a trusted role, make that clear via the `"privilege"` field (as specified in the JSON instructions).
- Focus on **present-state** bugs in the current code. Ignore one-time deployment/upgrade windows unless the same condition can be recreated or abused permissionlessly later.
- A valid finding must be:
  - In-scope,
  - Backed by a credible exploit path,
  - And clearly severity according to the rubric.
- If nothing meets these criteria, return `{{"findings":[]}}`.

## Severity rubric

{rubric}


## Exploit guidelines

- Severity priority: **Theft > DoS > accounting mismatch**.
- Bigger **blast radius** and simpler execution are more valuable.
- Assert conditions using `assertGt` / `assertEq`, not just logs.
- For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.

---

## {pattern_type} Overview

### Common Exploits

{exploit_bullets}

---

## {title_all_caps} TO ANALYZE

{full_spec}

---
        "#,
        pattern_type = issue_type,
        rubric = severity_rubic,
        exploit_bullets = exploit_bullets,
        full_spec = full_spec_of_issues,
        title_all_caps = issue_type.to_uppercase()
    )
}

pub fn get_findings_json_requirement<T>(
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

pub fn get_json_requirement_for_multipattern<T>(
    patterns: &[T],
    pattern_type: &str,
    repo: &RepoPaths,
) -> String
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

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {{
        "findings": [
            {{
            "derived_from": "Insert Title (or Predicate) of most relevant {pattern_type} this finding derives from",
            "title": "200 chars or less audit report friendly title i.e. DOS due to unbounded loop in <contract_name>.<function_name> bricking withdrawals",
            "description": "Detailed explanation + vulnerable snippet",
            "exploit_type": "{issue_list}",
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

        - Keep "derived_from" exactly as shown
        - *privilege* -> least privilege to trigger vulnerability
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
