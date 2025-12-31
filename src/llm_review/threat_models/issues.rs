// represents abstraction of Findings and ContractInvariants

use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, TokenType},
    llm_review::{
        agent::{
            agent_enums::AIAgent,
            agent_factory::{AgentConfig, AgentFactory},
        },
        dynamic_prompts::{
            findings_template::{
                get_post_findings_json_requirement, get_post_json_requirement_for_multipattern,
            },
            inv_findings::{
                generate_invariant_to_findings, generate_multi_invariant_to_findings_prompt,
            },
            invariants::{
                self, generate_all_invariants_verify_prompt, generate_invariant_verify_prompt,
                get_post_all_invariants_verify_json,
            },
        },
        findings::findings::{Finding, Findings},
        phases::{rounds::all_rounds::AllRoundLegitAnalysis, verify_rounds::FindingAnalysis},
        prompt_support::dedup::DEDUP_PROMPT_PATTERN,
        utils::prompt_context::{self, FindingReportType},
    },
    prepare_code::git_clone::RepoPaths,
    utils::semantic_compare,
};

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::llm_review::threat_models::{
    invariants::{ContractInvariants, InvariantFinding, InvariantType},
    pattern_category::PatternCategory,
};

pub enum IssuePrompt {
    Invariant(Vec<InvariantType>),
    Combined((Vec<PatternCategory>, Option<String>, Option<String>)),
}

#[async_trait]
pub trait IssueStructTrait: Send + Sync + Sized + 'static {
    type Spec: Send + Sync + Clone + IssueTrait + DeserializeOwned;
    fn issues(&self) -> &[Self::Spec];
    fn issues_mut(&mut self) -> &mut Vec<Self::Spec>;
    fn new(issues: Vec<Self::Spec>) -> Self;
    fn generate_verify_prompt(&self) -> String;
    fn verify_json_required_prompt() -> String;
    fn multi_issue_to_findings_prompt(&self, repo: &RepoPaths) -> String;
    fn multi_issue_findings_json_required_prompt(&self, repo: &RepoPaths) -> String;
    async fn dedup(self) -> anyhow::Result<Self>;
    fn issue_title(&self) -> String;
}

#[async_trait]
pub trait IssueTrait: Send + Sync {
    fn id(&self) -> Option<String>;
    fn hash(&self) -> String;
    async fn is_duplicate_issue(&self, issue: &Self, ai_agent: &AIAgent) -> anyhow::Result<bool>;
    fn get_issue_report(&self) -> String;
    fn title_str(&self) -> String;
    fn description(&self) -> String;
    fn generate_verify_prompt(&self) -> String;
    fn pattern_to_findings_prompt(&self, repo: &RepoPaths) -> String;
    fn findings_json_required_prompt(&self, repo: &RepoPaths) -> String;
}

#[async_trait]
impl IssueTrait for InvariantFinding {
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    fn hash(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.inv_type.to_string(),
            self.contract,
            self.function,
            self.status.to_string()
        )
    }
    async fn is_duplicate_issue(&self, issue: &Self, ai_agent: &AIAgent) -> anyhow::Result<bool> {
        is_duplicate_pattern(self, issue, ai_agent).await
    }
    fn get_issue_report(&self) -> String {
        invariants::generate_formatted_invariant_finding(&self)
    }
    fn description(&self) -> String {
        self.desc.clone()
    }
    fn title_str(&self) -> String {
        format!(
            "{} - {}.{}",
            self.inv_type.to_string(),
            self.contract,
            self.function
        )
    }
    fn generate_verify_prompt(&self) -> String {
        generate_invariant_verify_prompt(&self)
    }
    fn pattern_to_findings_prompt(&self, repo: &RepoPaths) -> String {
        generate_invariant_to_findings(self, repo)
    }
    fn findings_json_required_prompt(&self, repo: &RepoPaths) -> String {
        get_post_findings_json_requirement(&self.inv_type, &self.predicate, repo)
    }
}

#[async_trait]
impl IssueTrait for Finding {
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    fn hash(&self) -> String {
        format!("{}-{}", self.contract, self.function,)
    }
    async fn is_duplicate_issue(&self, issue: &Self, ai_agent: &AIAgent) -> anyhow::Result<bool> {
        self.is_duplicate_issue(issue, ai_agent).await
    }
    fn get_issue_report(&self) -> String {
        prompt_context::get_finding_report(&self, None, FindingReportType::NoPoC)
    }
    fn title_str(&self) -> String {
        self.title.clone()
    }
    fn generate_verify_prompt(&self) -> String {
        AllRoundLegitAnalysis::generate_verify_prompt()
    }
    fn description(&self) -> String {
        self.description.clone().unwrap_or_default()
    }
    // NOTE: not needed in this case
    fn pattern_to_findings_prompt(&self, _repo: &RepoPaths) -> String {
        unimplemented!("Not implimented for Finding");
    }
    // NOTE: not needed in this case
    fn findings_json_required_prompt(&self, _repo: &RepoPaths) -> String {
        unimplemented!("Not implimented for Finding");
    }
}

#[async_trait]
impl IssueStructTrait for ContractInvariants {
    type Spec = InvariantFinding;
    fn generate_verify_prompt(&self) -> String {
        generate_all_invariants_verify_prompt(&self)
    }
    fn verify_json_required_prompt() -> String {
        get_post_all_invariants_verify_json()
    }
    fn issues(&self) -> &[InvariantFinding] {
        &self.invariants
    }
    fn issues_mut(&mut self) -> &mut Vec<InvariantFinding> {
        &mut self.invariants
    }
    fn new(issues: Vec<InvariantFinding>) -> Self {
        Self {
            invariants: issues.to_vec(),
        }
    }
    async fn dedup(self) -> anyhow::Result<Self> {
        dedup_pattern(self).await
    }
    fn issue_title(&self) -> String {
        "invariant".to_string()
    }
    fn multi_issue_to_findings_prompt(&self, repo: &RepoPaths) -> String {
        generate_multi_invariant_to_findings_prompt(&self, repo)
    }
    fn multi_issue_findings_json_required_prompt(&self, repo: &RepoPaths) -> String {
        let invariants: Vec<InvariantType> = self.issues().iter().map(|p| p.inv_type).collect();
        get_post_json_requirement_for_multipattern(&invariants, "Invariant", repo)
    }
}

#[async_trait]
impl IssueStructTrait for Findings {
    type Spec = Finding;
    fn issues(&self) -> &[Finding] {
        &self.findings
    }
    fn issues_mut(&mut self) -> &mut Vec<Finding> {
        &mut self.findings
    }
    fn new(issues: Vec<Finding>) -> Self {
        Self { findings: issues }
    }
    async fn dedup(self) -> anyhow::Result<Self> {
        self.dedup().await
    }
    fn issue_title(&self) -> String {
        "finding".to_string()
    }
    // NOTE: not need for this case
    fn multi_issue_to_findings_prompt(&self, _repo: &RepoPaths) -> String {
        unimplemented!("Not implimented for Findings");
    }
    // NOTE: not need for this case
    fn multi_issue_findings_json_required_prompt(&self, _repo: &RepoPaths) -> String {
        unimplemented!("Not implimented for Findings");
    }
    // NOTE: not need for this case
    fn generate_verify_prompt(&self) -> String {
        unimplemented!("Not implimented for Findings");
    }
    // NOTE: not need for this case
    fn verify_json_required_prompt() -> String {
        unimplemented!("Not implimented for Findings");
    }
}

pub async fn dedup_pattern<T>(patterns: T) -> anyhow::Result<T>
where
    T: IssueStructTrait,
{
    if patterns.issues().is_empty() {
        return Ok(T::new(Vec::new()));
    }

    // // Build a lightweight OpenAI agent just for deduping comparisons
    // let openai_client = openai::Client::from_env();
    // let openai_agent = Arc::new(openai_client.agent("gpt-5").build());
    let openai_config = AgentConfig::new(None).with_model("gpt-5-mini");

    let openai_agent = Arc::new(AgentFactory::create_openai_agent(&openai_config)?);

    let mut findings_hash: HashMap<String, Vec<T::Spec>> = HashMap::new();

    for pattern in patterns.issues() {
        let hash = pattern.hash();
        findings_hash
            .entry(hash)
            .or_insert_with(Vec::new)
            .push(pattern.clone());
    }

    let arc_dedup_findings = Arc::new(Mutex::new(Vec::<T::Spec>::with_capacity(
        patterns.issues().len(),
    )));
    let mut handles = Vec::new();

    for findings in findings_hash.into_values() {
        let arc_findings = Arc::new(findings);
        let current_findings = Arc::clone(&arc_findings);
        let deduped_findings = Arc::clone(&arc_dedup_findings);
        let agent = Arc::clone(&openai_agent);
        let handle = tokio::spawn(async move {
            if current_findings.len() > 1 {
                match get_deduped_patterns_vec(&current_findings, &agent).await {
                    Ok(deduped) => {
                        let mut deduped_findings_lock = deduped_findings.lock().await;
                        deduped_findings_lock.extend(deduped);
                    }
                    Err(e) => {
                        log::error!("❌ deduping findngs failed: {e}");
                    }
                }
            } else {
                let mut deduped_findings_lock = deduped_findings.lock().await;
                deduped_findings_lock.extend(current_findings.iter().cloned())
            }
        });
        handles.push(handle);
    }

    // CRITICAL: Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }
    // Extract the Vec from Arc<Mutex<Vec<...>>>
    let deduped_findings = Arc::try_unwrap(arc_dedup_findings)
        .map_err(|_| anyhow::anyhow!("Failed to unwrap Arc"))?
        .into_inner();

    Ok(T::new(deduped_findings))
}

async fn get_deduped_patterns_vec<TSpec>(
    patterns: &Arc<Vec<TSpec>>,
    agent: &Arc<AIAgent>,
) -> anyhow::Result<Vec<TSpec>>
where
    TSpec: 'static + Send + Sync + Clone + IssueTrait + DeserializeOwned,
{
    // assigns bool to each finding index, is dup or not? assume not for initializing
    let size = patterns.len();
    let mut is_dup_vec: Vec<bool> = vec![false; size];

    for i in 0..size {
        if is_dup_vec[i] {
            continue;
        }
        for j in i + 1..size {
            if is_dup_vec[j] {
                continue;
            }
            let is_dup = patterns[i].is_duplicate_issue(&patterns[j], &agent).await?;
            if is_dup {
                is_dup_vec[j] = true;
                continue;
            }
        }
    }

    let findings: Vec<TSpec> = patterns
        .iter()
        .enumerate()
        .filter(|(idx, _)| !is_dup_vec[*idx])
        .map(|(_, f)| f)
        .cloned()
        .collect();

    Ok(findings)
}

async fn is_duplicate_pattern<T>(
    pattern: &T,
    issue: &T,
    ai_agent: &crate::llm_review::agent::agent_enums::AIAgent,
) -> anyhow::Result<bool>
where
    T: IssueTrait,
{
    let similiarity_score =
        semantic_compare::similarity_score(&pattern.description(), &issue.description());
    // contract, function and issue type MUST match
    if issue.hash() != pattern.hash() || similiarity_score <= 0.25 {
        // log::info!(
        //     "issue: {} \n pattern: {} \n ==> DIFFERENT",
        //     issue.description(),
        //     pattern.description()
        // );
        return Ok(false);
    } else if similiarity_score >= 0.65 || issue.description() == pattern.description() {
        // log::info!(
        //     "issue: {} \n pattern: {} \n ==> SAME",
        //     issue.description(),
        //     pattern.description()
        // );

        return Ok(true);
    }

    let prompt = DEDUP_PROMPT_PATTERN
        .replace("{report_a}", &pattern.get_issue_report())
        .replace("{report_b}", &issue.get_issue_report());

    log::info!("checking if is {} duplication", pattern.title_str());
    add_to_inference_cost_by_type(&prompt, &ai_agent.get_metadata(), TokenType::Input).await;

    let response = ai_agent.prompt(&prompt).await?;

    add_to_inference_cost_by_type(&response, &ai_agent.get_metadata(), TokenType::Output).await;

    Ok(response.trim().eq_ignore_ascii_case("YES"))
}
