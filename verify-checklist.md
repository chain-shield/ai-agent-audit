
1. **Scope Gate** → Is root cause in-scope?

   * OOS library root cause → **INVALID**.
   * OOS token behavior unless USDT or explicitly supported → **INVALID**.
   * View-only cosmetic/event-only w/o functional impact → **QA/Low**.

2. **Impact Gate (C4 definitions)**

   * Direct/indirect **asset loss/compromise** with real path → **High** (or Medium if low-likelihood).
   * Protocol function/value/availability impact (DoS, grief, accounting drift, price miscalc, governance blockage) with stated assumptions → **Medium**.
   * Only dust fees / readability → **QA/Low**.

3. **Likelihood Gate**

   * High-impact + low-likelihood → **Medium/High** (argue likelihood).
   * Low-impact + low-likelihood → **QA/Low**.

4. **Exploitability Gate (PoC)**

   * Minimal, reproducible PoC that **demonstrates state change** consistent with impact?
   * If not reproducible in clean scenario ⇒ **rework** or **downgrade**.

5. **Centralization Gate**

   * Requires admin misuse or negligence? → **QA/Low** or **INVALID** per C4.
   * Vulnerable even under **reasonable** privileged usage (or privilege escalation) → up to **Medium**.

6. **Unsupported Token Gate**

   * Fee-on-transfer/rebasing/decimals edge unless protocol **explicitly** supports → **OOS** (except **USDT**).

7. **Speculation Gate**

   * Root cause exists **now** and is exploitable with today’s code? If no → **speculative** (likely **LOW/INVALID**).
   * If future integration could trigger, argue **likelihood** & **path** explicitly.

If it passes all gates with a working PoC and the effect is ≥ Medium per above, **submit**.

---

## 1) Scope & Root-Cause Validation (GATE 1)

* **Contract in scope?** File matches contest scope list; cross-check imports.
* **Root cause location:** The bug arises in **in-scope code’s logic** (not in an OOS lib).

  * If the bug is **incorrect use** of an OOS lib by in-scope code → **valid**; cite misuse site.
  * If the bug is **inside** OOS lib → **OOS**.

**Checklist**

* [ ] File is in scope (path & commit).
* [ ] Root cause in in-scope contract/function.
* [ ] If involving tokens: non-standard behavior is **explicitly supported** (or token is **USDT**).

---

## 2) Impact Classification (GATE 2)

Map your effect to C4:

### High (3)

* Theft or permanent loss of assets (funds/NFTs); unauthorized drains; seizure of **authorization**; leakage of **private data** in a way that compromises assets.
* Economic attacks causing real capital loss (not dust), even if multi-step but **realistic**.

### Medium (2)

* No direct asset loss, but **protocol function/value/availability** harmed:

  * **DoS** of critical actions (e.g., can’t deposit/withdraw, can’t execute governance queue).
  * **Accounting drift** creating extractable value in plausible conditions.
  * **Rounding** leading to non-dust value loss/gain.
  * **Governance blockage**/grief preventing timelock execution under reasonable conditions.
  * **Oracle / price calc** issues enabling mispricing (without guaranteed drain).
  * **Privilege escalation** likelihood-dependent (up to Medium).

### QA/Low

* Dust amounts, stylistic issues, events inconsistencies without functional break, pure view-function errors.

**Checklist**

* [ ] Name the **asset or function at risk**.
* [ ] Quantify **magnitude** (≥ dust).
* [ ] Show **who benefits / who loses**.

---

## 3) Likelihood Assessment (GATE 3)

* Document **preconditions** (liquidity, timing, role possession, market setup).
* Decide: **Common**, **Occasional**, or **Edge-case**.
* High-impact + edge-case ⇒ **Medium or High** per C4.
* Low-impact + edge-case ⇒ **QA/Low**.

**Checklist**

* [ ] Explicit preconditions & external requirements.
* [ ] Attack steps are realistic on mainnet conditions (not contrived-only).
* [ ] No reliance on user negligence (that would downgrade).

---

## 4) Exploitability Proof (PoC) (GATE 4)

Produce a **minimal Foundry test** that:

* Sets state to legit scenario (fork or local deployment).
* Executes the **attack path** step-by-step.
* **Asserts** final state delta: balances, auth, timelock state, price, share accounting—**non-dust** effect.

**PoC skeleton**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {Target, Token} from "../src/Target.sol";

contract ExploitTest is Test {
    Target target;
    Token  token;
    address attacker = address(0xBEEF);

    function setUp() public {
        // deploy or fork setup
        target = new Target(/*...*/);
        token  = new Token(/*...*/);
        // seed attacker, pool, etc.
    }

    function test_attack_paths() public {
        // 1) Preconditions
        // 2) Trigger vulnerability
        // 3) Observe effects
        uint256 before = token.balanceOf(attacker);

        // ...attack steps...

        uint256 after_ = token.balanceOf(attacker);
        assertGt(after_ - before, 1e12, "non-dust profit");
        // Or assert state invariants broken / DoS demonstrated
    }
}
```

**Fuzz add-on (optional but strong):** a fuzz where invariant fails under a reasonable domain strengthens Medium/High.

**Checklist**

* [ ] Single-click `forge test` runs the PoC.
* [ ] Clear `assert` shows the **effect** magnitude.
* [ ] Uses realistic actors/roles and parameters.

---

## 5) Centralization & Roles (GATE 5)

* **Assume admins act correctly**; findings requiring admin misuse ⇒ **QA/Low/Invalid**.
* If the attack works **even when admins follow spec** (e.g., parameter constraints violated by code, or unavoidable route) ⇒ **Medium+**.
* **Privilege escalation** via code path under reasonable usage can be **Medium**.

**Checklist**

* [ ] Does not require guardian/governor negligence or malicious admin actions.
* [ ] If privilege involved, it’s an **escalation**, not normal usage.

---

## 6) Token Model Constraints (GATE 6)

* ERC-20 non-standard (rebasing/FoT/decimals mismatch) is **OOS unless explicitly supported** (USDT exception).
* If protocol **claims support**, demonstrate how the handling is insufficient to cause Medium+ effect.

**Checklist**

* [ ] If using non-standard tokens, cite docs proving they are supported.
* [ ] Otherwise, avoid that angle (or mark OOS).

---

## 7) Speculation & Integrations (GATE 7)

* **Root cause exists now**.
* If impact needs a future integration: explain **why that integration is plausible**, and how the root cause already enables it. Otherwise **QA/Low**.

**Checklist**

* [ ] No dependency on hypothetical future code, unless likelihood is argued & credible.

---

## 8) Quantification & Non-Dust Thresholds

* Show **numbers**: amounts at risk, percentage slippage, value drift per operation.
* Back results with math and PoC outputs (e.g., “profit = 0.42 ETH”, “protocol loses 0.3% TVL per cycle”).
* If rounding: prove it scales (loopable grief, first/last depositor attack, or affects many users).

**Checklist**

* [ ] Non-dust threshold exceeded (document the threshold for the asset).
* [ ] If cumulative, prove repeatability.

---

## 9) Duplicate / Non-Novel Check

* Search for the same root cause in prior issues 
* If dup found suggest combining with result with it (if different exploit) or picking one of the 2.


---
## 10) Reliability Boosters (to avoid down-grading)

* **State all assumptions** explicitly (liquidity, oracle freshness, fee settings, role addresses).
* **Show both sides**: a short note why alternate explanations are **not** required (e.g., “doesn’t rely on a revertible user step”).
* **Edge-cases**: include boundary tests (0, 1 wei, min/max inputs, rollover).
* **Events**: if events are involved, tie it to **functional** impact (bridging/proofs); otherwise judges cap at Low.
* **Clean language**: no speculative language without matching proof.

---

## 11) Quick Severity Rubric (snap-score)

* **HIGH** if: asset drain/theft **or** matured yield loss **with PoC**.
* **MEDIUM** if:

  * Functional DoS that blocks core actions, **or**
  * Governance blockage/queue corruption under reasonable conditions, **or**
  * Value leakage ≥ non-dust (loopable/abusable) without guaranteed drain, **or**
  * Price/accounting error affecting user equity in meaningful amounts, **or**
  * Likely privilege escalation.
* **QA/LOW** if:

  * Dust rounding only; stylistic; view-only misreports; event cosmetics; requires admin misuse; unsupported token quirk unless claimed supported.


