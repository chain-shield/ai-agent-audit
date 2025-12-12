# Validation Round Tests

This document describes the test suite for the validation round feature, which validates findings that were downgraded during verification rounds.

## Overview

The validation round feature is the 4th phase of the audit pipeline:
1. **Discovery** - Find potential vulnerabilities
2. **Deduplication** - Remove duplicate findings
3. **Verification Rounds** (3 rounds) - Validate findings and downgrade invalid ones
4. **Validation Round** - Re-validate downgraded findings to catch false negatives

## Test Files

### Unit Tests: `validation_round_unit_test.rs`

Tests the core validation logic without requiring API calls.

#### Test Coverage

| Test | Description | Purpose |
|------|-------------|---------|
| `test_validate_all_reasons_confirmed_invalid` | All downgrade reasons confirmed (true) | Ensures findings stay invalid when all reasons are valid |
| `test_validate_all_reasons_rejected_upgrade_to_valid` | All downgrade reasons rejected (false) | Ensures findings are upgraded to Valid when all reasons are invalid |
| `test_validate_partial_confirmation` | Some reasons confirmed, some rejected | Tests partial validation - only confirmed reasons remain |
| `test_validate_low_severity_reasons` | Low severity downgrade reasons | Tests LowSeverityDueToLowImpact and LowSeverityDueToRareLikelihood |
| `test_validate_finding_already_valid` | Finding already marked as Valid | Ensures Valid findings remain Valid |
| `test_validate_finding_no_status` | Finding with no status | Ensures None status is handled correctly |
| `test_validate_all_invalid_status_types` | All 9 invalid status types | Comprehensive test of all InvalidXXX statuses |
| `test_generate_validation_prompt_single_finding` | Prompt generation for 1 finding | Tests dynamic prompt generation |
| `test_generate_validation_prompt_multiple_findings` | Prompt generation for multiple findings | Tests prompt with multiple findings |
| `test_generate_dynamic_validation_json_single_finding` | JSON schema for 1 finding | Tests dynamic JSON schema generation |
| `test_generate_dynamic_validation_json_multiple_findings` | JSON schema for multiple findings | Tests JSON with different status types |

**Run unit tests:**
```bash
cargo test --test validation_round_unit_test
```

**Expected output:**
```
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Integration Tests: `integration_validation_round.rs`

Tests the complete validation workflow with real AI agent calls using Puppy Raffle findings.

#### Test Scenarios

The integration test creates 4 downgraded findings:

1. **H-1: Reentrancy** - Incorrectly marked as `InvalidBugDoesNotExist`
   - **Expected:** Should be upgraded to Valid (bug clearly exists)
   
2. **M-1: Centralization Risk** - Correctly marked as `InvalidOutOfScope`
   - **Expected:** Should remain invalid (centralization is out of scope)
   
3. **H-2: Integer Overflow** - Incorrectly marked as `InvalidGovernanceRisk`
   - **Expected:** Should be upgraded to Valid (not a governance risk)
   
4. **M-2: Weak Randomness** - Marked as both `InvalidBugDoesNotExist` and `LowSeverityDueToLowImpact`
   - **Expected:** Partial validation - bug exists and impact is medium

#### Running Integration Tests

**Prerequisites:**
- `ANTHROPIC_API_KEY` environment variable must be set
- `.env` file with valid API key

**Run integration test:**
```bash
cargo test --test integration_validation_round -- --nocapture
```

**Expected output:**
```
🧪 Starting Validation Round Integration Test with Puppy Raffle

📋 Created 4 downgraded findings for validation

📊 Downgraded Findings Summary:
  1. [High] Reentrancy vulnerability in refund function - Status: [InvalidBugDoesNotExist]
  2. [Medium] Centralization risk in owner functions - Status: [InvalidOutOfScope]
  3. [High] Integer overflow in fee calculation - Status: [InvalidGovernanceRisk]
  4. [Medium] Weak randomness in winner selection - Status: [InvalidBugDoesNotExist, LowSeverityDueToLowImpact]

✅ Agent initialized

🔍 Starting Validation Round...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Validating Verification Rounds
Validation Results: 2 upgraded to Valid, 1 confirmed invalid

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 VALIDATION RESULTS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✅ UPGRADED: Reentrancy vulnerability in refund function
     Original: Some([InvalidBugDoesNotExist])
     New: Valid

  ❌ CONFIRMED INVALID: Centralization risk in owner functions
     Status: Some([InvalidOutOfScope])

  ✅ UPGRADED: Integer overflow in fee calculation
     Original: Some([InvalidGovernanceRisk])
     New: Valid

  ➡️  PARTIAL: Weak randomness in winner selection
     Status: Some([LowSeverityDueToLowImpact])

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📈 SUMMARY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total findings validated: 4
  ✅ Upgraded to Valid: 2
  ❌ Confirmed Invalid: 1
  ➡️  Unchanged: 1
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ Validation Round Integration Test Passed!
```

## Test Data

### Puppy Raffle Contract

The tests use a simplified version of the Puppy Raffle contract with known vulnerabilities:

- **Reentrancy** in `refund()` - External call before state update
- **Integer Overflow** in `selectWinner()` - uint64 overflow in fee calculation
- **Weak Randomness** in `selectWinner()` - Uses block.timestamp and block.difficulty
- **Centralization** in `changeFeeAddress()` - Owner can change fee address

### Finding Status Types Tested

| Status | Description | Test Coverage |
|--------|-------------|---------------|
| `InvalidBugDoesNotExist` | Bug doesn't actually exist | ✅ Unit + Integration |
| `InvalidOutOfScope` | Finding is out of audit scope | ✅ Unit + Integration |
| `InvalidUserErrorOrMistake` | Results from user error | ✅ Unit |
| `InvalidGovernanceRisk` | Admin/governance risk | ✅ Unit + Integration |
| `InvalidERC20EdgeCase` | Requires non-standard token | ✅ Unit |
| `InvalidNotExploitable` | Not actually exploitable | ✅ Unit |
| `InvalidFutureSpeculation` | Only in future state | ✅ Unit |
| `InvalidByDesign` | Intentional design choice | ✅ Unit |
| `InvalidSafeGuardInPlace` | Safeguard prevents exploit | ✅ Unit |
| `LowSeverityDueToLowImpact` | Low impact | ✅ Unit + Integration |
| `LowSeverityDueToRareLikelihood` | Rare likelihood | ✅ Unit |

## Validation Logic

### How Validation Works

1. **Input:** Findings with downgrade statuses from verification rounds
2. **Filter:** Only process findings that are NOT already Valid
3. **Prompt Generation:** Create dynamic prompts based on specific downgrade reasons
4. **AI Analysis:** LLM validates each downgrade reason (true/false)
5. **Status Update:**
   - All reasons rejected (false) → Upgrade to Valid
   - All reasons confirmed (true) → Stay invalid
   - Partial → Keep only confirmed reasons
6. **Statistics:** Track upgraded vs confirmed invalid counts

### Example Validation Flow

```
Finding: "Reentrancy in refund"
Original Status: [InvalidBugDoesNotExist]

Validation Prompt:
  "Does the bug really not exist? Analyze the code..."

AI Response:
  {
    "does_bug_really_not_exist": false,
    "justification": "Bug clearly exists - external call before state update"
  }

Result: Upgraded to Valid ✅
```

## Running All Tests

```bash
# Run all validation tests
cargo test validation_round

# Run only unit tests
cargo test --test validation_round_unit_test

# Run only integration tests (requires API key)
cargo test --test integration_validation_round

# Run with output
cargo test validation_round -- --nocapture
```

## CI/CD Considerations

- **Unit tests:** Always run (no API key required)
- **Integration tests:** Skip if `ANTHROPIC_API_KEY` not present
- **Cost:** Integration test makes ~4 API calls (~$0.01 per run)

## Future Enhancements

- [ ] Add tests for edge cases (empty findings, all Valid, etc.)
- [ ] Test validation with different LLM providers (OpenAI, Gemini)
- [ ] Add performance benchmarks
- [ ] Test concurrent validation of multiple findings
- [ ] Add tests for validation statistics accuracy

