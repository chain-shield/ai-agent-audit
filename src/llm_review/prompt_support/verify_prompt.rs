pub const VERIFY_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **valid and worth fixing**. This includes:

1. **Actual vulnerabilities** – exploitable bugs, broken access control, reentrancy, overflow, etc.
2. **Security best practices violations** – unsafe patterns, missing event logs, unsafe external calls, unchecked return values, etc.
3. **Security-adjacent concerns** – issues that degrade transparency, auditability, maintainability, or correctness.
4. **Potential future risk** – minor today but can cause critical issues when upgraded or combined with other code.

You should return `"true"` if the issue meets **any** of these criteria.

Only return `"false"` if the issue is clearly meets **All** of these conditions:
- Already mitigated or impossible to exploit
- A deliberate pattern that is safe and idiomatic
- Fully unrelated to security, correctness, or best practice

Also if vulnerability is *out of scope* return `"false"` (applies only if scope is provided below)

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
