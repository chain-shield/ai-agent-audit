use std::path::{Path, PathBuf};

use crate::{
    enumerator::codeblock_db::CodeBlocksDb,
    llm_review::prompt_content::generate_context_for_code_review,
    prepare_code::git_clone::RepoPaths, reporting::save_file::save_file_locally,
};

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

pub async fn save_metadata(semantics_path: &Path, repo: &RepoPaths) -> anyhow::Result<()> {
    let metadata = generate_context_for_code_review(repo, semantics_path).await?;

    let filename = format!("metadata-{}.md", repo.unique_repo_hash());
    save_file_locally(&metadata, &filename)?;
    Ok(())
}
