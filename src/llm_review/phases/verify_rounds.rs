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
use serde::{Deserialize, Serialize};
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
    InvalidGoveranaceRisk,
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
}

pub trait FindingAnalysis {
    fn get_finding_status_array_from_analysis(&self) -> Vec<FindingStatus>;
    fn print_analysis_results(&self);
    fn generate_verify_prompt() -> String;
    fn generate_verify_json() -> String;
}

impl AnalysisRound for VerifyRoundOne {
    type Spec = RoundOneLegitAnalysis;
}

impl AnalysisRound for VerifyRoundTwo {
    type Spec = RoundTwoLegitAnalysis;
}

impl AnalysisRound for VerifyRoundThree {
    type Spec = RoundThreeLegitAnalysis;
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

    let verify_r1_prompt = RoundOneLegitAnalysis::generate_verify_prompt();

    let r1_prompt = if audit_scope.is_empty() {
        verify_r1_prompt.to_string()
    } else {
        format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &verify_r1_prompt, &audit_scope
        )
    };

    // ROUND 1 of VERIFICATON
    let verify_json = RoundOneLegitAnalysis::generate_verify_json();
    let post_verify_json = generate_post_round_verify_json_requirement(&verify_json);

    let instruction_prompt = generate_prompt_for_multi_finding_issue_check(
        &code_and_context,
        &deduped_findings,
        &r1_prompt,
        &post_verify_json,
        FindingReportType::NoPoC,
    );

    info!("Round 1 of Verification");
    let r1_analysis: VerifyRoundOne = agent.extract_with_retry(&instruction_prompt).await?;

    r1_analysis
        .findings
        .iter()
        .for_each(|r| r.print_analysis_results());

    let r1_findings: Vec<Finding> = deduped_findings
        .findings
        .iter()
        .map(|f| {
            let r1_option = r1_analysis.findings.iter().find(|r| r.finding_id == f.id);
            let finding_status_vec = match r1_option {
                Some(r1) => Some(r1.get_finding_status_array_from_analysis()),
                None => None,
            };

            let enriched_finding = Finding {
                status: finding_status_vec,
                ..f.clone()
            };
            enriched_finding
        })
        .collect();

    // filter out all findings that passed ALL r1 checks
    let mut clean_findings = Findings {
        findings: r1_findings
            .clone()
            .into_iter()
            .filter(|f| f.status == None)
            .collect::<Vec<_>>(),
    };

    //************************
    // ROUND 2 of VERIFICATON
    //************************
    let verify_r2_prompt = RoundTwoLegitAnalysis::generate_verify_prompt();

    let r2_prompt = if audit_scope.is_empty() {
        verify_r2_prompt.to_string()
    } else {
        format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &verify_r2_prompt, &audit_scope
        )
    };

    let verify_json = RoundTwoLegitAnalysis::generate_verify_json();
    let post_verify_json = generate_post_round_verify_json_requirement(&verify_json);

    let instruction_prompt = generate_prompt_for_multi_finding_issue_check(
        &code_and_context,
        &clean_findings,
        &r2_prompt,
        &post_verify_json,
        FindingReportType::NoPoC,
    );

    info!("Round 2 of Verification");
    let r2_analysis: VerifyRoundTwo = agent.extract_with_retry(&instruction_prompt).await?;

    // print results
    r2_analysis
        .findings
        .iter()
        .for_each(|r| r.print_analysis_results());

    let r2_findings: Vec<Finding> = r1_findings
        .into_iter()
        .map(|f| {
            let r2_option = r2_analysis.findings.iter().find(|r| r.finding_id == f.id);
            let finding_status_vec = match r2_option {
                Some(r2) => Some(r2.get_finding_status_array_from_analysis()),
                None => None,
            };

            let enriched_finding = Finding {
                status: finding_status_vec,
                ..f.clone()
            };
            enriched_finding
        })
        .collect();

    clean_findings = Findings {
        findings: r2_findings
            .clone()
            .into_iter()
            .filter(|f| f.status == None)
            .collect::<Vec<_>>(),
    };

    // ROUND 3 of VERIFICATON
    let verify_r3_prompt = RoundThreeLegitAnalysis::generate_verify_prompt();

    let r3_prompt = if audit_scope.is_empty() {
        verify_r3_prompt.to_string()
    } else {
        format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &verify_r3_prompt, &audit_scope
        )
    };

    let verify_json = RoundThreeLegitAnalysis::generate_verify_json();
    let post_verify_json = generate_post_round_verify_json_requirement(&verify_json);

    let instruction_prompt = generate_prompt_for_multi_finding_issue_check(
        &code_and_context,
        &clean_findings,
        &r3_prompt,
        &post_verify_json,
        FindingReportType::NoPoC,
    );

    info!("Round 3 of Verification");
    let r3_analysis: VerifyRoundThree = agent.extract_with_retry(&instruction_prompt).await?;

    r3_analysis
        .findings
        .iter()
        .for_each(|r| r.print_analysis_results());

    let r3_findings: Vec<Finding> = r2_findings
        .into_iter()
        .map(|f| {
            let r3_option = r3_analysis.findings.iter().find(|r| r.finding_id == f.id);
            let finding_status_vec = match r3_option {
                Some(r3) => Some(r3.get_finding_status_array_from_analysis()),
                None => None,
            };

            let enriched_finding = Finding {
                status: finding_status_vec,
                ..f.clone()
            };
            enriched_finding
        })
        .collect();

    let verified_findings: Vec<Finding> = r3_findings
        .into_iter()
        .map(|f| {
            if f.status == None {
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
