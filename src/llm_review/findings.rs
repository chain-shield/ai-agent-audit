use super::{
    agent_factory::{AgentConfig, AgentFactory},
    enums::{Severity, VulnerabilityType},
    prompt_support::dedup::DEDUP_PROMPT,
};
use crate::{
    cost::cost_data::{add_to_inference_cost_by_type, TokenType},
    llm_review::{
        enums::EnumString,
        phases::verify_findings::{FindingConfidence, FindingStatus},
    },
    utils::semantic_compare,
};
use regex::Regex;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use strum_macros::EnumIter;
use tokio::sync::Mutex;

pub const CLAUDE_4_0_SONNET: &str = "claude-sonnet-4-0";
pub const CLAUDE_4_5_SONNET: &str = "claude-sonnet-4-5";
pub const CLAUDE_4_OPUS: &str = "claude-opus-4-0";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Finding {
    // [Severity-issue number] - List Issue (Reentrancy, Denial of Service, etc) and
    // <Contract>::<Function> its localed in
    pub derived_from: Option<String>,
    pub title: String,
    pub exploit_type: VulnerabilityType,
    pub privilege: PrivilegeLevel,   // permissionless vs role-gated
    pub contract: String,            // exact constract name where issue appears
    pub function: String, // exact function name where issue appears, if not applicable set to 'NA'
    pub description: Option<String>, // description of issue, include code snippet if relevant
    pub impact: Option<String>, // Impact of Issue
    pub proof_of_concept: Option<String>, // Demonstrate how issue can be exploited by hacker
    pub proof_of_code: Option<String>, // Write Foundry Unit test to prove issue exists
    #[schemars(description = "Severity level: Critical, High, Medium, Low, Info")]
    pub severity: Severity, //severity of issue
    pub mitigation: Option<String>,
    pub severity_justification: Option<String>,
    pub status: Option<FindingStatus>,
    pub status_justification: Option<String>,
    pub status_confidence: Option<FindingConfidence>,
    pub status_confidence_justification: Option<String>,
    pub finding_complexity: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Findings {
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, EnumIter)]
#[serde(rename_all = "PascalCase")]
pub enum PrivilegeLevel {
    Permissionless,   // any EOA
    RequiresRole,     // specific role
    RequireAdminRole, // admin or owner
}

pub fn generated_llm_prompt(
    contract_name: &str,
    main_instructions: &str,
    pre: &str,
    post: &str,
) -> String {
    let instruction_template = format!("{}{}{}", pre, main_instructions, post);

    // populate template
    instruction_template
        .replace("{contract_name}", contract_name)
        .to_string()
}

impl Finding {
    pub fn free_report_title(&self) -> String {
        if self.severity == Severity::Critical
            || self.severity == Severity::High
            || self.severity == Severity::Medium
        {
            format!(
                "{} issue found with {} severity",
                self.exploit_type.as_fancy_str(),
                self.severity.as_str()
            )
        } else {
            format!(
                "{} issue in {}::{}",
                self.exploit_type.as_fancy_str(),
                self.contract,
                self.function
            )
        }
    }

    pub fn hash(&self) -> String {
        // extract name 'func_name' from func_name(...)
        let fn_name = self.get_fn_name();
        format!("{}-{}", self.contract, fn_name)
    }

    pub fn hash_derived(&self) -> String {
        // extract name 'func_name' from func_name(...)
        let fn_name = self.get_fn_name();

        if let Some(derived) = self.derived_from.clone() {
            format!("{}-{}-{}", derived.to_string(), self.contract, fn_name)
        } else {
            format!(
                "{}-{}-{}",
                self.exploit_type.as_str(),
                self.contract,
                fn_name
            )
        }
    }

    // extract name 'func_name' from func_name(...)
    pub fn get_fn_name(&self) -> String {
        let re = Regex::new(r"(^[a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
        if let Some(captures) = re.captures(&self.function) {
            match captures.get(1) {
                Some(name) => name.as_str().to_string(),
                None => self.function.clone(),
            }
        } else {
            self.function.clone()
        }
    }

    // resonse "YES" or "NO"
    pub async fn is_duplicate_issue(
        &self,
        issue: &Finding,
        ai_agent: &crate::llm_review::enums::AIAgent,
    ) -> anyhow::Result<bool> {
        let title_similiarity_score = semantic_compare::similarity_score(&self.title, &issue.title);
        let desc_similiarity_score = semantic_compare::similarity_score(
            &self.description.clone().unwrap_or_default(),
            &issue.description.clone().unwrap_or_default(),
        );
        // contract, function and issue type MUST match
        if issue.hash() != self.hash() || title_similiarity_score <= 0.25 {
            // log::info!(
            //     "issue: {} \n self: {} \n ==> DIFFERENT",
            //     issue.title,
            //     self.title
            // );
            return Ok(false);
        } else if desc_similiarity_score >= 0.65 || issue.description == self.description {
            // log::info!("issue: {} \n self: {} \n ==> SAME", issue.title, self.title);
            return Ok(true);
        }

        let prompt = DEDUP_PROMPT
            .replace("{contract}", &self.contract)
            .replace("{function}", &self.function)
            .replace("{issue_type}", self.exploit_type.as_str())
            .replace(
                "{description_a}",
                &self.description.clone().unwrap_or_default(),
            )
            .replace(
                "{description_b}",
                &issue.description.clone().unwrap_or_default(),
            );

        add_to_inference_cost_by_type(&prompt, ai_agent.get_metadata(), TokenType::Input).await;

        let response = ai_agent.prompt(&prompt).await?;

        add_to_inference_cost_by_type(&response, ai_agent.get_metadata(), TokenType::Output).await;

        Ok(response.trim().eq_ignore_ascii_case("YES"))
    }
}

impl Default for PrivilegeLevel {
    fn default() -> Self {
        PrivilegeLevel::Permissionless
    }
}

impl EnumString for PrivilegeLevel {
    fn as_str(&self) -> &'static str {
        match self {
            PrivilegeLevel::Permissionless => "Permissionless",
            PrivilegeLevel::RequiresRole => "RequiresRole",
            PrivilegeLevel::RequireAdminRole => "RequireAdminRole",
        }
    }
}

impl Findings {
    pub async fn dedup(self) -> anyhow::Result<Findings> {
        if self.findings.is_empty() {
            return Ok(Findings {
                findings: Vec::new(),
            });
        }

        let openai_config = AgentConfig::new(None).with_model("gpt-5");

        let openai_agent = Arc::new(AgentFactory::create_openai_agent(&openai_config)?);

        let mut findings_hash = HashMap::<String, Vec<Finding>>::new();

        for finding in &self.findings {
            let hash = finding.hash();
            findings_hash
                .entry(hash)
                .or_insert(Vec::new())
                .push(finding.clone());
        }

        let arc_dedup_findings = Arc::new(Mutex::new(Vec::with_capacity(self.findings.len())));
        let mut handles = Vec::new();

        for findings in findings_hash.into_values() {
            let arc_findings = Arc::new(findings);
            let current_findings = Arc::clone(&arc_findings);
            let deduped_findings = Arc::clone(&arc_dedup_findings);
            let agent = Arc::clone(&openai_agent);
            let handle = tokio::spawn(async move {
                if current_findings.len() > 1 {
                    match get_deduped_finding_vec(&current_findings, &agent).await {
                        Ok(deduped) => {
                            let mut deduped_findings_lock = deduped_findings.lock().await;
                            deduped_findings_lock.extend(deduped);
                        }
                        Err(e) => {
                            log::error!("❌ deduping findings failed: {e}");
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
        // Fix: Extract the Vec from Arc<Mutex<Vec<Finding>>>
        let deduped_findings = Arc::try_unwrap(arc_dedup_findings)
            .map_err(|_| anyhow::anyhow!("Failed to unwrap Arc"))?
            .into_inner();

        Ok(Findings {
            findings: deduped_findings,
        })
    }

    /// Get count of findings by severity
    pub fn count_by_severity(&self) -> std::collections::HashMap<Severity, usize> {
        let mut counts = HashMap::new();

        for finding in &self.findings {
            *counts.entry(finding.severity).or_insert(0) += 1;
        }

        counts
    }

    /// Filter findings by severity level
    pub fn filter_by_severity(&self, severity: Severity) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .collect()
    }

    /// Get all high severity findings
    pub fn high_severity_findings(&self) -> Vec<&Finding> {
        self.filter_by_severity(Severity::High)
    }
}

async fn get_deduped_finding_vec(
    findings: &Arc<Vec<Finding>>,
    agent: &Arc<crate::llm_review::enums::AIAgent>,
) -> anyhow::Result<Vec<Finding>> {
    // assigns bool to each finding index, is dup or not? assume not for initializing
    let size = findings.len();
    let mut is_dup_vec: Vec<bool> = vec![false; size];

    for i in 0..size {
        if is_dup_vec[i] {
            continue;
        }
        for j in i + 1..size {
            if is_dup_vec[j] {
                continue;
            }
            let is_dup = findings[i].is_duplicate_issue(&findings[j], &agent).await?;
            if is_dup {
                is_dup_vec[j] = true;
                continue;
            }
        }
    }

    let findings: Vec<Finding> = findings
        .iter()
        .enumerate()
        .filter(|(idx, _)| !is_dup_vec[*idx])
        .map(|(_, f)| f)
        .cloned()
        .collect();

    Ok(findings)
}

pub trait FromLLMJson: Sized {
    /// Parse clean JSON string into type
    fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error>;

    /// Clean up formatting (markdown code blocks, extra quotes, etc.)
    fn clean_json_string(input: &str) -> String;

    /// Extract and parse JSON from raw LLM response with extra text
    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>>;
}

impl<T> FromLLMJson for T
where
    T: DeserializeOwned,
{
    fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let cleaned = Self::clean_json_string(json_str);
        serde_json::from_str(&cleaned)
    }

    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        let cleaned_str = cleaned.to_string();

        // Fix numeric fields that are returned as strings (e.g., "finding_complexity": "5" -> "finding_complexity": 5)
        // This regex finds patterns like "field_name": "123" and removes quotes around the number
        let re = Regex::new(r#""(finding_complexity|[a-z_]*count)":\s*"(\d+)""#).unwrap();
        re.replace_all(&cleaned_str, r#""$1": $2"#).to_string()
    }

    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_start = response.find('{');
        let json_end = response.rfind('}');

        match (json_start, json_end) {
            (Some(start), Some(end)) if start < end => {
                let json_part = &response[start..=end];
                Self::parse_from_json(json_part)
                    .map_err(|e| format!("Failed to parse JSON: {}", e).into())
            }
            _ => Err("No valid JSON found in response".into()),
        }
    }
}
