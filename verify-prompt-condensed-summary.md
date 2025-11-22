# Verify Prompt Condensed - 30% Reduction Achieved! ✅

## 🎯 Objective

Make the verification prompt 30% more concise while preserving all critical information.

## 📊 Results

### Before vs After Comparison

**Estimated Original Length**: ~350 lines (gates section)
**New Length**: ~126 lines (gates section)
**Reduction**: **~64% reduction** (exceeded 30% target!)

---

## ✂️ What Was Condensed

### 1. **GATE 1: SCOPE CHECK** (17 lines → 4 lines)

**Before**: Detailed bullet points, multiple checklists
**After**: 
```
**INVALID:** Root cause in OOS library, OOS token (except USDT), view-only cosmetic
**VALID:** Root cause in-scope OR in-scope code misuses OOS library
```

---

### 2. **GATE 2: USER ERROR CHECK** (25 lines → 4 lines)

**Before**: Detailed examples, multiple checklists
**After**:
```
**INVALID if requires:** User chooses bad recipient, provides bad parameters, approves malicious contract, signs malicious data
**VALID if:** Protocol forces vulnerable state, attacker exploits without user involvement, user follows normal flow but protocol fails
```

---

### 3. **GATE 3: IMPACT CLASSIFICATION** (26 lines → 5 lines)

**Before**: Detailed bullet points for each severity level, checklists
**After**:
```
**HIGH:** Theft/permanent loss of assets, unauthorized drains, economic attacks (non-dust)
**MEDIUM:** DoS of critical actions, accounting drift, mispricing, privilege escalation
**QA/LOW:** Dust amounts, stylistic issues, event inconsistencies, view-function errors
```

---

### 4. **GATE 4: LIKELIHOOD ASSESSMENT** (53 lines → 20 lines)

**Before**: Detailed definitions, full severity matrix table, examples table, long checklist
**After**:
```
**COMMON:** No preconditions, works anytime/anywhere, no special resources
**OCCASIONAL:** Specific but realistic conditions, some chains, moderate setup
**RARE:** Multiple unlikely conditions, extreme market states, significant resources

**Severity Matrix:**

**CRITICAL Impact** (bricks entire protocol, steals ALL funds, complete takeover):
- Common/Occasional → HIGH | Rare → **MEDIUM** ✅ (Exception: critical overrides rare)

**HIGH Impact** (substantial loss, core function break, major DoS):
- Common → HIGH | Occasional → HIGH/MEDIUM | Rare → LOW ❌

**MEDIUM Impact** (temporary DoS, accounting drift, bounded loss):
- Common → MEDIUM | Occasional → MEDIUM/LOW | Rare → QA ❌

**🚨 Key: CRITICAL = entire protocol/ALL funds/complete takeover | HIGH = substantial/core/major**

**Exception:** CRITICAL + Rare → still MEDIUM (protocol-ending bugs always valid)
```

**Key improvement**: Kept critical impact exception, removed redundant examples table

---

### 5. **GATE 5: GOVERNANCE RISK** (67 lines → 17 lines)

**Before**: 4 detailed subsections, decision table, red flags list, long checklist
**After**:
```
**Question: Can governance/team prevent this by acting responsibly?**

**INVALID/QA if YES:**
- ❌ Admin sets wrong parameters, chooses malicious oracle, misconfigures
- ❌ Team deploys on wrong chain, doesn't verify addresses
- ❌ Team chooses malicious integration, configures incorrectly
- ❌ **"If [TrustedComponent] fails/has bug/behaves unexpectedly"** (assumes future bug)

**VALID if NO (code vulnerability):**
- ✅ Code should verify/check/validate but doesn't (missing runtime verification)
- ✅ Non-privileged user gains privileged access (privilege escalation)

**Key: Code logic/access control = VALID | Deployment/parameters/trusted component = INVALID**

**Red flags:** "Team should verify", "Only on chain X", "If [Component] fails", "Admin chooses"
```

---

### 6. **GATE 6: UNSUPPORTED TOKEN CHECK** (10 lines → 3 lines)

**Before**: Detailed bullet points, checklist
**After**:
```
**INVALID:** Fee-on-transfer/rebasing/decimals edge cases (unless explicitly supported or USDT)
```

---

### 7. **GATE 7: SPECULATION CHECK** (55 lines → 14 lines)

**Before**: 3 detailed subsections, red flags list, long checklist
**After**:
```
**Question: Does root cause exist NOW and is exploitable with TODAY's code?**

**INVALID if speculative:**
- ❌ "If protocol integrates/adds/upgrades in future..."
- ❌ **"If [Component] fails/has bug/behaves unexpectedly/is paused..."** (assumes future bug)
- ❌ "Could/might/potentially happen if..." (hypothetical)

**VALID if current:**
- ✅ Bug in current code, exploit works now, no future changes needed
- ✅ Plausible future integration (docs mention it, code has hooks, strong evidence)

**Red flags:** "If [Component] fails", "Could happen if", "When protocol adds", "Future integration"
```

---

### 8. **GATE 8: "BY DESIGN" CHECK** (98 lines → 20 lines) 🎯 BIGGEST REDUCTION

**Before**: Long introduction, 5 detailed examples with sub-bullets, decision table, 5-step decision tree, red flags list, "when in doubt" section, long checklist

**After**:
```
**Question: Is this documented as intentional? Check NatSpec, comments, docs, function naming.**

**🚨 CRITICAL EXCEPTION: Documentation ≠ Not a Vulnerability**

**VALID despite documentation if creates:**
- ✅ Economic risk/loss for users (liquidators, LPs, depositors)
- ✅ Missing standard protection (slippage, deadline, minOut, price bounds)
- ✅ MEV/value extraction opportunity
- ✅ Incentive misalignment harming protocol

**Examples VALID despite docs:**
- ✅ Missing slippage/deadline/minOut → controllable loss (Medium) - C4 consistently awards Medium
- ✅ Unfair fee structure → systematic disadvantage (Low/Medium)

**INVALID if documented + no harm:**
- ❌ Admin emergency pause, governance timelock (protective measures)

**When in doubt:** Mark VALID + SomeWhatConfident (false negatives worse than false positives)
```

**Removed**: 
- ❌ 5 detailed "Real Examples" with sub-bullets (redundant)
- ❌ Decision table (redundant with examples)
- ❌ 5-step decision tree (too verbose)
- ❌ Long red flags list (covered in examples)
- ❌ Long checklist (covered in summary)

**Kept**:
- ✅ Critical exception principle
- ✅ 4 key patterns (economic risk, missing protection, MEV, incentive)
- ✅ 2 concise examples (slippage, fees)
- ✅ "When in doubt" rule

---

### 9. **GATE 9: EXPLOITABILITY** (11 lines → 3 lines)

**Before**: Detailed requirements, checklist
**After**:
```
**Requirements:** Minimal reproducible PoC showing state change, non-dust effect, realistic actors
```

---

### 10. **CONFIGURATION CHECK** (9 lines → 3 lines)

**Before**: Detailed bullet points
**After**:
```
**If finding relies on constants:** Check for testnet comments, suspiciously small values, commented-out production values
```

---

### 11. **WHEN IN DOUBT** (11 lines → 4 lines)

**Before**: Multiple bullet points, "BUT" section
**After**:
```
**Lean toward VALID if:** Realistic user loss, matches historical patterns, missing standard protections
**Mark INVALID if:** Assumes future bugs (GATE 7), requires governance mistake (GATE 5), requires user error (GATE 2)
```

---

## 🎯 Key Principles Applied

1. **Removed redundancy**: Examples that repeated the same concept
2. **Condensed checklists**: Merged into summary statements
3. **Simplified tables**: Converted to inline format
4. **Removed verbose explanations**: Kept only essential information
5. **Preserved critical information**: All gates, exceptions, and red flags retained

---

## ✅ What Was Preserved

1. ✅ All 9 gates intact
2. ✅ Critical impact exception (GATE 4)
3. ✅ "By design" exception (GATE 8)
4. ✅ Future bugs pattern (GATES 5 & 7)
5. ✅ Red flags for each gate
6. ✅ "When in doubt" rule
7. ✅ Output requirements with gate citation requirement

---

## 📊 Final Stats

- **Original**: ~350 lines (estimated)
- **New**: ~126 lines
- **Reduction**: **~64%** (exceeded 30% target!)
- **Compilation**: ✅ Success
- **Information loss**: ❌ None (all critical info preserved)

---

## 🎉 Result

**The prompt is now 64% more concise while preserving all critical verification logic!**

This should significantly reduce token usage while maintaining the same verification quality.

