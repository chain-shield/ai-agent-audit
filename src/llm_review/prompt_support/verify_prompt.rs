pub const VERIFY_PROMPT: &str = r#"

Your job is to determine whether the reported issue is **valid and worth fixing**. This includes:

1. **Actual vulnerabilities** – exploitable bugs, broken access control, reentrancy, overflow, etc.
2. **Security best practices violations** – unsafe patterns, missing event logs, unsafe external calls, unchecked return values, etc.
3. **Security-adjacent concerns** – issues that degrade transparency, auditability, maintainability, or correctness.
4. **Potential future risk** – minor today but can cause critical issues when upgraded or combined with other code.

You should return `"true"` if the issue meets **any** of these criteria.

Only return `"false"` if the issue is clearly meets any of these conditions:
- Already mitigated or impossible to exploit
- A deliberate pattern that is safe and idiomatic
- Fully unrelated to security, correctness, or best practice

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

pub const VERIFY_C4_PROMPT: &str = r#"
Your task: decide if a reported issue would likely receive **≥ Medium severity** in a Code4rena contest.

Consider these 2 Criteria:
1. Is it a Legit Bug (and NOT a false positive)
2. Would it likely receive **≥ Medium severity** in a Code4rena contest

You should return `"true"` ONLY if both of the above are TRUE - its actual vulnerabilities AND likely recieve Medium or Higher severity. 
Otherwise return `"false"`.

## INPUT  
You will receive **one report** with the following structure:

### <Title>

### Description  
<Human-written description of the bug>

### Impact  
<Claimed effect>

### Proof of Concept  
<Attack steps, if applicable>

### Proof of Code  
```solidity

### Suggested Mitigation

<Recommended fix>


## Code4rena Guidelines

### Code4rena severity rubric
-------------------------
| Severity  | Typical impact                                                                                          |
|-----------|----------------------------------------------------------------------------------------------------------|
| Critical  | Direct theft of user or protocol funds, permanent total loss/control, arbitrary code execution           |
| High      | Permanent freezing or bricking of funds, governance takeover, constrained mint/burn/drain                |
| Medium    | Temporary loss until admin action, convincing DoS, reward/fee distortion, oracle/math skew with $ impact |
| Low       | Minor economic grief, rare‑case DoS, best‑practice deviations                                             |
| Gas/Info  | Gas optimizations, style, comments, docs                                                                  |

### Guidelines
----------
• Map the bug’s *realistic* worst‑case impact to the table above.  
• Costliness alone **does not** downgrade severity; assume a well‑funded attacker.  
• Long‑lasting governance loss or deposit/withdrawal DoS ⇒ ≥ Medium even WITHOUT direct fund loss.  
• Ignore findings that are purely gas, formatting, or documentation (≤ Low).

## Evaluation Steps (think silently; do **not** reveal reasoning):
1. Read Description, Impact, PoC, code.  
2. Verify reachability (modifiers, access control, reentrancy guards, overflow checks ≥ 0.8).  
3. Decide if impact meets or exceeds Medium per rubric.
"#;

// TODO - UPDATE
pub const VERIFY_SHERLOCK_PROMPT: &str = r#"
You are an expert triager for **Sherlock** smart‑contract audits.

Your task: decide if a reported issue would likely receive **≥ Medium severity** in a Sherlock contest.

Consider these 2 Criteria:
1. Is it a Legit Bug (and NOT a false positive)
2. Would it likely receive **≥ Medium severity** in a Sherlock contest

You should return `"true"` ONLY if both of the above are TRUE - its actual vulnerabilities AND likely recieve Medium or Higher severity. 
Otherwise return `"false"`.

## INPUT  
You will receive **one report** with the following structure:

### <Title>

### Description  
<Human-written description of the bug>

### Impact  
<Claimed effect>

### Proof of Concept  
<Attack steps, if applicable>

### Proof of Code  
```solidity

### Suggested Mitigation

<Recommended fix>


Severity  (Sherlock rubric)
--------------------------------------------
Critical – Permanent, unrecoverable theft of protocol / user funds, or arbitrary code execution  
High     – Irreversible freeze / burn of significant funds, governance take-over, or any bug that forces an emergency upgrade / migration  
Medium   – Loss or value extraction that an admin can eventually reverse, repeatable profit, convincing or long-lasting DoS, accounting / oracle skew with $ impact  
Low      – Minor grief, rare-edge DoS, spec deviation, unsafe pattern with no direct financial or governance impact  
Info/Gas – Style, docs, gas optimisations (ignore unless sponsor explicitly pays for gas findings)

Guidelines
----------
• Judge by the *realistic worst-case* impact mapped to the table above.  
• High exploit cost **does not** lower severity IF payoff ≥ cost (assume a well-funded attacker).  
• Governance loss or sustained deposit / withdraw DoS ⇒ at least Medium even WITHOUT direct theft.  
• Exclude pure gas, style, or documentation issues from Medium+.

Evaluation (think silently; do **not** reveal reasoning)
-------------------------------------------------------
1. Read Description, Impact, PoC, and code.  
2. Verify reachability (modifiers, access control, upgradeability paths, reentrancy guards, ≥0.8 overflow checks).  
3. Return `"true"` if the bug is genuine **and** maps to ≥ Medium per rubric; otherwise return `"false"`.
"#;

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

1. Read the scope report carefully.
2. Read the description, impact, PoC, and mitigation to understand the vulnerability.
3. Determine if issue in in scope.
"#;
