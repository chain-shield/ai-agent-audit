use crate::{
    config::AuditType,
    llm_review::prompt_support::severity_rubics::{
        CANTINA_SEVERITY_RUBRIC, SHERLOCK_SEVERITY_RUBRIC,
    },
};

use super::severity_rubics::CODE4RENA_SEVERITY_RUBRIC;
pub const VERIFY_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **valid and worth fixing**. This includes:

1. **Actual vulnerabilities** – exploitable bugs, broken access control, reentrancy, overflow, etc.
2. **Security best practices violations** – unsafe patterns, missing event logs, unsafe external calls, unchecked return values, etc.
3. **Security-adjacent concerns** – issues that degrade transparency, auditability, maintainability, or correctness.
4. **Potential future risk** – minor today but can cause critical issues when upgraded or combined with other code.

You should return `"true"` if the issue meets **any** of these criteria, otherwise return `"false"`.

"#;

pub fn generate_verify_prompt(audit_type: &AuditType) -> String {
    let (severity_rubic, contest) = match audit_type {
        AuditType::Code4rena => (CODE4RENA_SEVERITY_RUBRIC, "Code4rena"),
        AuditType::Sherlock => (SHERLOCK_SEVERITY_RUBRIC, "Sherlock"),
        AuditType::Cantina => (CANTINA_SEVERITY_RUBRIC, "Cantina"),
        _ => (CODE4RENA_SEVERITY_RUBRIC, "Private Audit"),
    };
    format!(
        r#"
Your task: decide if a reported issue is valid and would likely receive **≥ Medium severity** in a {contest} contest.

Consider the following Criteria:
1. Is issue in scope? (see scope provided below)
2. Is this issue valid? Does protocol have safeguards against it? Are there any external depedencies that cannot be seen and analyzed (creating uncertainly about validity of finding)?
3. Would it likely receive **≥ Medium severity** in a {contest} contest

Carefully trace the code to verify issue validity.

Based on your assessment please provided the following:

*Severity:* High | Medium | Low | Info  (only provide if finding is finding is NOT invalid and differs from listed severity)
*Finding Severity Justification:* Explain why you assigned this severity. 
*Finding Status:* Valid | Invalid | OutOfScope | NeedsMoreInfo
*Status Justification:* if invalid, out of scope, or needs more info, please explain why.
*Finding Status Confidence:* VeryConfident | Confident | SomewhatConfident 
*Finding Status Confidence Justification:* if Somewhat Confident, please explain why. 
*Finding Complexity:* How likely is it that other security researchers would find this?  1-10 scale, 10 being very unlikely. Higher the score the better as it will earn the researcher a higher bounty.

## {contest} Guidelines
# {contest} Severity Rubric (What {contest} Actually Pays For)

{severity_rubic}
"#
    )
}

// USE THIS STRICTER PROMPT FOR BUG BOUNTIES
pub const VERIFY_BUG_BOUNTY_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **valid and worth fixing**. This includes:

1. **Actual vulnerabilities or False Positive?** – exploitable bugs, broken access control, reentrancy, overflow, etc.
2. **Realistic for Attacker to Exploit?** - is it realistic and pratical for an attacker to exploit the bug?
Does protocol have safeguards against it? 

You should return `"true"` ONLY if both of the above are TRUE - its actual vulnerabilities AND realistic for attacker to exploit. 
Otherwise return `"false"`.

INPUT  
You will receive **one report** with the following structure:

## <Title>

## Description  
<Human-written description of the bug>

## Impact  
<Claimed effect>

## Proof of Concept  
<Attack steps, if applicable>

## Proof of Code  
```solidity

## Suggested Mitigation

<Recommended fix>

TASK

1. Read the description, impact, PoC, and mitigation to understand the claimed vulnerability.
2. Inspect every Solidity code block (vulnerable contract and PoC) and verify, line-by-line, whether the issue can actually occur in practice.
3. Watch for false positives (e.g., state changes before external calls, access-control modifiers, Solidity ≥ 0.8 overflow checks, built-in reentrancy guards, etc.).
4. Think step-by-step **silently**; **do not** reveal chain-of-thought.

**No other text, markdown, or punctuation is allowed in your final answer.**

"#;

// USE THIS STRICTER PROMPT FOR BUG BOUNTIES
pub const VERIFY_IN_SCOPE_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **IN SCOPE** based on the Scope report provided below.

You should return `"true"` ONLY if reported issue is 100% in scope, as determined by scope report.
Otherwise return `"false"`.

"#;
