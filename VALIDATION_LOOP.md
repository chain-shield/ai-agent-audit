# Validation Loop

This repository uses a versioned prompt-iteration loop to improve finding verification recall while minimizing false validations.

---

## Goal

The validator should:

- catch as many truly valid findings as possible;

- reject as many truly invalid findings as possible;

- improve through prompt revisions that stay protocol-agnostic and reusable across EVM Solidity codebases.

---

## Scoring Terms

For benchmark scoring, the **primary KPI layer is High/Medium only**.

- Include only findings whose report IDs begin with `H-` or `M-`.
- Exclude `L-` / `QA` findings from the primary confusion matrix unless a benchmark explicitly targets low-severity validation.
- Treat `Valid` as the positive class and `Invalid` as the negative class within that H/M subset.

### Core Outcomes

| Term | Meaning |
| --- | --- |
| `TP` | `True Positive`: a canonically valid finding that the validator marks `Valid` |
| `FP` | `False Positive`: a canonically invalid finding that the validator marks `Valid` |
| `TN` | `True Negative`: a canonically invalid finding that the validator marks `Invalid` |
| `FN` | `False Negative`: a canonically valid finding that the validator marks `Invalid` |

### Derived Metrics

| Metric | Formula | Question It Answers |
| --- | --- | --- |
| `Recall` | `TP / (TP + FN)` | Of the real bugs, how many did the validator keep? |
| `Precision` | `TP / (TP + FP)` | Of the findings the validator kept, how many were actually real? |
| `Specificity` | `TN / (TN + FP)` | Of the findings that should be rejected, how many did the validator correctly reject? |

> If a run uses a third label such as `Needs Review`, score it separately as an abstention bucket instead of silently collapsing it into `Valid` or `Invalid`.

### Unique Root-Cause KPIs

In addition to row-level H/M scoring, track unique canonical approved root-cause coverage.

| KPI | Meaning |
| --- | --- |
| `Total canonical approved H/M findings` | Total number of approved Code4rena High/Medium root causes in the canonical truth set |
| `Unique approved H/M findings present in audit-report.md` | Number of canonical approved root causes that appear anywhere in the app-generated report |
| `Unique approved H/M findings accepted as Valid` | Number of those canonical approved root causes that the validator actually keeps as `Valid` |
| `Present-root recall` | `unique accepted / unique present`; measures whether the validator kept the real approved bugs that were actually present in the report |
| `End-to-end unique recall` | `unique accepted / total canonical approved`; measures overall benchmark coverage versus the full approved C4 set |

These unique-root KPIs matter because duplicate findings can inflate row-level recall and precision while hiding whether the validator actually preserved the distinct real bugs.

---

## Iteration Loop

1. Start from the latest approved prompt in [validation-prompts](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-prompts).

2. Run the prompt against one benchmark at a time, evaluating each finding one-by-one instead of batching the entire report into one monolithic judgment.

3. Write the raw decisions into [validation-runs](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-runs).

4. Score the run against hidden ground truth stored outside the repository in the orchestrator-managed hidden truth store.

5. Write benchmark metrics, confusion matrix, gate-level miss analysis, and promotion decision into [validation-results](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-results).

6. Draft the next prompt version only from recurring error patterns.

7. Keep every prompt revision protocol-agnostic:

   - no protocol names;

   - no benchmark-specific contracts;

   - no contest-specific facts baked into the reusable logic.

8. Before promoting a new prompt version, regression-test it on older benchmarks so recall gains on one protocol do not come from overfitting.

9. Revise the stable prompt structure in place:

   - keep one durable `Core Validation Principles` section;

   - fold benchmark-learned lessons into the relevant pre-gate or gate whenever possible;

   - add a new gate only when the error pattern is genuinely distinct and recurring across benchmarks;

   - record version-to-version deltas in `validation-prompts/CHANGELOG.md` instead of piling version-specific principle sections into the live prompt.

---

## Scripted Controller

The automation should use [validation_loop.py](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/scripts/validation_loop.py) as the deterministic controller for queue state and worker prompts.

Do not re-derive the queue manually from prose when the script can answer it.

Recommended commands:

```bash
python3 scripts/validation_loop.py status --prompt-version v2 --benchmark 2026-01-olas --run-id run-001
```

```bash
python3 scripts/validation_loop.py next --prompt-version v2 --benchmark 2026-01-olas --run-id run-001 --write-prompt /tmp/validation-worker.md
```

The `next` command:

- initializes the raw run file if needed;

- identifies the next unfinished finding;

- emits `worker_type` as `raw-finding`, `scoring`, `prompt-revision`, or `none`;

- writes the exact worker prompt to the requested `--write-prompt` path;

- prevents the orchestrator from guessing line ranges, file paths, or the next phase.

The orchestrator still owns worker spawning:

1. run `next`;

2. read the generated worker prompt;

3. spawn the requested fresh worker with `fork_context=false`;

4. wait for completion;

5. close the worker;

6. run `next` again.

---

## Context Isolation Rule

To reduce anchoring and truth leakage, do **not** let one long-lived context handle the full loop end-to-end from memory.

Use isolated contexts for each phase:

1. `Raw Validation Context`

   - fresh context;

   - sees the benchmark report, the benchmark source root, relevant Solidity code, relevant benchmark docs, and the current validation prompt;

   - does **not** read the hidden truth store;

   - is **not told** the hidden truth store path or that such a store exists;

   - validates exactly one finding per worker invocation;

   - appends that one finding's result into the raw decision file before any scoring happens.

2. `Scoring Context`

   - fresh context;

   - sees the raw run, the hidden truth files exposed by the orchestrator, and the benchmark source root / benchmark docs when needed to interpret raw evidence precisely;

   - computes the primary H/M-only `TP`, `FP`, `TN`, `FN`, recall, precision, and specificity;

   - computes unique canonical H/M root-cause coverage KPIs;

   - performs false-negative and false-positive analysis.

3. `Prompt Revision Context`

   - fresh context;

   - sees the previous prompt, the scored results, the raw validation run, the benchmark report, the benchmark source root, relevant Solidity code, relevant benchmark docs, and the canonical truth artifacts such as approved findings;

   - proposes the next prompt version using only protocol-agnostic rules.

The orchestrator can stay in the same thread, but the actual validator, scorer, and prompt-revision work should be performed in fresh contexts each iteration.

### Worker Spawn Rule

When fresh worker contexts are available, the orchestrator should:

1. spawn a dedicated worker for each phase;

2. set `fork_context=false` so the worker does **not** inherit the orchestrator's accumulated thread memory;

3. pass only the minimum files and instructions required for that phase.

Recommended pattern:

- one fresh worker per finding during raw validation;

- one fresh worker for scoring;

- one fresh worker for prompt revision.

Do not reuse the same worker across phases within the same iteration.

Do not let one raw-validation worker adjudicate multiple findings from the same benchmark run.

### Worker Lifecycle Rule

Once a worker has:

- finished its assigned phase;

- written its required artifacts to disk; and

- returned the result back to the orchestrator;

the orchestrator should close that worker context immediately.

Do not keep completed workers open across iterations, since that increases context growth and raises the risk of hitting chat or token limits.

This prevents:

- anchoring on prior decisions;

- contamination from canonical truth during the raw validation pass;

- prompt revisions that memorize one benchmark instead of learning reusable rules.

### Per-Finding Raw Validation Mode

Raw validation should be decomposed into **one finding per fresh worker**.

The orchestrator should:

1. initialize the raw run file early if it does not exist yet;

2. determine the next unfinished finding by comparing the benchmark report against the findings already present in the raw run file;

3. spawn a fresh worker with `fork_context=false` for exactly that one finding;

4. have the worker append exactly one new finding block to the raw run file;

5. close the worker immediately after the append succeeds;

6. continue to the next unfinished finding only by spawning another fresh worker.

Critical safety rule:

- do not run two raw-validation workers concurrently against the same raw run file;

- serialize raw-finding workers so they do not race while appending to the same markdown artifact.

---

## Naming Convention

Use these names consistently:

### Prompt Version

`prompt-version` means the exact prompt file stem in [validation-prompts](/Users/apmfree/Desktop/CHAIN%20SHIELD/ai-agent-audit/validation-prompts).

Examples:

- `v1`

- `v2`

- `v17`

### Benchmark

`benchmark` means the exact top-level benchmark folder slug in the repository.

Examples:

- `2026-01-olas`

- `2025-12-panoptic`

- `2026-03-intuition`

### Benchmark Source Root

When a benchmark source repository exists under:

- `/Users/apmfree/Desktop/Audit/<benchmark>`

treat that directory as the benchmark's authoritative source root.

Example:

- benchmark: `2026-01-olas`

- benchmark source root: `/Users/apmfree/Desktop/Audit/2026-01-olas`

### Run ID

`run-id` means the sequential run number for a specific `(prompt-version, benchmark)` pair.

Format:

- `run-001`

- `run-002`

- `run-003`

The counter resets when either the prompt version or benchmark changes.

To determine the next run id:

- look inside `validation-runs/<prompt-version>/`;

- find all files matching `<benchmark>-run-XXX.*`;

- take the highest existing run number for that `(prompt-version, benchmark)` pair;

- use the next sequential value.

If none exist yet, start at `run-001`.

### File Paths

For a raw run:

- `validation-runs/<prompt-version>/<benchmark>-<run-id>.md`

- `validation-runs/<prompt-version>/<benchmark>-<run-id>.jsonl`

Example:

- `validation-runs/v1/2026-01-olas-run-001.md`

- `validation-runs/v1/2026-01-olas-run-001.jsonl`

For scored results:

- `validation-results/<prompt-version>/<benchmark>-<run-id>.md`

Example:

- `validation-results/v1/2026-01-olas-run-001.md`

For truth files:

- hidden truth store: `<hidden-truth-root>/<benchmark>.md`

Example:

- hidden truth file for `2026-01-olas`

For prompt evolution:

- current prompt: `validation-prompts/vN.md`

- next prompt: `validation-prompts/v{N+1}.md`

Create `validation-runs/<prompt-version>/` and `validation-results/<prompt-version>/` if those folders do not already exist.

---

## Benchmark Evidence Bundle

For each benchmark, workers should validate against an evidence bundle, not just the finding text.

Default evidence bundle:

1. the benchmark report in this repository;

2. the benchmark source root at `/Users/apmfree/Desktop/Audit/<benchmark>`, if it exists;

3. relevant Solidity source files under that source root;

4. relevant benchmark context documents under that source root, especially top-level files such as:

   - `README.md`

   - `<protocol>-docs.md`

   - `<protocol>-scope.md`

   - module `README.md` files

   - other benchmark-specific markdown files that clarify scope, invariants, intended support, or deployment assumptions

Critical rule:

- the benchmark report is a claim set, not the authoritative source of truth for code behavior;

- gates about invariants, supported integrations, trust boundaries, safeguards, scope, and expected runtime behavior should be checked against the actual Solidity code and benchmark documents whenever available.

- raw-validation workers should prioritize canonical sponsor / repository documents and avoid relying on local post-hoc analysis files, validator outputs, benchmark answer keys, or user-generated markdown that appears to summarize likely findings.

For Olas, workers should use:

- benchmark source root: `/Users/apmfree/Desktop/Audit/2026-01-olas`

- important starting docs:

   - `/Users/apmfree/Desktop/Audit/2026-01-olas/README.md`

   - `/Users/apmfree/Desktop/Audit/2026-01-olas/olas-docs.md`

   - `/Users/apmfree/Desktop/Audit/2026-01-olas/olas-scope.md`

---

## Iteration Semantics

The automation should run in **scheduled bounded chained mode**.

The scheduler should wake the orchestrator **12 times per night**, starting at `9:00 PM ET` and then repeating hourly through `8:00 AM ET`.

Each automation wakeup should start with the **oldest unfinished unit of work**.

### Controller Loop Requirement

Within a single automation wakeup, the orchestrator must run a tight controller loop:

1. inspect disk and choose the oldest unfinished unit;

2. spawn exactly the worker required for that unit;

3. wait for that worker to finish or hit the guardrail;

4. close the worker immediately after completion;

5. inspect disk again;

6. if another unit is now unlocked, spawn the next worker immediately.

Do **not** voluntarily stop after a successful raw-finding append while the same raw run still has unfinished findings.

Do **not** wait for the next scheduler tick when the next dependent worker is already known.

The wakeup should stop only when one of these is true:

- no unfinished work remains;

- the current worker is blocked or failed;

- the 45-minute no-progress guardrail is reached;

- the raw run completes and all immediately unlocked scoring / prompt-revision work for that chain also completes.

A unit of work may be:

- one serial raw-validation append for the next unfinished finding in one `(prompt-version, benchmark, run-id)`;

- one scoring pass for one `(prompt-version, benchmark, run-id)`;

- one prompt-revision pass that drafts a candidate prompt;

- one aggregate comparison pass for a candidate prompt.

If the current unit finishes cleanly and directly unlocks the next dependent phase, the orchestrator should continue immediately in the same wakeup with a **fresh worker** rather than waiting for the next scheduler tick.

Typical immediate handoffs are:

- next raw finding append -> next raw finding append for the same `(prompt-version, benchmark, run-id)` while unfinished findings remain and progress continues;

- final raw finding append -> scoring for the same `(prompt-version, benchmark, run-id)`;

- scoring -> prompt revision, but only when the scored evidence clearly justifies drafting a candidate prompt and no such draft exists yet;

- candidate raw validation -> candidate scoring for the same `(prompt-version, benchmark, run-id)`.

Do not keep chaining phases when the next step is ambiguous, when no dependent phase is unlocked, or when no unfinished work remains.

If no unfinished work remains, the wakeup should exit without inventing new work.

### Resume Rule

If the process is resumed later, the orchestrator should inspect disk first and continue the oldest unfinished unit of work.

Resume order:

1. if a raw run exists and benchmark findings are still missing from that raw run, continue the next unfinished raw finding append;

2. if a raw run exists, is complete, but the scored result does not, finish scoring that run;

3. if the scored result exists and explicitly calls for a next prompt draft that has not been written, draft that next prompt;

4. if the full iteration is complete, start the next run using the appropriate next `(prompt-version, benchmark, run-id)` combination.

This keeps the loop deterministic even if the thread is paused and resumed later.

### Heartbeat Guardrail

For heartbeat-driven runs, allow up to **45 minutes** between progress signals.

If a raw-validation pass is working through a large benchmark, prefer creating the target artifact early and updating it incrementally rather than waiting until the entire benchmark is complete before writing anything to disk.

If `45` minutes pass without a new artifact, meaningful artifact update, spawned worker, completed worker, or other clear progress signal, stop the run, report the blocker in the thread, and close any active worker instead of letting a stale context linger.

During a long raw-validation phase, a completed per-finding append counts as a valid progress signal.

---

## Candidate Evaluation Cycle

Drafting `v{N+1}.md` does **not** mean the new prompt is promoted.

Once a prompt revision worker drafts `v{N+1}.md`, treat it as a **candidate prompt** until it is benchmarked.

The orchestrator must then re-run the validation process for that candidate prompt across the full available truth benchmark set.

For each available truth benchmark:

1. spawn a fresh raw-validation worker with `fork_context=false`;

2. run the full raw validation pass for that benchmark using the candidate prompt;

   - use the benchmark source root and benchmark docs as part of the evidence bundle whenever available;

   - decompose that raw pass into one-finding raw workers, serialized against the candidate raw run file;

3. close the raw-validation worker once artifacts are written;

4. spawn a fresh scoring worker with `fork_context=false`;

5. score that candidate run against the benchmark's truth file;

6. close the scoring worker once artifacts are written.

Do this separately for **each** benchmark in the hidden truth store.

The orchestrator is responsible for scheduling and tracking this candidate-evaluation campaign.

If there are many truth benchmarks, the orchestrator may spread this campaign across multiple scheduled wakeups.

### Candidate Completion Rule

Do **not** compare or promote a candidate prompt until it has completed scored runs on every currently available truth benchmark.

Once the candidate has full benchmark coverage, the orchestrator should write an aggregate comparison summary for the candidate prompt, for example:

- `validation-results/v{N+1}/summary.md`

That summary should compare the candidate prompt against the current approved prompt across the full benchmark set.

---

## Promotion Rule

Promote `vN` to `v{N+1}` only after the orchestrator has completed the full candidate evaluation cycle and the candidate prompt improves the overall validator in a useful way.

Promotion should be based on the aggregate benchmark set, not a single benchmark in isolation.

The candidate must satisfy both primary H/M requirements:

- same or higher H/M recall than the current approved prompt;

- same or higher H/M precision than the current approved prompt.

Precision is a first-class promotion metric. Do not promote a candidate that improves recall by accepting many more invalid findings.

Secondary promotion support can include:

- materially lower false negatives without increasing false positives;

- clearer reasoning structure that reduces repeatable gate mistakes across multiple benchmarks;

- higher unique present-root recall on H/M findings without lowering precision or specificity.

Do not promote a version just because it performs better on one benchmark if it regresses badly on prior benchmarks.

If the candidate prompt does not beat the current approved prompt on the aggregate benchmark set, keep the current approved prompt and treat the candidate as rejected or not yet ready.

---

## Convergence Rule

The target convergence floor is **at least `50%` H/M precision** while preserving same-or-higher H/M recall.

Because recall can be artificially high when the validator over-accepts findings, precision is the key convergence metric once recall is near `100%`.

Treat the prompt as approaching convergence only when all of these are true:

- aggregate H/M recall is the same or higher than the current approved prompt;

- aggregate H/M precision is at least `50%`;

- no meaningful precision gain across `3` consecutive prompt versions;

- no meaningful specificity gain across the same window;

- no meaningful unique present-root recall gain across the same window.

At that point, shift effort from prompt rewriting to benchmark expansion and error taxonomy cleanup.

---

## Recommended Output Shape Per Finding

Each validation run should record, at minimum:

- `Finding ID`

- `Decision`

- `Confidence`

- `Bug Exists`

- `Severity Assessment`

- `Root Cause Family`

- `Checklist Gates Passed`

- `Checklist Gates Failed`

- `Reason`

- `Code Evidence`

This structure makes later error analysis and prompt revision much easier.

The validator should move toward:

- higher recall on truly valid H/M findings;

- at least `50%` precision on H/M findings;

- controlled false-positive rate on H/M findings;

- higher unique present-root recall on H/M findings;

- better end-to-end unique approved-root coverage across benchmarks;

- fewer over-aggressive invalidations from governance, speculation, and invariant gates;

- clearer, more auditable reasoning from run to run.
