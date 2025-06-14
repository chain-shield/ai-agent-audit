use crate::build_brain::callgraph::DotFunc;
use crate::utils::get_fn_name::get_function_name;

use super::fn_summaries::get_function_summaries;
use super::graph_db::GraphDb;
/// This module handles the enrichment of smart contract data using Slither analysis.
/// It provides functionality to extract intermediate representation (IR) and storage information
/// from Solidity contracts, and to build Forge projects.
use super::slither_ffi::{SlithIRFn, StorageVar};
use super::{callgraph, inheritance};
use anyhow::Result;
use log::{debug, info};
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

pub async fn build_semantics_db_from_call_graph(repo_root: &Path) -> Result<PathBuf> {
    // 1. extract DOT blobs
    let json = callgraph::generate_slither_call_graph(repo_root).await?;
    let blobs = callgraph::extract_dot_blobs(&json)?;
    let (funcs_id, edges) = callgraph::parse_dot_blobs(&blobs)?;
    let funcs = get_function_summaries(repo_root).await?;
    let inheritance_json = inheritance::generate_slither_inheritance(repo_root).await?;
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
        let modifiers = f.modifiers.join(",");
        // info!("freshly spilt modifiers => {:#?}", modifiers);
        // GET ID from funcs_id
        // info!("fn summary => {:#?}", f);
        // info!("funcs_id => {:#?}", funcs_id);

        let callgraph_func = funcs_id
            .clone()
            .into_iter()
            .find(|a| {
                let func_name = get_function_name(&f.name);
                a.contract == f.contract && a.name == func_name
            })
            .unwrap_or_default();

        if !callgraph_func.name.is_empty() {
            db.insert_function(
                &callgraph_func.full_id,
                &f.contract,
                &f.name,
                &f.visibility,
                &modifiers,
                &f.mutability,
            )?;
        }
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
