use tokio::fs;

use crate::prepare_code::git_clone::RepoPaths;

// returns file content assuming file is relative path to project root
pub async fn read_project_file(file: &str, repo: &RepoPaths) -> anyhow::Result<String> {
    let search_root = repo.get_protocol_root();
    let file_path = search_root.join(file);

    let file_content = fs::read_to_string(file_path).await?;

    Ok(file_content)
}
