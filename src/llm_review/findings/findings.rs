use crate::llm_review::{
    agent::agent_factory::{AgentConfig, AgentFactory},
    findings::finding_enums::{Severity, VulnerabilityType},
    phases::verify_rounds::FindingStatus,
    prompt_support::dedup::DEDUP_PROMPT,
};
use crate::{
    cost::cost_data::{TokenType, add_to_inference_cost_by_type},
    llm_review::phases::{add_poc_findings::PocStatus, create_report::CompetitionReport},
    utils::semantic_compare,
};
use regex::Regex;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use strum_macros::EnumIter;
use tokio::sync::Mutex;

pub const CLAUDE_4_0_SONNET: &str = "claude-sonnet-4-0";
pub const CLAUDE_4_5_SONNET: &str = "claude-sonnet-4-5";
pub const CLAUDE_4_OPUS: &str = "claude-opus-4-0";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Finding {
    // [Severity-issue number] - List Issue (Reentrancy, Denial of Service, etc) and
    // <Contract>::<Function> its localed in
    pub id: Option<String>,
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
    pub poc_test_file: Option<PathBuf>,
    pub poc_test_command: Option<String>,
    pub poc_test_status: Option<PocStatus>,
    #[schemars(description = "Severity level: Critical, High, Medium, Low, Info")]
    pub severity: Severity, //severity of issue
    pub mitigation: Option<String>,
    pub status: Option<Vec<FindingStatus>>,
    pub status_justification: Option<String>,
    // competition ready report (C4, Sherlock,etc) for issue, only produced if All Tests Passed for PoC
    pub competition_report: Option<CompetitionReport>,
    pub finding_complexity: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Findings {
    pub findings: Vec<Finding>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumIter,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[serde(rename_all = "PascalCase")]
pub enum PrivilegeLevel {
    Permissionless,    // any EOA
    RequiresRole,      // specific role
    RequiresAdminRole, // admin or owner
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
                self.severity.to_string()
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
            format!(
                "{}-{}-{}-{}",
                self.title.replace(" ", "-"),
                derived.to_string(),
                self.contract,
                fn_name
            )
        } else {
            format!(
                "{}-{}-{}-{}",
                self.title.replace(" ", "-"),
                self.exploit_type.to_string(),
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
        ai_agent: &crate::llm_review::agent::agent_enums::AIAgent,
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
            .replace("{issue_type}", &self.exploit_type.to_string())
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

impl Findings {
    pub async fn dedup(self) -> anyhow::Result<Findings> {
        if self.findings.is_empty() {
            return Ok(Findings {
                findings: Vec::new(),
            });
        }

        let openai_config = AgentConfig::new(None).with_model("gpt-5-mini");

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
    agent: &Arc<crate::llm_review::agent::agent_enums::AIAgent>,
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
        // Try parsing the raw JSON first - don't fix what isn't broken!
        match serde_json::from_str(json_str) {
            Ok(parsed) => {
                log::debug!("✅ Raw JSON parsed successfully without cleaning");
                return Ok(parsed);
            }
            Err(_) => {
                log::debug!("Raw JSON failed to parse - attempting to clean",);
            }
        }

        // If raw parsing failed, try cleaning the JSON
        let cleaned = Self::clean_json_string(json_str);

        // Debug: Log cleaned JSON if RUST_LOG=debug
        // log::debug!(
        //     "Cleaned JSON (first 500 chars): {}",
        //     &cleaned[..cleaned.len().min(500)]
        // );

        let result = serde_json::from_str(&cleaned);
        if let Err(ref _e) = result {
            log::error!("JSON parse error after cleaning!");
        }
        result
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

        let mut cleaned_str = cleaned.to_string();

        // Fix single quotes to double quotes (JavaScript object notation -> JSON)
        // This handles cases where LLMs return {'key': 'value'} instead of {"key": "value"}
        // We need to be careful to only replace quotes that are part of the JSON structure,
        // not quotes inside string values
        cleaned_str = cleaned_str.replace("'", "\"");

        // Fix broken JSON strings where Gemini splits a string value across lines
        // Two patterns to handle:
        // 1. Escaped newline: "proof_of_code": "function test() {...}\\n    \\\"severity\": \"High\""
        // 2. Actual newline: "proof_of_code": "function test() {...}\n    \"severity": "High"
        // This happens when Gemini truncates long strings and continues on next line
        // We need to close the broken string and properly start the next field

        // Pattern 1: Escaped backslash-n (\\n) followed by escaped quote (\\")
        // Matches: \\n    \\\"severity\": \"
        let escaped_newline_pattern = regex::Regex::new(r#"\\n\s*\\"([a-zA-Z_]+)":\s*""#).unwrap();
        cleaned_str = escaped_newline_pattern
            .replace_all(&cleaned_str, r#"", "$1": ""#)
            .to_string();

        // Pattern 2: Actual newline (\n) followed by escaped quote (\")
        // Matches: \n    \"severity": "
        let actual_newline_pattern = regex::Regex::new(r#"\n\s*\\"([a-zA-Z_]+)":\s*""#).unwrap();
        cleaned_str = actual_newline_pattern
            .replace_all(&cleaned_str, r#"", "$1": ""#)
            .to_string();

        // Strip // comments (sometimes LLMs include them from examples)
        let comment_re = Regex::new(r"\s*//[^\n]*").unwrap();
        cleaned_str = comment_re.replace_all(&cleaned_str, "").to_string();

        // Escape unescaped characters inside JSON string values
        // This fixes control characters AND unescaped quotes that break JSON parsing
        // Strategy: Track when we're inside a JSON string value (between unescaped quotes)
        // and escape any quotes we find inside that aren't already escaped
        let mut result = String::with_capacity(cleaned_str.len() * 2);
        let mut chars = cleaned_str.chars().peekable();
        let mut in_string = false;
        let mut prev_was_backslash = false;

        while let Some(ch) = chars.next() {
            match ch {
                '"' if !prev_was_backslash && !in_string => {
                    // Start of a JSON string value
                    in_string = true;
                    result.push(ch);
                    prev_was_backslash = false;
                }
                '"' if !prev_was_backslash && in_string => {
                    // This could be end of string OR an unescaped quote inside the string
                    // Look ahead to see if this looks like end of string (followed by : or , or })
                    let next_non_ws = chars.clone().skip_while(|c| c.is_whitespace()).next();
                    if matches!(
                        next_non_ws,
                        Some(':') | Some(',') | Some('}') | Some(']') | None
                    ) {
                        // End of JSON string value
                        in_string = false;
                        result.push(ch);
                    } else {
                        // Unescaped quote inside string - escape it!
                        result.push_str("\\\"");
                    }
                    prev_was_backslash = false;
                }
                '\\' => {
                    result.push(ch);
                    prev_was_backslash = !prev_was_backslash;
                }
                '\n' if in_string && !prev_was_backslash => {
                    result.push_str("\\n");
                    prev_was_backslash = false;
                }
                '\r' if in_string && !prev_was_backslash => {
                    result.push_str("\\r");
                    prev_was_backslash = false;
                }
                '\t' if in_string && !prev_was_backslash => {
                    result.push_str("\\t");
                    prev_was_backslash = false;
                }
                '\x08' if in_string && !prev_was_backslash => {
                    result.push_str("\\b");
                    prev_was_backslash = false;
                }
                '\x0C' if in_string && !prev_was_backslash => {
                    result.push_str("\\f");
                    prev_was_backslash = false;
                }
                c if in_string && c.is_control() && !prev_was_backslash => {
                    // Escape any other control characters as unicode
                    result.push_str(&format!("\\u{:04x}", c as u32));
                    prev_was_backslash = false;
                }
                _ => {
                    result.push(ch);
                    prev_was_backslash = false;
                }
            }
        }

        cleaned_str = result;

        // Fix numeric fields that are returned as strings (e.g., "finding_complexity": "5" -> "finding_complexity": 5)
        // This regex finds patterns like "field_name": "123" and removes quotes around the number
        let re = Regex::new(r#""(finding_complexity|[a-z_]*count)":\s*"(\d+)""#).unwrap();
        re.replace_all(&cleaned_str, r#""$1": $2"#).to_string()
    }

    fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 1) Fast path: the whole response is already JSON
        let trimmed = response.trim();

        let starts_ok = trimmed.starts_with('{') || trimmed.starts_with('[');
        let ends_ok = trimmed.ends_with('}') || trimmed.ends_with(']');

        log::debug!(
            "JSON extraction: response len={}, trimmed len={}, starts_with={{/[: {}, ends_with={{/]: {}, first 100: {}, last 100: {}",
            response.len(),
            trimmed.len(),
            starts_ok,
            ends_ok,
            &trimmed[..trimmed.len().min(100)],
            &trimmed[trimmed.len().saturating_sub(100)..]
        );

        if starts_ok && ends_ok {
            log::debug!("Fast path: attempting to parse complete JSON");
            match Self::parse_from_json(trimmed) {
                Ok(parsed) => {
                    log::debug!("✅ Fast path succeeded");
                    return Ok(parsed);
                }
                Err(e) => {
                    log::debug!(
                        "Fast path failed: {} - falling back to robust extraction",
                        e
                    );
                }
            }
        } else {
            log::debug!("Fast path skipped - falling back to robust extraction");
        }

        // 2) Robust path: models often include analysis + examples + then the JSON.
        // We scan for any JSON object/array embedded in the text and pick the best candidate
        // that deserializes into the target type.
        let cleaned = Self::clean_json_string(response);

        let mut best: Option<(usize, usize, Self)> = None;
        let mut candidates_checked: usize = 0;

        for (idx, ch) in cleaned.char_indices() {
            if ch != '{' && ch != '[' {
                continue;
            }

            // Avoid pathological worst-cases on giant responses
            candidates_checked += 1;
            if candidates_checked > 500 {
                break;
            }

            let slice = &cleaned[idx..];

            // Parse exactly one JSON value from this point and learn how many bytes it consumed.
            // NOTE: `byte_offset()` is available on the stream iterator in our serde_json version.
            let mut stream =
                serde_json::Deserializer::from_str(slice).into_iter::<serde_json::Value>();
            if stream.next().is_none_or(|r| r.is_err()) {
                continue;
            }

            let end = idx + stream.byte_offset();
            if end <= idx || end > cleaned.len() {
                continue;
            }

            let candidate = &cleaned[idx..end];

            // Now try to parse into the requested type.
            if let Ok(parsed) = Self::parse_from_json(candidate) {
                let cand_len = end - idx;
                let is_better = best
                    .as_ref()
                    .map(|(b_start, b_end, _)| cand_len > (b_end - b_start))
                    .unwrap_or(true);

                if is_better {
                    best = Some((idx, end, parsed));
                }
            }
        }

        if let Some((start, end, parsed)) = best {
            log::debug!(
                "✅ Robust path succeeded: extracted JSON candidate (start={}, len={}, first 500 chars): {}",
                start,
                end - start,
                &cleaned[start..end.min(start + 500)]
            );
            return Ok(parsed);
        }

        log::error!(
            "❌ No valid JSON found in response. Checked {} candidates. Response length: {}, first 200 chars: {}",
            candidates_checked,
            response.len(),
            &response[..response.len().min(200)]
        );
        Err("No valid JSON found in response".into())
    }
}

#[cfg(test)]
mod tests {
    use super::FromLLMJson;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Dummy {
        a: i64,
        b: String,
    }

    #[test]
    fn from_llm_json_extracts_embedded_json_object_even_with_other_json_present() {
        let response = r#"
Here is some analysis.

payload = {"model":"accounts/fireworks/models/kimi-k2p5","temperature":0.1}

Now the real answer:
{"a": 7, "b": "ok"}
"#;

        let parsed: Dummy = Dummy::parse_from_llm_response(response).unwrap();
        assert_eq!(
            parsed,
            Dummy {
                a: 7,
                b: "ok".to_string()
            }
        );
    }

    #[test]
    fn from_llm_json_extracts_embedded_json_array() {
        let response = r#"Some text before.

[{"a": 1, "b": "x"}, {"a": 2, "b": "y"}]

Some text after."#;

        let parsed: Vec<Dummy> = Vec::<Dummy>::parse_from_llm_response(response).unwrap();
        assert_eq!(
            parsed,
            vec![
                Dummy {
                    a: 1,
                    b: "x".to_string()
                },
                Dummy {
                    a: 2,
                    b: "y".to_string()
                }
            ]
        );
    }
}
