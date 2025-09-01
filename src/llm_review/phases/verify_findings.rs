/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    config::{AuditType, AUDIT_TYPE},
    error::Result,
    llm_review::{
        context_state::{generate_audit_scope, get_metadata_context, ContextType},
        enums::AIAgent,
        findings::{Finding, Findings},
        prompt_support::{
            post_verify::POST_VERIFY,
            verify_prompt::{VERIFY_C4_PROMPT, VERIFY_PROMPT, VERIFY_SHERLOCK_PROMPT},
        },
        semaphore::VERIFY_SEM,
        utils::prompt_context::generate_prompt_for_issue_check,
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitVulnerability {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_legit_vulnerability: bool,
    pub why_its_not_legit: Option<String>,
}

/// Helper function to deserialize boolean from string or boolean
pub fn deserialize_bool_from_str_or_bool<'de, D>(
    deserializer: D,
) -> std::result::Result<bool, D::Error>
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

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 4: Deduplicating and verifying findings...");

    let mut handles = vec![];
    let deduped_findings = Arc::new(findings.dedup().await?);
    let context = get_metadata_context(repo, &ContextType::Full)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);

    let dedup_finding_count = deduped_findings.findings.len();
    let is_legit_finding_vec: Arc<Mutex<Vec<bool>>> =
        Arc::new(Mutex::new(vec![true; dedup_finding_count]));

    info!("# of findings AFTER deduping => {}", dedup_finding_count);
    info!("now verifying each finding...");

    let audit_scope = generate_audit_scope(repo).await?;

    let verify_prompt = match AUDIT_TYPE {
        AuditType::Client => Arc::new(VERIFY_PROMPT.to_string()),
        AuditType::Code4rena => Arc::new(VERIFY_C4_PROMPT.to_string()),
        AuditType::Sherlock => Arc::new(VERIFY_SHERLOCK_PROMPT.to_string()),
    };

    let updated_verify_prompt = if audit_scope.is_empty() {
        Arc::new(verify_prompt.to_string())
    } else {
        Arc::new(format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT \n\n{}",
            &verify_prompt, &audit_scope
        ))
    };
    // info!("verify prompt + scope => {}", verify_prompt_plus_scope);

    for i in 0..dedup_finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_findings = Arc::clone(&deduped_findings);
        let arc_agent = Arc::clone(&agent);
        let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
        let verify_prompt_and_scope = Arc::clone(&updated_verify_prompt);
        let sem = Arc::clone(&VERIFY_SEM);

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                let instruction_prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    &verify_prompt_and_scope,
                    POST_VERIFY,
                );

                // add to cost
                info!("verifying finding #{}", i + 1);
                let is_legit_struct: LegitVulnerability =
                    arc_agent.extract_with_retry(&instruction_prompt).await?;

                let is_finding_legit = is_legit_struct.is_legit_vulnerability;
                if !is_finding_legit {
                    info!(
                        "{} is NOT legit => {}",
                        arc_findings.findings[i].title,
                        is_legit_struct.why_its_not_legit.unwrap_or_default()
                    );
                }
                let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                legit_findings_vec[i] = is_finding_legit;

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying finding {}: {:?}", i, e);
            }
        }));
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let legit_findings_vec = is_legit_finding_vec.lock().await;
    let verified_findings: Vec<Finding> = deduped_findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        .filter(|(idx, _)| legit_findings_vec[*idx])
        .map(|(_, f)| f.clone())
        .collect();

    info!(
        "✅ Phase 4 complete: {} Verified Findings!",
        verified_findings.len()
    );

    Ok(Findings {
        findings: verified_findings,
    })
}

/// Generates the combined content and context block for verification analysis
///
/// Combines the contract code with additional context information
/// in a structured format for optimal verification processing.
pub fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();

    code_plus_context.push_str("\n\n# SOLIDITY CONTRACT + STORAGE TO CODE REVIEW\n\n");
    code_plus_context.push_str(codeblock);

    code_plus_context
        .push_str("\n\n ## ADDITIONAL CONTEXT TO ASSIST WITH SECURITY REVIEW OF ABOVE CODE \n\n");
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
