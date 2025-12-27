use crate::llm_review::agent::agent_factory::{AgentConfig, AgentFactory};
use crate::llm_review::pattern_phases::pattern_to_findings::generate_content_plus_context_block;
use crate::llm_review::phases::rounds::all_rounds::{AllRoundLegitAnalysis, VerifyAllRound};
use crate::llm_review::phases::rounds::round_1::{RoundOneLegitAnalysis, VerifyRoundOne};
use crate::llm_review::phases::rounds::round_2::{RoundTwoLegitAnalysis, VerifyRoundTwo};
use crate::llm_review::phases::rounds::round_3::{RoundThreeLegitAnalysis, VerifyRoundThree};
use crate::llm_review::phases::rounds::utils::generate_post_round_verify_json_requirement;
use crate::llm_review::phases::rounds::validate_round::{
    generate_dynamic_validation_json, generate_round_validation_prompt, FindingDowngradeValidation,
    ValidateLegitAnalysis,
};
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

const RUN_SINGLE_ROUND: bool = true;

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
    fn round_number() -> usize;
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
    actors_capabilities: Option<Arc<String>>,
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
    // ALL ROUND VERIFICATON
    //************************

    let all_round_findings = run_all_round(
        deduped_findings,
        &code_and_context,
        actors_capabilities.clone(),
        &audit_scope,
        agent,
    )
    .await?;
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

    let verified_findings = run_round_validation(
        labeled_findings,
        &code_and_context,
        actors_capabilities,
        &audit_scope,
        repo,
    )
    .await?;

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

pub async fn run_all_round(
    findings: Findings,
    code_and_context: &str,
    actors_capabilities: Option<Arc<String>>,
    audit_scope: &str,
    agent: &AIAgent,
) -> Result<Findings> {
    run_round::<VerifyAllRound>(
        findings,
        code_and_context,
        actors_capabilities,
        audit_scope,
        agent,
    )
    .await
}

pub async fn run_round<T>(
    findings: Findings,
    code_and_context: &str,
    actors_capabilities: Option<Arc<String>>,
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

    let r_prompt = if audit_scope.is_empty() {
        verify_prompt.to_string()
    } else {
        format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT\n\n{}",
            &verify_prompt, &audit_scope
        )
    };

    let verify_json = T::Spec::generate_verify_json();
    let post_verify_json = generate_post_round_verify_json_requirement(&verify_json);

    let instruction_prompt = generate_prompt_for_multi_finding_issue_check(
        &code_and_context,
        &clean_findings,
        &r_prompt,
        actors_capabilities,
        &post_verify_json,
        FindingReportType::NoPoC,
    );

    info!("Round {} of Verification", T::Spec::round_number());
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
                    verification_rounds_passed: Some(T::Spec::round_number() as u8),
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

pub async fn run_round_validation(
    findings: Findings,
    code_and_context: &str,
    actors_capabilities: Option<Arc<String>>,
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

    let validation_prompt = generate_round_validation_prompt(&findings);

    let main_instructions = if audit_scope.is_empty() {
        validation_prompt.to_string()
    } else {
        format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &validation_prompt, &audit_scope
        )
    };

    let verify_json = generate_dynamic_validation_json(&clean_findings);

    let mut instruction_prompt = format!("{}\n\n", main_instructions);

    if let Some(actors) = actors_capabilities {
        instruction_prompt.push_str("\n\n");
        instruction_prompt.push_str(&format!("## POTENTIAL BAD ACTORS TO CONSIDER WHEN VERIFYING SECURITY VULNERABILITIES\n
                 **NOTE**: The actors below are pertinent to the codebase where vulnerability were found, please incorporate them in your verification analysis\n\n
                {}",actors));
        instruction_prompt.push_str("\n\n");
    }

    instruction_prompt.push_str("## CODEBASE WHERE FINDINGS WERE FOUND");
    instruction_prompt.push_str("\n\n");

    instruction_prompt.push_str(&code_and_context);
    instruction_prompt.push_str("\n\n");
    instruction_prompt.push_str(&verify_json);

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

            // If validation returns None, all downgrade reasons were rejected -> upgrade to Valid or NeedsMoreInfo
            // NOTE: only set as Valid if passed 2+ rounds (high confidence), otherwise set as NeedsMoreInfo
            let final_status = if updated_finding_status.is_none() {
                // If it passed 2 rounds, it means it failed Round 3, but that failure was overturned
                // in the final validation round, therefore the finding is now Valid.
                // If it passed fewer than 2 rounds, it needs human review (NeedsMoreInfo).
                // Note: Some(3) won't appear here because those findings are already marked Valid
                // and filtered out before validation.
                if RUN_SINGLE_ROUND {
                    Some(vec![FindingStatus::Valid])
                } else {
                    if f.verification_rounds_passed == Some(2)
                        || f.verification_rounds_passed == Some(3)
                    {
                        Some(vec![FindingStatus::Valid])
                    } else {
                        Some(vec![FindingStatus::NeedsMoreInfo])
                    }
                }
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
