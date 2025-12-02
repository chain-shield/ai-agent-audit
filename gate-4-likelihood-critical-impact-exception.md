# GATE 4: Likelihood Assessment - Critical Impact Exception

## 🚨 Important Nuance Added

### Problem Statement

**Original severity matrix was too strict**:
- High-impact + Rare → MEDIUM/LOW ❌ (likely rejected)

**Missing nuance**: **CRITICAL impact findings should still be valid even with rare likelihood!**

---

## 🎯 Updated GATE 4: Likelihood Assessment

### New Severity Matrix with Critical Impact Exception

#### **CRITICAL Impact** (Protocol-ending / Total loss):
- Critical-impact + Common → **HIGH** ✅
- Critical-impact + Occasional → **HIGH** ✅
- Critical-impact + Rare → **MEDIUM** ✅ **(Exception: Critical impact overrides rare likelihood)**

**Examples of CRITICAL Impact**:
- ✅ Permanently bricks **entire protocol** (not just one function)
- ✅ Steals **ALL funds** from protocol (not just one user or pool)
- ✅ Complete protocol takeover (attacker gains full control)
- ✅ Irreversible catastrophic failure (no recovery possible)

---

#### **HIGH Impact** (Significant loss / Major function break):
- High-impact + Common → **HIGH** ✅
- High-impact + Occasional → **HIGH/MEDIUM** ⚠️
- High-impact + Rare → **MEDIUM/LOW** ❌ (likely rejected unless critical)

**Examples of HIGH Impact**:
- ⚠️ Steals **substantial** funds (but not all)
- ⚠️ Breaks **core** functionality (but not entire protocol)
- ⚠️ Major DoS (but recoverable)

---

#### **MEDIUM Impact** (Limited loss / Function impairment):
- Medium-impact + Common → **MEDIUM** ✅
- Medium-impact + Occasional → **MEDIUM/LOW** ⚠️
- Medium-impact + Rare → **QA/LOW** ❌ (likely rejected)

**Examples of MEDIUM Impact**:
- ⚠️ Temporary DoS
- ⚠️ Accounting drift
- ⚠️ Bounded loss

---

## 🚨 CRITICAL vs HIGH Impact - Key Distinction

### **CRITICAL Impact** (Rare likelihood still valid):

**Characteristics**:
- ✅ **Entire protocol** affected (not just one component)
- ✅ **ALL funds** at risk (not just substantial)
- ✅ **Complete takeover** (not just privilege escalation)
- ✅ **Irreversible** (no recovery possible)

**Verdict**: Even if Rare → **MEDIUM** ✅ (critical impact overrides likelihood)

**Reasoning**: 
- Protocol-ending bugs are ALWAYS valid
- Even 1% chance of total protocol failure = Medium severity
- Risk = Impact × Likelihood, but critical impact has a floor

---

### **HIGH Impact** (Rare likelihood likely rejected):

**Characteristics**:
- ⚠️ **Substantial** funds at risk (but not all)
- ⚠️ **Core** functionality broken (but not entire protocol)
- ⚠️ **Major** DoS (but recoverable)
- ⚠️ **Significant** loss (but bounded)

**Verdict**: If Rare → **MEDIUM/LOW** ❌ (likely rejected)

**Reasoning**:
- High impact + rare likelihood = too unlikely
- Unless impact is truly critical (protocol-ending)

---

## 📊 Examples Table

| Finding | Impact | Likelihood | Severity | Reasoning |
|---------|--------|------------|----------|-----------|
| **Permanently bricks entire protocol** | **CRITICAL** | Rare | **MEDIUM** ✅ | Critical impact overrides rare likelihood |
| **Steals ALL funds from all pools** | **CRITICAL** | Rare | **MEDIUM** ✅ | Critical impact overrides rare likelihood |
| **Complete protocol takeover** | **CRITICAL** | Rare | **MEDIUM** ✅ | Critical impact overrides rare likelihood |
| **Irreversible total loss of all assets** | **CRITICAL** | Rare | **MEDIUM** ✅ | Critical impact overrides rare likelihood |
| Steals 50% of one pool | HIGH | Rare | **LOW** ❌ | High (not critical) + rare = likely rejected |
| DoS of one core function | HIGH | Rare | **LOW** ❌ | High (not critical) + rare = likely rejected |
| Breaks withdraw for one token | HIGH | Rare | **LOW** ❌ | High (not critical) + rare = likely rejected |
| Accounting drift in one pool | MEDIUM | Rare | **QA** ❌ | Medium + rare = likely rejected |

---

## 🎯 Real-World Examples

### Example 1: Critical Impact + Rare = MEDIUM ✅

**Finding**: "Attacker can permanently brick entire protocol by exploiting edge case in upgrade mechanism"

**Impact**: CRITICAL
- Entire protocol becomes unusable
- All funds permanently locked
- No recovery possible

**Likelihood**: Rare
- Requires specific upgrade state
- Requires attacker to front-run upgrade transaction
- Requires specific block timestamp alignment

**Verdict**: **MEDIUM** ✅

**Reasoning**: 
- Even though likelihood is rare, impact is protocol-ending
- Critical impact overrides rare likelihood
- 1% chance of total protocol failure = Medium severity

---

### Example 2: High Impact + Rare = LOW ❌

**Finding**: "Attacker can steal 50% of one pool by exploiting edge case in oracle update"

**Impact**: HIGH (but not CRITICAL)
- Substantial funds at risk (50% of one pool)
- But not ALL funds (other pools unaffected)
- Protocol continues to function

**Likelihood**: Rare
- Requires specific oracle malfunction
- Requires specific market conditions
- Requires attacker to front-run oracle update

**Verdict**: **LOW** ❌ (likely rejected)

**Reasoning**:
- Impact is high but not critical (not protocol-ending)
- Rare likelihood + non-critical impact = too unlikely
- Would need to be Common or Occasional to be valid

---

## 🚨 Updated Checklist

When assessing likelihood, now check:

- [ ] Explicit preconditions documented
- [ ] Attack steps are realistic on mainnet (not contrived-only)
- [ ] No reliance on user negligence
- [ ] Likelihood category assigned: **Common / Occasional / Rare**
- [ ] Impact category assigned: **Critical / High / Medium / Low**
- [ ] **If Rare: Is impact CRITICAL (protocol-ending)? If YES → still MEDIUM** ✅
- [ ] **If Rare + High (not critical): strong justification needed (likely rejected)**

---

## 🎯 Key Takeaway

**Critical impact findings are ALWAYS valid, even with rare likelihood!**

**Why?**
- Protocol-ending bugs cannot be ignored
- Even 1% chance of total failure = Medium severity
- Risk has a floor when impact is catastrophic
- Better to report and let team decide than miss protocol-ending bug

**Formula**:
- **Critical Impact + Rare** = **MEDIUM** ✅ (Exception!)
- **High Impact + Rare** = **LOW** ❌ (Rejected)
- **Medium Impact + Rare** = **QA** ❌ (Rejected)

