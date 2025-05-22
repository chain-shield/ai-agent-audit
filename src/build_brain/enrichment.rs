use super::graph_db::GraphDb;
/// This module handles the enrichment of smart contract data using Slither analysis.
/// It provides functionality to extract intermediate representation (IR) and storage information
/// from Solidity contracts, and to build Forge projects.
use super::slither_ffi::{get_ir_and_storage_vars_for_each_function, SlithIRFn, StorageVar};
use super::{callgraph, inheritance};
use anyhow::Result;
use log::info;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Contains the enriched data extracted from Solidity contracts.
/// This includes the intermediate representation (IR) of functions and storage variable information.
pub struct Enriched {
    /// Vector of SlithIR function representations
    pub ir: Vec<SlithIRFn>,
    /// Vector of storage variable information
    pub storage: Vec<StorageVar>,
}

/// Extracts IR and storage information from Solidity contracts using Slither.
///
/// This function uses the Slither static analysis framework to analyze Solidity contracts
/// and extract their intermediate representation and storage information.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing the enriched data
pub async fn enrich_with_slither(repo_root: &Path) -> Result<Enriched> {
    let (ir, storage) = get_ir_and_storage_vars_for_each_function(repo_root).await?;
    Ok(Enriched { ir, storage })
}

/// Builds a Forge project by running `forge install` and `forge build`.
///
/// This function executes the necessary Forge commands to install dependencies
/// and build the Solidity contracts in the repository.
///
/// @param repo_root - Path to the repository root containing the Forge project
/// @return Result indicating success or failure
pub fn forge_build(repo_root: &Path) -> Result<()> {
    // Step 1: Run `forge install` to pull in remappings
    let install_status = Command::new("forge")
        .current_dir(repo_root)
        .arg("install")
        .status()?;
    anyhow::ensure!(install_status.success(), "`forge install` failed");

    // Step 2: Now build
    let build_status = Command::new("forge")
        .current_dir(repo_root)
        .args(["-q", "build", "--build-info"])
        .status()?;
    anyhow::ensure!(build_status.success(), "`forge build` failed");
    Ok(())
}

pub fn build_semantics_db_from_call_graph(repo_root: &Path) -> Result<PathBuf> {
    // 1. extract DOT blobs
    let json = callgraph::generate_slither_call_graph(repo_root)?;
    let blobs = callgraph::extract_dot_blobs(&json)?;
    let (funcs, edges) = callgraph::parse_dot_blobs(&blobs)?;
    let inheritance_json = inheritance::generate_slither_inheritance(repo_root)?;
    let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
    // info!("dot functions => {:?}", funcs);
    // info!("dot edges => {:?}", edges);

    // 2. open DB file
    let db_path = repo_root.join(".cache").join("semantics.db");
    std::fs::create_dir_all(db_path.parent().unwrap())?;
    let db = GraphDb::create(&db_path)?;

    // 3. insert functions
    // info!("funcs => {:?}", funcs);
    for f in &funcs {
        db.insert_function(&f.full_id, &f.contract, &f.name)?;
    }

    // 4. insert edges
    // info!("edges => {:?}", edges);
    for e in &edges {
        db.insert_edge(&e.caller, &e.callee)?;
    }

    // info!("child/parent => {:?}", inheritance_edges);
    for (child, parent) in inheritance_edges {
        db.insert_inheritance(&child, &parent)?;
    }

    Ok(db_path)
}
