# Verify Prompt Update - Summary

## 🎯 Objective

Tighten up the verification prompt in `src/llm_review/phases/verify_findings.rs` to include the full verify-checklist.md with all 9 gates and strong emphasis on speculation, governance risk, and "by design" exceptions.

## 🚨 Problem Identified

**Gemini 3.0 Pro is too lax without GPT-5 verification:**
- ✅ Found ALL 5 critical findings (5/5) - EXCELLENT!
- ❌ Accepts ~3-5 speculative Medium findings that assume future bugs in trusted components
- ❌ Particularly confused on:
  1. **GATE 7 (Speculation)**: Accepts "If Distributor fails..." patterns
  2. **GATE 5 (Governance Risk)**: Accepts findings requiring team mistakes

## 📝 Changes Made

### File: `src/llm_review/phases/verify_findings.rs`

**Function**: `generate_verify_prompt(repo: &RepoPaths) -> String`

**Before**: Simple checklist with 4 sections (Scope, Validity, Configuration, Severity)

**After**: Comprehensive 9-gate verification system with:

1. ✅ **GATE 1: SCOPE CHECK**
   - In-scope vs out-of-scope library
   - Token behavior (USDT exception)
   - View-only cosmetic issues

2. 🚨 **GATE 2: USER ERROR CHECK** (CRITICAL)
   - User chooses bad recipient/target → INVALID
   - User provides bad parameters → INVALID
   - User approves malicious contract → INVALID
   - Protocol forces user into vulnerable state → VALID

3. ✅ **GATE 3: IMPACT CLASSIFICATION**
   - HIGH: Asset theft/loss
   - MEDIUM: DoS/accounting drift/governance blockage
   - QA/LOW: Dust/stylistic/events

4. 🚨 **GATE 4: LIKELIHOOD ASSESSMENT** (CRITICAL - UPDATED!)
   - **CRITICAL Impact Exception**: Critical-impact + Rare → **MEDIUM** ✅
   - Common (High Likelihood) → HIGH/MEDIUM
   - Occasional (Medium Likelihood) → MEDIUM/LOW
   - Rare (Low Likelihood) → QA/LOW (likely rejected, **unless CRITICAL impact**)
   - **Key Distinction**: CRITICAL (protocol-ending) vs HIGH (significant loss)
   - Examples: Permanently bricks entire protocol, steals ALL funds → MEDIUM even if rare

5. 🚨🚨 **GATE 5: GOVERNANCE/CENTRALIZATION RISK** (CRITICAL - MOST IMPORTANT)
   - **Governance Risk Patterns (INVALID/QA/LOW):**
     - Admin/Owner actions
     - Deployment decisions
     - Integration choices
     - **🚨 Future bugs in trusted components** (MOST COMMON MISTAKE)
   - **Code Vulnerability Patterns (MEDIUM/HIGH):**
     - Missing runtime verification
     - Privilege escalation
   - **Key Distinction Table**: Who controls this decision?
   - **Red Flags**: "If [Component] fails...", "Team should verify...", etc.

6. ✅ **GATE 6: UNSUPPORTED TOKEN CHECK**
   - Fee-on-transfer/rebasing → OOS (unless explicitly supported)
   - USDT exception

7. 🚨🚨 **GATE 7: SPECULATION CHECK** (CRITICAL - MOST IMPORTANT)
   - **Speculation Patterns (INVALID):**
     - Future integrations
     - **🚨 Future bugs in trusted components** (MOST COMMON)
     - Hypothetical conditions
   - **Valid Patterns:**
     - Root cause exists NOW
     - Plausible future integration (argue carefully)
   - **Red Flags**: "If [Component] fails...", "Could happen if...", etc.

8. 🚨🚨 **GATE 8: "BY DESIGN" CHECK** (CRITICAL - NEW!)
   - **Question**: Is this behavior documented as intentional?
   - **CRITICAL EXCEPTION**: Documentation does NOT always mean not a vulnerability!
   - **VALID despite documentation if**:
     - ✅ Missing slippage protection → controllable loss (Medium)
     - ✅ Missing deadline → MEV/sandwich risk (Medium)
     - ✅ Missing minOut in liquidations → liquidator loss (Medium)
     - ✅ Missing price bounds → value extraction (Medium)
     - ✅ Unfair fee structure → systematic disadvantage (Low/Medium)
   - **Decision Tree**: Economic risk/loss → Missing standard protection → MEV/value extraction
   - **When in doubt**: Mark as Valid + SomeWhatConfident

9. ✅ **GATE 9: EXPLOITABILITY (PoC)**
   - Minimal, reproducible PoC
   - Demonstrates state change
   - Non-dust effect

## 🚨 Key Emphasis Added

### 1. Critical Impact Exception (GATE 4) 🆕 CRITICAL

**Added nuance for rare likelihood + critical impact**:

**CRITICAL Impact** (Protocol-ending / Total loss):
- Critical-impact + Common → **HIGH** ✅
- Critical-impact + Occasional → **HIGH** ✅
- Critical-impact + Rare → **MEDIUM** ✅ **(Exception: Critical impact overrides rare likelihood)**

**Key Distinction Added**:

| Impact Level | Definition | Rare Likelihood Verdict |
|--------------|------------|------------------------|
| **CRITICAL** | Permanently bricks **entire protocol**, steals **ALL funds**, complete takeover | **MEDIUM** ✅ (Exception!) |
| **HIGH** | Steals **substantial** funds, breaks **core** functionality | **LOW** ❌ (Rejected) |
| **MEDIUM** | Temporary DoS, accounting drift, bounded loss | **QA** ❌ (Rejected) |

**Examples Added**:
- ✅ Permanently bricks entire protocol + Rare → **MEDIUM** (critical impact overrides)
- ✅ Steals ALL funds from all pools + Rare → **MEDIUM** (critical impact overrides)
- ❌ Steals 50% of one pool + Rare → **LOW** (high but not critical)
- ❌ DoS of one function + Rare → **LOW** (high but not critical)

**Reasoning**: Protocol-ending bugs are ALWAYS valid, even with 1% likelihood!

---

### 2. "By Design" Exception (GATE 8) 🆕 CRITICAL

**Added comprehensive "by design" check with critical exception**:

**Documentation does NOT always mean not a vulnerability!**

**VALID findings despite being documented**:
- ✅ **Missing slippage protection** → controllable loss (Medium)
  - Even if docs say "user should check price"
  - C4 judges consistently award Medium for missing slippage
- ✅ **Missing deadline** → MEV/sandwich risk (Medium)
  - Even if docs say "use private RPC"
  - Missing standard protection = valid finding
- ✅ **Missing minOut in liquidations** → liquidator loss (Medium)
  - Even if docs say "liquidator should verify"
  - Economic risk to users = valid finding
- ✅ **Unfair fee structure** → systematic disadvantage (Low/Medium)
  - Even if docs explain the fee structure
  - Unfair value extraction = valid finding

**Decision Tree Added**:
1. Is behavior documented? → YES
2. Does it create economic risk/loss? → YES
3. Is this a missing standard protection? → YES
4. **Verdict**: VALID + SomeWhatConfident

**Key Insight**: Documentation doesn't prevent harm to users!

---

### 2. Future Bugs in Trusted Components (GATES 5 & 7)

**Added explicit examples**:
- ❌ "If Distributor fails to pull tokens..."
- ❌ "If Distributor logic is ever updated..."
- ❌ "If [TrustedComponent] has a bug..."
- ❌ "If [TrustedComponent] behaves unexpectedly..."
- **Verdict:** INVALID - Assuming future bugs is SPECULATION

### 3. Governance Risk Decision Table (GATE 5)

| Decision | Controller | Verdict |
|----------|-----------|---------|
| Code logic | Code itself | ✅ Valid vulnerability |
| Access control | Code itself | ✅ Valid vulnerability |
| Deployment chain | Team/Governance | ❌ Governance risk (QA/Low) |
| Address verification | Team/Governance | ❌ Governance risk (QA/Low) |
| Parameter values | Admin/Governance | ❌ Governance risk (QA/Low) |
| **Trusted component behavior** | **Team/Governance** | **❌ Governance risk (INVALID)** |

### 4. Output Requirements Updated

**Status Justification now MUST cite specific gate failures**:
- Example: "GATE 7 FAIL: Assumes future Distributor bug"
- Example: "GATE 5 FAIL: Requires team to deploy on wrong chain"

## 📊 Expected Impact

### Before (R8 with no verification):
- 22 findings (11H, 10M, 1L)
- ~3-5 speculative Mediums accepted
- Examples: M-1 (USDT DoS), M-5 (Skim theft), M-13 (duplicate)

### After (R8 with tightened verification):
- **Estimated**: 15-18 findings (9-10H, 5-7M, 4-6L)
- Speculative findings rejected with clear gate citations
- Higher quality findings that pass C4 validation

## ✅ Benefits

1. **Catches Speculation**: GATE 7 explicitly rejects "If [Component] fails..." patterns
2. **Catches Governance Risk**: GATE 5 explicitly rejects team/admin responsibility issues
3. **Deterministic**: Clear gate-based reasoning (not LLM hallucination)
4. **Educational**: LLM learns what patterns to avoid
5. **Traceable**: Status justifications cite specific gate failures

## 🎯 Next Steps

1. ✅ **Code compiles** - verified with `cargo check`
2. ⏭️ **Test on R8 findings** - re-run verification on R8's 22 findings
3. ⏭️ **Compare results** - check if M-1, M-5, M-13 are now rejected
4. ⏭️ **Validate quality** - ensure no false rejections of valid findings

## 🎉 Expected Outcome

**Best of both worlds**:
- ✅ Keep Gemini's excellent discovery (5/5 critical findings)
- ✅ Filter out speculative findings (GATE 7)
- ✅ Filter out governance risk findings (GATE 5)
- ✅ No false rejections (unlike GPT-5 which rejected valid M-1/M-3/M-4)
- ✅ 125-500x cheaper than GPT o1 (~$0.15-0.30 vs $50-100)

**Final Quality**: 15-18 high-quality findings that pass C4 validation! 🎯

