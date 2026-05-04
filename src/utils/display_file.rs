use std::path::Path;

use crate::prepare_code::git_clone::RepoPaths;

pub fn display_file(file: &Path, repo: &RepoPaths) -> String {
    let search_root = repo.get_protocol_root();

    match file.strip_prefix(&search_root) {
        Ok(name) => name.to_string_lossy().into_owned(),
        Err(_) => file.to_string_lossy().to_string(),
    }
}
