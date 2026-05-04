use crate::llm_review::phases::rounds::utils::generate_pre_round_verify_json_requirement;
use crate::llm_review::phases::verify_rounds::{FindingAnalysis, FindingStatus};
use crate::utils::deserialize_bool::deserialize_bool_from_str_or_bool;
use log::info;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Verification result for a potential vulnerability
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
    pub requires_user_mistake_without_protocol_fault: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub requires_privileged_or_compromised_actor: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_or_bool")]
    pub future_speculation: bool,
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

        if self.requires_user_mistake_without_protocol_fault {
            finding_status_vec.push(FindingStatus::InvalidUserErrorOrMistake);
        }
        if self.requires_privileged_or_compromised_actor {
            finding_status_vec.push(FindingStatus::InvalidGovernanceRisk);
        }
        if self.future_speculation {
            finding_status_vec.push(FindingStatus::InvalidFutureSpeculation);
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
        info!(
            "requires user mistake without protocol fault: {}\n",
            self.requires_user_mistake_without_protocol_fault
        );
        info!(
            "requires privileged or compromised actor: {}\n",
            self.requires_privileged_or_compromised_actor
        );
        info!("is future speculation: {}\n", self.future_speculation);
        info!("Justification: {}\n", self.justification);
        info!("\n");
    }

    fn generate_verify_prompt() -> String {
        let verify_json = AllRoundLegitAnalysis::generate_verify_json();
        let pre_verify_json = generate_pre_round_verify_json_requirement(&verify_json);

        format!(
            r#"
        {pre_verify_json}

        Your task: run the following universal invalidity checks on EACH listed security finding.

        This verification round is not a severity judge. Do not downgrade or invalidate based on likelihood,
        contest severity rules, bounty payout criteria, platform-specific out-of-scope rules, known issues,
        prior audits, token support policy, or whether a PoC has already been written.

        When uncertain, keep the finding alive. Only mark a finding invalid when the invalidating reason is
        directly proven from the code, docs, or mechanical source scope.

        ## ROOT CAUSE EXISTS

        - Locate exact function/contract, verify vulnerable code path exists (not hallucinated)
        - Trace execution flow step-by-step, check if code matches finding's description
        - Preconditions must not be impossible
        - Do not reject merely because an invariant is undocumented. Implicit accounting, security, and economic invariants can be valid.

        **Bug does not exist only if:** Function/contract does not exist, the claimed code path is impossible,
        execution flow does not match, or preconditions are impossible.

        ## COMPLETE SAFEGUARD EXISTS

        Does code already have a safeguard that fully blocks the exact exploit path?

        Common safeguards to check:
        -  Reentrancy → has `nonReentrant`, CEI pattern, or guard
        -  Integer overflow → Solidity 0.8+ with built-in checks
        -  Access control → has `onlyOwner`, `onlyRole`, role checks
        -  Front-running → has commit-reveal, deadlines, slippage protection
        -  Oracle manipulation → has TWAP, multiple sources, price bounds
        -  DoS → has pagination, gas limits, circuit breakers
        -  Precision loss → has proper scaling, rounding checks

        Partial mitigation or uncertain guard does not invalidate. Mark `safeguard_against_it = true`
        only when the safeguard fully prevents the exact issue.

        ## MECHANICAL ANALYZED-CODE SCOPE

        Is the finding in analyzed production source code?

        Exclude tests, mocks, scripts, examples, generated artifacts, vendored dependencies, or files outside
        the analyzed code folders unless the audit scope explicitly includes them.

        Do not apply bounty-specific OOS rules, known-issue rules, prior-audit rules, payout rules, or platform
        eligibility rules in this round.

        ## EXPLICITLY BY DESIGN

        Is the exact risky behavior clearly documented and accepted as intentional protocol behavior?

        Generic docs, function names, or intended feature behavior are not enough. Mark `by_design = true`
        only when the exact risk is intentionally accepted.

        ## CURRENTLY EXPLOITABLE

        Does the root cause have a realistic execution path in today's code?

        No PoC is required at this stage. Do not reject for difficult setup, low frequency, or uncertain severity.

        ## REQUIRES PRIVILEGED OR COMPROMISED ACTOR

        Invalid only if exploitation requires admin/team/keeper/trusted-role abuse, leaked keys,
        compromised credentials, or operational misconfiguration.

        Do not invalidate non-privileged governance manipulation or attacks performed through public functions.

        ## REQUIRES USER MISTAKE WITHOUT PROTOCOL FAULT

        Invalid only if there is no protocol flaw and the issue solely depends on victim misuse,
        social engineering, malicious approval/signature, or arbitrary bad parameters.

        Normal attacker interaction with public protocol functions is not user error.

        ## FUTURE SPECULATION

        **Question: Does root cause exist NOW and is exploitable with TODAY's code?**

        **Security Finding is future speculation if:**
        -  "Finding depends on if protocol integrates/adds/upgrades in future..."
        -  "Could/might/potentially happen if..." (hypothetical)

        ## OUTPUT REQUIREMENTS

        Please continue until you have carefully evaluated ALL findings on EACH universal check.

        Based on your assessment please provide the following for EACH finding:

        *finding id*: insert finding id (from 'id' field)
        *finding_title*: insert finding 'title'
        *does bug exist*: true | false
        *safeguard against it*: true | false
        *in scope*: true | false
        *by design*: true | false
        *exploitable*: true | false
        *requires user mistake without protocol fault*: true | false
        *requires privileged or compromised actor*: true | false
        *future speculation*: true | false
        *justification:*: Please provide justification for your choices (under 400 words)

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
                "in_scope": true | false,
                "by_design": true | false,
                "exploitable": true | false,
                "requires_user_mistake_without_protocol_fault": true | false,
                "requires_privileged_or_compromised_actor": true | false,
                "future_speculation": true | false,
                "justification": "Please provide justification for your choices (under 400 words)."
            }}
        ]
    }}
"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_analysis() -> AllRoundLegitAnalysis {
        AllRoundLegitAnalysis {
            finding_id: "H-1".to_string(),
            finding_title: "Valid finding".to_string(),
            does_bug_exist: true,
            safeguard_against_it: false,
            by_design: false,
            in_scope: true,
            exploitable: true,
            requires_user_mistake_without_protocol_fault: false,
            requires_privileged_or_compromised_actor: false,
            future_speculation: false,
            justification: "looks plausible".to_string(),
        }
    }

    #[test]
    fn all_round_valid_analysis_returns_no_statuses() {
        assert!(
            valid_analysis()
                .get_finding_status_array_from_analysis()
                .is_none()
        );
    }

    #[test]
    fn all_round_missing_bug_maps_to_bug_does_not_exist() {
        let mut analysis = valid_analysis();
        analysis.does_bug_exist = false;

        assert_eq!(
            analysis.get_finding_status_array_from_analysis().unwrap(),
            vec![FindingStatus::InvalidBugDoesNotExist]
        );
    }

    #[test]
    fn all_round_privileged_actor_and_user_mistake_map_to_narrow_statuses() {
        let mut analysis = valid_analysis();
        analysis.requires_privileged_or_compromised_actor = true;
        analysis.requires_user_mistake_without_protocol_fault = true;

        assert_eq!(
            analysis.get_finding_status_array_from_analysis().unwrap(),
            vec![
                FindingStatus::InvalidUserErrorOrMistake,
                FindingStatus::InvalidGovernanceRisk
            ]
        );
    }

    #[test]
    fn all_round_prompt_is_not_a_severity_or_token_policy_filter() {
        let prompt = AllRoundLegitAnalysis::generate_verify_prompt();
        let prompt_lower = prompt.to_ascii_lowercase();

        assert!(prompt.contains("not a severity judge"));
        assert!(prompt.contains("When uncertain, keep the finding alive"));
        assert!(!prompt_lower.contains("likelihood assessment"));
        assert!(!prompt_lower.contains("non-standard erc20 token check"));
        assert!(!prompt.contains("LowSeverityDueToLowImpact"));
        assert!(!prompt.contains("LowSeverityDueToRareLikelihood"));
        assert!(!prompt.contains("InvalidERC20EdgeCase"));
    }
}
