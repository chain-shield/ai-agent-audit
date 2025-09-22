pub const EXTRACTOR_AGENT: &str = r#"
Below is solidity auditor’s findings that lists security vulnerabilities found with
the following fields provided:

1. **Description**: Detailed explanation including vulnerable code snippet 
2. **Issue Type**: AccessControl|ArrayLimits|ConfidentialData|DefaultVisibility|Dos|Inheritance|IntegerMath|Oracle|Pragma|Randomness|Reentrancy|ReplayAttack|SelfDestruct|ShortAddress|StorageLayout|TxOrigin|UncheckedReturn|UnexpectedEth|ZeroCode|FrontrunMev|UpgradeabilityInitializerSafety|PausableEmergencyStop|TimestampDependentLogic|FlashLoanEconomicManipulation|DelegatecallLowLevelOps|SignatureMalleability|EventConsistency|GasGriefBlockLimit|IntegerOverflow
3. **Contract**: The exact contract name where vulnerability is found 
4. **Function**: The exact function name where vulnerability is found, if not applicable return "NA"
5. **Impact**: Financial and security consequences 
6. **Proof of Concept**: Step-by-step exploitation scenario 
7. **Proof of Code**: Complete Foundry unit test demonstrating vulnerability
8. **Severity**: High/Medium/Low/Info based on table below
    | Severity | Definition |
    |----------|------------| | HIGH     | Steals, locks, or permanently harms a significant portion of funds/governance. |
    | MEDIUM   | Exploitable but needs favourable conditions or yields limited loss. |
    | LOW      | Minor financial or operational impact; edge-case or hard to exploit. |
    | INFO     | Non-safety best-practice / observability issue. | 
9. **Mitigation**: Suggested Mitigation with code example of fix


Please convert this list of security vulnerabilities into strict JSON, with following
EXACT format: 

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
