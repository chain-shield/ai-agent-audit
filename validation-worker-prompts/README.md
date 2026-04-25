# Validation Worker Prompts

These files hold the controller-generated worker prompts used by
[`scripts/validation_loop.py`](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/scripts/validation_loop.py).

Why they live here:

- easier to inspect and edit without digging through Python
- cleaner separation between workflow logic and prompt text
- safer prompt iteration because the controller only fills placeholders

Available templates:

- [raw-validation.md](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-worker-prompts/raw-validation.md)
- [scoring.md](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-worker-prompts/scoring.md)
- [prompt-analysis.md](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-worker-prompts/prompt-analysis.md)
- [prompt-revision.md](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-worker-prompts/prompt-revision.md)

Template format:

- placeholders use `{{PLACEHOLDER_NAME}}`
- the controller replaces those placeholders at runtime
- if any placeholder is left unresolved, `validation_loop.py` exits with an error

Editing guidance:

- keep the placeholders exactly intact unless you also update the controller
- feel free to rewrite wording, ordering, examples, and guardrails
- these are worker instructions, not the reusable validation checklist prompt versions in `validation-prompts/`
