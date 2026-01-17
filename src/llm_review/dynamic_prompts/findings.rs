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

    log::info!(
        "printing {} vulnerability patterns!",
        top_patterns_spec.len()
    );

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
