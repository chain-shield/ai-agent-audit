use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::enumerator::codeblock_db::CodeBlocksDb;

use super::codeblocks::generate_codeblock_from_codebase;

pub async fn generate_and_save_codeblocks_for_each_contract(
    repo_root: &Path,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    // Open the three databases
    log::info!("connecting to databases..");
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
