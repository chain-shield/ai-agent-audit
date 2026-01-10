Totally fair — my earlier 3-pass set didn’t explicitly cover your **full 11-gate verifier** (especially **Unsupported Token Gate** + **Speculation Gate**). Below are **three copy-paste Markdown checklists** that **cover all 11** and preserve your intent, while still keeping the benefits of splitting verification into passes.

I’ve mapped your 11 items like this:

* **Pass 1 (Existence & Reachability):** Gates **1 / 6 / 7** (+ the “root cause exists now” proof)
* **Pass 2 (Classification):** Gates **5 / user error / by design / duplicates**
* **Pass 3 (Severity Calibrator):** Gates **2 / 3 / 4 / 8 / 10 / 11**

---

# PASS 1 — Existence, Scope, Token Model, Speculation (Gates 1 / 6 / 7)

**Goal:** Prove the root cause is real, in-scope, and applicable *today* (not speculative / not OOS token behavior).
**If any FAIL → INVALID (stop).**

## 1) Scope & Root-Cause Validation (GATE 1)

* [ ] File/contract/function is **in scope** (path + commit).
* [ ] Root cause is in **in-scope logic**, not inside an OOS library.

  * [ ] If OOS lib is misused by in-scope code → ✅ **valid** (cite misuse site).
  * [ ] If bug is inside OOS lib implementation → ❌ **INVALID/OOS**.
* [ ] Not view-only/cosmetic/event-only without functional impact → else **QA/Low**.

**FAIL conditions**

* [ ] Root cause inside out-of-scope lib (not misuse).
* [ ] Only affects events / readability / view outputs without functional consequence.

## 2) Token Model Constraints (GATE 6)

**Question:** Does the exploit rely on non-standard ERC-20 behavior?

* [ ] Fee-on-transfer / rebasing / reflective / hook tokens / decimals edge?

  * [ ] If YES: protocol **explicitly supports** it (cite docs/tests), OR token is **USDT exception**.
  * [ ] If NOT supported → ❌ **INVALID/OOS**.
* [ ] If token-model dependent and “supported”: verify handling is actually insufficient (cite code path).

**FAIL conditions**

* [ ] Exploit requires token behavior not claimed supported (and not USDT exception).
* [ ] “Assume fee-on-transfer” but protocol never says it supports it.

## 3) Speculation & Integrations (GATE 7)

**Question:** Does the exploit work with today’s code and realistic environment?

* [ ] Root cause exists **now** and is exploitable **without hypothetical future contracts**.
* [ ] If relies on future integration:

  * [ ] Integration is **credible and likely** (cite roadmap/docs), AND
  * [ ] Path is already enabled by current code (not just “could be added later”).
* [ ] No “if they later add X” without strong likelihood + clear path.

**FAIL conditions**

* [ ] Requires new integration / configuration that does not exist today (no credible likelihood).
* [ ] “If oracle gets added later” / “if bridge uses this later” with no evidence.

## 4) Existence & Reachability Sanity Check

* [ ] Vulnerable code exists at cited location(s).
* [ ] Claimed call path is reachable (modifiers/conditions allow it).
* [ ] Preconditions are possible (state can be reached, roles exist, values can be set).
* [ ] If PoC exists: it actually hits the vulnerable path.

## PASS 1 OUTPUT (required)

* **Result:** ✅ PASS / ❌ FAIL
* **If FAIL:** one-line reason (“OOS token model”, “speculative future integration”, “root cause out of scope”, etc.)
* **Evidence:** code links + brief trace summary

---

# PASS 2 — Classification (User Error vs Governance/Admin Risk vs By-Design vs True Bug) + Dupes (Gates 5 / User Error / By Design / 9)

**Goal:** Prevent false positives by classifying correctly *before* severity.

## 1) Classification (pick exactly ONE)

* [ ] **A) User-error dependent**
* [ ] **B) Governance/Admin risk (centralization)**
* [ ] **C) By-design behavior**
* [ ] **D) True code vulnerability**

---

## 2) User Error Gate (explicit)

**Does it require user mistake to manifest?**

* [ ] Requires user choosing bad recipient/route/pool, bad slippage/deadline, unsafe params.
* [ ] Requires user approving malicious spender / signing malicious calldata/permit.
* [ ] Requires user ignoring warnings that protocol expects user to handle.

✅ If YES → classify as **A) User error** (usually **INVALID/QA/Low**, unless protocol forces the mistake).

## 3) Centralization / Governance Risk Gate (GATE 5)

**Does it require admin misconfiguration, negligence, or malicious admin?**

* [ ] Requires admin choosing wrong params/oracle/route/validator set.
* [ ] Requires admin failure to act (ops mistake) to avoid harm.
* [ ] Requires malicious admin actions (unless threat model includes it).

✅ If YES → classify as **B) Governance/Admin risk** (usually **QA/Low/INVALID** under C4 trusted-admin model).

✅ If attack works even with **reasonable admin behavior** or is a **privilege escalation** → classify as **D)**.

## 4) By-Design Gate (GATE 10 concept, but classification)

**Is the behavior intended? Verify if it is by carefully checking reviewing following:**

* [ ] Docs/scope/spec
* [ ] NatSpec/comments
* [ ] Tests asserting it’s expected behavior

Look for any concrete evidence that bug could be by design. Also, think along these lines, if I were designing this protocol, would it make sense to design it this way? Are there benefits to designing it this way that outweigh the costs? 

## 5) Duplicate / Non-Novel Check (GATE 9)

* [ ] Is the same root cause already present in your findings list / prior issues?
* [ ] If duplicate: merge, or keep only the best exploit path / highest impact variant.
* [ ] If two reports differ only by surface symptom but same root cause → treat as duplicate.

## PASS 2 OUTPUT (required)

* **Classification:** A / B / C / D
* **One-line reason** (user mistake? admin risk? design evidence?)
* **If A/B/C:** stop (no severity calcs)
* **If D:** proceed to Pass 3

---

# PASS 3 — Severity Calibration (Impact + Likelihood + PoC + Quant + Reliability) (Gates 2 / 3 / 4 / 8 / 10 / 11)

**Goal:** Prevent impact/likelihood inflation and ensure PoC/quant backing.

## 1) Impact Gate (C4 definitions) (GATE 2)

* [ ] Name the **asset or core function at risk**.
* [ ] Choose impact class:

  * [ ] **High:** asset theft/loss/authorization compromise with real path
  * [ ] **Medium:** DoS of core actions, accounting drift with EV, governance blockage, oracle mispricing (non-guaranteed drain), likely priv-esc
  * [ ] **QA/Low:** dust, style, events-only, view-only
* [ ] State **worst credible outcome** (not maximum theoretical).

## 2) Likelihood Gate (GATE 3)

* [ ] List **explicit preconditions** (≥2):

  * [ ] roles/permissions
  * [ ] required state (epoch, config, paused)
  * [ ] market/liquidity/oracle conditions
  * [ ] timing/MEV ordering
  * [ ] attacker capital/resources
* [ ] Rate likelihood:

  * [ ] Common / Occasional / Edge-case (Rare)
* [ ] Apply rule:

  * [ ] High impact + edge-case → often **Medium/High** (argue)
  * [ ] Low impact + edge-case → **QA/Low**

**Anti-inflation rule:** If preconditions aren’t explicit → default **Edge-case/Rare**.

## 3) Exploitability / PoC Gate (GATE 4)

* [ ] Do we have a minimal reproducible PoC (Foundry) that demonstrates the effect?
* [ ] PoC asserts **state change consistent with impact** (`assertEq/assertGt`).
* [ ] If no PoC:

  * [ ] Mark “REWORK needed” or downgrade confidence/likelihood
  * [ ] Do not claim guaranteed drain/DoS without executable demonstration

## 4) Quantification & Non-Dust Thresholds (GATE 8)

* [ ] Quantify loss/EV/DoS cost.
* [ ] Prove non-dust:

  * [ ] absolute amount (e.g. ETH/USDC)
  * [ ] % of TVL or user equity
* [ ] If cumulative: show repeatability (per tx / per epoch).

## 5) Reliability Boosters (GATE 10)

* [ ] State assumptions explicitly (oracle freshness, liquidity, fees, roles).
* [ ] Boundary tests / edge inputs considered (0/1/min/max).
* [ ] If events: tie to functional security impact, otherwise cap severity.
* [ ] Avoid speculative language unless matched by proof.

## 6) Quick Severity Snap-Score (GATE 11)

* [ ] **HIGH** if: asset drain/theft OR permanent loss with PoC/credible path.
* [ ] **MEDIUM** if: core DoS, governance blockage under reasonable conditions, EV leakage ≥ non-dust, meaningful accounting/price error, likely priv-esc.
* [ ] **QA/LOW** if: dust only, view/events only, requires admin misuse, unsupported token quirk, speculative-only.

## PASS 3 OUTPUT (required)

* **Impact:** 1–2 lines (worst credible) + bound
* **Likelihood:** rating + preconditions
* **PoC:** yes/no + what it proves
* **Final severity:** High / Medium / QA
* **Confidence:** High / Medium / Low (based on evidence + PoC)

---

If you want, I can also rewrite your verifier prompt so it runs these **three passes automatically**, and outputs a strict JSON verdict like:

* `pass1_scope_token_speculation`
* `pass2_classification`
* `pass3_severity`

…so Augment can’t “skip” the token/speculation checks.
