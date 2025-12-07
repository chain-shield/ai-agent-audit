use crate::llm_review::phases::rounds::utils::generate_pre_round_verify_json_requirement;
use crate::llm_review::phases::verify_rounds::{FindingAnalysis, FindingStatus};
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
use log::info;

use crate::llm_review::agent::agent_enums::{all_enum_variants, generate_enum_list};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    JsonSchema,
    EnumIter,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum Impact {
    High,
    #[default]
    Medium,
    Low,
}

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    JsonSchema,
    EnumIter,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
)]
pub enum Likelihood {
    Common,
    #[default]
    Occasional,
    Rare,
}

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoundOneLegitAnalysis {
    pub finding_id: String,
    pub finding_title: String,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub user_error_or_mistake: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub governance_risk: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub future_speculation: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub non_standard_token: bool,
    pub impact: Impact,
    pub likelihood: Likelihood,
    pub justification: String,
}

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VerifyRoundOne {
    pub findings: Vec<RoundOneLegitAnalysis>,
}

impl FindingAnalysis for RoundOneLegitAnalysis {
    fn id(&self) -> String {
        self.finding_id.clone()
    }
    fn get_justification(&self) -> String {
        self.justification.clone()
    }
    fn get_finding_status_array_from_analysis(&self) -> Option<Vec<FindingStatus>> {
        let mut finding_status_vec = Vec::new();

        if self.user_error_or_mistake {
            finding_status_vec.push(FindingStatus::InvalidUserErrorOrMistake);
        }
        if self.governance_risk {
            finding_status_vec.push(FindingStatus::InvalidGoveranaceRisk);
        }
        if self.future_speculation {
            finding_status_vec.push(FindingStatus::InvalidFutureSpeculation);
        }
        if self.non_standard_token {
            finding_status_vec.push(FindingStatus::InvalidERC20EdgeCase);
        }
        if self.impact == Impact::Low {
            finding_status_vec.push(FindingStatus::LowSeverityDueToLowImpact);
        }
        if self.likelihood == Likelihood::Rare {
            finding_status_vec.push(FindingStatus::LowSeverityDueToRareLikelihood);
        }

        if finding_status_vec.is_empty() {
            None
        } else {
            Some(finding_status_vec)
        }
    }

    fn print_analysis_results(&self) {
        info!("\n");
        info!("{}: \n", self.finding_title);
        info!("is user error or mistake: {}\n", self.user_error_or_mistake);
        info!("is governance_risk: {}\n", self.governance_risk);
        info!("is future speculation: {}\n", self.future_speculation);
        info!("is non standard token: {}\n", self.non_standard_token);
        info!("justification: {}\n", self.justification);
        info!("\n");
    }

    fn generate_verify_prompt() -> String {
        let verify_json = RoundOneLegitAnalysis::generate_verify_json();
        let pre_verify_json = generate_pre_round_verify_json_requirement(&verify_json);
        let likelihood_list = generate_enum_list(all_enum_variants::<Likelihood>().as_slice());
        let impact_list = generate_enum_list(all_enum_variants::<Impact>().as_slice());

        format!(
            r#"
        {pre_verify_json}

    Your task: to run the following 6 checks on EACH listed security finding: 

    ## USER ERROR CHECK: Does Security Finding require User error or Mistake?

    - User chooses bad recipient, provides bad parameters, approves malicious contract, signs malicious data, etc.

    ## IMPACT CLASSIFICATION CHECK: What is objective impact of Security Finding?

    **High:** Theft/permanent loss of assets, unauthorized drains, economic attacks (non-dust)
    **Medium:** DoS of critical actions, accounting drift, mispricing, privilege escalation
    **Low:** Dust amounts, stylistic issues, event inconsistencies, view-function errors

    ## LIKELIHOOD ASSESSMENT CHECK: What is likelihood of Security Finding being exploited?

    **Common:** No preconditions, works anytime/anywhere, no special resources
    **Occasional:** Specific but realistic conditions, some chains, moderate setup
    **Rare:** Multiple unlikely conditions, extreme market states, significant resources

    ## GOVERNANCE/CENTRALIZATION RISK

    **Question: Can governance/team prevent this by acting responsibly?**

    **Security Finding is Governance Risk if:**
    - Admin sets wrong parameters, chooses malicious oracle, misconfigures
    - Team deploys on wrong chain, doesn't verify addresses
    - Team chooses malicious integration, configures incorrectly
    - Security Finding can we remedied by Admin/privileged Users making different choices/decisions (without code changes or redeploying, of course)

    ---

    ## NON-STANDARD ERC20 TOKEN CHECK: Does Security Finding depending on contract interaction with Non-standard token?

    - Fee-on-transfer/rebasing/decimals edge cases (unless explicitly supported or USDT)

    ---

    ## SPECULATION CHECK: Is Security Finding dependin on future state of the code?

    **Question: Does root cause exist NOW and is exploitable with TODAY's code?**

    **Security Finding is future speculation if:**
    -  "Finding depends on if protocol integrates/adds/upgrades in future..."
    -  "Could/might/potentially happen if..." (hypothetical)

    # OUTPUT REQUIREMENTS

    Based on your assessment please provided the following for EACH finding:

    *finding id*: insert finding id (from 'id' field)
    *finding title*: insert finding title
    *user error or mistake*: true | false
    *governance risk*: true | false
    *future speculation*: true | false
    *non standard token*: true | false
    *impact*: {impact_list}
    *likelihood*: {likelihood_list}
    *justification:*: Please provide justification for your choices (under 200 words)

"#
        )
    }

    fn generate_verify_json() -> String {
        let likelihood_list = generate_enum_list(all_enum_variants::<Likelihood>().as_slice());
        let impact_list = generate_enum_list(all_enum_variants::<Impact>().as_slice());
        format!(
            r#"
    [
        {{
            "finding_id: "'id' field from finding",
            "finding_title: "'title' field from finding",
            "user_error_or_mistake": true | false,
            "governance_risk": true | false,
            "future_speculation": true | false,
            "non_standard_token": true | false,
            "impact": "{impact_list}",
            "likelihood": "{likelihood_list}",
            "justification": "Please provide justification for your choices (under 200 words)."
        }}
    ]
    "#
        )
    }
}
