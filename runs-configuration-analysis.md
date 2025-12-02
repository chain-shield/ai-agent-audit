# RUNS Configuration Analysis - Cost Optimization

## Current Configuration (src/config.rs)

### Discovery Runs Constants

```rust
pub const PATTERN_DISCOVERY_RUNS: usize = 5;  // ← Used for pattern discovery
pub const INVARIANT_DISCOVERY_RUNS: usize = 3;
pub const ACTOR_DISCOVERY_RUNS: usize = 5;
```

### Pattern Category Runs (Max runs per vulnerability category)

```rust
pub const INVARIANT_RUNS: usize = 0;           // 0 for large protocols; default: 3
pub const MAX_PATTERN_RUN_TOP: usize = 2;      // 2 for large protocol, default: 3
pub const MAX_PATTERN_RUN_RARE: usize = 2;     // 2 for large protocol, default: 3
pub const MAX_PATTERN_RUN_MOST: usize = 0;     // 0 for large protocol, default: 3
pub const MAX_PATTERN_RUN_FREQUENT: usize = 2; // 2 for large protocol, default: 3
pub const MAX_PATTERN_LIBRARY: usize = 1;
pub const MAX_PATTERN_NICHE: usize = 3;        // 3 for large protocol, default: 4
pub const MAX_PATTERN_GENERAL: usize = 0;      // 0 for large protocol, default: 4
```

---

## How RUNS Works

### Pattern Discovery Phase (Phase 1)

For each contract, the tool runs **PATTERN_DISCOVERY_RUNS = 5** parallel discovery rounds to find vulnerability patterns.

**Code location**: `src/llm_review/pattern_phases/pattern_to_findings.rs:58`

```rust
for i in 0..PATTERN_DISCOVERY_RUNS {
    // Spawn parallel task to discover patterns
}
```

### Pattern to Findings Phase (Phase 3)

For each discovered pattern, the tool runs **PATTERN_DISCOVERY_RUNS = 5** parallel rounds to generate findings from that pattern.

**Code location**: `src/llm_review/pattern_phases/multipattern_to_findings.rs:57-63`

```rust
let runs = if issue_title == "invariant" {
    INVARIANT_DISCOVERY_RUNS  // 3
} else if issue_title == "actor" {
    ACTOR_DISCOVERY_RUNS      // 5
} else {
    PATTERN_DISCOVERY_RUNS    // 5
};
```

---

## Cost Impact Analysis

### Current Setup (R3 - GPT o1-preview with RUNS=5)

**Assumptions:**
- 2 contracts analyzed (GTELaunchpadV2Pair, Distributor)
- ~10 patterns discovered per contract
- GPT o1-preview cost: ~$15-60 per 1M tokens (input) + $60-240 per 1M tokens (output)

**Estimated API calls:**
- **Phase 1 (Pattern Discovery)**: 2 contracts × 5 runs = **10 calls**
- **Phase 3 (Pattern → Findings)**: 2 contracts × 10 patterns × 5 runs = **100 calls**
- **Total**: ~110 high-cost GPT o1-preview calls

### Proposed Optimization (RUNS=3)

**Estimated API calls:**
- **Phase 1 (Pattern Discovery)**: 2 contracts × 3 runs = **6 calls** (40% reduction)
- **Phase 3 (Pattern → Findings)**: 2 contracts × 10 patterns × 3 runs = **60 calls** (40% reduction)
- **Total**: ~66 high-cost GPT o1-preview calls

**Cost Savings**: ~40% reduction in API calls

### Aggressive Optimization (RUNS=2)

**Estimated API calls:**
- **Phase 1 (Pattern Discovery)**: 2 contracts × 2 runs = **4 calls** (60% reduction)
- **Phase 3 (Pattern → Findings)**: 2 contracts × 10 patterns × 2 runs = **40 calls** (60% reduction)
- **Total**: ~44 high-cost GPT o1-preview calls

**Cost Savings**: ~60% reduction in API calls

---

## Recommendation

### Test Strategy

1. **Baseline (R3)**: RUNS=5 → Found 22 findings (4/4 C4-validated Highs) ✅
2. **Test Run (R5)**: RUNS=3 → See if it still finds all 4 C4-validated Highs
3. **Aggressive Test (R6)**: RUNS=2 → See if it still finds all 4 C4-validated Highs

### Expected Outcome

**RUNS=3** should still find most/all critical findings because:
- Multiple runs provide redundancy for pattern discovery
- 3 runs still gives good coverage
- GPT o1-preview is very capable, so fewer runs may be sufficient

**RUNS=2** is more risky:
- Less redundancy
- May miss some patterns that only appear in certain runs
- But could still work if GPT o1-preview is consistent

---

## How to Change RUNS

Edit `src/config.rs` line 32:

```rust
// Current
pub const PATTERN_DISCOVERY_RUNS: usize = 5;

// Proposed
pub const PATTERN_DISCOVERY_RUNS: usize = 3;  // 40% cost reduction

// Aggressive
pub const PATTERN_DISCOVERY_RUNS: usize = 2;  // 60% cost reduction
```

Then rebuild:
```bash
cargo build --release
```

---

## Success Criteria

After running with RUNS=3, compare against R3 (RUNS=5):

✅ **Must find these C4-validated findings:**
- H-20: Fee double-counting in swap (complexity 7)
- H-21/H-22: Pool-aliasing via wrong quote token (complexity 5)
- H-13: LP shares double-count fees (complexity 6)
- H-18/H-19: Rewards to wrong recipient (complexity 3-4)
- M-9: Flash-liquidity fee bypass (complexity 5)

If RUNS=3 finds all 5 critical findings → **SUCCESS, use RUNS=3 going forward**

If RUNS=3 misses any → **Revert to RUNS=5** or try RUNS=4

