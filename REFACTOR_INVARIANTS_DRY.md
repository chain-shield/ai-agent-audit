# Refactoring: ContractInvariants to use FromLLMJson Trait

## Summary

Successfully refactored `ContractInvariants` to use the `FromLLMJson` trait, eliminating duplicate `clean_json_string()` implementation and ensuring DRY (Don't Repeat Yourself) principles.

## Changes Made

### 1. Removed Duplicate Code from `invariants.rs`

**File:** `src/llm_review/threat_models/invariants.rs`

**Removed:**
- `ContractInvariants::parse_from_json()` method (lines 287-296)
- `ContractInvariants::clean_json_string()` method (lines 298-324)

**Kept:**
- `ContractInvariants::get_all_violations()` method (still needed for business logic)

**Before:**
```rust
impl ContractInvariants {
    pub fn parse_from_json(json_str: &str) -> Result<ContractInvariants, serde_json::Error> {
        let cleaned_json = Self::clean_json_string(json_str);
        serde_json::from_str(&cleaned_json)
    }

    fn clean_json_string(input: &str) -> String {
        // ... 25 lines of duplicate code ...
    }

    pub fn get_all_violations(self) -> Vec<InvariantFinding> {
        // ... kept ...
    }
}
```

**After:**
```rust
impl ContractInvariants {
    pub fn get_all_violations(self) -> Vec<InvariantFinding> {
        self.invariants
            .into_iter()
            .filter(|inv| inv.status == InvariantStatus::PossibleViolation)
            .collect::<Vec<InvariantFinding>>()
    }
}
```

### 2. How It Works Now

`ContractInvariants` automatically gets the `FromLLMJson` trait implementation because:

1. **Trait Definition** (`src/llm_review/findings/findings.rs:329-338`):
   ```rust
   pub trait FromLLMJson: Sized {
       fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error>;
       fn clean_json_string(input: &str) -> String;
       fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>>;
   }
   ```

2. **Blanket Implementation** (`src/llm_review/findings/findings.rs:340-480`):
   ```rust
   impl<T> FromLLMJson for T
   where
       T: DeserializeOwned,
   {
       fn parse_from_json(json_str: &str) -> Result<Self, serde_json::Error> {
           let cleaned = Self::clean_json_string(json_str);
           // ... parsing logic with error logging ...
       }

       fn clean_json_string(input: &str) -> String {
           // ... IMPROVED implementation with unescaped quote handling ...
       }

       fn parse_from_llm_response(response: &str) -> Result<Self, Box<dyn std::error::Error>> {
           // ... extract JSON from raw LLM response ...
       }
   }
   ```

3. **ContractInvariants Qualifies** (`src/llm_review/threat_models/invariants.rs:71-74`):
   ```rust
   #[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
   pub struct ContractInvariants {
       pub invariants: Vec<InvariantFinding>,
   }
   ```
   
   Since `ContractInvariants` derives `Deserialize`, it implements `DeserializeOwned` and automatically gets the trait.

### 3. Benefits

1. **DRY Compliance**: Single source of truth for JSON cleaning logic
2. **Bug Fixes Propagate**: The recent fix for unescaped quotes (Gemini issue) now automatically applies to `ContractInvariants`
3. **Consistent Behavior**: All LLM JSON parsing uses the same robust cleaning logic
4. **Less Code**: Removed ~40 lines of duplicate code
5. **Better Error Logging**: Inherits the improved error logging from the trait implementation

### 4. Usage Examples

**Direct Parsing:**
```rust
let invariants = ContractInvariants::parse_from_json(json_str)?;
```

**From LLM Response:**
```rust
let invariants = ContractInvariants::parse_from_llm_response(llm_response)?;
```

**Via Agent (most common):**
```rust
let invariants: ContractInvariants = agent.extract_with_retry(&prompt).await?;
```

All three methods now use the same underlying `FromLLMJson` trait implementation.

## Testing

### New Test File: `tests/invariants_json_parsing_test.rs`

Created comprehensive tests to verify `ContractInvariants` works correctly with the trait:

1. ✅ **test_invariants_parse_with_unescaped_quotes** - Handles Gemini's unescaped quotes
2. ✅ **test_invariants_parse_from_llm_response** - Extracts JSON from raw LLM text
3. ✅ **test_invariants_parse_with_markdown_code_blocks** - Handles markdown formatting
4. ✅ **test_invariants_empty_list** - Handles empty results

**Test Results:**
```
running 4 tests
✅ SUCCESS: Parsed empty invariants list
✅ SUCCESS: Parsed 1 invariants from markdown
✅ SUCCESS: Parsed 1 invariants from LLM response
✅ SUCCESS: Parsed 1 invariants
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

### Existing Tests Still Pass

All existing tests continue to pass, confirming backward compatibility:
- `cargo test --test gemini_json_parsing_test` ✅
- `cargo check` ✅

## No Breaking Changes

- All existing code continues to work
- The trait provides the same method signatures as the removed implementation
- No changes needed to call sites (they use `agent.extract_with_retry()` which already uses the trait)

## Files Modified

1. `src/llm_review/threat_models/invariants.rs` - Removed duplicate implementation
2. `tests/invariants_json_parsing_test.rs` - Added comprehensive tests (NEW)

## Files Referenced (No Changes)

1. `src/llm_review/findings/findings.rs` - Contains the `FromLLMJson` trait
2. `src/utils/extract_retry.rs` - Uses `FromLLMJson::parse_from_llm_response()`
3. `src/llm_review/agent/agent_enums.rs` - Provides `extract_with_retry()` method

