use super::{
    enums::{InvariantStatus, InvariantType, Severity, VulnerabilityType},
    prompt_support::dedup::DEDUP_PROMPT,
};
use crate::{
    master_prompts::{
        prompt_2x_a::PROMPT_2X_A, prompt_2x_b::PROMPT_2X_B, prompt_3x_a::PROMPT_3X_A,
        prompt_3x_b::PROMPT_3X_B, prompt_3x_c::PROMPT_3X_C,
    },
    prompts::{
        access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
        confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
        dos::DOS, inheritance::WRONG_INHERITANCE, integer_overflow::INTEGER_OVERFLOW, mev::MEV,
        oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
        reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK,
        self_destruct::SELF_DESTRUCT, storage_variables::STORAGE_VARIABLE, tx_origin::TX_ORIGIN,
        unchecked_return_value::UNCHECK_RETURN_VALUES, unexpected_eth::UNEXPECTED_ETH,
        zero_code::CONTRACTS_WITH_ZERO_CODE,
    },
};
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    completion::{CompletionModel, Prompt},
    providers::{
        azure::GPT_4O,
        openai::{self},
    },
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageModel {
    OpenAI,
    Anthropic,
}

pub const CLAUDE_4_0_SONNET: &str = "claude-sonnet-4-0";
pub const CLAUDE_4_OPUS: &str = "claude-opus-4-0";
pub const LANGUAGE_MODEL: LanguageModel = LanguageModel::Anthropic;
pub const RUNS: usize = 3;
pub const INSTRUCTION_PROMPTS: [&str; 2] = [PROMPT_2X_A, PROMPT_2X_B];

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Finding {
    // [Severity-issue number] - List Issue (Reentrancy, Denial of Service, etc) and
    // <Contract>::<Function> its localed in
    pub issue_type: VulnerabilityType,
    pub contract: String,            // exact constract name where issue appears
    pub function: String, // exact function name where issue appears, if not applicable set to 'NA'
    pub description: Option<String>, // description of issue, include code snippet if relevant
    pub impact: Option<String>, // Impact of Issue
    pub proof_of_concept: Option<String>, // Demonstrate how issue can be exploited by hacker
    pub proof_of_code: Option<String>, // Write Foundry Unit test to prove issue exists
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Severity, //severity of issue
    pub mitigation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Findings {
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InvariantFinding {
    pub id: String,
    #[schemars(
        description = "Type: Arithmetic, Balance, Permission, Temporal, Referential, StateMachine"
    )]
    pub inv_type: InvariantType,
    pub desc: String,
    #[schemars(description = "Status: HOLDS, VIOLATION")]
    pub status: InvariantStatus,
    pub pre_state: Option<String>,
    pub post_state: Option<String>,
    pub impact: Option<String>,
    pub poc: Option<String>,
    pub mitigation: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContractInvariants {
    pub contract: String,
    pub intention: String,
    pub invariants: Vec<InvariantFinding>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitVulnerability {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_legit_vulnerability: bool,
    pub why_its_not_legit: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VulnerabilityQualityCheck {
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub is_quality_check_passed: bool, // quality check passes with no changes/update needed, true|false
    pub where_quality_lacks: Option<String>, // breif description
    pub impact: Option<String>,              // updated impact (if necessary)
    pub proof_of_concept: Option<String>,    // updated POC (if necessary)
    pub proof_of_code: Option<String>,       // updated Foundry Unit test (if necessary)
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Option<Severity>, // updated severity of issue (if necessary)
    pub mitigation: Option<String>,          // updated mitigation (if necessary)
}

// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
// pub struct InvariantFindings {
//     pub findings: Vec<InvariantFinding>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DuplicateFindings {
    pub titles: Vec<String>,
}

pub const SECURITY_PROMPT_ENUMS: [VulnerabilityType; 8] = [
    VulnerabilityType::Reentrancy,
    VulnerabilityType::AccessControl,
    // VulnerabilityType::ArrayLimits,
    // VulnerabilityType::DefaultVisibility,
    VulnerabilityType::Dos,
    VulnerabilityType::IntegerMath,
    // VulnerabilityType::ConfidentialData,
    // VulnerabilityType::Inheritance,
    // VulnerabilityType::Oracle,
    VulnerabilityType::Pragma,
    VulnerabilityType::Randomness,
    // VulnerabilityType::ReplayAttack,
    // VulnerabilityType::SelfDestruct,
    // VulnerabilityType::StorageLayout,
    // VulnerabilityType::TxOrigin,
    // VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UnexpectedEth,
    // VulnerabilityType::ZeroCode,
    VulnerabilityType::FrontrunMev,
    // VulnerabilityType::ShortAddress,
];

pub const QUALITY_CHECK_ENUMS: [VulnerabilityType; 24] = [
    VulnerabilityType::Reentrancy,
    VulnerabilityType::AccessControl,
    VulnerabilityType::ArrayLimits,
    VulnerabilityType::Dos,
    VulnerabilityType::IntegerMath,
    VulnerabilityType::ConfidentialData,
    VulnerabilityType::Inheritance,
    VulnerabilityType::Oracle,
    VulnerabilityType::Randomness,
    VulnerabilityType::ReplayAttack,
    VulnerabilityType::SelfDestruct,
    VulnerabilityType::StorageLayout,
    VulnerabilityType::TxOrigin,
    VulnerabilityType::UncheckedReturn,
    VulnerabilityType::UnexpectedEth,
    VulnerabilityType::FrontrunMev,
    VulnerabilityType::UpgradeabilityInitializerSafety,
    VulnerabilityType::PausableEmergencyStop,
    VulnerabilityType::TimestampDependentLogic,
    VulnerabilityType::FlashLoanEconomicManipulation,
    VulnerabilityType::DelegatecallLowLevelOps,
    VulnerabilityType::SignatureMalleability,
    VulnerabilityType::GasGriefBlockLimit,
    VulnerabilityType::IntegerOverflow,
];

pub const SECURITY_PROMPTS: [&str; 19] = [
    REENTRANCY,                  //DONE
    ACCESS_CONTROL,              //DONE
    ACCESS_OUTSIDE_ARRAY_LIMITS, //DONE
    DEFAULT_VISIBILITIES,        //DONE
    DOS,                         //DONE
    INTEGER_OVERFLOW,            // DONE
    SAVING_CONFIDENTIAL_DATA,    //DONE
    WRONG_INHERITANCE,           // DONE
    ORACLE_MANIPULATION,         // DONE
    FLOATING_PRAGMA,             //DONE
    RANDOMNESS,                  // DONE
    REPLAY_SIGNATURES_ATTACK,    //DONE
    SELF_DESTRUCT,               //DONE
    STORAGE_VARIABLE,            //DONE
    TX_ORIGIN,                   //DONE
    UNCHECK_RETURN_VALUES,       //DONE
    UNEXPECTED_ETH,              //DONE
    CONTRACTS_WITH_ZERO_CODE,    //DONE
    // SHORT_ADDRESS_ATTACK,        // DONE - this is only issue for very old contracts
    MEV, //DONE
];

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

fn deserialize_bool_from_str_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let val: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match val {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(s) => match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(serde::de::Error::custom("expected 'true' or 'false'")),
        },
        _ => Err(serde::de::Error::custom("expected boolean or string")),
    }
}

impl Finding {
    pub fn title(&self) -> String {
        format!(
            "{} issue in {}::{}",
            self.issue_type.as_fancy_str(),
            self.contract,
            self.function
        )
    }

    pub fn hash(&self) -> String {
        format!(
            "{}-{}-{}",
            self.issue_type.as_str(),
            self.contract,
            self.function
        )
    }

    // resonse "YES" or "NO"
    pub async fn is_duplicate_issue<M>(
        &self,
        issue: &Finding,
        ai_agent: &Agent<M>,
    ) -> anyhow::Result<bool>
    where
        M: CompletionModel,
    {
        // contract, function and issue type MUST match
        if issue.hash() != self.hash() {
            return Ok(false);
        } else if issue.description == self.description {
            return Ok(true);
        }

        let prompt = DEDUP_PROMPT
            .replace("{contract}", &self.contract)
            .replace("{function}", &self.function)
            .replace("{issue_type}", self.issue_type.as_str())
            .replace(
                "{description_a}",
                &self.description.clone().unwrap_or_default(),
            )
            .replace(
                "{description_b}",
                &issue.description.clone().unwrap_or_default(),
            );

        let response = ai_agent.prompt(prompt).await?;

        Ok(response.trim().eq_ignore_ascii_case("YES"))
    }
}

impl ContractInvariants {
    /// Parse JSON string containing findings from LLM response
    /// Handles both clean JSON and JSON wrapped in markdown code blocks
    pub fn parse_from_json(json_str: &str) -> Result<ContractInvariants, serde_json::Error> {
        // Clean the input - remove markdown code blocks and extra quotes/escapes
        let cleaned_json = Self::clean_json_string(json_str);

        // Parse the cleaned JSON
        serde_json::from_str(&cleaned_json)
    }

    /// Clean JSON string by removing markdown code blocks, escaped quotes, and extra formatting
    fn clean_json_string(input: &str) -> String {
        let mut cleaned = input.trim();

        // Remove outer quotes if present (from string literals)
        if cleaned.starts_with('"') && cleaned.ends_with('"') {
            cleaned = &cleaned[1..cleaned.len() - 1];
        }

        // Remove markdown code blocks
        if cleaned.starts_with("```json") {
            cleaned = cleaned.strip_prefix("```json").unwrap_or(cleaned);
        }

        if cleaned.ends_with("```") {
            cleaned = cleaned.strip_suffix("```").unwrap_or(cleaned);
        }

        // Replace escaped quotes and newlines
        // cleaned
        //     .replace("\\\"", "\"")
        //     .replace("\\n", "\n")
        //     .replace("\\\n", "\n")
        //     .trim()
        //     .to_string()
        cleaned.to_string()
    }

    pub fn get_all_violations(self) -> Vec<InvariantFinding> {
        self.invariants
            .into_iter()
            .filter(|inv| inv.status == InvariantStatus::VIOLATION)
            .collect::<Vec<InvariantFinding>>()
    }
}
impl Findings {
    pub async fn dedup(self) -> anyhow::Result<Findings> {
        let openai_client = openai::Client::from_env();
        let openai_agent = openai_client.agent(GPT_4O).temperature(1.0).build();

        let size = self.findings.len();
        // assigns bool to each finding index, is dup or not? assume not for initializing
        let mut is_dup_vec: Vec<bool> = vec![false; size];

        for i in 0..size {
            if is_dup_vec[i] {
                continue;
            }
            for j in i + 1..size {
                if is_dup_vec[j] {
                    continue;
                }
                let is_dup = self.findings[i]
                    .is_duplicate_issue(&self.findings[j], &openai_agent)
                    .await?;
                if is_dup {
                    is_dup_vec[j] = true;
                    continue;
                }
            }
        }

        let findings: Vec<Finding> = self
            .findings
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !is_dup_vec[*idx])
            .map(|(_, f)| f)
            .collect();

        Ok(Findings { findings })
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

        cleaned.to_string()
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
