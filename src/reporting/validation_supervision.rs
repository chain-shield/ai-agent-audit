use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use chrono::Utc;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{prepare_code::git_clone::RepoPaths, reporting::three_shot_config};

const THREE_SHOT_DIR: &str = "validation-three-shot";
const JOBS_DIR: &str = "jobs";

#[derive(Debug, Deserialize)]
struct ThreeShotConfig {
    benchmark: String,
    validation_profile: String,
    prompt_version: String,
    run_id: String,
    paths: ThreeShotPaths,
    #[serde(default)]
    context_docs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ThreeShotPaths {
    source_root: String,
    audit_root: String,
    audit_report: String,
}

#[derive(Debug, Serialize)]
struct ValidationJobManifest {
    schema_version: u8,
    mode: String,
    status: String,
    benchmark: String,
    run_id: String,
    prompt_version: String,
    validation_profile: String,
    repo_name: String,
    repo_root: String,
    config_path: String,
    source_config_path: String,
    job_dir: String,
    source_root: String,
    audit_root: String,
    audit_report: String,
    context_docs: Vec<String>,
    expected_outputs: BTreeMap<String, String>,
    supervisor_prompt: String,
    status_file: String,
    events_file: String,
    ready_marker: String,
    poll_interval_minutes: u8,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct ExistingJobManifest {
    #[serde(default)]
    status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationJobPaths {
    pub job_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub config_snapshot_path: PathBuf,
    pub supervisor_prompt_path: PathBuf,
    pub status_path: PathBuf,
    pub events_path: PathBuf,
    pub ready_marker_path: PathBuf,
}

pub fn write_gui_job(
    repo: &RepoPaths,
    config_path: &Path,
    overwrite_existing: bool,
) -> Result<ValidationJobPaths> {
    let config_path = absolute_path(config_path)?;
    let config_text = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    let config: ThreeShotConfig = serde_yaml::from_str(&config_text)
        .with_context(|| format!("Failed to parse {}", config_path.display()))?;

    let repo_root = std::env::current_dir().context("Failed to resolve current project root")?;
    let job_dir = job_dir_for(&config.benchmark, &config.run_id);
    let manifest_path = job_dir.join("manifest.json");
    let config_snapshot_path = job_dir.join("config.yaml");
    let supervisor_prompt_path = job_dir.join("supervisor.md");
    let status_path = job_dir.join("status.md");
    let events_path = job_dir.join("events.jsonl");
    let ready_marker_path = job_dir.join("ready");

    refuse_existing_non_terminal_job(&manifest_path, overwrite_existing)?;

    fs::create_dir_all(&job_dir)
        .with_context(|| format!("Failed to create {}", job_dir.display()))?;
    if ready_marker_path.exists() {
        fs::remove_file(&ready_marker_path)
            .with_context(|| format!("Failed to remove {}", ready_marker_path.display()))?;
    }

    let now = Utc::now().to_rfc3339();
    let expected_outputs = expected_outputs(
        &config.prompt_version,
        &config.benchmark,
        &config.run_id,
        &repo_root,
    );
    let manifest = ValidationJobManifest {
        schema_version: 1,
        mode: "gui-supervised".to_string(),
        status: "pending".to_string(),
        benchmark: config.benchmark.clone(),
        run_id: config.run_id.clone(),
        prompt_version: config.prompt_version.clone(),
        validation_profile: config.validation_profile.clone(),
        repo_name: repo.repo_name.clone(),
        repo_root: repo_root.to_string_lossy().to_string(),
        config_path: absolute_path(&config_snapshot_path)?
            .to_string_lossy()
            .to_string(),
        source_config_path: config_path.to_string_lossy().to_string(),
        job_dir: absolute_path(&job_dir)?.to_string_lossy().to_string(),
        source_root: config.paths.source_root.clone(),
        audit_root: config.paths.audit_root.clone(),
        audit_report: config.paths.audit_report.clone(),
        context_docs: config.context_docs.clone(),
        expected_outputs,
        supervisor_prompt: absolute_path(&supervisor_prompt_path)?
            .to_string_lossy()
            .to_string(),
        status_file: absolute_path(&status_path)?.to_string_lossy().to_string(),
        events_file: absolute_path(&events_path)?.to_string_lossy().to_string(),
        ready_marker: absolute_path(&ready_marker_path)?
            .to_string_lossy()
            .to_string(),
        poll_interval_minutes: 5,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    fs::write(&config_snapshot_path, config_text)
        .with_context(|| format!("Failed to write {}", config_snapshot_path.display()))?;
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).context("Failed to serialize manifest")? + "\n",
    )
    .with_context(|| format!("Failed to write {}", manifest_path.display()))?;
    fs::write(
        &supervisor_prompt_path,
        render_supervisor_prompt(&manifest, &manifest_path),
    )
    .with_context(|| format!("Failed to write {}", supervisor_prompt_path.display()))?;
    fs::write(&status_path, render_initial_status(&manifest))
        .with_context(|| format!("Failed to write {}", status_path.display()))?;
    fs::write(&events_path, render_initial_event(&manifest)?)
        .with_context(|| format!("Failed to write {}", events_path.display()))?;
    fs::write(&ready_marker_path, format!("ready_at={now}\n"))
        .with_context(|| format!("Failed to write {}", ready_marker_path.display()))?;

    info!(
        "Wrote Codex GUI validation job: benchmark={}, run_id={}, job_dir={}, supervisor_prompt={}",
        manifest.benchmark,
        manifest.run_id,
        job_dir.display(),
        supervisor_prompt_path.display()
    );

    Ok(ValidationJobPaths {
        job_dir,
        manifest_path,
        config_snapshot_path,
        supervisor_prompt_path,
        status_path,
        events_path,
        ready_marker_path,
    })
}

pub fn refuse_existing_non_terminal_job_for_repo(
    repo: &RepoPaths,
    run_id: &str,
    overwrite_existing: bool,
) -> Result<()> {
    let benchmark = three_shot_config::config_slug(&repo.repo_name);
    let manifest_path = job_dir_for(&benchmark, run_id).join("manifest.json");
    refuse_existing_non_terminal_job(&manifest_path, overwrite_existing)
}

fn refuse_existing_non_terminal_job(manifest_path: &Path, overwrite_existing: bool) -> Result<()> {
    if !manifest_path.exists() {
        return Ok(());
    }

    let manifest_text = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read existing {}", manifest_path.display()))?;
    let existing: ExistingJobManifest = serde_json::from_str(&manifest_text)
        .with_context(|| format!("Failed to parse existing {}", manifest_path.display()))?;
    let status = existing.status.as_deref().unwrap_or("missing");

    if is_terminal_job_status(status) {
        return Ok(());
    }

    if overwrite_existing {
        warn!(
            "Overwriting active Codex GUI validation job at {} because validation_supervision_overwrite is enabled and manifest status is '{}'",
            manifest_path.display(),
            status
        );
        return Ok(());
    }

    bail!(
        "Refusing to overwrite active Codex GUI validation job at {} because manifest status is '{}'. Delete the job folder, change run_id, or set validation_supervision_overwrite: true after confirming the current validation job can be replaced.",
        manifest_path.display(),
        status
    );
}

fn is_terminal_job_status(status: &str) -> bool {
    matches!(
        status.trim().to_ascii_lowercase().as_str(),
        "completed" | "complete" | "blocked" | "cancelled" | "canceled" | "failed"
    )
}

fn job_dir_for(benchmark: &str, run_id: &str) -> PathBuf {
    PathBuf::from(THREE_SHOT_DIR)
        .join(JOBS_DIR)
        .join(path_component(benchmark))
        .join(path_component(run_id))
}

fn expected_outputs(
    prompt_version: &str,
    benchmark: &str,
    run_id: &str,
    repo_root: &Path,
) -> BTreeMap<String, String> {
    let run_name = format!("{benchmark}-{run_id}");
    let root = repo_root.join(THREE_SHOT_DIR);
    BTreeMap::from([
        (
            "r1_scope_screen".to_string(),
            root.join("scope-screens")
                .join(prompt_version)
                .join(format!("{run_name}.md"))
                .to_string_lossy()
                .to_string(),
        ),
        (
            "r2_token_screen".to_string(),
            root.join("token-screens")
                .join(prompt_version)
                .join(format!("{run_name}.md"))
                .to_string_lossy()
                .to_string(),
        ),
        (
            "r3_final_validation".to_string(),
            root.join("stage3-runs")
                .join(prompt_version)
                .join(format!("{run_name}.md"))
                .to_string_lossy()
                .to_string(),
        ),
        (
            "assembled_run".to_string(),
            root.join("runs")
                .join(prompt_version)
                .join(format!("{run_name}.md"))
                .to_string_lossy()
                .to_string(),
        ),
        (
            "submission_candidates".to_string(),
            root.join("submission-candidates")
                .join(prompt_version)
                .join(format!("{run_name}.md"))
                .to_string_lossy()
                .to_string(),
        ),
        (
            "results_markdown".to_string(),
            root.join("results")
                .join(prompt_version)
                .join(format!("{run_name}.md"))
                .to_string_lossy()
                .to_string(),
        ),
        (
            "results_json".to_string(),
            root.join("results")
                .join(prompt_version)
                .join(format!("{run_name}.json"))
                .to_string_lossy()
                .to_string(),
        ),
    ])
}

fn render_supervisor_prompt(manifest: &ValidationJobManifest, manifest_path: &Path) -> String {
    format!(
        r#"# AI Agent Audit Validation Supervisor

You are the visible Codex GUI supervisor for one AI Agent Audit validation job.

## Job

- Manifest: `{manifest_path}`
- Benchmark: `{benchmark}`
- Run ID: `{run_id}`
- Profile: `{profile}`
- Config: `{config_path}`
- Status file: `{status_file}`
- Events file: `{events_file}`

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
python3 scripts/three_shot_round.py prepare-scope --config "{config_path}" --write-prompt /tmp/{benchmark}-{run_id}-r1.md
python3 scripts/three_shot_round.py prepare-token --config "{config_path}" --write-prompt /tmp/{benchmark}-{run_id}-r2.md
python3 scripts/three_shot_round.py prepare-final --config "{config_path}" --write-prompt /tmp/{benchmark}-{run_id}-r3.md
python3 scripts/three_shot_round.py assemble --config "{config_path}"
```

Then run the optional/profile-dependent and per-finding stages described in `validation-three-shot/README.md`.

Start now by reading the manifest and running preflight.
"#,
        manifest_path = absolute_display(manifest_path),
        benchmark = manifest.benchmark,
        run_id = manifest.run_id,
        profile = manifest.validation_profile,
        config_path = manifest.config_path,
        status_file = manifest.status_file,
        events_file = manifest.events_file,
    )
}

fn render_initial_status(manifest: &ValidationJobManifest) -> String {
    format!(
        r#"# {benchmark} {run_id} Validation Status

Status: Pending
Created: {created_at}
Mode: GUI-supervised
Poll interval: 5 minutes

## Files

- Manifest: `{job_dir}/manifest.json`
- Config: `{config_path}`
- Audit report: `{audit_report}`
- Source root: `{source_root}`

## Preflight

Not started.

## Rounds

| Round | Status | Artifact |
| --- | --- | --- |
| R1 | Pending | `{r1}` |
| R2 | Pending | `{r2}` |
| R3 | Pending | `{r3}` |
| Assemble | Pending | `{assembled}` |
| R4+ | Pending | `{candidates}` |
| Results | Pending | `{results}` |
"#,
        benchmark = manifest.benchmark,
        run_id = manifest.run_id,
        created_at = manifest.created_at,
        job_dir = manifest.job_dir,
        config_path = manifest.config_path,
        audit_report = manifest.audit_report,
        source_root = manifest.source_root,
        r1 = manifest
            .expected_outputs
            .get("r1_scope_screen")
            .map(String::as_str)
            .unwrap_or("-"),
        r2 = manifest
            .expected_outputs
            .get("r2_token_screen")
            .map(String::as_str)
            .unwrap_or("-"),
        r3 = manifest
            .expected_outputs
            .get("r3_final_validation")
            .map(String::as_str)
            .unwrap_or("-"),
        assembled = manifest
            .expected_outputs
            .get("assembled_run")
            .map(String::as_str)
            .unwrap_or("-"),
        candidates = manifest
            .expected_outputs
            .get("submission_candidates")
            .map(String::as_str)
            .unwrap_or("-"),
        results = manifest
            .expected_outputs
            .get("results_markdown")
            .map(String::as_str)
            .unwrap_or("-"),
    )
}

fn render_initial_event(manifest: &ValidationJobManifest) -> Result<String> {
    Ok(serde_json::to_string(&json!({
        "ts": manifest.created_at,
        "event": "job_created",
        "benchmark": manifest.benchmark,
        "run_id": manifest.run_id,
        "status": manifest.status,
    }))
    .context("Failed to serialize initial validation event")?
        + "\n")
}

fn absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn absolute_display(path: &Path) -> String {
    absolute_path(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .to_string()
}

fn path_component(value: &str) -> String {
    let component = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    if component.is_empty() {
        "job".to_string()
    } else {
        component
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_component_strips_path_separators() {
        assert_eq!(
            path_component("legion/protocol contracts"),
            "legion-protocol-contracts"
        );
        assert_eq!(path_component("../"), "job");
    }

    #[test]
    fn existing_non_terminal_job_is_not_overwritten() {
        let temp = tempfile::tempdir().unwrap();
        let manifest = temp.path().join("manifest.json");
        fs::write(&manifest, r#"{ "status": "running" }"#).unwrap();

        let err = refuse_existing_non_terminal_job(&manifest, false).unwrap_err();

        assert!(
            err.to_string()
                .contains("Refusing to overwrite active Codex GUI validation job")
        );
    }

    #[test]
    fn existing_terminal_job_can_be_replaced() {
        let temp = tempfile::tempdir().unwrap();
        let manifest = temp.path().join("manifest.json");
        fs::write(&manifest, r#"{ "status": "completed" }"#).unwrap();

        refuse_existing_non_terminal_job(&manifest, false).unwrap();
    }

    #[test]
    fn existing_manifest_without_status_is_not_overwritten() {
        let temp = tempfile::tempdir().unwrap();
        let manifest = temp.path().join("manifest.json");
        fs::write(&manifest, "{}").unwrap();

        let err = refuse_existing_non_terminal_job(&manifest, false).unwrap_err();

        assert!(err.to_string().contains("status is 'missing'"));
    }

    #[test]
    fn existing_non_terminal_job_can_be_overwritten_when_explicitly_enabled() {
        let temp = tempfile::tempdir().unwrap();
        let manifest = temp.path().join("manifest.json");
        fs::write(&manifest, r#"{ "status": "pending" }"#).unwrap();

        refuse_existing_non_terminal_job(&manifest, true).unwrap();
    }
}
