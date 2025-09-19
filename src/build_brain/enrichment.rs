use crate::build_brain::callgraph::DotFunc;
use crate::config::{CHAINSHIELD_DB_FOLDER, SEMANTIC_DB};
use crate::enumerator::utils::get_code_ir_map;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::get_function_name_from_interface;

use super::fn_summaries::get_function_summaries;
use super::graph_db::GraphDb;
/// Smart contract data enrichment using Slither static analysis.
///
/// This module builds semantic databases containing call graphs, inheritance hierarchies,
/// and function metadata extracted from Solidity contracts using Slither analysis.
use super::slither_ffi::{self, SlithIRFn, StorageVar};
use super::{callgraph, inheritance};
use log::info;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
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
    // Create database file in cache directory
    let db_path = Path::new(&format!("{}/{}", CHAINSHIELD_DB_FOLDER, SEMANTIC_DB)).to_path_buf();
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
    let db = Arc::new(Mutex::new(GraphDb::create(&db_path)?));
    let repo = Arc::new(repo);

    // Process call graph data in parallel task
    let repo_func = Arc::clone(&repo);
    let db_func = Arc::clone(&db);
    let handle = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract call graph data from Slither
            info!("extracting DOT blobs");
            let json = slither_ffi::run_printer_json(&repo_func, "call-graph").await?;
            let blobs = callgraph::extract_dot_blobs(&json)?;
            let mut rows = Vec::new();
            let function_to_ir_map = get_code_ir_map(&repo_func).await?;
            let (funcs_id, edges) = callgraph::parse_dot_blobs(&blobs, &repo_func)?;
            let func_index: HashMap<(String, String), DotFunc> = funcs_id
                .into_iter()
                .map(|node| {
                    // if node.contract == "GovernedBase" || node.contract == "AssetManagerInit" {
                    //     info!("node: {:#?} \n", node);
                    // }
                    ((node.contract.clone(), node.name.clone()), node)
                })
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
                    rows.push((
                        node.full_id.clone(),
                        &repo_func.project_id,
                        f.contract.clone(),
                        f.name.clone(),
                        slither_ir_fn.ir,
                        f.visibility.clone(),
                        f.modifiers.join(","),
                        f.mutability.clone(),
                    ))
                }
            }
            // single lock, batch insert
            {
                let db = db_func.lock().await;
                for row in rows {
                    db.insert_function(
                        &row.0, &row.1, &row.2, &row.3, &row.4, &row.5, &row.6, &row.7,
                    )?;
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

        if let Err(e) = result {
            log::error!("Error processing call graph data: {:#}", e);
        }

        Ok::<_, AuditError>(())
    });

    // Process inheritance data in parallel task
    let db_inheritance = Arc::clone(&db);
    let repo_inheritance = Arc::clone(&repo);
    let handle_inheritance = tokio::spawn(async move {
        let result: Result<()> = async move {
            // Extract inheritance hierarchy from Slither
            info!("generating inheritance json");
            let inheritance_json =
                slither_ffi::run_printer_json(&repo_inheritance, "inheritance").await?;
            let inheritance_edges = inheritance::parse_inheritance_json(&inheritance_json)?;
            info!("{} inheritance edges", inheritance_edges.len());

            // Insert inheritance relationships into database
            for (child, parent) in inheritance_edges {
                let db_guard = db_inheritance.lock().await;
                db_guard.insert_inheritance(&repo_inheritance.project_id, &child, &parent)?;
            }
            info!("done generating inheritance edges");

            Ok(())
        }
        .await;

        if let Err(e) = result {
            log::error!("Error processing inheritance data: {:#}", e);
        }
        Ok::<_, AuditError>(())
    });

    // Wait for both parallel tasks to complete
    info!("waiting for meta data analysis to complete...");
    let (_func_result, _inheritance_result) = tokio::try_join!(handle, handle_inheritance)?;
    Ok(db_path)
}
