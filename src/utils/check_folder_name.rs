use std::path::Path;

fn path_has_any_segment(file: &Path, root: &Path, segments: &[&str]) -> bool {
    if file.is_dir() {
        return false;
    }
    if let Some(parent) = file.parent() {
        segments.iter().any(|s| root.join(s) == parent)
    } else {
        false
    }
}

pub fn is_test_file(file: &Path, root: &Path) -> bool {
    path_has_any_segment(file, root, &["test", "tests"])
}

pub fn is_script_file(file: &Path, root: &Path) -> bool {
    path_has_any_segment(file, root, &["script", "scripts", "deploy"])
}

pub fn is_config_file(file: &Path, root: &Path) -> bool {
    // make sure file is in root
    if let Some(parent) = file.parent() {
        if parent != root {
            return false;
        }
    } else {
        return false;
    };

    let filename = file.file_name().unwrap_or_default().to_string_lossy();

    let config_files = [
        "foundry.toml",
        "hardhat.config.js",
        "remappings.txt",
        "package.json",
        ".env.sample",
    ];

    let filename_str = filename.as_ref();
    config_files.iter().any(|f| *f == filename_str)
}
