/// Code block generation orchestration for contract analysis.
///
/// This module coordinates the generation of contextual code blocks for each
/// contract in a repository, managing database connections and processing
/// parameters for optimal AI analysis.
use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use super::codeblocks::generate_codeblock_from_codebase;
use crate::{
    config::{CHAINSHIELD_DB_FOLDER, CODEBLOCK_DB},
    enumerator::codeblock_db::CodeBlocksDb,
    prepare_code::git_clone::RepoPaths,
};

/// Generates and saves contextual code blocks for all contracts in a repository.
///
/// This function orchestrates the code block generation process by connecting
/// to semantic and slice databases, then generating focused code slices for
/// each contract using call graph traversal within specified constraints.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_db` - Path to semantic analysis database
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `PathBuf` - Path to the generated slice database
pub async fn generate_and_save_codeblocks_for_each_contract(
    repo: &RepoPaths,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // Open the three databases
    log::info!("connecting to databases..");
    let semantic_conn = Connection::open(semantics_db)?;
    let codeblock_path =
        Path::new(&format!("{}/{}", CHAINSHIELD_DB_FOLDER, CODEBLOCK_DB)).to_path_buf();
    let slice_db = CodeBlocksDb::open(&codeblock_path)?;

    // Fetch all seeds and process each one
    generate_codeblock_from_codebase(repo, &semantic_conn, &slice_db, max_depth, token_budget)
        .await?;

    Ok(codeblock_path)
}
