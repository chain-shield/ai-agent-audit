use crate::{
    cost::cost_data::get_token_count,
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::{
        analysis::context_state::get_metadata_context, contract::contract_file_map::ContractType,
        utils::contract_in_scope::contract_scope_and_type,
    },
    prepare_code::git_clone::RepoPaths,
    reporting::save_file::save_file_locally,
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
pub async fn save_codeblocks_locally(
    codeblocks_path: &PathBuf,
    repo: &RepoPaths,
) -> anyhow::Result<()> {
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    log::info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts(repo)?;
    let output_dir = Path::new(&repo.repo_name);

    for (contract, codeblock) in contracts {
        // check contract in inscope!
        let (is_contract_in_scope, contract_type_option) =
            contract_scope_and_type(&contract, repo).await?;

        let contract_type = contract_type_option.unwrap_or(ContractType::Contract);

        if !is_contract_in_scope {
            continue;
        }

        let token_count = get_token_count(&codeblock);
        let filename = format!("{}-{}-size-{}.md", contract_type, contract, token_count);
        let full_path = output_dir.join(filename);
        save_file_locally(&codeblock, &full_path)?;
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
pub async fn save_metadata(repo: &RepoPaths) -> anyhow::Result<()> {
    let metadata = get_metadata_context(repo)
        .await
        .expect("cannot load metadata");

    let output_dir = Path::new(&repo.repo_name);
    let filename = format!("metadata-{}.md", repo.unique_repo_hash());
    let full_path = output_dir.join(filename);
    save_file_locally(&metadata, &full_path)?;
    Ok(())
}
