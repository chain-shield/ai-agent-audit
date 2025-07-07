use std::fs;
use std::path::Path;

use anyhow::Result;

/// Deletes the Docker volume directory for a given repository.
/// This removes the repo and all its build artifacts from the host.
///
/// Example:
/// cleanup_repo_volume("my-repo-1a2b3c");
pub fn cleanup_repo_volume(root: &Path) -> Result<()> {
    if root.exists() {
        fs::remove_dir_all(root).expect(&format!("Failed to remove Docker volume at {:?}", root));
        println!("[INFO] Cleaned up Docker volume: {:?}", root);
    } else {
        println!("[WARN] No Docker volume found for cleanup at {:?}", root);
    }

    Ok(())
}
