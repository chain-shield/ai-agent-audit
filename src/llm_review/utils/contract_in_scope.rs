use std::{fs, path::Path};

use crate::{
    llm_review::contract_file_map::get_file_from_contract, prepare_code::git_clone::RepoPaths,
    utils::contract_name_check::has_non_mock_contract,
};

pub async fn is_contract_in_scope(contract: &str, repo: &RepoPaths) -> anyhow::Result<bool> {
    match get_file_from_contract(contract, repo).await {
        Some(file) => {
            // check if file is in scoped_files
            let scoped_files = repo.extract_scoped_files()?;

            if scoped_files.is_empty() {
                return Ok(true);
            }

            let is_library_file = file.ancestors().any(|ancestor| {
                matches!(ancestor.file_name(), Some(name) if name == "lib" || name == "library" || name == "libraries")
            });
            let is_in_scope = scoped_files.contains(&file);

            // skip if contract if does not have have at least one line that start with contract and contract
            // name does NOT contain 'mock' (case insensative)
            let content = fs::read_to_string(&file)?;
            let has_non_mock_contract = has_non_mock_contract(&content);

            Ok(is_in_scope && has_non_mock_contract && !is_library_file)
        }
        None => Ok(false),
    }
}
