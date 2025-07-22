use std::fs;

use anyhow::Result;
use log::info;

use crate::prepare_code::git_clone::RepoPaths;

pub fn extract_content_from_docs(repo: &RepoPaths) -> Result<String> {
    let mut docs = String::new();

    for doc in &repo.docs {
        // Skip directories and symlinks
        if fs::symlink_metadata(doc)?.file_type().is_symlink() {
            continue;
        }

        let filename = doc
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = match fs::read_to_string(doc) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read {}: {}", doc.display(), e);
                continue;
            }
        };

        if content.trim().is_empty() {
            continue; // skip empty files
        }
        info!("extracting content from {} doc file", filename);
        docs.push_str(&format!("### {}\n\n{}\n\n", filename, content));
    }
    Ok(docs)
}

pub fn extract_content_from_source_code(repo: &RepoPaths) -> Result<String> {
    let mut source_code = String::new();

    for code in &repo.sol_files {
        // Skip directories and symlinks
        if fs::symlink_metadata(code)?.file_type().is_symlink() {
            continue;
        }

        let filename = code
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();
        let content = match fs::read_to_string(code) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read {}: {}", code.display(), e);
                continue;
            }
        };

        if content.trim().is_empty() {
            continue; // skip empty files
        }
        source_code.push_str(&format!("### {}\n\n{}\n\n", filename, content));
    }
    Ok(source_code)
}
