/// This module provides functionality for generating and saving code slices from
/// Slither analysis seeds. It serves as the main entry point for the code slicing process.
use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::static_scanning::seed_db::SeedDb;

use super::{
    codeblocks::generate_codeblock_from_codebase, path_enum::generate_codeblock_from_slither_seed,
    slice_db::CodeBlocksDb,
};

/// Generates and saves code slices for all seeds in the seeds database.
///
/// This function:
/// 1. Opens the necessary databases (semantics, seeds, and slices)
/// 2. Retrieves all seeds from the seeds database
/// 3. For each seed, generates a code slice using the path_enum module
/// 4. Returns the path to the slice database
///
/// # Arguments
/// * `repo_root` - Path to the repository root
/// * `semantics_db` - Path to the semantics database containing function and call graph information
/// * `max_depth` - Maximum depth for BFS traversal of the call graph
/// * `token_budget` - Maximum token count for each generated codeblock
///
/// # Returns
/// * `Result<PathBuf>` - Path to the slice database if successful, Error otherwise
pub async fn generate_and_save_code_slices_from_slither_seeds(
    repo_root: &Path,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // Open the three databases
    let semantic_conn = Connection::open(semantics_db)?;
    let seed_db = SeedDb::open(&repo_root.join(".cache").join("seeds.db"))?;
    let slice_path = repo_root.join(".cache").join("slice.db");
    let slice_db = CodeBlocksDb::open(&slice_path)?;

    // Fetch all seeds and process each one
    let seeds = seed_db.all_seeds()?;
    for seed in seeds.into_iter() {
        generate_codeblock_from_slither_seed(
            repo_root,
            &seed,
            &semantic_conn,
            &slice_db,
            max_depth,
            token_budget,
        )
        .await?;
    }

    Ok(slice_path)
}

pub async fn generate_and_save_codeblocks_for_each_contract(
    repo_root: &Path,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // Open the three databases
    let semantic_conn = Connection::open(semantics_db)?;
    let slice_path = repo_root.join(".cache").join("slice.db");
    let slice_db = CodeBlocksDb::open(&slice_path)?;

    // Fetch all seeds and process each one
    generate_codeblock_from_codebase(
        repo_root,
        &semantic_conn,
        &slice_db,
        max_depth,
        token_budget,
    )
    .await?;

    Ok(slice_path)
}
