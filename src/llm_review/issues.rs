// represents abstraction of Findings and ContractInvariants

use crate::{
    cost::cost_data::{TokenType, add_to_inference_cost_by_type},
    llm_review::{
        agent_factory::{AgentConfig, AgentFactory},
        dynamic_prompts::{
            findings_template::get_findings_json_requirement,
            inv_findings::generate_invariant_to_findings,
            invariants::generate_invariant_verify_prompt,
            pattern_findings::generate_pattern_to_findings_prompt,
            patterns::generate_pattern_verify_prompt,
        },
        enums::{AIAgent, EnumString},
        prompt_support::dedup::DEDUP_PROMPT_PATTERN,
        utils::prompt_context::{generate_formatted_invariant_finding, generate_formatted_pattern},
    },
    utils::semantic_compare,
};

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::llm_review::{
    invariants::{ContractInvariants, InvariantFinding, InvariantType},
    pattern_category::PatternCategory,
    patterns::{Pattern, Patterns},
};

pub enum IssuePrompt {
    Pattern(Vec<PatternCategory>),
    Invariant(Vec<InvariantType>),
}

#[async_trait]
pub trait IssueStructTrait: Send + Sync + Sized + 'static {
    type Spec: Send + Sync + Clone + IssueTrait + DeserializeOwned;
    fn issues(&self) -> &[Self::Spec];
    fn issues_mut(&mut self) -> &mut Vec<Self::Spec>;
    fn new(issues: Vec<Self::Spec>) -> Self;
    async fn dedup(self) -> anyhow::Result<Self>;
    fn issue_title(&self) -> String;
}

#[async_trait]
pub trait IssueTrait: Send + Sync {
    fn hash(&self) -> String;
    async fn is_duplicate_issue(&self, issue: &Self, ai_agent: &AIAgent) -> anyhow::Result<bool>;
    fn get_issue_report(&self) -> String;
    fn title_str(&self) -> String;
    fn description(&self) -> String;
    fn generate_verify_prompt(&self) -> String;
    fn pattern_to_findings_prompt(&self) -> String;
    fn findings_json_required_prompt(&self) -> String;
}

#[async_trait]
impl IssueTrait for InvariantFinding {
    fn hash(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.inv_type.as_str(),
            self.contract,
            self.function,
            self.status.as_str()
        )
    }
    async fn is_duplicate_issue(&self, issue: &Self, ai_agent: &AIAgent) -> anyhow::Result<bool> {
        is_duplicate_pattern(self, issue, ai_agent).await
    }
    fn get_issue_report(&self) -> String {
        generate_formatted_invariant_finding(&self)
    }
    fn description(&self) -> String {
        self.desc.clone()
    }
    fn title_str(&self) -> String {
        format!(
            "{} - {}.{}",
            self.inv_type.as_str(),
            self.contract,
            self.function
        )
    }
    fn generate_verify_prompt(&self) -> String {
        generate_invariant_verify_prompt(&self)
    }
    fn pattern_to_findings_prompt(&self) -> String {
        generate_invariant_to_findings(self)
    }
    fn findings_json_required_prompt(&self) -> String {
        get_findings_json_requirement(&self.inv_type, &self.predicate)
    }
}

#[async_trait]
impl IssueTrait for Pattern {
    fn hash(&self) -> String {
        format!(
            "{}-{}-{}",
            self.issue_type.as_str(),
            self.contract,
            self.function,
        )
    }
    async fn is_duplicate_issue(&self, issue: &Self, ai_agent: &AIAgent) -> anyhow::Result<bool> {
        is_duplicate_pattern(self, issue, ai_agent).await
    }
    fn get_issue_report(&self) -> String {
        generate_formatted_pattern(&self)
    }
    fn title_str(&self) -> String {
        format!(
            "{} - {}.{}",
            self.issue_type.as_str(),
            self.contract,
            self.function
        )
    }
    fn generate_verify_prompt(&self) -> String {
        generate_pattern_verify_prompt(&self)
    }
    fn description(&self) -> String {
        self.description.clone()
    }
    fn pattern_to_findings_prompt(&self) -> String {
        generate_pattern_to_findings_prompt(self)
    }
    fn findings_json_required_prompt(&self) -> String {
        get_findings_json_requirement(&self.issue_type, &self.title)
    }
}

#[async_trait]
impl IssueStructTrait for ContractInvariants {
    type Spec = InvariantFinding;
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
}

#[async_trait]
impl IssueStructTrait for Patterns {
    type Spec = Pattern;
    fn issues(&self) -> &[Pattern] {
        &self.patterns
    }
    fn issues_mut(&mut self) -> &mut Vec<Pattern> {
        &mut self.patterns
    }
    fn new(issues: Vec<Pattern>) -> Self {
        Self { patterns: issues }
    }
    async fn dedup(self) -> anyhow::Result<Self> {
        dedup_pattern(self).await
    }
    fn issue_title(&self) -> String {
        "pattern".to_string()
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
    // Create O3 agent with flex tier for cost optimization
    let openai_config = AgentConfig::new(None)
        .with_model("o3")
        .with_openai_service_tier("flex");

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
    ai_agent: &crate::llm_review::enums::AIAgent,
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
