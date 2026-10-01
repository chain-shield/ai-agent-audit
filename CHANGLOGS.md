# Change Logs

## 2026-10-01

- Clear Cargo Clippy warnings with simpler conditionals, optional-value logging, smaller enum storage, and one address-regex construction per metadata parse.
- Add regression coverage for verification JSON formatting and multi-implementation metadata extraction.
- Upgrade the primary OpenAI audit model to GPT-6.1 Sol with high reasoning; retain GPT-6 Luna medium for deduplication.
- Register GPT-6.1 Sol standard token estimates at $2 input and $10 output per million tokens, avoiding unknown-model warnings.
- Extend configuration, model-rate, and API reasoning tests for GPT-6.1 Sol.

## 2026-09-29

- Estimate GPT-6 Sol and Luna token costs at current standard API rates, and label Codex dollar totals as estimates.
- Find the Codex CLI bundled with ChatGPT.app by default on macOS, while retaining the legacy Codex.app fallback.
- Set the primary OpenAI audit model to GPT-6 Sol with high reasoning.
- Set the OpenAI deduplication model to GPT-6 Luna with medium reasoning.
- Preserve reasoning parameters for GPT-6 models when using the direct OpenAI API backend.
- Update configuration tests for the GPT-6 model defaults.
