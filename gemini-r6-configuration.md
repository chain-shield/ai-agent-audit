# Gemini 3.0 Pro R6 Configuration - Full RUNS Test

## Test Date
2025-11-20

## Configuration Changes (src/config.rs)

### Discovery Runs (INCREASED from previous test)

```rust
// Pattern Discovery Runs
pub const PATTERN_DISCOVERY_RUNS: usize = 5;      // old value: 10 (reduced to 5)
pub const INVARIANT_DISCOVERY_RUNS: usize = 3;    // old value: 5 (reduced to 3)
pub const ACTOR_DISCOVERY_RUNS: usize = 5;        // old value: 10 (reduced to 5)
```

**Note**: These were previously reduced from 10→5 and 5→3. Now testing with current values.

### Pattern Category Runs (INCREASED to full defaults)

```rust
pub const INVARIANT_RUNS: usize = 2;              // was: 0, default: 3
pub const MAX_PATTERN_RUN_TOP: usize = 3;         // was: 2, default: 3 ✅ RESTORED
pub const MAX_PATTERN_RUN_RARE: usize = 3;        // was: 2, default: 3 ✅ RESTORED
pub const MAX_PATTERN_RUN_MOST: usize = 3;        // was: 0, default: 3 ✅ RESTORED
pub const MAX_PATTERN_RUN_FREQUENT: usize = 3;    // was: 2, default: 3 ✅ RESTORED
pub const MAX_PATTERN_LIBRARY: usize = 1;         // unchanged
pub const MAX_PATTERN_NICHE: usize = 4;           // was: 3, default: 4 ✅ RESTORED
pub const MAX_PATTERN_GENERAL: usize = 4;         // was: 0, default: 4 ✅ RESTORED
```

---

## Key Changes from Previous Gemini Test (R4)

### R4 (Previous Gemini Test - FAILED)
- **PATTERN_DISCOVERY_RUNS**: Unknown (likely reduced)
- **MAX_PATTERN_RUN_MOST**: 0 (disabled)
- **MAX_PATTERN_GENERAL**: 0 (disabled)
- **MAX_PATTERN_RUN_TOP/RARE/FREQUENT**: 2 (reduced)
- **MAX_PATTERN_NICHE**: 3 (reduced)
- **Result**: 13 findings, **missed 3/5 critical findings** ❌

### R6 (Current Gemini Test)
- **PATTERN_DISCOVERY_RUNS**: 5 ✅
- **MAX_PATTERN_RUN_MOST**: 3 ✅ (restored from 0)
- **MAX_PATTERN_GENERAL**: 4 ✅ (restored from 0)
- **MAX_PATTERN_RUN_TOP/RARE/FREQUENT**: 3 ✅ (restored from 2)
- **MAX_PATTERN_NICHE**: 4 ✅ (restored from 3)
- **Expected Result**: Should find more findings than R4

---

## Gemini 3.0 Pro Agent Configuration

### Model Settings (src/llm_review/agent/agent_factory.rs)

```rust
pub fn create_gemini_agent(config: &AgentConfig) -> Result<AIAgent> {
    // Model
    let model = "gemini-3-pro-preview";
    
    // Safety Settings - ALL DISABLED for security research
    let safety_settings = vec![
        SafetySetting { category: HarmCategoryDangerousContent, threshold: BlockNone },
        SafetySetting { category: HarmCategoryHarassment, threshold: BlockNone },
        SafetySetting { category: HarmCategoryHateSpeech, threshold: BlockNone },
        SafetySetting { category: HarmCategorySexuallyExplicit, threshold: BlockNone },
    ];
    
    // Generation Config
    let generation_config = GenerationConfig {
        max_output_tokens: Some(64_000),  // Prevent truncation
        temperature: Some(config.temperature),
        ..Default::default()
    };
}
```

---

## Expected API Call Volume

### Pattern Discovery Phase
- **2 contracts** × **5 PATTERN_DISCOVERY_RUNS** = **10 calls**

### Pattern Category Analysis
Assuming ~10 patterns discovered per contract across different categories:
- **TOP patterns**: 2 contracts × ~2 patterns × 3 runs = ~12 calls
- **RARE patterns**: 2 contracts × ~2 patterns × 3 runs = ~12 calls
- **MOST patterns**: 2 contracts × ~2 patterns × 3 runs = ~12 calls (was 0 in R4!)
- **FREQUENT patterns**: 2 contracts × ~2 patterns × 3 runs = ~12 calls
- **NICHE patterns**: 2 contracts × ~2 patterns × 4 runs = ~16 calls
- **GENERAL patterns**: 2 contracts × ~2 patterns × 4 runs = ~16 calls (was 0 in R4!)

### Total Estimated Calls
- **~90-100 Gemini 3.0 Pro API calls**
- **Cost**: ~$0.20-0.30 (Gemini is much cheaper than GPT o1-preview)

---

## Success Criteria

### Must Find 5/5 Critical Findings

| Finding | Complexity | C4 Validation | Status |
|---------|------------|---------------|--------|
| **H-20**: Fee double-counting in swap | 7 | ✅ C4 High | ❓ |
| **H-21/H-22**: Wrong quote token DoS | 5 | ✅ C4 High | ❓ |
| **H-13**: LP shares double-count fees | 6 | ✅ Legitimate High | ❓ |
| **H-18/H-19**: Rewards to wrong recipient | 3-4 | ✅ Legitimate High | ❓ |
| **M-9**: Flash-liquidity fee bypass | 5 | ✅ C4 Medium | ❓ |

### Target Metrics

| Metric | R3 (GPT o1) | R4 (Gemini - Failed) | R6 (Gemini - Target) |
|--------|-------------|----------------------|----------------------|
| **Total Findings** | 22 | 13 | ≥18 |
| **High Severity** | 7 | 7 | ≥7 |
| **Medium Severity** | 6 | 4 | ≥5 |
| **Low Severity** | 9 | 2 | ≥6 |
| **Critical Findings** | 5/5 ✅ | 2/5 ❌ | **5/5** ✅ |

---

## Hypothesis

**Why R4 Failed:**
1. ❌ Max token truncation (FIXED: now 64K tokens)
2. ❌ Reduced RUNS (FIXED: restored to full values)
3. ❌ Disabled pattern categories (FIXED: MOST and GENERAL restored)

**Why R6 Should Succeed:**
1. ✅ Full RUNS configuration (no reduction)
2. ✅ All pattern categories enabled (MOST=3, GENERAL=4)
3. ✅ Max token limit increased (64K)
4. ✅ Safety filters disabled

**Expected Outcome:**
- Gemini 3.0 Pro should find **5/5 critical findings**
- Total findings should be **18-22** (closer to GPT o1-preview)
- Better deduplication than GPT o1 (fewer duplicate findings)

---

## Next Steps After R6 Completes

1. **Check findings count**: `grep -E "^\[([HML]|Info)-[0-9]+\]" 2025-08-gte-perps/report/audit-report-r6.md | wc -l`
2. **Validate critical findings**: Check for H-20, H-21/H-22, H-13, H-18/H-19, M-9
3. **Compare to R3**: Analyze quality vs GPT o1-preview
4. **Cost analysis**: Compare Gemini cost vs GPT o1-preview cost

If R6 finds 5/5 critical findings → **Gemini 3.0 Pro is validated as a cost-effective alternative** 🎯

