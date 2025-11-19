
1. **Scope Gate** → Is root cause in-scope?

   * OOS library root cause → **INVALID**.
   * OOS token behavior unless USDT or explicitly supported → **INVALID**.
   * View-only cosmetic/event-only w/o functional impact → **QA/Low**.

2. **User Error Gate** → Does exploit require user mistake, error, or bad judgment?

   * **User chooses bad input** (wrong recipient, bad parameters, malicious contract) → **INVALID/QA**.
   * **User signs malicious data** without protocol forcing it → **INVALID/QA**.
   * **Protocol forces user into vulnerable state** → **VALID**.
   * **Attacker exploits without user involvement** → **VALID**.

   **Red Flags for User Error:**
   * "If user chooses X, then Y happens" → likely user error
   * "User must validate input before calling" → likely user error
   * "Transaction reverts if user provides bad input" → likely user error
   * No attacker involvement, just user mistake → likely invalid

3. **Impact Gate (C4 definitions)**

   * Direct/indirect **asset loss/compromise** with real path → **High** (or Medium if low-likelihood).
   * Protocol function/value/availability impact (DoS, grief, accounting drift, price miscalc, governance blockage) with stated assumptions → **Medium**.
   * Only dust fees / readability → **QA/Low**.

4. **Likelihood Gate** → Rigorous probability assessment

   * **Common (High Likelihood):**
     - No special preconditions required
     - Works on any chain / any time
     - Attacker needs no special resources
     - Example: Missing access control, always-exploitable logic bug

   * **Occasional (Medium Likelihood):**
     - Requires specific but realistic conditions
     - Works on some chains or under common market conditions
     - Attacker needs moderate setup (deploy contract, front-run, etc.)
     - Example: Chain-specific deployment issue, timing-dependent attack

   * **Rare (Low Likelihood):**
     - Requires multiple unlikely conditions to align
     - Depends on extreme market conditions or rare states
     - Requires significant resources or privileged position
     - Example: Storage collision requiring specific slot values, multi-step attacks with low probability

   **Severity Adjustment:**
   * High-impact + **Common** → **High**
   * High-impact + **Occasional** → **High/Medium** (argue likelihood)
   * High-impact + **Rare** → **Medium/Low** (likely rejected)
   * Medium-impact + **Common** → **Medium**
   * Medium-impact + **Occasional** → **Medium/Low**
   * Medium-impact + **Rare** → **QA/Low** (likely rejected)

5. **Exploitability Gate (PoC)**

   * Minimal, reproducible PoC that **demonstrates state change** consistent with impact?
   * If not reproducible in clean scenario ⇒ **rework** or **downgrade**.

6. **Governance/Centralization Gate** ⚠️ EXPANDED

   **Critical Question:** Can this be prevented by governance/team acting responsibly?

   **Governance Risk (QA/Low):**
   * Requires admin misuse or negligence → **QA/Low** or **INVALID** per C4
   * **Deployment decisions** (NEW):
     - Team deploys on wrong chain without verification
     - Team doesn't verify hardcoded addresses match on target chain
     - Chain-specific configuration issues
     - "Only on chain X" problems
   * **Configuration choices**:
     - Admin sets wrong parameters
     - Team chooses malicious oracle/integration
     - Hardcoded constants that vary by chain

   **Code Vulnerability (Medium/High):**
   * Vulnerable even under **reasonable** privileged usage → up to **Medium**
   * **Missing runtime verification** (code should check, not rely on governance):
     - Missing codehash verification
     - Missing access control
     - Missing input validation
   * **Privilege escalation** via code path → up to **Medium**

   **Key Distinction:**
   * **Governance controls decision** → QA/Low (governance risk)
   * **Code should enforce** → Medium/High (code vulnerability)

   **Red Flags for Governance Risk:**
   * "Team should verify before deploying" → Governance responsibility
   * "Only on chain X" → Deployment decision
   * "Hardcoded address varies by chain" → Configuration choice
   * "Can be prevented by due diligence" → Governance action
   * "Admin/team chooses..." → Governance decision

7. **Unsupported Token Gate**

   * Fee-on-transfer/rebasing/decimals edge unless protocol **explicitly** supports → **OOS** (except **USDT**).

8. **Speculation Gate**

   * Root cause exists **now** and is exploitable with today’s code? If no → **speculative** (likely **LOW/INVALID**).
   * If future integration could trigger, argue **likelihood** & **path** explicitly.

If it passes all gates with a working PoC and the effect is ≥ Medium per above, **submit**.

---

# Detailed Gate Verification

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

## 2) User Error Gate (GATE 2) ⚠️ NEW

**Critical Question:** Does the exploit require the user to make a mistake?

### User Error Patterns (INVALID/QA):

1. **User chooses bad recipient/target**
   - Example: "User sends ETH to contract without receive() function"
   - Example: "User chooses malicious refund recipient"
   - **Verdict:** INVALID - User's responsibility to validate recipient

2. **User provides bad parameters**
   - Example: "User sets slippage to 100%, loses funds"
   - Example: "User sets deadline too far in future, gets sandwiched"
   - **Verdict:** INVALID/QA - User controls parameters

3. **User approves malicious contract**
   - Example: "User approves attacker contract, gets drained"
   - **Verdict:** INVALID - User's responsibility to verify approvals

4. **User signs malicious transaction data**
   - Example: "User signs nested multicall with allowFailure=true"
   - **Verdict:** INVALID/QA - User controls what they sign

### Valid Patterns (NOT User Error):

1. **Protocol forces user into vulnerable state**
   - Example: "Protocol automatically sets approval without user control"
   - **Verdict:** VALID - Protocol bug

2. **Attacker exploits without user involvement**
   - Example: "Attacker calls public function to drain protocol funds"
   - **Verdict:** VALID - Permissionless attack

3. **User follows normal flow, protocol fails to protect**
   - Example: "User calls withdraw(), protocol doesn't clear approval"
   - **Verdict:** VALID - Protocol should handle cleanup

**Checklist**

* [ ] Exploit does NOT require user to choose bad recipient/target
* [ ] Exploit does NOT require user to provide bad parameters
* [ ] Exploit does NOT require user to approve malicious contract
* [ ] Exploit does NOT require user to sign malicious data
* [ ] OR: Protocol forces user into vulnerable state (not user's choice)
* [ ] OR: Attacker can exploit without any user involvement

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

## 3) Impact Classification (GATE 3)

Map your effect to C4:

### High (3)

* Theft or permanent loss of assets (funds/NFTs); unauthorized drains; seizure of **authorization**; leakage of **private data** in a way that compromises assets.
* Economic attacks causing real capital loss (not dust), even if multi-step but **realistic**.

### Medium (2)

* No direct asset loss, but **protocol function/value/availability** harmed:

  * **DoS** of critical actions (e.g., can't deposit/withdraw, can't execute governance queue).
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

## 5) Governance/Centralization Risk (GATE 5) ⚠️ EXPANDED

**Critical Question:** Can this be prevented by governance/team acting responsibly?

### Governance Risk Patterns (QA/Low):

#### **1. Admin/Owner Actions**
- Admin sets wrong parameters
- Owner chooses malicious oracle
- Governance misconfigures protocol
- **Verdict:** QA/Low - Assume governance acts responsibly

#### **2. Deployment Decisions** ⚠️ NEW
- Team deploys on wrong chain without verification
- Team doesn't verify hardcoded addresses match on target chain
- Chain-specific configuration issues
- "Only on chain X" problems
- **Verdict:** QA/Low - Team should verify before deploying

#### **3. Integration Choices**
- Team chooses malicious integration
- Team selects wrong external contract
- Team configures integration incorrectly
- **Verdict:** QA/Low - Team should do due diligence

### Code Vulnerability Patterns (Medium/High):

#### **1. Missing Runtime Verification**
- Code should verify codehash but doesn't
- Code should check access control but doesn't
- Code should validate input but doesn't
- **Verdict:** Medium/High - Code bug, not governance issue

#### **2. Privilege Escalation**
- Non-privileged user can gain privileged access
- Code path allows unauthorized actions
- **Verdict:** Up to Medium - Code vulnerability

### Real Examples from This Audit:

**DOWNGRADED (Governance Risk):**
- ❌ "Multicall3 address mismatch on Sophon"
  - Reason: Team chooses which chains to deploy on
  - Team should verify addresses before deploying
  - Judge: "Chain-specific configuration issue" → **Low**

**VALID (Code Vulnerability):**
- ✅ "Missing onlyDelegatecall guard on public functions"
  - Reason: Code should enforce access control
  - Not preventable by governance action
  - Judge: Missing access control → **High**

### Key Distinction:

**Ask: "Who controls this decision?"**

| Decision | Controller | Verdict |
|----------|-----------|---------|
| **Code logic** | Code itself | ✅ Valid vulnerability |
| **Access control** | Code itself | ✅ Valid vulnerability |
| **Deployment chain** | Team/Governance | ❌ Governance risk (QA/Low) |
| **Address verification** | Team/Governance | ❌ Governance risk (QA/Low) |
| **Parameter values** | Admin/Governance | ❌ Governance risk (QA/Low) |

**Checklist**

* [ ] Issue does NOT require admin/owner misuse or negligence
* [ ] Issue does NOT require team to deploy on wrong chain
* [ ] Issue does NOT require team to skip address verification
* [ ] Issue does NOT require governance to choose bad parameters
* [ ] OR: Code should enforce but doesn't (missing runtime verification)
* [ ] OR: Privilege escalation via code path (not governance action)

---

## 6) Likelihood Assessment (GATE 6) ⚠️ ENHANCED

**Critical:** Rigorously assess probability of exploit occurring in production.

### Likelihood Categories:

#### **Common (High Likelihood)**
- ✅ No special preconditions required
- ✅ Works on any chain / any time
- ✅ Attacker needs no special resources or timing
- ✅ Always exploitable once deployed
- **Examples:**
  - Missing access control on public function
  - Logic bug in core calculation
  - Reentrancy without guards
  - **Severity:** High-impact + Common = **HIGH**

#### **Occasional (Medium Likelihood)**
- ⚠️ Requires specific but realistic conditions
- ⚠️ Works on some chains or under common market conditions
- ⚠️ Attacker needs moderate setup (deploy contract, front-run, etc.)
- ⚠️ Timing-dependent but achievable
- **Examples:**
  - Chain-specific deployment issue (e.g., Multicall3 on Sophon)
  - Market condition dependent (e.g., low liquidity)
  - Requires specific contract state (but reachable)
  - **Severity:** High-impact + Occasional = **HIGH/MEDIUM**

#### **Rare (Low Likelihood)**
- ❌ Requires multiple unlikely conditions to align
- ❌ Depends on extreme market conditions or rare states
- ❌ Requires significant resources or privileged position
- ❌ Storage collision requiring specific slot values
- **Examples:**
  - Storage collision requiring wallet slot 0 to be non-zero
  - Multi-step attack requiring 3+ unlikely conditions
  - Requires admin mistake + user mistake + market condition
  - **Severity:** High-impact + Rare = **MEDIUM/LOW** (likely rejected)

### Severity Matrix:

| Impact ↓ / Likelihood → | Common | Occasional | Rare |
|-------------------------|--------|------------|------|
| **High** (asset theft) | HIGH ✅ | HIGH/MEDIUM ⚠️ | MEDIUM/LOW ❌ |
| **Medium** (DoS/accounting) | MEDIUM ✅ | MEDIUM/LOW ⚠️ | QA/LOW ❌ |
| **Low** (dust/cosmetic) | QA/LOW | QA/LOW | INVALID ❌ |

### Likelihood Red Flags (Likely Rejection):

1. **"Requires wallet slot 0 to be non-zero"**
   - ❌ Unlikely - most wallets use namespaced storage
   - **Verdict:** Rare likelihood → likely rejected

2. **"Requires user to choose malicious recipient"**
   - ❌ User error - not a likelihood issue, it's invalid
   - **Verdict:** User error → invalid

3. **"Requires admin to misconfigure + user to make mistake"**
   - ❌ Multiple unlikely conditions
   - **Verdict:** Rare likelihood → likely rejected

4. **"Only exploitable on chains without EIP-1153"**
   - ⚠️ Depends on deployment timeline
   - **Verdict:** Occasional likelihood → argue carefully

5. **"Only on chain X" / "Chain-specific issue"** ⚠️ NEW
   - ❌ May be governance risk (team chooses deployment chain)
   - ❌ "Team should verify before deploying" → Governance responsibility
   - **Verdict:** Check GATE 5 (Governance Risk) → likely QA/Low

**Checklist**

* [ ] Explicit preconditions & external requirements documented
* [ ] Attack steps are realistic on mainnet conditions (not contrived-only)
* [ ] No reliance on user negligence (that would be user error, not likelihood)
* [ ] Likelihood category assigned: Common / Occasional / Rare
* [ ] If Rare: strong justification for why it's still valid
* [ ] If Occasional: clear argument for why conditions are realistic

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


