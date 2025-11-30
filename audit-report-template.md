# Code4rena High/Medium Severity Finding Report Template

Use the following template for each **High** or **Medium** severity issue you submit.

# **Code4rena High/Medium Severity Finding Report Template (Ultra-Concise, Primary-Optimized)**

## **General Guidelines**

* Be **extremely concise** — judges reward brevity and clarity.
* Target **≈200-300 words** (excluding code snippets).
* No icons, no fluff, no background sections.
* Only **two sections**: Finding Description + Impact & Mitigation. Note sub sections like root cause, exploit path, impact justification will be merged into finding description and impact section.
* Use GitHub links to **full function blocks**, not individual lines.
* No need to literally include format hints such as(3–6 concise steps), (1–4 bullets), etc ... in final report.
* NO augment_code_snippet tags

### Main Title (One line, high-signal)
**Format:** *Actionable + specific consequence + key condition*
Examples:
* “`withdraw()` allows repeated claims via missing state update (reentrancy)”
* “Share mint uses wrong rounding → first depositor can steal yield via donation attack”

**Severity:** High / Medium
**Affected Contracts:**
Link to readable function-level code block (example):
`Contract.sol::withdraw()` — [https://github.com/.../Contract.sol#L88-L109](https://github.com/.../Contract.sol#L88-L109)


# **1) Finding Description and Impact**

(*Contains: bug → root cause → exploit path → minimal evidence*)
**NOTE** -> **Judge-first rule:** If the judge can’t understand **bug → root cause → exploit path → impact → mitigation** in **60–120 seconds**, your report is too long.

### **Bug (1–2 sentences)**

State what breaks and why it matters.

### **Root Cause (1–3 bullets)**

Examples:

* Missing check / wrong execution order
* Unsafe token/oracle assumptions
* Incorrect math / rounding / overflow

> **Template:**
> “`X` fails to `Y`, allowing attacker to `Z` under condition `C`. This enables `impact` (loss/lock/DoS/incorrect accounting) for `who`.”

### **Code Evidence (~1–3 codeblocks)**

```solidity
// Only include relevant code sections that directly show the root cause. Don't show single lines, must have readable block of code for clariy. 
```

### **Exploit Path (3–6 concise steps)**

1. Preconditions.
2. Attacker calls `X`.
3. Protocol uses unsafe assumption or miscomputes.
4. Attacker extracts value / drains funds / breaks invariant.
5. (Optional) Attack can repeat or scale.

---


### **Impact (1–4 bullets)**

* What the attacker gains (drain, steal, lock, misprice, force liquidation).
* Who is affected (users, LPs, vault, protocol solvency).
* Worst credible case (full loss / permanent lock / systemic insolvency).
* Scope or constraints (if any).

### **Impact Justification (1 sentence)**

A crisp, direct statement explaining **why the issue is High or Medium**.
Example: *“Because this leads to a realistic and repeatable loss of user funds, this issue warrants a High severity.”*

### **2) Recommended Mitigation Steps (1–5 bullets)**

* Specific, actionable fix that addresses the **root cause**.
* Reference exact location (contract/function + GitHub link).
* Prefer safe defaults (state updates before external calls, sane bounds, correct rounding).

# **Concision Rules (How to Stay Short but Lethal)**

### **MAX impact / MIN words**

* No repetition — every line must add new signal.
* No background sections.
* Use bullets over paragraphs.
* One finding = one isolated vulnerability.

### **Compression patterns**

* “Because `X`, attacker can `Y`, resulting in `Z`.”
* “Invariant violated: `A` should equal `B`, but becomes `A < B`.”
* “Protocol assumes `transfer()` moves exact `amount`; this fails for FOT tokens.”

### **Hard Caps**

* Bug: 1–2 sentences
* Root cause: ≤3 bullets
* Exploit path: 3–6 short steps
* Impact: ≤4 bullets
* Mitigation: 1–5 bullets
* **Total prose: ≤200–300 words** (excluding code)
