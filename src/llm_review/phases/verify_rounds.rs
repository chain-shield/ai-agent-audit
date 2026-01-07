use crate::config::CODE_SECTION;
use crate::llm_review::agent::agent_factory::{AgentConfig, AgentFactory};
use crate::llm_review::dynamic_prompts::prompt_index;
use crate::llm_review::pattern_phases::generate_patterns::generate_content_plus_context_block;
use crate::llm_review::phases::rounds::all_rounds::{AllRoundLegitAnalysis, VerifyAllRound};
use crate::llm_review::phases::rounds::utils::generate_post_round_verify_json_requirement;
use crate::llm_review::phases::rounds::validate_round::{
    generate_dynamic_validation_json, generate_round_validation_prompt, FindingDowngradeValidation,
    ValidateLegitAnalysis,
};
/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::llm_review::utils::prompt_context::{
    self, generate_prompt_for_multi_finding_issue_check,
};
use crate::reporting::save_file;
use crate::{
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::context_state::{generate_audit_scope, get_metadata_context},
        findings::findings::{Finding, Findings},
        utils::prompt_context::FindingReportType,
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use strum_macros::EnumIter;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumIter,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum FindingStatus {
    #[default]
    Valid,
    InvalidBugDoesNotExist,
    InvalidOutOfScope,
    InvalidUserErrorOrMistake,
    InvalidGovernanceRisk,
    InvalidERC20EdgeCase,
    InvalidNotExploitable,
    InvalidFutureSpeculation,
    InvalidByDesign,
    InvalidSafeGuardInPlace,
    LowSeverityDueToLowImpact,
    LowSeverityDueToRareLikelihood,
    InvalidOtherReason,
    NeedsMoreInfo,
}

pub trait AnalysisRound {
    type Spec: FindingAnalysis;
    fn findings(&self) -> &[Self::Spec];
}

pub trait FindingAnalysis {
    fn get_finding_status_array_from_analysis(&self) -> Option<Vec<FindingStatus>>;
    fn print_analysis_results(&self);
    fn generate_verify_prompt() -> String;
    fn generate_verify_json() -> String;
    fn id(&self) -> String;
    fn get_justification(&self) -> String;
}

impl AnalysisRound for VerifyAllRound {
    type Spec = AllRoundLegitAnalysis;
    fn findings(&self) -> &[Self::Spec] {
        &self.findings
    }
}

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute_rounds(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 4: Deduplicating and verifying findings...");

    let deduped_findings = findings.dedup().await?;
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");

    let code_with_header = {
        let section_8_header = prompt_index::generated_section_header(
            "CODEBASE WHERE FINDINGS WHERE DISCOVERED",
            CODE_SECTION,
        );

        format!(
            r#"

{section_8_header}

{code}
"#
        )
    };

    let code_and_context = generate_content_plus_context_block(&code_with_header, &context);

    let dedup_finding_count = deduped_findings.findings.len();

    info!("# of findings AFTER deduping => {}", dedup_finding_count);
    info!("now verifying each finding...");

    let audit_scope = generate_audit_scope(repo).await?;

    //************************
    // VERIFICATON ROUND
    //************************

    let all_round_findings =
        run_verification_round(deduped_findings, &code_and_context, &audit_scope, agent).await?;
    info!(
        "{} finding tagged as low or invalid",
        tagged_findings(&all_round_findings)
    );

    // Label findings with status=None as Valid (they passed all checks)
    let all_round_findings_labeled: Vec<Finding> = all_round_findings
        .findings
        .into_iter()
        .map(|f| {
            if f.status.is_none() {
                Finding {
                    status: Some(vec![FindingStatus::Valid]),
                    ..f
                }
            } else {
                f
            }
        })
        .collect();

    let labeled_findings = Findings {
        findings: all_round_findings_labeled,
    };

    //************************
    // VERIFICATON ROUND
    //************************

    let verified_findings =
        run_validation_round(labeled_findings, &code_and_context, &audit_scope, repo).await?;

    let verify_findings_vec: Vec<Finding> = verified_findings
        .findings
        .into_iter()
        .filter(|f| {
            f.status.as_ref().is_some_and(|s| {
                s.len() == 1
                    || (s.len() <= 2 // edge case in case is Critical Impact and rare  => Medium
                        && s.contains(&FindingStatus::LowSeverityDueToRareLikelihood)
                        && !s.contains(&FindingStatus::LowSeverityDueToLowImpact))
            })
        })
        .collect();

    info!(
        "✅ Phase 4 complete: {} Validated Findings!",
        verify_findings_vec
            .iter()
            .filter(|f| f
                .status
                .as_ref()
                .is_some_and(|s| s.contains(&FindingStatus::Valid)))
            .count()
    );

    Ok(Findings {
        findings: verify_findings_vec,
    })
}

pub fn tagged_findings(findings: &Findings) -> usize {
    findings
        .findings
        .iter()
        .filter(|f| f.status.is_some())
        .count()
}

pub async fn run_verification_round(
    findings: Findings,
    code_and_context: &str,
    audit_scope: &str,
    agent: &AIAgent,
) -> Result<Findings> {
    run_round::<VerifyAllRound>(findings, code_and_context, audit_scope, agent).await
}

pub async fn run_round<T>(
    findings: Findings,
    code_and_context: &str,
    audit_scope: &str,
    agent: &AIAgent,
) -> Result<Findings>
where
    T: AnalysisRound + DeserializeOwned,
{
    // filter out all findings that got tagged on ALL previous checks
    let clean_findings = Findings {
        findings: findings
            .findings
            .iter()
            .filter(|f| f.status.is_none())
            .cloned()
            .collect::<Vec<_>>(),
    };

    if clean_findings.findings.is_empty() {
        return Ok(findings);
    }

    let verify_prompt = T::Spec::generate_verify_prompt();
    let section_10_header = prompt_index::generated_section_header(
        "AUDIT SCOPE AND KEY INVARIANTS PROVIDED BY CLIENT",
        10,
    );

    let scope = format!(
        r#"
{section_10_header}

{audit_scope}
"#
    );

    let verify_json = T::Spec::generate_verify_json();
    let post_verify_json = generate_post_round_verify_json_requirement(&verify_json);

    let instruction_prompt = generate_prompt_for_multi_finding_issue_check(
        &verify_prompt,
        &clean_findings,
        &code_and_context,
        &scope,
        &post_verify_json,
        FindingReportType::NoPoC,
    );

    save_file::save_file_locally(
        &instruction_prompt,
        &PathBuf::from("verify_finding_prompt.md"),
    )?;
    info!("Verification Round");
    let r_analysis: T = agent.extract_with_retry(&instruction_prompt).await?;

    // show analysis results
    // r_analysis
    //     .findings()
    //     .iter()
    //     .for_each(|r| r.print_analysis_results());

    let r_map: HashMap<String, &T::Spec> =
        r_analysis.findings().iter().map(|r| (r.id(), r)).collect();

    let r_findings: Vec<Finding> = findings
        .findings
        .into_iter()
        .map(|f| {
            let f_id = f.id.clone().unwrap_or_default();
            let r_option = r_map.get(&f_id);
            let (finding_status_vec, justification) = match r_option {
                Some(r) => (
                    r.get_finding_status_array_from_analysis(),
                    Some(r.get_justification()),
                ),
                None => (None, None),
            };

            let updated_status = if f.status.is_none() {
                finding_status_vec
            } else if finding_status_vec.is_none() {
                // finding passed!
                return Finding {
                    status_justification: justification,
                    ..f
                };
            } else {
                f.status
                    .clone()
                    .zip(finding_status_vec)
                    .map(|(mut a, mut b)| {
                        a.append(&mut b);
                        a
                    })
            };

            let enriched_finding = Finding {
                status: updated_status,
                status_justification: justification,
                ..f
            };
            enriched_finding
        })
        .collect();

    Ok(Findings {
        findings: r_findings,
    })
}

pub async fn run_validation_round(
    findings: Findings,
    code_and_context: &str,
    audit_scope: &str,
    repo: &RepoPaths,
) -> Result<Findings> {
    // custom agent for validation
    let validation_config = AgentConfig::new(Some(repo.clone()))
        .with_model("gpt-5.2")
        .with_preamble("You are a world-class expert at Solidity EVM smart contract auditing, and Top Code4rena Judge.")
        .with_file_retrieval(false)
        .with_openai_reasoning_effort("high");

    let validation_agent = Arc::new(AgentFactory::create_openai_agent(&validation_config)?);

    // Only validate findings that were downgraded (not Valid, not None)
    let clean_findings = Findings {
        findings: findings
            .findings
            .iter()
            .filter(|f| {
                f.status.is_some()
                    && !f
                        .status
                        .as_ref()
                        .is_some_and(|s| s.contains(&FindingStatus::Valid))
            })
            .cloned()
            .collect::<Vec<_>>(),
    };

    if clean_findings.findings.is_empty() {
        return Ok(findings);
    }

    let validation_prompt = generate_round_validation_prompt(&clean_findings);
    let section_10_header = prompt_index::generated_section_header(
        "AUDIT SCOPE AND KEY INVARIANTS PROVIDED BY CLIENT",
        10,
    );

    let scope = format!(
        r#"
{section_10_header}

**NOTE**: Only findings within the scope below are legitimate.

{audit_scope}
"#
    );

    let verify_json = generate_dynamic_validation_json(&clean_findings);

    let mut instruction_prompt = format!("{}\n\n", validation_prompt);

    instruction_prompt.push_str("\n");

    instruction_prompt.push_str(&code_and_context);
    instruction_prompt.push_str("\n\n");
    instruction_prompt.push_str(&scope);
    instruction_prompt.push_str("\n\n");
    instruction_prompt.push_str(&verify_json);

    save_file::save_file_locally(&instruction_prompt, &PathBuf::from("validation_prompt.md"))?;

    info!("Validating Verification Rounds");
    let validation_analysis: FindingDowngradeValidation = validation_agent
        .extract_with_retry(&instruction_prompt)
        .await?;

    // show analysis results
    // validation_analysis
    //     .findings
    //     .iter()
    //     .for_each(|r| r.print_analysis_results());

    let validation_map: HashMap<String, ValidateLegitAnalysis> = validation_analysis
        .findings
        .into_iter()
        .map(|r| (r.id(), r))
        .collect();

    let mut confirmed_invalid_count = 0;

    let r_validated_findings: Vec<Finding> = findings
        .findings
        .into_iter()
        .map(|f| {
            let f_id = f.id.clone().unwrap_or_default();
            let validation_analysis_option = validation_map.get(&f_id);

            // findings that are already marked as valid will not have validation analysis, skip
            // those
            if validation_analysis_option.is_none() {
                return f;
            }

            // now can safely unwrap
            let validation_analysis = validation_analysis_option.unwrap();

            let updated_finding_status = validation_analysis.get_fixed_finding_status(&f);

            // If validation returns None, all downgrade reasons were rejected -> upgrade to Valid
            let final_status = if updated_finding_status.is_none() {
                Some(vec![FindingStatus::Valid])
            } else {
                updated_finding_status
            };

            let updated_justification = if validation_analysis.justification.is_some() {
                validation_analysis.clone().justification
            } else {
                f.status_justification.clone()
            };

            let is_still_invalid = final_status.as_ref().is_some_and(|s| {
                !s.is_empty()
                    && !s.contains(&FindingStatus::Valid)
                    && !s.contains(&FindingStatus::NeedsMoreInfo)
            });

            if is_still_invalid {
                confirmed_invalid_count += 1;
            }

            let rectified_finding = Finding {
                status: final_status,
                status_justification: updated_justification,
                ..f
            };
            rectified_finding
        })
        .collect();

    info!(
        "Validation Results: {} Likely Valid,  {} Confirmed Invalid",
        r_validated_findings.len() - confirmed_invalid_count,
        confirmed_invalid_count
    );

    Ok(Findings {
        findings: r_validated_findings,
    })
}
