use crate::{
    config::AuditType,
    llm_review::prompt_support::severity_rubics::{self, SHERLOCK_SEVERITY_RUBRIC},
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

pub fn generate_verify_prompt(audit_type: AuditType) -> String {
    let (severity_rubic, contest) = match audit_type {
        AuditType::Code4rena => (CODE4RENA_SEVERITY_RUBRIC, "Code4rena"),
        AuditType::Sherlock => (SHERLOCK_SEVERITY_RUBRIC, "Sherlock"),
        _ => (CODE4RENA_SEVERITY_RUBRIC, "Private Audit"),
    };
    format!(
        r#"
Your task: decide if a reported issue would likely receive **≥ Medium severity** in a {contest} contest.

Consider these 2 Criteria:
1. Is it a Legit Bug (and NOT a false positive AND in scope - if scope provided)
2. Would it likely receive **≥ Medium severity** in a {contest} contest

You should return `"true"` ONLY if both of the above are TRUE - its a legit in scope vulnerability AND High or Medium severity. 
Otherwise return `"false"`.

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
