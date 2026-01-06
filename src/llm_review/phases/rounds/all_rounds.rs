use crate::llm_review::agent::agent_enums::{all_enum_variants, generate_enum_list};
use crate::llm_review::dynamic_prompts::prompt_index;
use crate::llm_review::phases::verify_rounds::{FindingAnalysis, FindingStatus};
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
use log::info;

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
        let likelihood_list = generate_enum_list(all_enum_variants::<Likelihood>().as_slice());
        let toc = prompt_index::generate_verification_round_toc();
        let impact_list = generate_enum_list(all_enum_variants::<Impact>().as_slice());
        let section_1_header = prompt_index::generated_section_header("CORE INSTRUCTIONS", 1);
        let section_1_1 =
            prompt_index::generated_sub_header("VERIFY SECURITY FINDING EXISTS", 1, 1);
        let section_1_2 = prompt_index::generated_sub_header("EXISTING SAFEGUARDS CHECK", 1, 2);
        let section_1_3 = prompt_index::generated_sub_header("SCOPE CHECK", 1, 3);
        let section_1_4 = prompt_index::generated_sub_header("BY DESIGN CHECK", 1, 4);
        let section_1_5 = prompt_index::generated_sub_header("EXPLOITABILITY CHECK", 1, 5);
        let section_1_6 = prompt_index::generated_sub_header("IMPACT CLASSIFICATION CHECK", 1, 6);
        let section_1_7 = prompt_index::generated_sub_header("LIKELIHOOD ASSESSMENT CHECK", 1, 7);
        let section_1_8 = prompt_index::generated_sub_header("USER ERROR CHECK", 1, 8);
        let section_1_9 =
            prompt_index::generated_sub_header("GOVERNANCE/CENTRALIZATION RISK CHECK", 1, 9);
        let section_1_10 = prompt_index::generated_sub_header("SPECULATION CHECK", 1, 10);
        let section_1_11 =
            prompt_index::generated_sub_header("NON-STANDARD ERC20 TOKEN CHECK", 1, 11);

        format!(
            r#"
{toc}

{section_1_header}

Your task: to run the following 11 checks on EACH listed security finding:

{section_1_1}

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

{section_1_2}

Does code already have safeguards that mitigate/eliminate this security finding?

        **Common safeguards**
        -  Reentrancy → has `nonReentrant`, CEI pattern, or guard
        -  Integer overflow → Solidity 0.8+ with built-in checks
        -  Access control → has `onlyOwner`, `onlyRole`, role checks
        -  Front-running → has commit-reveal, deadlines, slippage protection
        -  Oracle manipulation → has TWAP, multiple sources, price bounds
        -  DoS → has pagination, gas limits, circuit breakers
        -  Precision loss → has proper scaling, rounding checks

       **How to Check:** Search codebase for modifiers/guards, verify vulnerable function uses them, test if bypassable

{section_1_3}

Is Security Finding in Scope?

**The SCOPE FOR SECURITY AUDIT section below has a number of publicly known issues and findings that are OUT OF SCOPE. Please carefully review.**

{section_1_4}

Is this Security Finding really just documented as intentional behavior of protocol?

        Carefully check NatSpec, comments, docs, function naming.

{section_1_5}

Is this Security Finding exploitable?

- Is it possible to produce a Minimal reproducible PoC showing state change, non-dust effect, realistic actors, validating the Security Finding?

{section_1_6}

What is objective impact of Security Finding?

        **High:** Theft/permanent loss of assets, unauthorized drains, economic attacks (non-dust)
        **Medium:** DoS of critical actions, accounting drift, mispricing, privilege escalation
        **Low:** Dust amounts, stylistic issues, event inconsistencies, view-function errors

{section_1_7}

What is likelihood of Security Finding being exploited?

        **Common:** No preconditions, works anytime/anywhere, no special resources
        **Occasional:** Specific but realistic conditions, some chains, moderate setup
        **Rare:** Multiple unlikely conditions, extreme market states, significant resources

{section_1_8}

Does Security Finding require User error or Mistake?

- User chooses bad recipient, provides bad parameters, approves malicious contract, signs malicious data, etc.

{section_1_9}

Can governance/team prevent this by acting responsibly?

        **Security Finding is Governance Risk if:**
        - Admin sets wrong parameters, chooses malicious oracle, misconfigures
        - Team deploys on wrong chain, doesn't verify addresses
        - Team chooses malicious integration, configures incorrectly
        - Security Finding can we remedied by Admin/privileged Users making different choices/decisions (without code changes or redeploying, of course)

{section_1_10}

Is Security Finding depending on future state of the code?

**Question: Does root cause exist NOW and is exploitable with TODAY's code?**

        **Security Finding is future speculation if:**
        -  "Finding depends on if protocol integrates/adds/upgrades in future..."
        -  "Could/might/potentially happen if..." (hypothetical)

{section_1_11}

Does Security Finding depend on contract interaction with Non-standard token?

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
