use std::{fs, path::Path};

use anyhow::Result;
use walkdir::WalkDir;

pub fn extract_content_from_docs(repo_root: &Path) -> Result<String> {
    let mut out = String::new();

    for entry in WalkDir::new(&repo_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        let filename = path
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = fs::read_to_string(entry.path())?;
        out.push_str(&format!("{}\n\n{}\n\n", filename, content));
    }
    Ok(out)
}
