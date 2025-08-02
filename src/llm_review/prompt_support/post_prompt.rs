use crate::{
    config::{AuditType, AUDIT_TYPE},
    llm_review::prompt_support::severity_rubics::{
        CODE4RENA_SEVERITY_RUBRIC, DEFAULT_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
    },
};

pub const POST_PROMPT_AGENT: &str = r#"
### FINAL OUTPUT REQUIREMENTS 

**AFTER** you are done requesting any files using the retrieve_file_content tool, and your
security audit is complete, return the following **For Every VIOLATION**:

 **For Every VIOLATION** return:
1. **Description**: Detailed explanation including vulnerable code snippet 
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow
3. **Contract**: The exact contract name where vulnerability is found 
4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
5. **Impact**: Financial and security consequences 
6. **Proof of Concept**: Step-by-step exploitation scenario 
7. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
8. **Severity**: High/Medium/Low/Info based on table below
    | Severity | Definition |
    |----------|------------|
    | HIGH     | Steals, locks, or permanently harms a significant portion of funds/governance. |
    | MEDIUM   | Exploitable but needs favourable conditions or yields limited loss. |
    | LOW      | Minor financial or operational impact; edge-case or hard to exploit. |
    | INFO     | Non-safety best-practice / observability issue. | 
9. **Mitigation**: Suggested Mitigation with code example of fix

"#;

pub const POST_PROMPT_JSON: &str = r#"

### OUTPUT REQUIREMENTS 

*Please respond with ONLY valid JSON in the following exact format:*

{
  "findings": [
    {
      "description": "Detailed explanation if vulnerability including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", 
      "function": "<Function>", 
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "High",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

- If no findings return: 

{
  "findings": []
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
**Please double-check opening and closing brakets: `}` and `]`, make sure 
they match up correctly.

"#;

pub const POST_PROMPT_STATIC: &str = r#"

### OUTPUT REQUIREMENTS 

 **For Every VIOLATION** return:
1. **Description**: Detailed explanation including vulnerable code snippet 
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow
3. **Contract**: The exact contract name where vulnerability is found 
4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
5. **Impact**: Financial and security consequences 
6. **Proof of Concept**: Step-by-step exploitation scenario 
7. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
8. **Severity**: High/Medium/Low/Info based on table below

| Severity     | Typical impact examples                                                                                                                                                                                                            | What it signals to the team               |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **Critical** | - Direct theft of any funds - Permanent, **total** loss or control of all user or protocol funds - Arbitrary code execution                                                                                                  | “Drop everything—patch immediately.”      |
| **High**     | - **Permanent freezing** or bricking of user or protocol funds (can’t be reversed without privileged migration) - Loss of governance control - Logic that lets an attacker mint/ burn / drain but under specific constraints | “Must fix before next release / upgrade.” |
| **Medium**   | - Temporary loss (funds stuck until admin action) - Convincing grief / DoS that makes the protocol unusable - Oracle or math bugs that skew accounting but don’t directly drain value                                        | “Important, schedule a patch.”            |
| **Low**      | - Minor economic grief (extra gas, incorrect event data) - Edge-case DoS that requires unusual conditions - Best-practice deviations with limited real-world impact                                                          | “Fix in regular development cycle.”       |
| **Insight**  | Code cleanliness, documentation issues, minor style or test suggestions                                                                                                                                                            | “Nice-to-have, no security impact.”       |

9. **Mitigation**: Suggested Mitigation with code example of fix

*Please respond with ONLY valid JSON in the following exact format:*

{
  "findings": [
    {
      "description": "Detailed explanation if vulnerability including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
      "contract": "{contract_name}", 
      "function": "<Function>", 
      "impact": "Business and security consequences of the vulnerability",
      "proof_of_concept": "Step-by-step exploitation scenario",
      "proof_of_code": "Complete Foundry unit test demonstrating the vulnerability",
      "severity": "Critical | High | Medium | Low | Info",
      "mitigation": "suggested mitigation with code example for the fix"
    }
  ]
}

- If no vulnerabilities are found, return: 

{
  "findings": []
}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
**Please double-check opening and closing brakets: `}` and `]`, make sure 
they match up correctly.

"#;

pub fn generate_post_prompt(contract_name: &str) -> String {
    let security_rubric = match AUDIT_TYPE {
        AuditType::Code4rena => CODE4RENA_SEVERITY_RUBRIC,
        AuditType::Sherlock => SHERLOCK_SEVERITY_RUBRIC,
        AuditType::Client => DEFAULT_SEVERITY_RUBRIC,
    };

    format!(
        r#"

### OUTPUT REQUIREMENTS 

 **For Every VIOLATION** return:
1. **Description**: Detailed explanation including vulnerable code snippet 
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow
3. **Contract**: The exact contract name where vulnerability is found 
4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
5. **Impact**: Financial and security consequences 
6. **Proof of Concept**: Step-by-step exploitation scenario 
7. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
8. **Severity**: High/Medium/Low/Info based on table below

{security_rubric}

9. **Mitigation**: Suggested Mitigation with code example of fix

*Please respond with ONLY valid JSON in the following exact format:*

{{ 
  "findings": [
    {{
      "description": "Detailed explanation if vulnerability including vulnerable code snippet",
      "issue_type": "AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow",
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

- If no vulnerabilities are found, return: 

{{
  "findings": []
}}

**Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
**Please double-check opening and closing brakets: `}}` and `]`, make sure 
they match up correctly.

"#
    )
}
