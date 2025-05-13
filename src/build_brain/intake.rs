// crates/intake/src/lib.rs
use anyhow::Result;
use git2::Repository;
use ignore::gitignore::GitignoreBuilder;
use std::path::PathBuf;
use walkdir::WalkDir;

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
        let p = entry.path();
        if ign.matched(p, false).is_ignore() {
            continue;
        }
        match p.extension().and_then(|s| s.to_str()) {
            Some("sol") => sol_files.push(p.to_path_buf()),
            // accept any Markdown / reStructuredText / plain‑text file *whose name is README.md*
            Some("md" | "rst" | "txt")
                if p.file_name()
                    .and_then(|f| f.to_str()) // Option<&str>
                    .map(|s| s.eq_ignore_ascii_case("README.md"))
                    .unwrap_or(false) =>
            {
                docs.push(p.to_path_buf())
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
