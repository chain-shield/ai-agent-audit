use crate::llm_review::pattern_phases::pattern_to_findings::generate_content_plus_context_block;
use crate::llm_review::phases::rounds::round_1::{RoundOneLegitAnalysis, VerifyRoundOne};
use crate::llm_review::phases::rounds::round_2::{RoundTwoLegitAnalysis, VerifyRoundTwo};
use crate::llm_review::phases::rounds::round_3::{RoundThreeLegitAnalysis, VerifyRoundThree};
use crate::llm_review::phases::rounds::utils::generate_post_round_verify_json_requirement;
/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::llm_review::utils::prompt_context::generate_prompt_for_multi_finding_issue_check;
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

impl AnalysisRound for VerifyRoundOne {
    type Spec = RoundOneLegitAnalysis;
    fn findings(&self) -> &[Self::Spec] {
        &self.findings
    }
}

impl AnalysisRound for VerifyRoundTwo {
    type Spec = RoundTwoLegitAnalysis;
    fn findings(&self) -> &[Self::Spec] {
        &self.findings
    }
}

impl AnalysisRound for VerifyRoundThree {
    type Spec = RoundThreeLegitAnalysis;
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

    let code_and_context = generate_content_plus_context_block(code, &context);

    let dedup_finding_count = deduped_findings.findings.len();

    info!("# of findings AFTER deduping => {}", dedup_finding_count);
    info!("now verifying each finding...");

    let audit_scope = generate_audit_scope(repo).await?;

    //************************
    // ROUND 1 of VERIFICATON
    //************************
    let r1_findings = run_round_1(deduped_findings, &code_and_context, &audit_scope, agent).await?;
    info!(
        "{} finding tagged as low or invalid",
        tagged_findings(&r1_findings)
    );

    //************************
    // ROUND 2 of VERIFICATON
    //************************
    let r2_findings = run_round_2(r1_findings, &code_and_context, &audit_scope, agent).await?;
    info!(
        "{} finding tagged as low or invalid",
        tagged_findings(&r2_findings)
    );

    //************************
    // ROUND 3 of VERIFICATON
    //************************
    let r3_findings = run_round_3(r2_findings, &code_and_context, &audit_scope, agent).await?;
    info!(
        "{} finding tagged as low or invalid",
        tagged_findings(&r3_findings)
    );

    let verified_findings: Vec<Finding> = r3_findings
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

    info!(
        "✅ Phase 4 complete: {} Validated Findings!",
        verified_findings
            .iter()
            .filter(|f| f
                .status
                .as_ref()
                .is_some_and(|s| s.contains(&FindingStatus::Valid)))
            .count()
    );

    Ok(Findings {
        findings: verified_findings,
    })
}

pub fn tagged_findings(findings: &Findings) -> usize {
    findings
        .findings
        .iter()
        .filter(|f| f.status.is_some())
        .count()
}

pub async fn run_round_1(
    findings: Findings,
    code_and_context: &str,
    audit_scope: &str,
    agent: &AIAgent,
) -> Result<Findings> {
    run_round::<VerifyRoundOne>(1, findings, code_and_context, audit_scope, agent).await
}

pub async fn run_round_2(
    findings: Findings,
    code_and_context: &str,
    audit_scope: &str,
    agent: &AIAgent,
) -> Result<Findings> {
    run_round::<VerifyRoundTwo>(2, findings, code_and_context, audit_scope, agent).await
}

pub async fn run_round_3(
    findings: Findings,
    code_and_context: &str,
    audit_scope: &str,
    agent: &AIAgent,
) -> Result<Findings> {
    run_round::<VerifyRoundThree>(3, findings, code_and_context, audit_scope, agent).await
}

pub async fn run_round<T>(
    round_number: usize,
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

    let verify_prompt = T::Spec::generate_verify_prompt();

    let r_prompt = if audit_scope.is_empty() {
        verify_prompt.to_string()
    } else {
        format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &verify_prompt, &audit_scope
        )
    };

    let verify_json = T::Spec::generate_verify_json();
    let post_verify_json = generate_post_round_verify_json_requirement(&verify_json);

    let instruction_prompt = generate_prompt_for_multi_finding_issue_check(
        &code_and_context,
        &clean_findings,
        &r_prompt,
        &post_verify_json,
        FindingReportType::NoPoC,
    );

    info!("Round {} of Verification", round_number);
    let r_analysis: T = agent.extract_with_retry(&instruction_prompt).await?;

    r_analysis
        .findings()
        .iter()
        .for_each(|r| r.print_analysis_results());

    let r_map: HashMap<String, &T::Spec> =
        r_analysis.findings().iter().map(|r| (r.id(), r)).collect();

    let r_findings: Vec<Finding> = findings
        .findings
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            let f_id = f.id.clone().unwrap_or_default();
            let r_option = r_map.get(&f_id);
            let (finding_status_vec, justification) = match r_option {
                Some(r) => (
                    r.get_finding_status_array_from_analysis(),
                    Some(r.get_justification()),
                ),
                None => {
                    // Log warning if LLM didn't return analysis for this finding
                    log::warn!(
                        "Round {}: LLM did not return analysis for finding #{} (id: {}, title: {})",
                        round_number,
                        idx + 1,
                        f_id,
                        f.title
                    );
                    (None, None)
                }
            };

            // Properly concatenate justifications from all rounds
            let updated_justification = match (&f.status_justification, justification) {
                (Some(existing), Some(new)) => Some(format!(
                    "{}\n\n--- Round {} ---\n{}",
                    existing, round_number, new
                )),
                (Some(existing), None) => Some(existing.clone()),
                (None, Some(new)) => Some(format!("--- Round {} ---\n{}", round_number, new)),
                (None, None) => None,
            };

            let updated_status = if f.status.is_none() {
                finding_status_vec
            } else if finding_status_vec.is_none() {
                f.status.clone()
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
                status_justification: updated_justification,
                ..f.clone()
            };
            enriched_finding
        })
        .collect();

    Ok(Findings {
        findings: r_findings,
    })
}
