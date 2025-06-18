use std::path::{Path, PathBuf};

use crate::{
    build_brain::git_clone::RepoPaths, enumerator::codeblock_db::CodeBlocksDb,
    llm_review::prompt_content::generate_context_for_code_review,
    reporting::save_file::save_file_locally,
};

pub fn save_contract_and_fn_ir(codeblocks_path: &PathBuf, repo: &RepoPaths) -> anyhow::Result<()> {
    let protocol_name = repo.repo_name.clone();
    let commit = repo.commit_hash.clone();

    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    log::info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts()?;

    for (contract, codeblock) in contracts {
        let filename = format!("{}-{}-{}.md", contract, protocol_name, &commit[..6]);
        save_file_locally(&codeblock, &filename)?;
    }
    Ok(())
}

pub async fn save_metadata(semantics_path: &Path, repo: &RepoPaths) -> anyhow::Result<()> {
    let protocol_name = repo.repo_name.clone();
    let commit = repo.commit_hash.clone();

    let metadata = generate_context_for_code_review(&repo.root, semantics_path).await?;

    let filename = format!("metadata-{}-{}.md", protocol_name, &commit[..6]);
    save_file_locally(&metadata, &filename)?;
    Ok(())
}
