use crate::{
    config::AuditType,
    error::{AuditError, Result},
    llm_review::{
        dynamic_prompts::findings_template::get_pre_json_requirement_for_multipattern,
        prompt_support::severity_rubics::{
            CANTINA_SEVERITY_RUBRIC, CODE4RENA_BOUNTY_SEVERITY_RUBRIC, CODE4RENA_SEVERITY_RUBRIC,
            SHERLOCK_SEVERITY_RUBRIC,
        },
        threat_models::{
            pattern_category,
            patterns::{
                VULNERABILITY_PATTERN_LIBRARY, VulnerabilityPattern, VulnerabilityPatternSpec,
            },
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use rand::seq::SliceRandom;
use std::path::{Path, PathBuf};

fn severity_rubric_for_repo(repo: &RepoPaths) -> Result<String> {
    match repo.audit_type {
        AuditType::Code4renaBounty => load_code4rena_bounty_severity_rubric(repo),
        AuditType::ImmunefiBugBounty => load_immunefi_severity_rubric(repo),
        AuditType::Sherlock => Ok(SHERLOCK_SEVERITY_RUBRIC.to_string()),
        AuditType::Cantina => Ok(CANTINA_SEVERITY_RUBRIC.to_string()),
        _ => Ok(CODE4RENA_SEVERITY_RUBRIC.to_string()),
    }
}

fn load_code4rena_bounty_severity_rubric(repo: &RepoPaths) -> Result<String> {
    let Some(path) = code4rena_bounty_severity_rubric_path(repo) else {
        return Ok(CODE4RENA_BOUNTY_SEVERITY_RUBRIC.to_string());
    };
    std::fs::read_to_string(&path).map_err(|source| AuditError::FileSystem {
        path: path.display().to_string(),
        message: "failed to read generated Code4rena bounty severity rubric".to_string(),
        source: Some(Box::new(source)),
    })
}

fn code4rena_bounty_severity_rubric_path(repo: &RepoPaths) -> Option<PathBuf> {
    repo.docs
        .iter()
        .find(|path| is_code4rena_bounty_severity_rubric(path))
        .cloned()
}

fn is_code4rena_bounty_severity_rubric(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with("-code4rena-severity-rubric.md"))
}

fn load_immunefi_severity_rubric(repo: &RepoPaths) -> Result<String> {
    let Some(path) = immunefi_severity_rubric_path(repo) else {
        return Err(AuditError::Configuration {
            setting: "immunefi severity rubric".to_string(),
            message: format!(
                "AuditType::ImmunefiBugBounty requires a generated Immunefi severity rubric. Expected a repo doc ending in `-immunefi-severity-rubric.md` under audit-docs/<protocol>/ for `{}`. Regenerate audit context before discovery.",
                repo.repo_name
            ),
        });
    };
    std::fs::read_to_string(&path).map_err(|source| AuditError::FileSystem {
        path: path.display().to_string(),
        message: "failed to read generated Immunefi severity rubric".to_string(),
        source: Some(Box::new(source)),
    })
}

fn immunefi_severity_rubric_path(repo: &RepoPaths) -> Option<PathBuf> {
    repo.docs
        .iter()
        .find(|path| is_immunefi_severity_rubric(path))
        .cloned()
}

fn is_immunefi_severity_rubric(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with("-immunefi-severity-rubric.md"))
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
) -> Result<String> {
    let category_spec =
        pattern_category::get_category_library_spec(category).expect("could not find category");
    let pattern_categories = generate_formated_list_from_pattern_data(category_spec.issues);
    let severity_rubic = severity_rubric_for_repo(repo)?;

    let pre_json = get_pre_json_requirement_for_multipattern(
        category_spec.issues,
        "security vulnerability pattern",
        repo,
    );

    Ok(generate_shared_findings_prompt_body(
        &pre_json,
        category_spec.title,
        &pattern_categories,
        &severity_rubic,
    ))
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

    // log::info!(
    //     "printing {} vulnerability patterns!",
    //     top_patterns_spec.len()
    // );

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
