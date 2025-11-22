/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::{context_state::get_metadata_context, semaphore::GENERAL_SEM},
        threat_models::{
            actors::ActorAbuses,
            invariants::ContractInvariants,
            issues::{IssueStructTrait, IssueTrait},
            patterns::Patterns,
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Verification result for a potential pattern
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitPattern {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_legit_pattern: bool,
    pub why_its_not_legit: Option<String>,
}

/// Verification result for a potential pattern
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitInvariant {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_legit_invariant: bool,
    pub why_its_not_legit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct LegitActorMalice {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    is_legit_abuse: bool,
    why_its_not_legit: Option<String>,
}

pub trait IsLegit {
    fn is_legit(&self) -> bool;
    fn why_not_legit(&self) -> String;
}

impl IsLegit for LegitActorMalice {
    fn is_legit(&self) -> bool {
        self.is_legit_abuse
    }
    fn why_not_legit(&self) -> String {
        self.why_its_not_legit.clone().unwrap_or_default()
    }
}

impl IsLegit for LegitInvariant {
    fn is_legit(&self) -> bool {
        self.is_legit_invariant
    }
    fn why_not_legit(&self) -> String {
        self.why_its_not_legit.clone().unwrap_or_default()
    }
}

impl IsLegit for LegitPattern {
    fn is_legit(&self) -> bool {
        self.is_legit_pattern
    }
    fn why_not_legit(&self) -> String {
        self.why_its_not_legit.clone().unwrap_or_default()
    }
}

pub async fn verify_patterns(
    patterns: Patterns,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Patterns> {
    execute::<Patterns, LegitPattern>(patterns, code, agent, repo).await
}

pub async fn verify_invariants(
    patterns: ContractInvariants,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<ContractInvariants> {
    execute::<ContractInvariants, LegitInvariant>(patterns, code, agent, repo).await
}

pub async fn verify_actor_abuses(
    patterns: ActorAbuses,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<ActorAbuses> {
    execute::<ActorAbuses, LegitActorMalice>(patterns, code, agent, repo).await
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
    M: Clone + DeserializeOwned + JsonSchema + IsLegit + Send + Sync,
{
    let issue_title = patterns.issue_title();
    info!("🔍 Phase 2: Deduplicating and verifying {}...", issue_title);

    let mut handles = vec![];
    let deduped_patterns: Arc<T> = Arc::new(patterns.dedup().await?);
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);

    let dedup_pattern_count = deduped_patterns.issues().len();
    let is_legit_pattern_vec: Arc<Mutex<Vec<bool>>> =
        Arc::new(Mutex::new(vec![true; dedup_pattern_count]));

    info!(
        "# of {} AFTER deduping => {}",
        &deduped_patterns.issue_title(),
        dedup_pattern_count
    );
    info!("now verifying each {}...", &deduped_patterns.issue_title());

    for i in 0..dedup_pattern_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_patterns = Arc::clone(&deduped_patterns);
        let arc_agent = Arc::clone(&agent);
        let arc_legit_patterns_vec = Arc::clone(&is_legit_pattern_vec);
        let sem = Arc::clone(&GENERAL_SEM);
        let title = issue_title.clone();

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                let instruction_prompt = arc_patterns.issues()[i].generate_verify_prompt();

                // info!("verify instruction prompt => {}", instruction_prompt);

                let full_prompt = format!("{}{}", instruction_prompt, codeblock_plus_context);

                // add to cost
                info!("verifying {} #{}", title, i + 1);
                let is_legit_struct: M = arc_agent.extract_with_retry(&full_prompt).await?;

                let is_finding_legit = is_legit_struct.is_legit();
                if !is_finding_legit {
                    info!(
                        "{} is NOT legit => {}",
                        arc_patterns.issues()[i].title_str(),
                        is_legit_struct.why_not_legit()
                    );
                }
                let mut legit_patterns_vec = arc_legit_patterns_vec.lock().await;
                legit_patterns_vec[i] = is_finding_legit;

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying {} {}: {:?}", title, i, e);
            }
        }));
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let legit_findings_vec = is_legit_pattern_vec.lock().await;
    let verified_patterns: Vec<<T as IssueStructTrait>::Spec> = deduped_patterns
        .issues()
        .iter()
        .enumerate()
        .filter(|(idx, _)| legit_findings_vec[*idx])
        .map(|(_, f)| f.clone())
        .collect();

    info!(
        "✅ Phase 2 complete: {} Verified {}!",
        verified_patterns.len(),
        deduped_patterns.issue_title()
    );

    Ok(T::new(verified_patterns))
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
