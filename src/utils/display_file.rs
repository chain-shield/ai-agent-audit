use std::path::PathBuf;

use crate::prepare_code::git_clone::RepoPaths;

pub fn display_file(file: &PathBuf, repo: &RepoPaths) -> String {
    let search_root = repo.root.join(&repo.repo_name);

    match file.strip_prefix(&search_root) {
        Ok(name) => name.to_string_lossy().into_owned(),
        Err(_) => file.to_string_lossy().to_string(),
    }
}
