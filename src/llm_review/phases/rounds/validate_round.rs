use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::llm_review::{
    findings::findings::{Finding, Findings},
    phases::verify_rounds::FindingStatus,
    utils::prompt_context::{get_finding_report, FindingReportType},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FindingDowngradeValidation {
    pub findings: Vec<ValidateLegitAnalysis>,
}

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidateLegitAnalysis {
    pub finding_id: String,
    pub finding_title: String,
    pub is_really_user_error_or_mistake: Option<bool>,
    pub is_really_governance_risk: Option<bool>,
    pub is_really_future_speculation: Option<bool>,
    pub is_really_non_standard_token: Option<bool>,
    pub is_really_low_impact: Option<bool>,
    pub is_really_low_likelihood: Option<bool>,
    pub does_bug_really_not_exist: Option<bool>,
    pub is_there_really_safeguard_against_it: Option<bool>,
    pub is_bug_really_by_design: Option<bool>,
    pub is_really_out_of_scope: Option<bool>,
    pub is_really_not_exploitable: Option<bool>,
    pub justification: Option<String>,
}

impl ValidateLegitAnalysis {
    pub fn id(&self) -> String {
        self.finding_id.clone()
    }

    pub fn print_analysis_results(&self) {
        if self.justification.is_some() {
            info!("\n");
            info!("id: {}\n", self.id());
            info!("title: {}\n", self.finding_title);

            if self.does_bug_really_not_exist.is_some() {
                info!(
                    "does_bug_really_not_exist: {}\n",
                    self.does_bug_really_not_exist.unwrap()
                );
            }

            if self.is_really_out_of_scope.is_some() {
                info!(
                    "is_really_out_of_scope: {}\n",
                    self.is_really_out_of_scope.unwrap()
                );
            }

            if self.is_really_user_error_or_mistake.is_some() {
                info!(
                    "is_really_user_error_or_mistake: {}\n",
                    self.is_really_user_error_or_mistake.unwrap()
                );
            }

            if self.is_really_governance_risk.is_some() {
                info!(
                    "is_really_governance_risk: {}\n",
                    self.is_really_governance_risk.unwrap()
                );
            }

            if self.is_really_future_speculation.is_some() {
                info!(
                    "is_really_future_speculation: {}\n",
                    self.is_really_future_speculation.unwrap()
                );
            }

            if self.is_really_non_standard_token.is_some() {
                info!(
                    "is_really_non_standard_token: {}\n",
                    self.is_really_non_standard_token.unwrap()
                );
            }

            if self.is_really_low_impact.is_some() {
                info!(
                    "is_really_low_impact: {}\n",
                    self.is_really_low_impact.unwrap()
                );
            }

            if self.is_really_low_likelihood.is_some() {
                info!(
                    "is_really_low_likelihood: {}\n",
                    self.is_really_low_likelihood.unwrap()
                );
            }

            if self.is_there_really_safeguard_against_it.is_some() {
                info!(
                    "is_there_really_safeguard_against_it: {}\n",
                    self.is_there_really_safeguard_against_it.unwrap()
                );
            }

            if self.is_bug_really_by_design.is_some() {
                info!(
                    "is_bug_really_by_design: {}\n",
                    self.is_bug_really_by_design.unwrap()
                );
            }

            if self.is_really_not_exploitable.is_some() {
                info!(
                    "is_really_not_exploitable: {}\n",
                    self.is_really_not_exploitable.unwrap()
                );
            }

            info!(
                "justification: {}\n",
                self.justification.clone().unwrap_or_default()
            );
            info!("\n");
        }
    }

    pub fn get_fixed_finding_status(&self, finding: &Finding) -> Option<Vec<FindingStatus>> {
        let mut fixed_finding_status = Vec::new();

        //sanity check
        if finding.status.is_none() {
            return None;
        } else if finding
            .status
            .clone()
            .is_some_and(|s| s.contains(&FindingStatus::Valid))
        {
            return finding.status.clone();
        }

        let status_array = finding.status.clone().unwrap();

        for status in &status_array {
            match status {
                FindingStatus::InvalidBugDoesNotExist
                    if self.does_bug_really_not_exist.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidBugDoesNotExist)
                }
                FindingStatus::InvalidOutOfScope
                    if self.is_really_out_of_scope.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidOutOfScope)
                }
                FindingStatus::InvalidUserErrorOrMistake
                    if self.is_really_user_error_or_mistake.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidUserErrorOrMistake)
                }
                FindingStatus::InvalidGovernanceRisk
                    if self.is_really_governance_risk.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidGovernanceRisk)
                }
                FindingStatus::InvalidERC20EdgeCase
                    if self.is_really_non_standard_token.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidERC20EdgeCase)
                }
                FindingStatus::InvalidNotExploitable
                    if self.is_really_not_exploitable.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidNotExploitable)
                }
                FindingStatus::InvalidFutureSpeculation
                    if self.is_really_future_speculation.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidFutureSpeculation)
                }
                FindingStatus::InvalidByDesign
                    if self.is_bug_really_by_design.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidByDesign)
                }
                FindingStatus::InvalidSafeGuardInPlace
                    if self.is_there_really_safeguard_against_it.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::InvalidSafeGuardInPlace)
                }
                FindingStatus::LowSeverityDueToLowImpact
                    if self.is_really_low_impact.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::LowSeverityDueToLowImpact)
                }
                FindingStatus::LowSeverityDueToRareLikelihood
                    if self.is_really_low_likelihood.is_some_and(|v| v) =>
                {
                    fixed_finding_status.push(FindingStatus::LowSeverityDueToRareLikelihood)
                }
                // Note: FindingStatus::Valid, InvalidOtherReason, and NeedsMoreInfo are deprecated
                // and will not appear in validation (Valid findings are filtered out before validation)
                _ => {}
            }
        }

        if fixed_finding_status.is_empty() {
            None
        } else {
            Some(fixed_finding_status)
        }
    }

    // for given finding get bullet description of each finding status, and a json field
    // corresponding to a ValidateLegitAnalysis field
    fn extract_finding_status_info(finding: &Finding) -> (String, String) {
        // sanity check
        if finding.status.is_none() {
            return (String::new(), String::new());
        }
        let mut json_fields = String::new();
        let mut status_list = String::new();

        let finding_status = finding.status.as_ref().unwrap();

        for status in finding_status {
            let (status, field) = match status {
                FindingStatus::InvalidBugDoesNotExist => (
                    "- finding does not exist".to_string(),
                    "\"does_bug_really_not_exist\": true | false".to_string(),
                ),
                FindingStatus::InvalidOutOfScope => (
                    "- finding is out of scope".to_string(),
                    "\"is_really_out_of_scope\": true | false".to_string(),
                ),
                FindingStatus::InvalidUserErrorOrMistake => (
                    "- finding results from user error or mistake".to_string(),
                    "\"is_really_user_error_or_mistake\": true | false".to_string(),
                ),
                FindingStatus::InvalidGovernanceRisk => (
                    "- finding is governance risk, resulting from admin/privileged user error or mistake"
                        .to_string(),
                    "\"is_really_governance_risk\": true | false".to_string(),
                ),
                FindingStatus::InvalidERC20EdgeCase => (
                    "- finding requires non standard ERC20 token other than USDT"
                        .to_string(),
                    "\"is_really_non_standard_token\": true | false".to_string(),
                ),
                FindingStatus::InvalidNotExploitable => (
                    "- finding is not exploitable"
                        .to_string(),
                    "\"is_really_not_exploitable\": true | false".to_string(),
                ),
                FindingStatus::InvalidFutureSpeculation => (
                    "- finding only exists is possible future state of code or post upgrade"
                        .to_string(),
                    "\"is_really_future_speculation\": true | false".to_string(),
                ),
                FindingStatus::InvalidByDesign => (
                    "- finding is actually by design, therefore not real vulnerability"
                        .to_string(),
                    "\"is_bug_really_by_design\": true | false".to_string(),
                ),
                FindingStatus::InvalidSafeGuardInPlace => (
                    "- finding is invalid because there is a safeguard in place against it"
                        .to_string(),
                    "\"is_there_really_safeguard_against_it\": true | false".to_string(),
                ),
                FindingStatus::LowSeverityDueToLowImpact => (
                    "- finding is low impact, and therefore low severity"
                        .to_string(),
                    "\"is_really_low_impact\": true | false".to_string(),
                ),
                FindingStatus::LowSeverityDueToRareLikelihood => (
                    "- finding has rare likelihood of occuring, and therefore low severity"
                        .to_string(),
                    "\"is_really_low_likelihood\": true | false".to_string(),
                ),
                // Note: FindingStatus::Valid, InvalidOtherReason, and NeedsMoreInfo are deprecated
                // and will not appear in validation (Valid findings are filtered out before validation)
                _ => (String::new(), String::new())
            };

            if !status.is_empty() {
                status_list.push_str(&status);
                status_list.push_str("\n");
            }

            if !field.is_empty() {
                json_fields.push_str(&field);
                json_fields.push_str("\n");
            }
        }

        (status_list, json_fields)
    }
}

pub fn generate_round_validation_prompt(findings: &Findings) -> String {
    let mut prompt = String::new();
    let finding_count = findings.findings.len();

    prompt.push_str(&format!(r#"
        
        Your task is the evaluate EACH of the below triaged {finding_count} security findings.
        For 1 or more reasons each security finding has been downgraded to low severity or invalid.

        For each and every finding, your job is to evaluate the validity of each reason each finding was downgraded
        (governance risk, does not exist, user mistake, future speculation, etc..), and mark each reason as true or false.

        # Security Findings + Reasons They were Downgraded

        "#));

    for (num, finding) in findings.findings.iter().enumerate() {
        let (reasons_finding_downgraded, _) =
            ValidateLegitAnalysis::extract_finding_status_info(finding);
        let finding_report = get_finding_report(finding, None, FindingReportType::NoPoC);
        let id = finding.id.clone().unwrap_or_default();
        let downgrade_justifcation = finding.status_justification.clone().unwrap_or_default();

        prompt.push_str(&format!(
            r#"
        
                ## Below Security Findings was Downgraded for Following reasons
                {reasons_finding_downgraded}

                ### Justifcation for downgrade
                {downgrade_justifcation}

                ## Security Finding #{num}, id: {id}

                {finding_report}

                "#
        ));
    }

    prompt
}

pub fn generate_dynamic_validation_json(findings: &Findings) -> String {
    let mut json = r#"

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {{
            "findings": [
        "#
    .to_string();

    let last_finding = findings.findings.len() - 1;

    for (num, finding) in findings.findings.iter().enumerate() {
        let (_, json_fields) = ValidateLegitAnalysis::extract_finding_status_info(finding);

        json.push_str(
        &format!(
                r#"
                        {{
                            "finding_id": "'id' field from finding",
                            "finding_title": "'title' field from finding",
                            {json_fields}
                            "justification": "For any field marked as false, please provide brief justification. If all fields set to true, omit justification."
                "#)
            );

        if num < last_finding {
            json.push_str("}},");
        } else {
            json.push_str(
                r#"
                    }}
                    ]
                }}

        - all fields must be set to true or false base on your analysis, if in doubt set as true.
        
        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON. 
        **Please double-check opening and closing brackets: `}}` and `]`, make sure 
        they match up correctly.
        "#,
            );
        }
    }

    json
}
