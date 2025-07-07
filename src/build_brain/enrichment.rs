use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::get_function_name;

use super::fn_summaries::get_function_summaries;
use super::graph_db::GraphDb;
/// This module handles the enrichment of smart contract data using Slither analysis.
/// It provides functionality to extract intermediate representation (IR) and storage information
/// from Solidity contracts, and to build Forge projects.
use super::slither_ffi::{self, SlithIRFn, StorageVar};
use super::{callgraph, inheritance};
use anyhow::Result;
use log::info;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

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

pub async fn build_semantics_db_from_call_graph(repo: RepoPaths) -> Result<PathBuf> {
    // 1. open DB file
    let db_path = repo.root.join(".cache").join("semantics.db");
    std::fs::create_dir_all(db_path.parent().unwrap())?;
    let db = Arc::new(Mutex::new(GraphDb::create(&db_path)?));
    let repo = Arc::new(repo);

    let repo_func = Arc::clone(&repo);
    let db_func = Arc::clone(&db);
    let handle = tokio::spawn(async move {
        let result: Result<()> = async move {
            // 2. extract DOT blobs
            info!("extracting DOT blobs");
            let json = slither_ffi::run_printer_json(&repo_func, "call-graph").await?;
            let blobs = callgraph::extract_dot_blobs(&json)?;
            let (funcs_id, edges) = callgraph::parse_dot_blobs(&blobs)?;
            info!("getting function summaries...");
            let funcs = get_function_summaries(&repo_func).await?;

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
                    let db_guard = db_func.lock().await;
                    db_guard.insert_function(
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
                let db_guard = db_func.lock().await;
                db_guard.insert_edge(&e.caller, &e.callee)?;
            }
            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing user: {:#}", e);
        }

        Ok::<_, anyhow::Error>(())
    });

    let db_inheritance = Arc::clone(&db);
    let repo_inheritance = Arc::clone(&repo);
    let handle_inheritance = tokio::spawn(async move {
        let result: Result<()> = async move {
            info!("generating inheritance json");
            let inheritance_json =
                slither_ffi::run_printer_json(&repo_inheritance, "inheritance").await?;
            let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
            // info!("dot functions => {:?}", funcs);
            // info!("dot edges => {:?}", edges);

            // info!("child/parent => {:?}", inheritance_edges);
            for (child, parent) in inheritance_edges {
                let db_guard = db_inheritance.lock().await;
                db_guard.insert_inheritance(&child, &parent)?;
            }

            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing user: {:#}", e);
        }
        Ok::<_, anyhow::Error>(())
    });
    // Wait for both tasks to complete
    let (_func_result, _inheritance_result) = tokio::try_join!(handle, handle_inheritance)?;
    Ok(db_path)
}
