
# Code4rena High/Medium Severity Finding Report Template

Use the following template for each **High** or **Medium** severity issue you submit.
This format is aligned with what judges look for in **primary findings**, including all necessary details.

## General Guidelines
- be CONCISE and to the point, while still providing all necessary details.
- maximum 500 words (excluding code snippets) - most top C4 submission are just ~200 words long.
- professional presentation, no icons.
- DO NOT add extra sections like 'Proof of Code', etc. Follow template structure exactly.
- please, no extra examples outside of exploitation scenario

## Descriptive Issue Title

**Severity:** High (or Medium)
**Affected Contracts and Lines of Code:** Provide actual links to effected code snippets. i.g. https://github.com/code-423n4/2025-10-covenant/blob/7cd409f4b6ad7134e8926feeafb15eb811d2503c/src/curators/oracles/CrossAdapter.sol#L88-L109 
*(points to the contracts and locations where the issue occurs, do not link to single lines preferable a readable block of code of entire function.)*

## Summary

A brief summary of the vulnerability and its impact. State the **core problem** in 1–3 sentences.

> Example:
> “Lack of input validation in `vulnerableFunction()` allows an attacker to withdraw all funds from the contract without authorization.”

This section should quickly answer:

* What is the bug?
* Why does it matter?


## Description

Provide a detailed description of the vulnerability, explaining **how it works** and **why it arises**.

**NOTE**: Description MUST include relevant code snippets wrapped in: ```solidity ... ```
ALSO please include actual github links to code where relevant, instead of just listing line numbers.

### Root Cause Analysis

Identify the **exact root cause** in the code.

> Example:
> “The contract does not check that `msg.sender == owner` before executing critical function `X`, which means anyone can call it.”

Quote relevant code snippets or link to line numbers if possible to support your explanation.
This proves you’ve pinpointed the bug.

### Exploit Scenario

Describe step-by-step how an attacker or user can exploit this issue.

**Example format:**

1. **Step 1:** State initial conditions
   e.g. “Attacker acquires 1 ETH of the token and calls `approve()` on the victim contract…”
2. **Step 2:** Describe the malicious action
   e.g. “Attacker calls `vulnerableFunction(1 ETH)` which, due to missing check, transfers funds…”
3. **Step 3:** Describe the outcome
   e.g. “The contract transfers the attacker’s token plus all other users’ tokens to the attacker.”

This step-by-step explanation from root cause to impact is **exactly what judges look for** in a high-quality report.

### Relevant Context (Optional)

If needed, mention any contextual details:

* “This issue is only possible after the initialization period.”
* “It’s similar to a known vulnerability pattern.”

Keep it focused on helping the reader understand the bug.
By the end of this section, the reader (and judge) should **fully grasp how the bug works and why it exists.**


## Impact

Explain the **impact and severity** of the vulnerability.

Be explicit about what could happen in the worst case:

> **High:** An attacker can drain all deposits from the protocol, leading to total loss of user funds.
> **Medium:** DoS of a feature, partial loss, or incorrect accounting limited to certain funds.

This section should **justify** why you chose the severity level in terms of potential harm.

Tips:

* Quantify impact if possible (percentage of funds at risk, users affected).
* Describe the **worst credible scenario**, not just the minimal case.

## Recommended Mitigation

Suggest a **clear, specific fix** for the issue.

Examples:

* “Add `require(msg.sender == owner)` at the start of `vulnerableFunction` to restrict access.”
* “Use OpenZeppelin’s `SafeERC20` to handle token transfers safely.”
* “Implement a cap on maximum deposits to avoid integer overflow.”

Be as **concrete** as possible:

* Point out the **exact place in code** (e.g. `ContractName.sol#L123`).
* Ensure your fix addresses the **root cause**, not just symptoms.

High-quality (primary) reports have strong remediation suggestions — weak reports often skip this or suggest incomplete fixes.

## References (Optional)

If relevant, include supporting references:

* Similar vulnerabilities in past audits.
* Blog posts or documentation.
* Ethereum or OpenZeppelin references.

> Example:
> “This issue is similar to the XYZ exploit in [ProjectName Audit Report].”
> “See [Ethereum Reentrancy Docs](https://docs.code4rena.com) for why state updates should happen before external calls.”

Include references only if they **add clarity or credibility**.

