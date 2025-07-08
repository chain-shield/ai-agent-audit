use std::fs;

use anyhow::Result;
use walkdir::WalkDir;

use crate::prepare_code::git_clone::RepoPaths;

pub fn extract_content_from_docs(repo: &RepoPaths) -> Result<String> {
    let mut out = String::new();

    let repo_code_root = repo.root.join(repo.repo_name.clone());
    for entry in WalkDir::new(&repo_code_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();

        // Skip directories and symlinks
        if fs::symlink_metadata(path)?.file_type().is_symlink() {
            continue;
        }

        let filename = path
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = fs::read_to_string(entry.path())?;
        out.push_str(&format!("{}\n\n{}\n\n", filename, content));
    }
    Ok(out)
}
