use crate::llm_review::phases::rounds::utils::generate_pre_round_verify_json_requirement;
use crate::llm_review::phases::verify_rounds::{FindingAnalysis, FindingStatus};
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoundTwoLegitAnalysis {
    pub finding_id: String,
    pub finding_title: String,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub does_bug_exist: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub safeguard_against_it: bool,
    pub justification: String,
}

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VerifyRoundTwo {
    pub findings: Vec<RoundTwoLegitAnalysis>,
}

impl FindingAnalysis for RoundTwoLegitAnalysis {
    fn id(&self) -> String {
        self.finding_id.clone()
    }
    fn get_justification(&self) -> String {
        self.justification.clone()
    }
    fn get_finding_status_array_from_analysis(&self) -> Option<Vec<FindingStatus>> {
        let mut finding_status_vec = Vec::new();

        if !self.does_bug_exist {
            finding_status_vec.push(FindingStatus::InvalidBugDoesNotExist);
        }
        if self.safeguard_against_it {
            finding_status_vec.push(FindingStatus::InvalidSafeGuardInPlace);
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
        // info!("does bug exist: {}\n", self.does_bug_exist);
        // info!("is safeguard against it: {}\n", self.safeguard_against_it);
        info!("justification: {}\n", self.justification);
        info!("\n");
    }

    fn generate_verify_prompt() -> String {
        let verify_json = RoundTwoLegitAnalysis::generate_verify_json();
        let pre_verify_json = generate_pre_round_verify_json_requirement(&verify_json);

        format!(
            r#"
        {pre_verify_json}

    Your task: to run the following 2 checks on EACH listed security finding: 

    ## VERIFY SECURITY FINDING EXISTS

    **Step 1: Trace the Code Path**
    - Locate exact function/contract, verify vulnerable code path exists (not hallucinated)
    - Trace execution flow step-by-step, check if code matches finding's description

    **Bug does not exist if:** Function/contract doesn't exist, code path impossible, execution flow doesn't match

    **Step 2: Verify Invariant Actually Exists**
    - Check if claimed invariant is documented (NatSpec, comments, docs)
    - Verify invariant is enforced elsewhere, confirm it's a real protocol requirement

    **Bug does not exist if:** Invariant not documented, not enforced elsewhere, assumed but not required

    **Step 3: Reproduce the Issue**
    - Can you trace exact steps to trigger the bug? 

    **Bug does not exist if:** Cannot trace execution path, preconditions impossible

    ## EXISTING SAFEGUARDS CHECK: Does code already have safeguards that mitigate/eliminate this security finding?**

    **Common safeguards**
    -  Reentrancy → has `nonReentrant`, CEI pattern, or guard
    -  Integer overflow → Solidity 0.8+ with built-in checks
    -  Access control → has `onlyOwner`, `onlyRole`, role checks
    -  Front-running → has commit-reveal, deadlines, slippage protection
    -  Oracle manipulation → has TWAP, multiple sources, price bounds
    -  DoS → has pagination, gas limits, circuit breakers
    -  Precision loss → has proper scaling, rounding checks

    **How to Check:** Search codebase for modifiers/guards, verify vulnerable function uses them, test if bypassable


    ## OUTPUT REQUIREMENTS

    Based on your assessment please provided the following for EACH finding:

    *finding id*: insert finding id (from 'id' field)
    *finding title*: insert finding title
    *does bug exist*: true | false
    *safeguard against it*: true | false
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
                    "does_bug_exist": true | false,
                    "safeguard_against_it": true | false,
                    "justification": "Please provide justification for your choices (under 200 words)."
                }}
            ]
        }}
"#
        )
    }
}
