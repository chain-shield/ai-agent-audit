/// Phase 4: Quality assurance and final finding refinement
///
/// This phase performs final quality checks on verified findings and enhances
/// them with improved details, impact analysis, and mitigation strategies.
use crate::{
    config::AuditType,
    error::Result,
    llm_review::{
        context_state::get_metadata_context,
        enums::{AIAgent, Severity, all_enum_variants, generate_enum_list},
        findings::{Finding, Findings},
        prompt_support::severity_rubics::{
            CANTINA_SEVERITY_RUBRIC, CODE4RENA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
        },
        semaphore::GENERAL_SEM,
        utils::prompt_context::{FindingReportType, generate_prompt_for_issue_check},
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::sync::Mutex;

/// Quality check result for a vulnerability finding
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VulnerabilityQualityCheck {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_quality_check_passed: bool, // quality check passes with no changes/update needed, true|false
    pub where_quality_lacks: Option<String>, // brief description
    pub impact: Option<String>,              // updated impact (if necessary)
    pub proof_of_concept: Option<String>,    // updated POC (if necessary)
    pub proof_of_code: Option<String>,       // updated proof of code (if necessary)
    pub severity: Option<Severity>,          // updated severity of issue (if necessary)
    pub mitigation: Option<String>,          // updated mitigation (if necessary)
}

/// Helper function to deserialize boolean from string or boolean
fn deserialize_bool_from_str_or_bool<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let val: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    match val {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(s) => match s.to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(serde::de::Error::custom("expected boolean or string")),
        },
        _ => Err(serde::de::Error::custom("expected boolean or string")),
    }
}

const QUALIFY_PROMPT: &str = r#"

**Inputs you will receive (per request)**  
1. A draft vulnerability write-up authored by another auditor (sections: Description, Impact, Proof of Concept, Proof of Code, Suggested Mitigation, Severity).  
2. The relevant Solidity contract (or excerpt) for context.

Your tasks for vulnerability write-up are:

1. **Proof-of-Concept (PoC) check** – does the current PoC really show how an attacker can exploit it?  
   - If it misses an attack vector or is incorrect, write a *revised* PoC that clearly demonstrates exploitation.

2. **Impact & Severity check** – is the stated impact accurate and is the severity level appropriate (High / Medium / Low / Info)?  
   - If not, provide an updated *impact* paragraph and/or change the *severity*.

3. **Foundry unit-test check** – will the `proof_of_code` test compile and reliably prove the issue?  
   - If it is wrong, incomplete, or non-deterministic, supply a corrected Foundry test (keep it minimal but runnable).

4. **Mitigation check** – will the suggested fix fully eliminate the vulnerability?  
   - If it is insufficient or can be improved, provide an updated mitigation.
**Tasks**

For vulnerability write-up perform the checks below and update anything that is wrong, missing, or can be improved:
"#;

/// Executes the quality check phase
///
/// Performs final quality assurance on verified findings, enhancing them
/// with improved details and ensuring they meet quality standards.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 5: Quality checking findings...");

    let mut handles = vec![];
    let findings = Arc::new(findings);
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);
    let arc_repo = Arc::new(repo.clone());

    let finding_count = findings.findings.len();
    // create vec (is_quality_check_passed, updated_finding) for each finding
    // assume all initially pass
    let quality_check_passed_vec: Arc<Mutex<Vec<(bool, Finding)>>> =
        Arc::new(Mutex::new(vec![(true, Finding::default()); finding_count]));

    info!("now quality check on each finding...");

    for i in 0..finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_agent = Arc::clone(agent);
        let repo_clone = Arc::clone(&arc_repo);
        let arc_findings = Arc::clone(&findings);
        let arc_legit_findings_vec = Arc::clone(&quality_check_passed_vec);
        let sem = Arc::clone(&GENERAL_SEM);

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                let post_qualify = generate_quality_check_json_requirement(&repo_clone);
                let prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    QUALIFY_PROMPT,
                    &post_qualify,
                    FindingReportType::Standard,
                );
                info!("quality checking finding #{}", i + 1);
                let qualify_checked_finding: VulnerabilityQualityCheck =
                    arc_agent.extract_with_retry(&prompt).await?;

                let quality_check_passed = qualify_checked_finding.is_quality_check_passed;
                if !quality_check_passed {
                    info!(
                        "{} did not pass quality check ",
                        arc_findings.findings[i].title,
                    );
                    let updated_finding = Finding {
                        impact: Some(qualify_checked_finding.impact.clone().unwrap_or(
                            arc_findings.findings[i].impact.clone().unwrap_or_default(),
                        )),
                        proof_of_code: Some(
                            qualify_checked_finding.proof_of_code.clone().unwrap_or(
                                arc_findings.findings[i]
                                    .proof_of_code
                                    .clone()
                                    .unwrap_or_default(),
                            ),
                        ),
                        proof_of_concept: Some(
                            qualify_checked_finding.proof_of_concept.clone().unwrap_or(
                                arc_findings.findings[i]
                                    .proof_of_concept
                                    .clone()
                                    .unwrap_or_default(),
                            ),
                        ),
                        mitigation: Some(
                            qualify_checked_finding.mitigation.clone().unwrap_or(
                                arc_findings.findings[i]
                                    .mitigation
                                    .clone()
                                    .unwrap_or_default(),
                            ),
                        ),
                        severity: qualify_checked_finding
                            .severity
                            .unwrap_or(arc_findings.findings[i].severity),
                        ..arc_findings.findings[i].clone()
                    };
                    let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                    legit_findings_vec[i] = (quality_check_passed, updated_finding);
                } else {
                    let mut quality_checked_findings_vec = arc_legit_findings_vec.lock().await;
                    quality_checked_findings_vec[i] = (true, arc_findings.findings[i].clone());
                }

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying finding {}: {:?}", i, e);
            }
        }));
    }

    // Wait for all quality check tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let qualify_checked_findings_vec = quality_check_passed_vec.lock().await;
    let qualified_findings: Vec<Finding> = findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            if qualify_checked_findings_vec[idx].0 {
                // if quality check passes, no changes needed
                f.clone()
            } else {
                // if failed submit updated finding
                qualify_checked_findings_vec[idx].1.clone()
            }
        })
        .collect();

    let num_findings_updated = qualify_checked_findings_vec
        .iter()
        .filter(|(passed, _)| !*passed)
        .count();

    info!(
        "✅ Phase 5 complete: {} Verified Findings with {} updated findings!",
        qualified_findings.len(),
        num_findings_updated
    );

    Ok(Findings {
        findings: qualified_findings,
    })
}

/// Generates the combined content and context block for quality check analysis
///
/// Combines the contract code with additional context information
/// in a structured format for optimal quality check processing.
fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\nSOLIDITY CONTRACT + STORAGE TO CODE REVIEW\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context.push_str("\n\n ## ADDITIONAL CONTEXT \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}

fn generate_quality_check_json_requirement(repo: &RepoPaths) -> String {
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

    let severity_rubic = match repo.audit_type {
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Cantina => CANTINA_SEVERITY_RUBRIC,
        _ => CODE4RENA_SEVERITY_RUBRIC,
    };

    format!(
        r#"

## Severity Rubric 
{severity_rubic}

### OUTPUT REQUIREMENTS 
*Please respond with ONLY valid JSON in the following exact format:*

{{
  "is_quality_check_passed": true | false,  
  "where_quality_lacks": "Brief summary of problems you fixed (omit if passed)",
  "impact": "Updated impact (omit if no update needed)",
  "proof_of_concept": "Revised PoC (omit if no update needed)",
  "proof_of_code": "Revised Foundry test (omit if no update needed)",
  "severity": "{severity_list} (omit if no update needed)",
  "mitigation": "Improved mitigation (omit if no update needed)"
}}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON.

"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AuditType;
    use std::path::PathBuf;

    /// Test quality check prompt generation for all audit types
    #[test]
    fn test_quality_check_prompt_generation_all_audit_types() {
        use crate::prepare_code::git_clone::PocConfig;

        println!("\n{}", "=".repeat(80));
        println!("QUALITY CHECK PROMPT GENERATION TEST");
        println!("{}\n", "=".repeat(80));

        let audit_types = vec![
            AuditType::Code4rena,
            AuditType::Sherlock,
            AuditType::Cantina,
            AuditType::Client,
        ];

        for audit_type in audit_types {
            println!("\n{}", "-".repeat(80));
            println!("AUDIT TYPE: {:?}", audit_type);
            println!("{}\n", "-".repeat(80));

            let repo = RepoPaths {
                github_url: "https://github.com/test/repo".to_string(),
                project_id: "test-project".to_string(),
                root: PathBuf::from("/tmp"),
                sol_files: vec![],
                test_files: vec![],
                script_files: vec![],
                config_files: vec![],
                lib_config_files: vec![],
                source_code_folders: vec![],
                docs: vec![],
                repo_name: "test-repo".to_string(),
                audit_scope: None,
                excluded_folders: None,
                scoped_files: None,
                monorepo_folders: None,
                commit_hash: "abc123".to_string(),
                audit_type,
                poc: PocConfig::default(),
            };

            // Test 1: Quality check prompt (QUALIFY_PROMPT constant)
            println!("📝 QUALITY CHECK PROMPT (QUALIFY_PROMPT):");
            println!("{}", "-".repeat(80));
            println!("{}", QUALIFY_PROMPT);

            // Test 2: JSON requirement
            println!("\n📋 QUALITY CHECK JSON REQUIREMENT:");
            println!("{}", "-".repeat(80));
            let json_req = generate_quality_check_json_requirement(&repo);
            println!("{}", json_req);

            println!("\n");
        }

        println!("\n{}", "=".repeat(80));
        println!("END OF QUALITY CHECK PROMPT GENERATION TEST");
        println!("{}\n", "=".repeat(80));
    }

    /// Test VulnerabilityQualityCheck serialization
    #[test]
    fn test_vulnerability_quality_check_serialization() {
        let quality_check = VulnerabilityQualityCheck {
            is_quality_check_passed: false,
            where_quality_lacks: Some("PoC needs improvement".to_string()),
            impact: Some("Updated impact description".to_string()),
            proof_of_concept: Some("Revised PoC".to_string()),
            proof_of_code: Some("Updated Foundry test".to_string()),
            severity: Some(Severity::High),
            mitigation: Some("Improved mitigation".to_string()),
        };

        let json = serde_json::to_string(&quality_check).expect("Failed to serialize");
        assert!(json.contains(r#""is_quality_check_passed":false"#));
        assert!(json.contains(r#""severity":"High""#));
    }

    /// Test VulnerabilityQualityCheck deserialization with boolean as string
    #[test]
    fn test_vulnerability_quality_check_bool_from_string() {
        let json = r#"{
            "is_quality_check_passed": "true",
            "where_quality_lacks": null,
            "impact": null,
            "proof_of_concept": null,
            "proof_of_code": null,
            "severity": null,
            "mitigation": null
        }"#;

        let quality_check: VulnerabilityQualityCheck =
            serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(quality_check.is_quality_check_passed, true);
    }

    /// Test VulnerabilityQualityCheck deserialization with boolean as bool
    #[test]
    fn test_vulnerability_quality_check_bool_from_bool() {
        let json = r#"{
            "is_quality_check_passed": false,
            "where_quality_lacks": "Needs work",
            "impact": null,
            "proof_of_concept": null,
            "proof_of_code": null,
            "severity": "Medium",
            "mitigation": null
        }"#;

        let quality_check: VulnerabilityQualityCheck =
            serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(quality_check.is_quality_check_passed, false);
        assert_eq!(quality_check.severity, Some(Severity::Medium));
    }
}
