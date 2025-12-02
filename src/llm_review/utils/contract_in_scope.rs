use std::fs;

use crate::{
    llm_review::contract::contract_file_map::{ContractType, get_file_from_contract},
    prepare_code::git_clone::RepoPaths,
    utils::contract_name_check::has_non_mock_contract,
};
/// Is contract in scope (as defined by  project scope txt) and what type of contract is it
pub async fn contract_scope_and_type(
    contract: &str,
    repo: &RepoPaths,
) -> anyhow::Result<(bool, Option<ContractType>)> {
    match get_file_from_contract(contract, repo).await {
        Some((file, contract_type)) => {
            // check type
            if contract_type == ContractType::Interface {
                return Ok((false, Some(ContractType::Interface)));
            }

            // check if file is in scoped_files
            let scoped_files = repo.extract_scoped_files()?;

            if scoped_files.is_empty() {
                return Ok((true, Some(contract_type)));
            }

            let is_in_scope = scoped_files.contains(&file);

            // skip if contract if does not have have at least one line that start with: contract,
            // abstract contract, or library
            // name does NOT contain 'mock' (case insensative)
            let content = fs::read_to_string(&file)?;
            let has_non_mock_contract = has_non_mock_contract(&content);

            Ok((is_in_scope && has_non_mock_contract, Some(contract_type)))
        }
        None => Ok((false, None)),
    }
}
