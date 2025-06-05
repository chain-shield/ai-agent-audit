use super::seed_db::{Seed, SeedDb};
use anyhow::{anyhow, Result};
use chrono::Utc;
use serde_sarif::sarif::Sarif;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use uuid::Uuid;

pub fn run_slither_sarif(repo: &Path) -> Result<String> {
    let out = Command::new("slither")
        .current_dir(repo)
        .args([
            ".",
            "--sarif",
            "-",
            "--foundry-ignore-compile",
            "--foundry-out-directory",
            "out",
        ])
        .output()?;
    // anyhow::ensure!(out.status.success(), "slither --sarif failed");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    if stdout.trim().is_empty() {
        // If we get nothing in stdout, it's likely a real failure
        anyhow::bail!(
            "slither --sarif failed:\nstdout: {}\nstderr: {}",
            stdout,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn sarif_to_seeds(sarif_json: &str) -> Result<Vec<Seed>> {
    let sarif: Sarif = serde_json::from_str(sarif_json)?;
    let run = sarif.runs.get(0).ok_or_else(|| anyhow!("no runs"))?;
    let results = match &run.results {
        Some(results) => results,
        None => return Err(anyhow!("could not deserialize slither sarif")),
    };
    let mut out = Vec::new();

    for result in results {
        let (file, start, end) = if let Some(loc) = result
            .locations
            .as_ref()
            .expect("location field not found in sarif")
            .get(0)
        {
            let phys = loc
                .physical_location
                .as_ref()
                .cloned()
                .and_then(|p| p.region)
                .unwrap_or_default();
            let uri = loc
                .physical_location
                .as_ref()
                .cloned()
                .and_then(|p| p.artifact_location)
                .and_then(|artifact| artifact.uri)
                .unwrap_or_default();
            (
                uri,
                phys.start_line.unwrap_or(0) as u64,
                phys.end_line.unwrap_or(0) as u64,
            )
        } else {
            (String::new(), 0, 0)
        };
        // 3. markdown or text
        let message = result.message.clone().markdown.unwrap_or_default();
        let severity_level = match &result.level {
            Some(serde_json::Value::String(severity_level)) => severity_level.to_string(),
            Some(_) | None => "no severity provided".to_string(),
        };

        out.push(Seed {
            id: Uuid::new_v4().to_string(),
            detector: result.rule_id.clone().unwrap_or_default(),
            file,
            start_line: start,
            end_line: end,
            severity: severity_level,
            message: message.clone(),
            created_at: Utc::now().timestamp_millis(),
        });
    }
    // log::info!("slither sarif => {:#?}", out);
    Ok(out)
}

/// Public façade: run Slither, parse SARIF, write seeds.db, return path
pub fn slither_scan_and_store_to_db(repo_root: &Path) -> Result<PathBuf> {
    let json = run_slither_sarif(repo_root)?;
    let seeds = sarif_to_seeds(&json)?;

    let db_path = repo_root.join(".cache").join("seeds.db");
    std::fs::create_dir_all(db_path.parent().unwrap())?;
    let db = SeedDb::open(&db_path)?;

    for s in &seeds {
        db.insert(s)?;
    }
    println!(
        "🟢  Phase-2: {} seeds stored in {}",
        seeds.len(),
        db_path.display()
    );
    Ok(db_path)
}
