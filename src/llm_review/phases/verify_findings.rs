/// Phase 3: Deduplication and verification of discovered security findings
///
/// This phase removes duplicate findings and verifies the legitimacy of each
/// discovered vulnerability using AI-powered analysis.
use crate::{
    error::Result,
    llm_review::{
        context_state::{generate_audit_scope, get_metadata_context},
        enums::{AIAgent, Severity},
        findings::{Finding, Findings},
        prompt_support::severity_rubics::CODE4RENA_SEVERITY_RUBRIC,
        semaphore::VERIFY_SEM,
        utils::prompt_context::{generate_prompt_for_issue_check, FindingReportType},
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;

use crate::{
    config::AuditType,
    llm_review::{
        enums::{all_enum_variants, generate_enum_list},
        prompt_support::severity_rubics::{CANTINA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::Mutex;

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
    Valid,
    #[default]
    Invalid,
    OutOfScope,
    NeedsMoreInfo,
}

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
    EnumIter,
    JsonSchema,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum FindingConfidence {
    VeryConfident,
    Confident,
    #[default]
    SomeWhatConfident,
}

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegitVulnerability {
    pub severity: Option<Severity>,
    pub severity_justification: Option<String>,
    pub status: FindingStatus,
    pub status_justification: Option<String>,
    pub status_confidence: FindingConfidence,
    pub status_confidence_justification: Option<String>,
    #[serde(deserialize_with = "deserialize_finding_complexity")]
    pub finding_complexity: u8,
}

/// Helper function to deserialize boolean from string or boolean
/// Used by scope_findings.rs
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

/// Helper function to deserialize u8 from string or number with validation (1-10)
fn deserialize_finding_complexity<'de, D>(deserializer: D) -> std::result::Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let val: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    let num = match val {
        serde_json::Value::Number(n) => {
            n.as_u64()
                .ok_or_else(|| serde::de::Error::custom("expected valid number"))? as u8
        }
        serde_json::Value::String(s) => s
            .parse::<u8>()
            .map_err(|_| serde::de::Error::custom("expected numeric string"))?,
        _ => return Err(serde::de::Error::custom("expected number or string")),
    };

    // Validate range 1-10
    if num < 1 || num > 10 {
        log::warn!(
            "finding_complexity {} is out of range (1-10), clamping to valid range",
            num
        );
        Ok(num.clamp(1, 10))
    } else {
        Ok(num)
    }
}

/// Executes the verification phase
///
/// Deduplicates findings and verifies each one using AI analysis to ensure
/// only legitimate vulnerabilities are retained.
pub async fn execute(
    findings: Findings,
    code: &str,
    agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<Findings> {
    info!("🔍 Phase 4: Deduplicating and verifying findings...");

    let mut handles = vec![];
    let deduped_findings = Arc::new(findings.dedup().await?);
    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");
    let code_and_context = generate_content_plus_context_block(code, &context);
    let arc_code_context = Arc::new(code_and_context);
    let arc_repo = Arc::new(repo.clone());

    let dedup_finding_count = deduped_findings.findings.len();
    let is_legit_finding_vec: Arc<Mutex<Vec<LegitVulnerability>>> = Arc::new(Mutex::new(vec![
            LegitVulnerability::default();
            dedup_finding_count
        ]));

    info!("# of findings AFTER deduping => {}", dedup_finding_count);
    info!("now verifying each finding...");

    let audit_scope = generate_audit_scope(repo).await?;

    let verify_prompt = generate_verify_prompt(repo);

    let updated_verify_prompt = if audit_scope.is_empty() {
        Arc::new(verify_prompt.to_string())
    } else {
        Arc::new(format!(
            "{}\n\n ## SCOPE FOR SECURITY AUDIT - ONLY FINDINGS WITHIN BELOW SCOPE ARE LEGIT\n\n{}",
            &verify_prompt, &audit_scope
        ))
    };
    // info!("verify prompt + scope => {}", verify_prompt_plus_scope);

    for i in 0..dedup_finding_count {
        let codeblock_plus_context = Arc::clone(&arc_code_context);
        let arc_findings = Arc::clone(&deduped_findings);
        let arc_agent = Arc::clone(&agent);
        let repo_clone = Arc::clone(&arc_repo);
        let arc_legit_findings_vec = Arc::clone(&is_legit_finding_vec);
        let verify_prompt_and_scope = Arc::clone(&updated_verify_prompt);
        let sem = Arc::clone(&VERIFY_SEM);

        handles.push(tokio::spawn(async move {
            // ── acquire permit ────────────────────────
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result: Result<()> = async {
                let post_verify_json = generate_post_verify_json_requirement(&repo_clone);
                let instruction_prompt = generate_prompt_for_issue_check(
                    &codeblock_plus_context,
                    &arc_findings.findings[i],
                    &verify_prompt_and_scope,
                    &post_verify_json,
                    FindingReportType::Standard,
                );

                // add to cost
                info!("verifying finding #{}", i + 1);
                let is_legit_struct: LegitVulnerability =
                    arc_agent.extract_with_retry(&instruction_prompt).await?;

                let is_finding_legit = is_legit_struct.status == FindingStatus::Valid;
                if !is_finding_legit {
                    info!(
                        "{} is {} => {}",
                        arc_findings.findings[i].title,
                        arc_findings.findings[i].status.unwrap_or_default(),
                        &is_legit_struct
                            .status_justification
                            .clone()
                            .unwrap_or_default()
                    );
                }

                let mut legit_findings_vec = arc_legit_findings_vec.lock().await;
                legit_findings_vec[i] = is_legit_struct;

                Ok(())
            }
            .await;

            if let Err(e) = result {
                log::error!("Error verifying finding {}: {:?}", i, e);
            }
        }));
    }

    // Wait for all verification tasks to complete
    for h in handles {
        let _ = h.await;
    }

    let legit_findings_vec = is_legit_finding_vec.lock().await;
    let verified_findings: Vec<Finding> = deduped_findings
        .as_ref()
        .findings
        .iter()
        .enumerate()
        .filter(|(idx, _)| {
            legit_findings_vec[*idx].status == FindingStatus::Valid
                || legit_findings_vec[*idx].status == FindingStatus::NeedsMoreInfo
        })
        .map(|(idx, f)| {
            let legit_findings = legit_findings_vec[idx].clone();
            let enriched_finding = Finding {
                severity: legit_findings.severity.unwrap_or(f.severity),
                severity_justification: legit_findings.severity_justification,
                status: Some(legit_findings.status),
                status_justification: legit_findings.status_justification,
                status_confidence: Some(legit_findings.status_confidence),
                status_confidence_justification: legit_findings.status_confidence_justification,
                finding_complexity: Some(legit_findings.finding_complexity),
                ..f.clone()
            };
            enriched_finding
        })
        .collect();

    info!(
        "✅ Phase 4 complete: {} Verified Findings!",
        verified_findings.len()
    );

    Ok(Findings {
        findings: verified_findings,
    })
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

struct VerifyEnumLists {
    severity_list: String,           //  High | Medium | Low | Info | Invalid
    finding_status_list: String,     // Valid | Invalid | OutOfScope | NeedsMoreInfo
    finding_confidence_list: String, // VeryConfident | Confident | SomeWhatConfident
}

fn generate_verify_enum_lists(repo: &RepoPaths) -> VerifyEnumLists {
    let severity_enums_standard: Vec<Severity> = Severity::iter()
        .filter(|s| *s != Severity::Critical)
        .collect();
    let severity_enums_list_standard = generate_enum_list(severity_enums_standard.as_slice());
    let severity_list = match repo.audit_type {
        AuditType::Code4rena => severity_enums_list_standard,
        AuditType::Sherlock => severity_enums_list_standard,
        AuditType::Cantina => severity_enums_list_standard,
        _ => generate_enum_list(all_enum_variants::<Severity>().as_slice()),
    };

    let finding_status_list = generate_enum_list(all_enum_variants::<FindingStatus>().as_slice());
    let finding_confidence_list =
        generate_enum_list(all_enum_variants::<FindingConfidence>().as_slice());

    VerifyEnumLists {
        severity_list,
        finding_status_list,
        finding_confidence_list,
    }
}

pub fn generate_verify_prompt(repo: &RepoPaths) -> String {
    let (severity_rubic, contest) = match repo.audit_type {
        AuditType::Code4rena => (CODE4RENA_SEVERITY_RUBRIC, "Code4rena"),
        AuditType::Sherlock => (SHERLOCK_SEVERITY_RUBRIC, "Sherlock"),
        AuditType::Cantina => (CANTINA_SEVERITY_RUBRIC, "Cantina"),
        _ => (CODE4RENA_SEVERITY_RUBRIC, "Private Audit"),
    };

    let VerifyEnumLists {
        severity_list,
        finding_status_list,
        finding_confidence_list,
    } = generate_verify_enum_lists(repo);

    let pre_verify_json = generate_pre_verify_json_requirement(repo);

    format!(
        r#"
        {pre_verify_json}

Your task: decide if a reported finding is Valid and to accurately assess its Severity in a {contest} contest.


### COFIGURATION CHECK
    **Does finding rely on constants (time, thresholds, buffers)?**
    If YES, check:
    - `// for testnet`, `// for testing`, `// temporary` comments
    - Suspiciously small values (WEEK=1800 vs 604800, BUFFER=300 vs 3600)
    - Commented-out production values
    - Re-assess with production config

### SCOPE CHECK
    - Is finding in scope? (see scope provided below)

### BY DESIGN CHECK
    Check ALL:
    - NatSpec (@dev, @notice, @custom)
    - Inline comments (`// NOTE:`, `// IMPORTANT:`)
    - docs and scope provided below (Known Limitations, Design Decisions)
    - Function placement (Emergency/Admin sections)
    - Function naming (emergencyPause, adminOnly)

### VALIDITY CHECK
    - Trace execution path - does attack work?
    - Check safeguards: access control, reentrancy guards, pauses, timelocks, validation
    - Verify root cause is correct
    - Check external dependencies - are assumptions reasonable?

### 5. SEVERITY && LIKELIHOOD CHECK
    - Does finding Impact and Likelihood justify current Severity score?
    - Please use Severity Rubric below to evaluate Severity Score


Based on your assessment please provided the following:

*Severity:* {severity_list} 
*Finding Severity Justification:* Explain why you assigned this severity. 
*Finding Status:* {finding_status_list}
*Status Justification:* if invalid, out of scope, or needs more info, please explain why.
*Finding Status Confidence:* {finding_confidence_list}
*Finding Status Confidence Justification:* if Somewhat Confident, please explain why. 
*Finding Complexity:* How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty.

## {contest} Guidelines
# {contest} Severity Rubric (What {contest} Actually Pays For)

{severity_rubic}
"#
    )
}

pub fn generate_post_verify_json_requirement(repo: &RepoPaths) -> String {
    let json = generate_verify_json(repo);

    format!(
        r#"

### OUTPUT REQUIREMENTS 
*Please respond with ONLY valid JSON in the following exact format:*

{json}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 

"#
    )
}

pub fn generate_pre_verify_json_requirement(repo: &RepoPaths) -> String {
    let json = generate_verify_json(repo);

    format!(
        r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{json}

"#
    )
}

fn generate_verify_json(repo: &RepoPaths) -> String {
    let VerifyEnumLists {
        severity_list,
        finding_status_list,
        finding_confidence_list,
    } = generate_verify_enum_lists(repo);

    format!(
        r#"
{{
    "severity": "{severity_list}",
    "severity_justification": "Explain why you assigned this severity",
    "status": "{finding_status_list}",
    "status_justification": "if invalid, out of scope, or needs more info, please explain why",
    "status_confidence": "{finding_confidence_list}",
    "status_confidence_justification": "if Somewhat Confident, please explain why",
    "finding_complexity": How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty. This value is a number (NOT a string)
}}
"#
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Test serialization of LegitVulnerability with all fields populated
    #[test]
    fn test_legit_vulnerability_serialization_complete() {
        let vuln = LegitVulnerability {
            severity: Some(Severity::High),
            severity_justification: Some("Direct fund loss possible".to_string()),
            status: FindingStatus::Valid,
            status_justification: Some("Confirmed vulnerability".to_string()),
            status_confidence: FindingConfidence::VeryConfident,
            status_confidence_justification: Some("Clear attack path demonstrated".to_string()),
            finding_complexity: 8,
        };

        let json = serde_json::to_string(&vuln).expect("Failed to serialize");
        assert!(json.contains(r#""severity":"High"#));
        assert!(json.contains(r#""status":"Valid"#));
        assert!(json.contains(r#""status_confidence":"VeryConfident"#));
        assert!(json.contains(r#""finding_complexity":8"#));
    }

    /// Test serialization with minimal fields (using defaults and None)
    #[test]
    fn test_legit_vulnerability_serialization_minimal() {
        let vuln = LegitVulnerability {
            severity: Some(Severity::Info),
            severity_justification: None,
            status: FindingStatus::Invalid,
            status_justification: None,
            status_confidence: FindingConfidence::SomeWhatConfident,
            status_confidence_justification: None,
            finding_complexity: 1,
        };

        let json = serde_json::to_string(&vuln).expect("Failed to serialize");
        assert!(json.contains(r#""severity":"Info"#));
        assert!(json.contains(r#""status":"Invalid"#));
        assert!(json.contains(r#""finding_complexity":1"#));
    }

    /// Test deserialization of clean, well-formed JSON
    #[test]
    fn test_legit_vulnerability_deserialization_clean() {
        let json = r#"{
            "severity": "High",
            "severity_justification": "Direct fund loss",
            "status": "Valid",
            "status_justification": "Confirmed",
            "status_confidence": "VeryConfident",
            "status_confidence_justification": "Clear path",
            "finding_complexity": 7
        }"#;

        let vuln: LegitVulnerability = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(vuln.severity, Some(Severity::High));
        assert_eq!(vuln.status, FindingStatus::Valid);
        assert_eq!(vuln.status_confidence, FindingConfidence::VeryConfident);
        assert_eq!(vuln.finding_complexity, 7);
    }

    /// Test deserialization with finding_complexity as string (common LLM mistake)
    #[test]
    fn test_legit_vulnerability_deserialization_complexity_as_string() {
        let json = r#"{
            "severity": "Medium",
            "severity_justification": null,
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "Confident",
            "status_confidence_justification": "Good evidence",
            "finding_complexity": "5"
        }"#;

        let vuln: LegitVulnerability = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(vuln.finding_complexity, 5);
    }

    /// Test deserialization with out-of-range finding_complexity (should clamp)
    #[test]
    fn test_legit_vulnerability_deserialization_complexity_out_of_range() {
        // Test value > 10
        let json_high = r#"{
            "severity": "Low",
            "severity_justification": null,
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "SomeWhatConfident",
            "status_confidence_justification": "",
            "finding_complexity": 15
        }"#;

        let vuln: LegitVulnerability =
            serde_json::from_str(json_high).expect("Failed to deserialize");
        assert_eq!(vuln.finding_complexity, 10); // Should clamp to 10

        // Test value < 1
        let json_low = r#"{
            "severity": "Low",
            "severity_justification": null,
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "SomeWhatConfident",
            "status_confidence_justification": "",
            "finding_complexity": 0
        }"#;

        let vuln: LegitVulnerability =
            serde_json::from_str(json_low).expect("Failed to deserialize");
        assert_eq!(vuln.finding_complexity, 1); // Should clamp to 1
    }

    /// Test all Severity enum variants (exact PascalCase as LLMs are instructed)
    #[test]
    fn test_severity_enum_deserialization_all_variants() {
        let test_cases = vec![
            (r#"{"severity": "Critical"}"#, Severity::Critical),
            (r#"{"severity": "High"}"#, Severity::High),
            (r#"{"severity": "Medium"}"#, Severity::Medium),
            (r#"{"severity": "Low"}"#, Severity::Low),
            (r#"{"severity": "Info"}"#, Severity::Info),
        ];

        for (json, expected) in test_cases {
            #[derive(Deserialize)]
            struct TestSeverity {
                severity: Severity,
            }
            let result: TestSeverity = serde_json::from_str(json)
                .unwrap_or_else(|_| panic!("Failed to deserialize: {}", json));
            assert_eq!(result.severity, expected, "Failed for JSON: {}", json);
        }
    }

    /// Test all FindingStatus enum variants
    #[test]
    fn test_finding_status_enum_all_variants() {
        let test_cases = vec![
            (r#"{"status": "Valid"}"#, FindingStatus::Valid),
            (r#"{"status": "Invalid"}"#, FindingStatus::Invalid),
            (r#"{"status": "OutOfScope"}"#, FindingStatus::OutOfScope),
            (
                r#"{"status": "NeedsMoreInfo"}"#,
                FindingStatus::NeedsMoreInfo,
            ),
        ];

        for (json, expected) in test_cases {
            #[derive(Deserialize)]
            struct TestStatus {
                status: FindingStatus,
            }
            let result: TestStatus = serde_json::from_str(json)
                .unwrap_or_else(|_| panic!("Failed to deserialize: {}", json));
            assert_eq!(result.status, expected, "Failed for JSON: {}", json);
        }
    }

    /// Test all FindingConfidence enum variants
    #[test]
    fn test_finding_confidence_enum_all_variants() {
        let test_cases = vec![
            (
                r#"{"confidence": "VeryConfident"}"#,
                FindingConfidence::VeryConfident,
            ),
            (
                r#"{"confidence": "Confident"}"#,
                FindingConfidence::Confident,
            ),
            (
                r#"{"confidence": "SomeWhatConfident"}"#,
                FindingConfidence::SomeWhatConfident,
            ),
        ];

        for (json, expected) in test_cases {
            #[derive(Deserialize)]
            struct TestConfidence {
                confidence: FindingConfidence,
            }
            let result: TestConfidence = serde_json::from_str(json)
                .unwrap_or_else(|_| panic!("Failed to deserialize: {}", json));
            assert_eq!(result.confidence, expected, "Failed for JSON: {}", json);
        }
    }

    /// Test serialization of all enum variants
    #[test]
    fn test_enum_serialization() {
        // Test Severity
        assert_eq!(
            serde_json::to_string(&Severity::Critical).unwrap(),
            r#""Critical""#
        );
        assert_eq!(serde_json::to_string(&Severity::High).unwrap(), r#""High""#);
        assert_eq!(
            serde_json::to_string(&Severity::Medium).unwrap(),
            r#""Medium""#
        );
        assert_eq!(serde_json::to_string(&Severity::Low).unwrap(), r#""Low""#);
        assert_eq!(serde_json::to_string(&Severity::Info).unwrap(), r#""Info""#);

        // Test FindingStatus
        assert_eq!(
            serde_json::to_string(&FindingStatus::Valid).unwrap(),
            r#""Valid""#
        );
        assert_eq!(
            serde_json::to_string(&FindingStatus::Invalid).unwrap(),
            r#""Invalid""#
        );
        assert_eq!(
            serde_json::to_string(&FindingStatus::OutOfScope).unwrap(),
            r#""OutOfScope""#
        );
        assert_eq!(
            serde_json::to_string(&FindingStatus::NeedsMoreInfo).unwrap(),
            r#""NeedsMoreInfo""#
        );

        // Test FindingConfidence
        assert_eq!(
            serde_json::to_string(&FindingConfidence::VeryConfident).unwrap(),
            r#""VeryConfident""#
        );
        assert_eq!(
            serde_json::to_string(&FindingConfidence::Confident).unwrap(),
            r#""Confident""#
        );
        assert_eq!(
            serde_json::to_string(&FindingConfidence::SomeWhatConfident).unwrap(),
            r#""SomeWhatConfident""#
        );
    }

    /// Test messy LLM output: extra whitespace, mixed formatting
    #[test]
    fn test_legit_vulnerability_messy_llm_output_whitespace() {
        let json = r#"
        {
            "severity"  :  "High"  ,
            "severity_justification"  :  "Direct fund loss"  ,
            "status"  :  "Valid"  ,
            "status_justification"  :  "Confirmed"  ,
            "status_confidence"  :  "VeryConfident"  ,
            "status_confidence_justification"  :  "Clear path"  ,
            "finding_complexity"  :  7
        }
        "#;

        let vuln: LegitVulnerability = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(vuln.severity, Some(Severity::High));
        assert_eq!(vuln.status, FindingStatus::Valid);
        assert_eq!(vuln.finding_complexity, 7);
    }

    /// Test messy LLM output: finding_complexity as string with whitespace
    #[test]
    fn test_legit_vulnerability_messy_complexity_string_whitespace() {
        let json = r#"{
            "severity": "Medium",
            "severity_justification": null,
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "Confident",
            "status_confidence_justification": "Good",
            "finding_complexity": " 6 "
        }"#;

        // Note: This will fail because serde doesn't trim strings before parsing
        // This is expected behavior - the clean_json_string in findings.rs should handle this
        let result: std::result::Result<LegitVulnerability, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "Should fail with whitespace in numeric string"
        );
    }

    /// Test LLM output with informational severity (alternative spelling)
    #[test]
    fn test_legit_vulnerability_informational_severity() {
        // Note: "informational" is mentioned in prompts but Severity enum only has "Info"
        let json = r#"{
            "severity": "Info",
            "severity_justification": "Best practice",
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "Confident",
            "status_confidence_justification": "Standard pattern",
            "finding_complexity": 2
        }"#;

        let vuln: LegitVulnerability = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(vuln.severity, Some(Severity::Info));
    }

    /// Test round-trip serialization/deserialization
    #[test]
    fn test_legit_vulnerability_round_trip() {
        let original = LegitVulnerability {
            severity: Some(Severity::Critical),
            severity_justification: Some("Complete protocol takeover".to_string()),
            status: FindingStatus::Valid,
            status_justification: Some("Exploit confirmed in tests".to_string()),
            status_confidence: FindingConfidence::VeryConfident,
            status_confidence_justification: Some("PoC provided".to_string()),
            finding_complexity: 9,
        };

        // Serialize
        let json = serde_json::to_string(&original).expect("Failed to serialize");

        // Deserialize
        let deserialized: LegitVulnerability =
            serde_json::from_str(&json).expect("Failed to deserialize");

        // Compare
        assert_eq!(deserialized.severity, original.severity);
        assert_eq!(
            deserialized.severity_justification,
            original.severity_justification
        );
        assert_eq!(deserialized.status, original.status);
        assert_eq!(
            deserialized.status_justification,
            original.status_justification
        );
        assert_eq!(deserialized.status_confidence, original.status_confidence);
        assert_eq!(
            deserialized.status_confidence_justification,
            original.status_confidence_justification
        );
        assert_eq!(deserialized.finding_complexity, original.finding_complexity);
    }

    /// Test default values
    #[test]
    fn test_legit_vulnerability_defaults() {
        let vuln = LegitVulnerability::default();
        assert_eq!(vuln.severity, None); // Option<Severity> defaults to None
        assert_eq!(vuln.status, FindingStatus::Invalid); // Default from FindingStatus enum
        assert_eq!(vuln.status_confidence, FindingConfidence::SomeWhatConfident); // Default from FindingConfidence enum
        assert_eq!(vuln.finding_complexity, 0); // Default u8
    }

    /// Test prompt generation for all audit types
    #[test]
    fn test_verify_prompt_generation_all_audit_types() {
        use crate::config::AuditType;
        use crate::prepare_code::git_clone::PocConfig;
        use std::path::PathBuf;

        println!("\n{}", "=".repeat(80));
        println!("VERIFY FINDINGS PROMPT GENERATION TEST");
        println!("{}\n", "=".repeat(80));

        let audit_types = vec![
            AuditType::Code4rena,
            AuditType::Sherlock,
            AuditType::Cantina,
            AuditType::Client,
        ];

        for audit_type in audit_types {
            println!("\n{}", "-".repeat(80));
            println!("AUDIT TYPE: {:?}", audit_type);
            println!("{}\n", "-".repeat(80));

            let repo = RepoPaths {
                github_url: "https://github.com/test/repo".to_string(),
                project_id: "test-project".to_string(),
                root: PathBuf::from("/tmp"),
                sol_files: vec![],
                test_files: vec![],
                script_files: vec![],
                config_files: vec![],
                lib_config_files: vec![],
                source_code_folders: vec![],
                docs: vec![],
                repo_name: "test-repo".to_string(),
                audit_scope: None,
                excluded_folders: None,
                scoped_files: None,
                monorepo_folders: None,
                commit_hash: "abc123".to_string(),
                audit_type,
                poc: PocConfig::default(),
            };

            // Test 1: Pre-verify JSON requirement
            println!("📋 PRE-VERIFY JSON REQUIREMENT:");
            println!("{}", "-".repeat(80));
            let pre_json = generate_pre_verify_json_requirement(&repo);
            println!("{}", pre_json);

            // Test 2: Main verify prompt
            println!("\n📝 MAIN VERIFY PROMPT:");
            println!("{}", "-".repeat(80));
            let verify_prompt = generate_verify_prompt(&repo);
            println!("{}", verify_prompt);

            // Test 3: Post-verify JSON requirement
            println!("\n📋 POST-VERIFY JSON REQUIREMENT:");
            println!("{}", "-".repeat(80));
            let post_json = generate_post_verify_json_requirement(&repo);
            println!("{}", post_json);

            // Test 4: Verify JSON structure
            println!("\n🔧 VERIFY JSON STRUCTURE:");
            println!("{}", "-".repeat(80));
            let json_structure = generate_verify_json(&repo);
            println!("{}", json_structure);

            println!("\n");
        }

        println!("\n{}", "=".repeat(80));
        println!("END OF VERIFY FINDINGS PROMPT GENERATION TEST");
        println!("{}\n", "=".repeat(80));
    }

    /// Test boundary values for finding_complexity
    #[test]
    fn test_finding_complexity_boundary_values() {
        // Test minimum valid value
        let json_min = r#"{
            "severity": "Low",
            "severity_justification": null,
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "Confident",
            "status_confidence_justification": "",
            "finding_complexity": 1
        }"#;
        let vuln: LegitVulnerability =
            serde_json::from_str(json_min).expect("Failed to deserialize");
        assert_eq!(vuln.finding_complexity, 1);

        // Test maximum valid value
        let json_max = r#"{
            "severity": "Low",
            "severity_justification": null,
            "status": "Valid",
            "status_justification": null,
            "status_confidence": "Confident",
            "status_confidence_justification": "",
            "finding_complexity": 10
        }"#;
        let vuln: LegitVulnerability =
            serde_json::from_str(json_max).expect("Failed to deserialize");
        assert_eq!(vuln.finding_complexity, 10);
    }

    /// Test OutOfScope status
    #[test]
    fn test_legit_vulnerability_out_of_scope() {
        let json = r#"{
            "severity": "High",
            "severity_justification": "Would be high if in scope",
            "status": "OutOfScope",
            "status_justification": "Finding is in external library",
            "status_confidence": "VeryConfident",
            "status_confidence_justification": "Clearly out of audit scope",
            "finding_complexity": 5
        }"#;

        let vuln: LegitVulnerability = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(vuln.status, FindingStatus::OutOfScope);
        assert_eq!(
            vuln.status_justification,
            Some("Finding is in external library".to_string())
        );
    }

    /// Test NeedsMoreInfo status
    #[test]
    fn test_legit_vulnerability_needs_more_info() {
        let json = r#"{
            "severity": "Medium",
            "severity_justification": "Potential impact unclear",
            "status": "NeedsMoreInfo",
            "status_justification": "Cannot determine if external dependency is vulnerable",
            "status_confidence": "SomeWhatConfident",
            "status_confidence_justification": "Missing context about external system",
            "finding_complexity": 4
        }"#;

        let vuln: LegitVulnerability = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(vuln.status, FindingStatus::NeedsMoreInfo);
        assert_eq!(vuln.status_confidence, FindingConfidence::SomeWhatConfident);
    }
}
