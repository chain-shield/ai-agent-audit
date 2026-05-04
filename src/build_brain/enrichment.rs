use crate::build_brain::callgraph::DotFunc;
use crate::config::{SEMANTIC_DB, app_db_path};
use crate::enumerator::utils::get_code_ir_map;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::get_function_name_from_interface;

use super::callgraph;
use super::fn_summaries::get_function_summaries;
use super::graph_db::{GraphDb, SmartContractFunction};
/// Smart contract data enrichment using Slither static analysis.
///
/// This module builds semantic databases containing call graphs, inheritance hierarchies,
/// and function metadata extracted from Solidity contracts using Slither analysis.
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Builds a semantic database containing call graphs and inheritance data.
///
/// This function extracts call graph and inheritance information from Slither analysis
/// and stores it in a SQLite database for efficient querying during code analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `PathBuf` - Path to the created semantic database
pub async fn build_semantics_db_from_call_graph(repo: RepoPaths) -> Result<PathBuf> {
    // Patch foundry.toml to remove custom solc paths before running Slither
    // Create database file in cache directory
    let db_path = app_db_path(SEMANTIC_DB);
    let cache_dir = db_path.parent().ok_or_else(|| {
        AuditError::file_system(
            db_path.to_string_lossy().to_string(),
            "Invalid database path - no parent directory",
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"),
        )
    })?;
    std::fs::create_dir_all(cache_dir).map_err(|e| {
        AuditError::file_system(
            cache_dir.to_string_lossy().to_string(),
            "Failed to create cache directory",
            e,
        )
    })?;

    // Delete the semantic database from previous run to ensure fresh data
    if db_path.exists() {
        info!("Deleting old semantic database from previous run");
        std::fs::remove_file(&db_path).map_err(|e| {
            AuditError::file_system(
                db_path.to_string_lossy().to_string(),
                "Failed to delete old semantic database",
                e,
            )
        })?;
    }

    let db = Arc::new(Mutex::new(GraphDb::create(&db_path)?));
    let repo = Arc::new(repo);

    // Process call graph data in parallel task
    let repo_func = Arc::clone(&repo);
    let db_func = Arc::clone(&db);
    let handle = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract call graph data from Slither
            info!("extracting DOT blobs");
            let (funcs_id, edges) = callgraph::get_dot_funcs_and_dot_edges(&repo_func).await?;
            let mut rows = Vec::new();
            let function_to_ir_map = get_code_ir_map(&repo_func).await?;
            let func_index: HashMap<(String, String), DotFunc> = funcs_id
                .into_iter()
                .map(|node| ((node.contract.clone(), node.name.clone()), node))
                .collect();

            // Get function summaries for metadata
            info!("getting function summaries...");
            let funcs = get_function_summaries(&repo_func).await?;

            info!("insert function metadata into database..");
            info!("{} function summaries", funcs.len());
            // Insert function metadata into database
            for f in &funcs {
                let func_name = get_function_name_from_interface(&f.name);
                let slither_ir_fn = function_to_ir_map
                    .get(&(f.contract.clone(), func_name.clone()))
                    .cloned()
                    .unwrap_or_default();
                if let Some(node) = func_index.get(&(f.contract.clone(), func_name)) {
                    // if f.contract == "GovernedBase" || f.contract == "AssetManagerInit" {
                    //     info!("node: {:#?} \n", node);
                    //     info!("IR: {}", slither_ir_fn.ir);
                    // }
                    rows.push(SmartContractFunction {
                        id: node.full_id.clone(),
                        project_id: repo_func.project_id.clone(),
                        contract: f.contract.clone(),
                        name: f.name.clone(),
                        ir: slither_ir_fn.ir,
                        visibility: f.visibility.clone(),
                        modifiers: f.modifiers.clone(),
                        mutability: f.mutability.clone(),
                    })
                }
            }
            // single lock, batch insert
            {
                let db = db_func.lock().await;
                for row in rows {
                    db.insert_function(&row)?;
                }
            }
            info!("done inserting function metadata into database");

            info!("insert {} call graph edges in db", edges.len());
            // Insert call graph edges
            for e in &edges {
                let db_guard = db_func.lock().await;
                db_guard.insert_edge(&repo_func.project_id, &e.caller, &e.callee)?;
            }
            info!("dot edges in db complete");
            Ok(())
        }
        .await;

        if let Err(e) = &result {
            log::error!("Error processing call graph data: {:#}", e);
        }

        result
    });

    // Wait for call graph processing to complete
    info!("waiting for meta data analysis to complete...");
    let func_result = handle.await?;
    func_result?;
    Ok(db_path)
}
