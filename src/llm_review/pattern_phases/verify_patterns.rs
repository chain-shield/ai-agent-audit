use crate::llm_review::pattern_phases::generate_patterns;
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::context_state::get_metadata_context,
        threat_models::{
            invariants::ContractInvariants,
            issues::{IssueStructTrait, IssueTrait},
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub trait IsLegit {
    fn id(&self) -> String;
    fn is_legit(&self) -> bool;
    fn get_justification(&self) -> String;
}

pub trait InvariantAnalysis {
    type Spec: IsLegit;
    fn findings(&self) -> &[Self::Spec];
}

/// Verification result for a potential pattern
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitInvariant {
    pub invariant_id: String,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_invariant_valid: bool,
    pub is_invariant_violated: Option<bool>,
    pub why_its_not_valid: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VerifyInvariants {
    pub findings: Vec<LegitInvariant>,
}

impl InvariantAnalysis for VerifyInvariants {
    type Spec = LegitInvariant;
    fn findings(&self) -> &[Self::Spec] {
        &self.findings
    }
}

impl IsLegit for LegitInvariant {
    fn id(&self) -> String {
        self.invariant_id.clone()
    }
    fn is_legit(&self) -> bool {
        self.is_invariant_valid
    }
    fn get_justification(&self) -> String {
        self.why_its_not_valid.clone().unwrap_or_default()
    }
}

pub async fn verify_invariants(
    patterns: ContractInvariants,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<ContractInvariants> {
    execute::<ContractInvariants, VerifyInvariants>(patterns, code, agent, repo).await
}

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute<T, M>(
    patterns: T,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<T>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
    <T as IssueStructTrait>::Spec: Send + Sync + Clone + DeserializeOwned + IssueTrait + 'static,
    M: Clone + DeserializeOwned + JsonSchema + InvariantAnalysis + Send + Sync,
    <M as InvariantAnalysis>::Spec: Send + Sync + IsLegit + JsonSchema,
{
    let issue_title = patterns.issue_title();
    info!("🔍 Phase 2: Deduplicating and verifying {}...", issue_title);

    let deduped_patterns: T = patterns.dedup().await?;
    let dedup_pattern_count = deduped_patterns.issues().len();

    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_patterns::generate_content_plus_context_block(code, &context);

    info!(
        "# of {} AFTER deduping => {}",
        &deduped_patterns.issue_title(),
        dedup_pattern_count
    );
    info!("now verifying each {}...", &deduped_patterns.issue_title());

    let verify_prompt = deduped_patterns.generate_verify_prompt();
    let post_verify_json = T::verify_json_required_prompt();

    let full_prompt = format!("{}{}{}", verify_prompt, code_and_context, post_verify_json);

    info!(
        "Verifying {} {} in batch...",
        dedup_pattern_count,
        deduped_patterns.issue_title()
    );
    let invariant_analysis: M = agent.extract_with_retry(&full_prompt).await?;

    let r_map: HashMap<String, &M::Spec> = invariant_analysis
        .findings()
        .iter()
        .map(|inv| (inv.id(), inv))
        .collect();

    let verified_patterns: Vec<<T as IssueStructTrait>::Spec> = deduped_patterns
        .issues()
        .iter()
        .filter(|p| {
            let p_id = p.id().unwrap_or_default();
            let r_option = r_map.get(&p_id);
            match r_option {
                Some(r) => {
                    if !r.is_legit() {
                        info!("Invariant is Invalid: {}", r.get_justification());
                        return false;
                    } else {
                        return true;
                    }
                }
                None => {
                    log::warn!(
                        "Invariant '{}' (ID: {}) was not verified by LLM - filtering out",
                        p.title_str(),
                        p_id
                    );
                    return false;
                }
            }
        })
        .cloned()
        .collect();

    info!(
        "✅ Phase 2 complete: {} Verified {}!",
        verified_patterns.len(),
        deduped_patterns.issue_title()
    );

    Ok(T::new(verified_patterns))
}
