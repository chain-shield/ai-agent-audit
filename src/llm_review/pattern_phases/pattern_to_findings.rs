/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    config::DISCOVERY_RUNS,
    error::Result,
    llm_review::{
        context_state::{generate_audit_scope, get_metadata_context, ContextType},
        enums::AIAgent,
        findings::Findings,
        issues::{IssueStructTrait, IssueTrait},
        semaphore::VERIFY_SEM,
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use serde::{de::DeserializeOwned, Deserializer};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute<T>(
    patterns: T,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
    <T as IssueStructTrait>::Spec: Send + Sync + Clone + DeserializeOwned + IssueTrait + 'static,
{
    let issue_title = patterns.issue_title();
    info!("🔍 Phase 3: Mining Findings from each {}...", issue_title);

    let all_findings = Arc::new(Mutex::new(Findings {
        findings: Vec::new(),
    }));

    let mut handles = vec![];
    let context = get_metadata_context(repo, &ContextType::Full)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);
    let pattern_count = patterns.issues().len();
    let audit_scope = Arc::new(generate_audit_scope(repo).await?);

    for j in 0..pattern_count {
        let arc_pattern = Arc::new(patterns.issues()[j].clone());
        for i in 0..DISCOVERY_RUNS {
            let codeblock_plus_context = Arc::clone(&arc_code_context);
            let arc_agent = Arc::clone(&agent);
            let pattern_clone = Arc::clone(&arc_pattern);
            let sem = Arc::clone(&VERIFY_SEM);
            let shared_findings = Arc::clone(&all_findings);
            let title = issue_title.clone();
            let scope = Arc::clone(&audit_scope);

            handles.push(tokio::spawn(async move {
                // ── acquire permit ────────────────────────
                let _permit = sem.acquire_owned().await.expect("semaphore closed");
                let result: Result<()> = async {
                    info!(
                        "Round {} of mining {} to generate findings from {}",
                        i + 1,
                        title,
                        pattern_clone.title_str()
                    );
                    let core_instructions = pattern_clone.pattern_to_findings_prompt();
                    // info!("core_instructions size: {}", core_instructions.len());
                    let instruction_prompt = if scope.is_empty() {
                        core_instructions
                    } else {
                        format!(
                    "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY REPORT FINDINGS WITHIN SCOPE \n\n{}",
                    &core_instructions, &scope)
                    };
                    // info!(
                    //     "core_instructions + scope size: {}",
                    //     instruction_prompt.len()
                    // );

                    let json_requirement_prompt = pattern_clone.findings_json_required_prompt();
                    let full_prompt = format!(
                        "{}{}{}",
                        instruction_prompt, codeblock_plus_context, json_requirement_prompt
                    );

                    // info!("full_prompt => {}", full_prompt);

                    let findings: Findings = arc_agent.extract_with_retry(&full_prompt).await?;

                    let issues_found = findings.findings.len();
                    info!(
                        "{} findings found from {}!",
                        issues_found,
                        pattern_clone.title_str()
                    );

                    // 3. Merge results (if any) into the shared accumulator
                    if issues_found > 0 {
                        let mut guard = shared_findings.lock().await;
                        guard.findings.extend(findings.findings);
                    }

                    Ok(())
                }
                .await;

                if let Err(e) = result {
                    log::error!("Error extracting findings from pattern: {}", e);
                }
            }));
        }
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let findings = all_findings.lock().await;

    if !findings.findings.is_empty() {
        info!(
            "✅ Phase 3 complete: {} findings BEFORE deduping",
            findings.findings.len()
        );
    }
    Ok(findings.clone())
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
