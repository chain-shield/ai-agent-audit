use crate::{
    config::{AuditType, AUDIT_TYPE},
    llm_review::{
        enums::{all_enum_variants, generate_enum_list},
        findings::PrivilegeLevel,
    },
};

pub const PRE_PROMPT: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "findings": [
    {
      "title": "200 chars or less audit report friendly title",
      "description": "Detailed explanation including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", // the exact contract name where vulnerability is found
      "function": "<Function>", // exact function name where vulnerability is found, if not applicable set to "NA"
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "Critical|High|Medium|Low|Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

"#;

pub const PRE_COMP_AUDIT_PROMPT: &str = r#"

Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{
  "findings": [
    {
      "title": "200 chars or less audit report friendly title",
      "description": "Detailed explanation including vulnerable code snippet",
      "issue_type": "AccessControl | Reentrancy | Oracle | PricePrecision | RoundingError | FeeOnTransferAssumption | UncheckedERC20Return | Dos | SignatureReplay | AuthByPass | UntrustedDelegateCall | TimestampManipulation | CrossChainMessageSpoofing | AccountingInvariantViolation | SlippageMissingOrInsufficient | FlashLoanEconomicManipulation",
      "contract": "{contract_name}", // the exact contract name where vulnerability is found
      "function": "<Function>", // exact function name where vulnerability is found, if not applicable set to "NA"
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "Critical|High|Medium|Low|Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

"#;

pub fn generate_pre_prompt(contract_name: &str) -> String {
    let comp_audit_issue_type = "AccessControl | Reentrancy | Oracle | PricePrecision | RoundingError | FeeOnTransferAssumption | UncheckedERC20Return | Dos | SignatureReplay | AuthByPass | UntrustedDelegateCall | TimestampManipulation | CrossChainMessageSpoofing | AccountingInvariantViolation | SlippageMissingOrInsufficient | FlashLoanEconomicManipulation";
    let default_issue_type = "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow";

    let privileges = generate_enum_list(all_enum_variants::<PrivilegeLevel>().as_slice());
    let issue_type = match AUDIT_TYPE {
        AuditType::Code4rena | AuditType::Sherlock => comp_audit_issue_type,
        AuditType::Client => default_issue_type,
    };

    format!(
        r#"


Before instructions are provided on the task please note required output format:

## JSON Output Requirement

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):

{{ 
  "findings": [
    {{
      "title": "200 chars or less audit report friendly title",
      "description": "Detailed explanation if vulnerability including vulnerable code snippet",
      "issue_type": "{issue_type}",
      "privilege": "{privileges}",
      "contract": "{contract_name}", 
      "function": "<Function>", 
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "Critical | High | Medium | Low | Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }}
  ]
}}

"#
    )
}
