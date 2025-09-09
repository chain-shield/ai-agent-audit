use crate::{
    llm_review::contract_file_map::get_file_from_contract, prepare_code::git_clone::RepoPaths,
};

pub async fn is_contract_in_scope(contract: &str, repo: &RepoPaths) -> anyhow::Result<bool> {
    match get_file_from_contract(contract, repo).await {
        Some(file) => {
            // check if file is in scoped_files
            let scoped_files = repo.extract_scoped_files()?;

            if scoped_files.is_empty() {
                return Ok(true);
            }

            let is_in_scope = scoped_files.contains(&file);

            Ok(is_in_scope)
        }
        None => Ok(false),
    }
}
