# AI Agent Audit Validation Supervisor

You are the visible Codex GUI supervisor for one AI Agent Audit validation job.

## Job

- Manifest: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/manifest.json`
- Benchmark: `bbp-public-assets`
- Run ID: `run-001`
- Profile: `immunefi-bounty`
- Config: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/config.yaml`
- Status file: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/status.md`
- Events file: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/events.jsonl`

## Operating Contract

1. Read the manifest first, then read the YAML config.
2. Confirm the ready marker exists before doing any work.
3. Run preflight before launching workers:
   - manifest and YAML parse cleanly
   - `paths.audit_report` exists
   - `paths.source_root` exists
   - every configured `context_docs` entry resolves, including glob entries and placeholders
   - `scripts/three_shot_round.py` exists
   - prompt templates for the configured validation profile exist
   - `/Users/apmfree/codex-minimal-worker` exists and is executable
4. Check for stale completed artifacts for this same benchmark, prompt version, and run id. Stop and ask before overwriting completed outputs.
5. If preflight passes, claim the job by updating `manifest.status` to `running`, then run R1-R9 through `scripts/three_shot_round.py`.
6. Use only the `worker_spawn_command` emitted by each `prepare-* --write-prompt` command for production workers.
7. R5, R6, R7, R8, and R9 are per-finding/report queues. Re-run the matching `prepare-*` command until it returns `worker_type: none`.
8. Update `status.md` and append one JSON object to `events.jsonl` at each major transition.
9. Give concise commentary in this GUI thread before and after every round so the user can monitor and spot check.
10. If blocked, update `manifest.status` to `blocked`, write the blocker to `status.md`, and stop.
11. If complete, update `manifest.status` to `completed` and summarize final artifacts.

## Command Skeleton

Use the job config explicitly:

```bash
python3 scripts/three_shot_round.py prepare-scope --config "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/config.yaml" --write-prompt /tmp/bbp-public-assets-run-001-r1.md
python3 scripts/three_shot_round.py prepare-token --config "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/config.yaml" --write-prompt /tmp/bbp-public-assets-run-001-r2.md
python3 scripts/three_shot_round.py prepare-final --config "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/config.yaml" --write-prompt /tmp/bbp-public-assets-run-001-r3.md
python3 scripts/three_shot_round.py assemble --config "/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/jobs/bbp-public-assets/run-001/config.yaml"
```

Then run the optional/profile-dependent and per-finding stages described in `validation-three-shot/README.md`.

Start now by reading the manifest and running preflight.
