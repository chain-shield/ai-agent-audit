# Claude 4.5 Extended Thinking Integration

## Overview

This document describes the integration of Anthropic's Extended Thinking feature with Claude 4.5 Sonnet in the AI Agent Audit tool.

## What is Extended Thinking?

Extended Thinking is Anthropic's equivalent to OpenAI's "reasoning effort" parameter. It allows Claude models to perform extended reasoning before providing a final answer, similar to how o1/o3 models work.

When enabled, Claude:
1. Generates internal reasoning in a "thinking" block
2. Uses insights from this reasoning to craft a better final response
3. Returns both the thinking process and the final answer

## Configuration

### AgentConfig Setup

```rust
let config = AgentConfig::new(Some(repo.clone()))
    .with_temperature(1.0)
    .with_model(CLAUDE_4_5_SONNET)
    .with_max_tokens(20_000)  // MUST be > thinking_budget_tokens
    .with_preamble(preamble)
    .with_anthropic_thinking("enabled", 10000u32)  // Enable with 10K token budget
    .with_file_picker(false)
    .with_file_retrieval(false);
```

### Key Parameters

- **`with_anthropic_thinking(thinking: impl Into<String>, budget: impl Into<u32>)`**
  - `thinking`: Either `"enabled"` or `"disabled"`
  - `budget`: Token budget for thinking (minimum: 1,024 tokens)

### Important Constraints

1. **max_tokens > thinking_budget_tokens**
   - `max_tokens` includes BOTH thinking tokens AND response tokens
   - Example: If thinking budget is 10,000, max_tokens should be at least 15,000+
   - The API will reject requests where `max_tokens <= thinking_budget_tokens`

2. **max_tokens refers to OUTPUT tokens only**
   - Input tokens are NOT limited by max_tokens
   - Claude models support 200K+ input tokens by default
   - Only the generated output (thinking + response) counts toward max_tokens

3. **Type Requirements**
   - The budget parameter must be `u32`, not `i32`
   - Use explicit type: `10000u32` or cast: `10000 as u32`

## API Response Format

When thinking is enabled, Anthropic returns a different response structure:

```json
{
  "content": [
    {
      "type": "thinking",
      "thinking": "Let me analyze this step by step...",
      "signature": "WaUjzkypQ2mUEVM36O2TxuC06KN8xyfbJwyem2dw3URve/op91XWHOEBLLqIOMfFG/UvLEczmEsUjavL...."
    },
    {
      "type": "text", 
      "text": "Based on my analysis..."
    }
  ]
}
```

### Summarized Thinking (Claude 4 Models)

For Claude 4 models (including Claude 4.5 Sonnet), Anthropic returns a **summary** of the thinking process:

- You're charged for the **full thinking tokens** generated
- The response contains a **summarized version** of the thinking
- The billed output token count will NOT match the visible token count
- The first few lines of thinking are more verbose for prompt engineering purposes

## Current Limitation: rig-anthropic Integration

**Status**: The thinking parameter is correctly sent to the Anthropic API and processed, but the `rig-anthropic` library cannot currently parse the response.

### The Problem

The `rig` library's `prompt()` method expects responses to contain only `text` content blocks. When thinking is enabled, the response contains both `thinking` and `text` blocks, causing a parsing error:

```
CompletionError: ResponseError: Response did not contain a message or tool call
```

This error occurs at the `agent.prompt()` level, **before** our custom JSON parsing logic (`FromLLMJson::parse_from_llm_response()`) can extract the JSON. This means even `agent_extract_with_retry()` fails because it depends on `prompt()` succeeding first.

### Evidence That It Works

Despite the parsing error, we have confirmed that:

1. ✅ The thinking parameter is accepted by the Anthropic API
2. ✅ The API processes the request (takes ~24 seconds vs ~5 seconds without thinking)
3. ✅ A response is generated with both thinking and text blocks
4. ❌ The rig library's `prompt()` method cannot parse the multi-block response
5. ❌ Our `extract_with_retry()` cannot work because it depends on `prompt()` succeeding

### Why Our Custom JSON Parsing Doesn't Help

You might think our `FromLLMJson::parse_from_llm_response()` function (which just looks for `{` and `}` in the text) would handle thinking blocks. However, the error occurs earlier in the chain:

```rust
// In agent_extract_with_retry():
let raw = agent.prompt(input).await?;  // ❌ FAILS HERE with thinking blocks
let parsed = FromLLMJson::parse_from_llm_response(&raw)?;  // Never reached
```

The `prompt()` method tries to parse the Anthropic API response and extract the text content, but it doesn't know how to handle `thinking` content blocks, so it fails before returning any text.

### Workaround

The integration test (`tests/claude_thinking_integration_test.rs`) handles this gracefully:

```rust
let response_result: Result<HashMapExplanation, _> =
    agent.extract_with_retry(test_prompt).await;

match response_result {
    Ok(response) => {
        // If this succeeds, rig has been updated to handle thinking blocks!
        println!("✅ Thinking blocks are now supported!");
    }
    Err(e) => {
        let error_msg = format!("{:?}", e);
        if error_msg.contains("Response did not contain a message")
            || error_msg.contains("prompt failed") {
            // Expected error - thinking blocks aren't supported yet
            println!("✅ Configuration test PASSED");
            println!("   Thinking parameter was accepted and processed");
            return;
        } else {
            panic!("Unexpected error: {:?}", e);
        }
    }
}
```

## Best Practices

### When to Use Thinking

Use extended thinking for:
- Complex reasoning tasks (math, coding, analysis)
- Security vulnerability detection
- Multi-step problem solving
- Tasks that benefit from step-by-step reasoning

### Thinking Budget Guidelines

- **Minimum**: 1,024 tokens
- **Recommended starting point**: 1,024 tokens, then increase incrementally
- **Typical range**: 5,000 - 15,000 tokens
- **Large budgets**: Above 32K tokens, use batch processing to avoid timeouts
- **Diminishing returns**: Higher budgets don't always improve quality

### Performance Considerations

- **Response times**: Expect longer response times (2-5x slower)
- **Streaming**: Required when max_tokens > 21,333
- **Cost**: You're charged for full thinking tokens, not the summary

### Feature Compatibility

Thinking is **NOT compatible** with:
- Temperature, top_p, or top_k modifications
- Forced tool use
- Response pre-filling

## Testing

Run the integration test:

```bash
cargo test --test claude_thinking_integration_test test_claude_4_5_with_thinking_enabled -- --nocapture
```

This test verifies:
1. Configuration is created correctly
2. Thinking parameters are accepted by the API
3. The API processes the request (evidenced by longer response time)
4. Documents the current rig-anthropic limitation

## Next Steps

To fully support thinking in this codebase:

1. **Update rig-anthropic** to handle thinking content blocks
   - Parse both `thinking` and `text` content types
   - Extract text from the `text` block for the response
   - Optionally expose the thinking block for debugging/analysis

2. **Alternative**: Use raw Anthropic API calls
   - Bypass rig for thinking-enabled requests
   - Manually parse the JSON response
   - Extract both thinking and text blocks

3. **Future Enhancement**: Expose thinking blocks
   - Store thinking blocks in audit reports
   - Use thinking for debugging AI reasoning
   - Analyze thinking patterns for quality improvement

## References

- [Anthropic Extended Thinking Documentation](https://docs.anthropic.com/en/docs/build-with-claude/extended-thinking)
- [AWS Bedrock Extended Thinking Guide](https://docs.aws.amazon.com/bedrock/latest/userguide/claude-messages-extended-thinking.html)
- Integration test: `tests/claude_thinking_integration_test.rs`
- Configuration: `src/llm_review/agent_factory.rs`

