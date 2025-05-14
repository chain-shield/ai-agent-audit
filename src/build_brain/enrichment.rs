use super::slither_ffi::{dump_ir_and_storage, SlithIRFn, StorageVar};
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct Enriched {
    pub ir: Vec<SlithIRFn>,
    pub storage: Vec<StorageVar>,
}

pub fn enrich_with_slither(repo_root: &Path) -> Result<Enriched> {
    let (ir, storage) = dump_ir_and_storage(repo_root)?;
    Ok(Enriched { ir, storage })
}

pub fn forge_build(repo_root: &Path) -> Result<()> {
    let status = Command::new("forge")
        .current_dir(repo_root)
        .args(["build", "--json"])
        .status()?;
    anyhow::ensure!(status.success(), "`forge build` failed");
    Ok(())
}

// you can then read ./out/*.json artefacts for ABIs, bytecode sizes, etc.
