# Validation Truth

This folder is only a placeholder describing the hidden truth system.

The actual benchmark truth files should be stored **outside the repository** in a hidden home-directory location managed by the orchestrator.

That hidden truth location should be exposed only to the workers that need it, such as scoring workers and prompt-revision workers.

Raw validation workers should not be told that the hidden truth store exists.

Guidelines:

- Ground truth should stay separate from prompts and raw runs.
- The raw validator should not see the hidden truth store during the validation pass.
- Store truth at the root-cause level, not just exact title matching.
- Keep a short explanation for every canonical valid finding and why it is considered the same root cause.

Example contents:

```text
<hidden-truth-root>/
  2026-01-olas.md
  2025-11-megapot.md
  2025-12-panoptic.md
```
