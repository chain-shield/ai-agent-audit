use crate::{
    config::AuditType,
    llm_review::{
        dynamic_prompts::patterns,
        prompt_support::severity_rubics::{
            CANTINA_SEVERITY_RUBRIC, CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
        },
        threat_models::pattern_category,
    },
    prepare_code::git_clone::RepoPaths,
};

pub fn generate_pattern_category_to_findings_prompt(
    category: &pattern_category::PatternCategory,
    repo: &RepoPaths,
) -> String {
    let category_spec =
        pattern_category::get_category_library_spec(&category).expect("could not find category");
    let pattern_categories =
        patterns::generate_formated_list_from_pattern_data(&category_spec.issues);
    let severity_rubic = match repo.audit_type {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Cantina => CANTINA_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    };

    // NOTE: LARGE INSTRUCTIONS SET
    format!(
        r#"
     You are a top Code4rena Security Warden. 

     ## **Persist until you've thoroughly analyzed ALL possible exploits from provided patterns**
     - Your goal is **maximum coverage** – unearth **EVERY** valid security finding.
     - **Persist** until you unearth every valid finding

     Please analyse the main target contract below for
     *each* {title} security vulnerability pattern listed below:

     ## {title_all_caps} VULNERABILITIES TO LOOK FOR
     {categories}

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

    ## Severity rubric each finding should adhere to

    {severity_rubic}

    ## Exploit guidelines

    - Severity priority: **Theft > DoS > accounting mismatch**.
    - Bigger **blast radius** and simpler execution are more valuable.
    - Assert conditions using `assertGt` / `assertEq`, not just logs.
    - For `"proof_of_code"`, the PoC should correspond to a **compilable Foundry test** (for example using `forge-std`, `vm.prank(attacker)`, etc.), as required by the JSON schema that follows.

     "#,
        title = category_spec.title,
        title_all_caps = category_spec.title.to_uppercase(),
        categories = pattern_categories,
    )
}
