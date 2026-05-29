use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::OnceLock,
};

use chrono::Utc;
use serde::Serialize;

use crate::{
    config::{
        ACTOR_RUNS, DISCOVERY_PROVIDER, INVARIANT_RUNS, MAX_PATTERN_GENERAL, MAX_PATTERN_LIBRARY,
        MAX_PATTERN_NICHE, MAX_PATTERN_RELEVANT_FREQUENT, MAX_PATTERN_RUN_FREQUENT,
        MAX_PATTERN_RUN_MOST, MAX_PATTERN_RUN_RARE, MAX_PATTERN_RUN_TOP, OPENAI_DEDUP_MODEL,
        OPENAI_DEDUP_REASONING_EFFORT, OPENAI_MODEL, OPENAI_REASONING_EFFORT, R1_RUNS, R2_RUNS,
        SKIP_ACTOR_PATTERN_RUNS, SKIP_INVARIANT_RUNS, SKIP_LIBRARIES, app_data_dir,
    },
    prepare_code::git_clone::RepoPaths,
};

const ENABLE_ENV: &str = "AI_AGENT_AUDIT_BENCHMARK_TELEMETRY";
const DIR_ENV: &str = "AI_AGENT_AUDIT_BENCHMARK_DIR";
const RUN_ID_ENV: &str = "AI_AGENT_AUDIT_BENCHMARK_RUN_ID";

static RUN_ID: OnceLock<String> = OnceLock::new();

#[derive(Serialize)]
struct TelemetryEnvelope<'a, T: Serialize + ?Sized> {
    timestamp: String,
    run_id: &'a str,
    project_id: &'a str,
    repo_name: &'a str,
    audit_type: String,
    artifact: &'a str,
    payload: &'a T,
}

pub fn enabled() -> bool {
    env::var(ENABLE_ENV)
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

pub fn run_id() -> &'static str {
    RUN_ID
        .get_or_init(|| {
            env::var(RUN_ID_ENV).unwrap_or_else(|_| Utc::now().format("%Y%m%dT%H%M%SZ").to_string())
        })
        .as_str()
}

pub fn run_dir(repo: &RepoPaths) -> PathBuf {
    let root = env::var(DIR_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|_| app_data_dir().join("benchmark-runs"));

    root.join(sanitize_path_segment(&repo.project_id))
        .join(run_id())
}

pub fn append_jsonl<T: Serialize + ?Sized>(
    repo: &RepoPaths,
    file_name: &str,
    artifact: &str,
    payload: &T,
) {
    if !enabled() {
        return;
    }

    let dir = run_dir(repo);
    if let Err(err) = fs::create_dir_all(&dir) {
        log::warn!(
            "Failed to create benchmark telemetry dir {}: {}",
            dir.display(),
            err
        );
        return;
    }

    let envelope = TelemetryEnvelope {
        timestamp: Utc::now().to_rfc3339(),
        run_id: run_id(),
        project_id: &repo.project_id,
        repo_name: &repo.repo_name,
        audit_type: repo.audit_type.to_string(),
        artifact,
        payload,
    };

    let line = match serde_json::to_string(&envelope) {
        Ok(line) => line,
        Err(err) => {
            log::warn!("Failed to serialize benchmark telemetry record: {}", err);
            return;
        }
    };

    let path = dir.join(file_name);
    let mut file = match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(file) => file,
        Err(err) => {
            log::warn!(
                "Failed to open benchmark telemetry file {}: {}",
                path.display(),
                err
            );
            return;
        }
    };

    if let Err(err) = writeln!(file, "{line}") {
        log::warn!(
            "Failed to write benchmark telemetry file {}: {}",
            path.display(),
            err
        );
    }
}

pub fn write_run_manifest(repo: &RepoPaths) {
    if !enabled() {
        return;
    }

    let payload = serde_json::json!({
        "run_id": run_id(),
        "project_id": repo.project_id.clone(),
        "repo_name": repo.repo_name.clone(),
        "github_url": repo.github_url.clone(),
        "commit_hash": repo.commit_hash.clone(),
        "audit_type": repo.audit_type.to_string(),
        "root": repo.root.to_string_lossy(),
        "source_code_folders": repo.source_code_folders.iter().map(|path| path.to_string_lossy().to_string()).collect::<Vec<_>>(),
        "docs": repo.docs.iter().map(|path| path.to_string_lossy().to_string()).collect::<Vec<_>>(),
        "audit_scope": repo.audit_scope.as_ref().map(|path| path.to_string_lossy().to_string()),
        "scoped_files": repo.scoped_files.as_ref().map(|path| path.to_string_lossy().to_string()),
        "excluded_folders": repo.excluded_folders.as_ref().map(|paths| paths.iter().map(|path| path.to_string_lossy().to_string()).collect::<Vec<_>>()),
        "config_snapshot": {
            "r1_runs": R1_RUNS,
            "r2_runs": R2_RUNS,
            "invariant_runs": INVARIANT_RUNS,
            "actor_runs": ACTOR_RUNS,
            "skip_libraries": SKIP_LIBRARIES,
            "skip_invariant_runs": SKIP_INVARIANT_RUNS,
            "skip_actor_pattern_runs": SKIP_ACTOR_PATTERN_RUNS,
            "discovery_provider": DISCOVERY_PROVIDER,
            "openai_model": OPENAI_MODEL,
            "openai_reasoning_effort": OPENAI_REASONING_EFFORT,
            "openai_dedup_model": OPENAI_DEDUP_MODEL,
            "openai_dedup_reasoning_effort": OPENAI_DEDUP_REASONING_EFFORT,
            "max_pattern_run_top": MAX_PATTERN_RUN_TOP,
            "max_pattern_run_rare": MAX_PATTERN_RUN_RARE,
            "max_pattern_run_most": MAX_PATTERN_RUN_MOST,
            "max_pattern_run_frequent": MAX_PATTERN_RUN_FREQUENT,
            "max_pattern_relevant_frequent": MAX_PATTERN_RELEVANT_FREQUENT,
            "max_pattern_library": MAX_PATTERN_LIBRARY,
            "max_pattern_niche": MAX_PATTERN_NICHE,
            "max_pattern_general": MAX_PATTERN_GENERAL,
        },
        "telemetry_dir": run_dir(repo).to_string_lossy(),
    });

    let dir = run_dir(repo);
    if let Err(err) = fs::create_dir_all(&dir) {
        log::warn!(
            "Failed to create benchmark telemetry dir {}: {}",
            dir.display(),
            err
        );
        return;
    }

    let path = dir.join("run_manifest.json");
    let manifest = match serde_json::to_string_pretty(&payload) {
        Ok(manifest) => manifest,
        Err(err) => {
            log::warn!("Failed to serialize benchmark run manifest: {}", err);
            return;
        }
    };

    if let Err(err) = fs::write(&path, format!("{manifest}\n")) {
        log::warn!(
            "Failed to write benchmark run manifest {}: {}",
            path.display(),
            err
        );
    }
}

fn sanitize_path_segment(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>();

    let sanitized = sanitized.trim_matches('-');
    if sanitized.is_empty() {
        "benchmark-run".to_string()
    } else {
        sanitized.to_string()
    }
}
