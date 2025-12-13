use crate::llm_review::agent::agent_enums::{all_enum_variants, generate_enum_list};
use crate::llm_review::phases::rounds::round_1::{Impact, Likelihood};
use crate::llm_review::phases::rounds::utils::generate_pre_round_verify_json_requirement;
use crate::llm_review::phases::verify_rounds::{FindingAnalysis, FindingStatus};
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Verification result for a potential vulnerability
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AllRoundLegitAnalysis {
    pub finding_id: String,
    pub finding_title: String,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub does_bug_exist: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub safeguard_against_it: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub by_design: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub in_scope: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub exploitable: bool,
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
pub struct VerifyAllRound {
    pub findings: Vec<AllRoundLegitAnalysis>,
}

impl FindingAnalysis for AllRoundLegitAnalysis {
    fn round_number() -> usize {
        3
    }
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

        if self.by_design {
            finding_status_vec.push(FindingStatus::InvalidByDesign);
        }
        if !self.exploitable {
            finding_status_vec.push(FindingStatus::InvalidNotExploitable);
        }
        if !self.in_scope {
            finding_status_vec.push(FindingStatus::InvalidOutOfScope);
        }

        if self.user_error_or_mistake {
            finding_status_vec.push(FindingStatus::InvalidUserErrorOrMistake);
        }
        if self.governance_risk {
            finding_status_vec.push(FindingStatus::InvalidGovernanceRisk);
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
        info!("id: {}\n", self.id());
        info!("does bug exist: {}\n", self.does_bug_exist);
        info!("is safeguard against it: {}\n", self.safeguard_against_it);
        info!("is by design: {}\n", self.by_design);
        info!("is in scope: {}\n", self.in_scope);
        info!("is exploitable: {}\n", self.exploitable);
        info!("Impact: {}\n", self.impact.to_string());
        info!("Likelihood: {}\n", self.likelihood.to_string());
        info!("is user error or mistake: {}\n", self.user_error_or_mistake);
        info!("is governance_risk: {}\n", self.governance_risk);
        info!("is future speculation: {}\n", self.future_speculation);
        info!("is non standard token: {}\n", self.non_standard_token);
        info!("Justification: {}\n", self.justification);
        info!("\n");
    }

    fn generate_verify_prompt() -> String {
        let verify_json = AllRoundLegitAnalysis::generate_verify_json();
        let pre_verify_json = generate_pre_round_verify_json_requirement(&verify_json);
        let likelihood_list = generate_enum_list(all_enum_variants::<Likelihood>().as_slice());
        let impact_list = generate_enum_list(all_enum_variants::<Impact>().as_slice());

        format!(
            r#"
        {pre_verify_json}

        Your task: to run the following 11 checks on EACH listed security finding: 

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

        ## SCOPE CHECK: Is Security Finding in Scope?

        **Check if finding is in scope in accordance with scope provided below.**

        ## "BY DESIGN" CHECK

        **Is this Security Finding really just documented as intentional behavior of protocol?**

        Carefully check NatSpec, comments, docs, function naming.

        ## EXPLOITABILITY: Is this Security Finding exploitable?

        - Is it possible to produce a Minimal reproducible PoC showing state change, non-dust effect, realistic actors, validating the Security Finding?

        ## IMPACT CLASSIFICATION CHECK: What is objective impact of Security Finding?

        **High:** Theft/permanent loss of assets, unauthorized drains, economic attacks (non-dust)
        **Medium:** DoS of critical actions, accounting drift, mispricing, privilege escalation
        **Low:** Dust amounts, stylistic issues, event inconsistencies, view-function errors

        ## LIKELIHOOD ASSESSMENT CHECK: What is likelihood of Security Finding being exploited?

        **Common:** No preconditions, works anytime/anywhere, no special resources
        **Occasional:** Specific but realistic conditions, some chains, moderate setup
        **Rare:** Multiple unlikely conditions, extreme market states, significant resources

        ## USER ERROR CHECK: Does Security Finding require User error or Mistake?

        - User chooses bad recipient, provides bad parameters, approves malicious contract, signs malicious data, etc.

        ## GOVERNANCE/CENTRALIZATION RISK

        **Question: Can governance/team prevent this by acting responsibly?**

        **Security Finding is Governance Risk if:**
        - Admin sets wrong parameters, chooses malicious oracle, misconfigures
        - Team deploys on wrong chain, doesn't verify addresses
        - Team chooses malicious integration, configures incorrectly
        - Security Finding can we remedied by Admin/privileged Users making different choices/decisions (without code changes or redeploying, of course)

        ## SPECULATION CHECK: Is Security Finding dependin on future state of the code?

        **Question: Does root cause exist NOW and is exploitable with TODAY's code?**

        **Security Finding is future speculation if:**
        -  "Finding depends on if protocol integrates/adds/upgrades in future..."
        -  "Could/might/potentially happen if..." (hypothetical)

        ## NON-STANDARD ERC20 TOKEN CHECK: Does Security Finding depending on contract interaction with Non-standard token?

        - Fee-on-transfer/rebasing/decimals edge cases (unless explicitly supported or USDT)

        ## OUTPUT REQUIREMENTS

        Please continue until you have carefully evaulated ALL findings on EACH of the 11 checks.

        Based on your assessment please provided the following for EACH finding:

        *finding id*: insert finding id (from 'id' field)
        *finding_title*: insert finding 'title'
        *does bug exist*: true | false
        *safeguard against it*: true | false
        *in scope*: true | false
        *by design*: true | false
        *exploitable*: true | false
        *impact*: {impact_list}
        *likelihood*: {likelihood_list}
        *user error or mistake*: true | false
        *governance risk*: true | false
        *future speculation*: true | false
        *non standard token*: true | false
        *justification:*: Please provide justification for your choices (under 400 words)

"#
        )
    }

    fn generate_verify_json() -> String {
        let likelihood_list = generate_enum_list(all_enum_variants::<Likelihood>().as_slice());
        let impact_list = generate_enum_list(all_enum_variants::<Impact>().as_slice());
        format!(
            r#"
    {{
        "findings": [
            {{
                "finding_id": "'id' field from finding",
                "finding_title": "'title' field from finding",
                "does_bug_exist": true | false,
                "safeguard_against_it": true | false,
                "in_scope": true | false,
                "by_design": true | false,
                "exploitable": true | false,
                "impact": "{impact_list}",
                "likelihood": "{likelihood_list}",
                "user_error_or_mistake": true | false,
                "governance_risk": true | false,
                "future_speculation": true | false,
                "non_standard_token": true | false,
                "justification": "Please provide justification for your choices (under 400 words)."
            }}
        ]
    }}
"#
        )
    }
}
