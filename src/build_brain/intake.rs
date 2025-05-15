use anyhow::Result;
use git2::Repository;
use ignore::gitignore::GitignoreBuilder;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct RepoPaths {
    pub root: PathBuf,
    pub sol_files: Vec<PathBuf>,
    pub docs: Vec<PathBuf>,
}

pub fn clone_and_filter(url: &str) -> Result<RepoPaths> {
    let root = tempfile::tempdir()?.keep();
    Repository::clone(url, &root)?;

    // drop build artefacts / node_modules in one pass
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    ign.add_line(None, "node_modules")?;
    let ign = ign.build()?;

    let mut sol_files = Vec::new();
    let mut docs = Vec::new();

    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if ign.matched(path, false).is_ignore() {
            continue;
        }
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("sol") => sol_files.push(path.to_path_buf()),
            // accept any Markdown / reStructuredText / plain‑text file *whose name is README.md*
            Some("md" | "rst" | "txt")
                if path
                    .file_name()
                    .and_then(|file| file.to_str()) // Option<&str>
                    .map(|filename| filename.eq_ignore_ascii_case("README.md"))
                    .unwrap_or(false) =>
            {
                docs.push(path.to_path_buf())
            }
            _ => {}
        }
    }

    Ok(RepoPaths {
        root,
        sol_files,
        docs,
    })
}
