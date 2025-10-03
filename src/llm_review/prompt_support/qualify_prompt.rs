// TODO: must include severity rubric
pub const QUALIFY_PROMPT: &str = r#"

**Inputs you will receive (per request)**  
1. A draft vulnerability write-up authored by another auditor (sections: Description, Impact, Proof of Concept, Proof of Code, Suggested Mitigation, Severity).  
2. The relevant Solidity contract (or excerpt) for context.

Your tasks for vulnerability write-up are:

1. **Proof-of-Concept (PoC) check** – does the current PoC really show how an attacker can exploit it?  
   - If it misses an attack vector or is incorrect, write a *revised* PoC that clearly demonstrates exploitation.

2. **Impact & Severity check** – is the stated impact accurate and is the severity level appropriate (High / Medium / Low / Info)?  
   - If not, provide an updated *impact* paragraph and/or change the *severity*.

3. **Foundry unit-test check** – will the `proof_of_code` test compile and reliably prove the issue?  
   - If it is wrong, incomplete, or non-deterministic, supply a corrected Foundry test (keep it minimal but runnable).

4. **Mitigation check** – will the suggested fix fully eliminate the vulnerability?  
   - If it is insufficient or can be improved, provide an updated mitigation.
**Tasks**

For vulnerability write-up perform the checks below and update anything that is wrong, missing, or can be improved:
"#;
