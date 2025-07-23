use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::utils::prompt_context::generate_context_for_code_review,
    prepare_code::git_clone::RepoPaths, reporting::save_file::save_file_locally,
};
/// Contract data export utilities for analysis artifacts.
///
/// This module provides functions to export contract analysis data including
/// generated code blocks and metadata to markdown files for external use
/// and documentation purposes.
use std::path::{Path, PathBuf};

/// Saves individual contract analysis data to markdown files.
///
/// Exports generated code blocks for each contract to separate markdown files
/// with standardized naming conventions for easy reference and documentation.
///
/// # Arguments
// * `codeblocks_path` - Path to the code blocks database
/// * `repo` - Repository paths and metadata for naming
pub fn save_contract_and_fn_ir(codeblocks_path: &PathBuf, repo: &RepoPaths) -> anyhow::Result<()> {
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    log::info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    for (contract, codeblock) in contracts {
        let filename = format!("{}-{}.md", contract, repo.unique_repo_hash());
        save_file_locally(&codeblock, &filename)?;
    }
    Ok(())
}

/// Saves protocol metadata and context information to a markdown file.
///
/// Exports comprehensive protocol metadata including summaries, semantic data,
/// and contextual information used by AI agents during analysis.
///
/// # Arguments
/// * `semantics_path` - Path to the semantic analysis database
/// * `repo` - Repository paths and metadata for naming
pub async fn save_metadata(semantics_path: &Path, repo: &RepoPaths) -> anyhow::Result<()> {
    let metadata = generate_context_for_code_review(repo, semantics_path).await?;

    let filename = format!("metadata-{}.md", repo.unique_repo_hash());
    save_file_locally(&metadata, &filename)?;
    Ok(())
}
