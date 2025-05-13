// crates/enrichment/src/lib.rs
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn forge_build(repo_root: &Path) -> Result<()> {
    let status = Command::new("forge")
        .current_dir(repo_root)
        .args(["build", "--json"])
        .status()?;
    anyhow::ensure!(status.success(), "`forge build` failed");
    Ok(())
}

// you can then read ./out/*.json artefacts for ABIs, bytecode sizes, etc.
