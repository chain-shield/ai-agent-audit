use crate::llm_review::phases::rounds::utils::generate_pre_round_verify_json_requirement;
use crate::llm_review::phases::verify_rounds::{FindingAnalysis, FindingStatus};
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoundThreeLegitAnalysis {
    pub finding_id: String,
    pub finding_title: String,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub by_design: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub in_scope: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub exploitable: bool,
    pub justification: String,
}

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VerifyRoundThree {
    pub findings: Vec<RoundThreeLegitAnalysis>,
}

impl FindingAnalysis for RoundThreeLegitAnalysis {
    fn id(&self) -> String {
        self.finding_id.clone()
    }
    fn get_justification(&self) -> String {
        self.justification.clone()
    }
    fn get_finding_status_array_from_analysis(&self) -> Option<Vec<FindingStatus>> {
        let mut finding_status_vec = Vec::new();

        if self.by_design {
            finding_status_vec.push(FindingStatus::InvalidByDesign);
        }
        if !self.exploitable {
            finding_status_vec.push(FindingStatus::InvalidNotExploitable);
        }
        if !self.in_scope {
            finding_status_vec.push(FindingStatus::InvalidOutOfScope);
        }

        if finding_status_vec.is_empty() {
            None
        } else {
            Some(finding_status_vec)
        }
    }

    fn print_analysis_results(&self) {
        info!("\n");
        // info!("{}: \n", self.finding_title);
        // info!("is by_design: {}\n", self.by_design);
        // info!("is in scope: {}\n", self.in_scope);
        // info!("is exploitable: {}\n", self.exploitable);
        info!("justification: {}\n", self.justification);
        info!("\n");
    }

    fn generate_verify_prompt() -> String {
        let verify_json = RoundThreeLegitAnalysis::generate_verify_json();
        let pre_verify_json = generate_pre_round_verify_json_requirement(&verify_json);

        format!(
            r#"
        {pre_verify_json}

    Your task: to run the following 3 checks on EACH listed security finding: 

    ## SCOPE CHECK: Is Security Finding in Scope?

    **Check if finding is in scope in accordance with scope provided below.**

    ## "BY DESIGN" CHECK

    **Is this Security Finding really just documented as intentional behavior of protocol?**

    Carefully check NatSpec, comments, docs, function naming.

    ##  EXPLOITABILITY: Is this Security Finding exploitable?

    - Is it possible to produce a Minimal reproducible PoC showing state change, non-dust effect, realistic actors, validating the Security Finding?

    ## OUTPUT REQUIREMENTS

    Based on your assessment please provided the following for EACH finding:

    *finding id*: insert finding id (from 'id' field)
    *by design*: true | false
    *in scope*: true | false
    *exploitable*: true | false
    *justification:*: Please provide justification for your choices (under 200 words)

"#
        )
    }

    fn generate_verify_json() -> String {
        format!(
            r#"
    {{
        "findings": [
            {{
                "finding_id": "'id' field from finding",
                "finding_title": "'title' field from finding",
                "by_design": true | false,
                "in_scope": true | false,
                "exploitable": true | false,
                "justification": "Please provide justification for your choices (under 200 words)."
            }}
        ]
    }}
"#
        )
    }
}
