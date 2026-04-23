# Validation Prompts

This folder stores versioned verification prompts.

Rules:

- `v1.md` is the copied baseline checklist prompt.
- Each later prompt should be a complete runnable prompt, not a patch note.
- Changes between versions must remain protocol-agnostic and reusable across EVM Solidity codebases.
- If a change only helps one benchmark because it bakes in protocol-specific knowledge, do not promote it.
- Keep one stable prompt structure. Revise shared sections in place instead of piling version-named guidance blocks into the active prompt.
- Track prompt-to-prompt deltas in [CHANGELOG.md](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-prompts/CHANGELOG.md).

Suggested flow:

- `v1.md`: baseline checklist copied from [VERIFY_CHECKLIST.md](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/VERIFY_CHECKLIST.md)
- `v2.md`, `v3.md`, ...: refined prompts based on scored validation errors

Recommended structure inside each active prompt:

- short task framing
- evidence hierarchy
- one durable `Core Validation Principles` section
- pre-gate sanity checks
- numbered validation gates
- final tie-breaker / output rules
