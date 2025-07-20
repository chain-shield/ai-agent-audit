use anyhow::Result;
/// Docker volume cleanup utilities for secure analysis environment.
///
/// This module provides cleanup functions to remove Docker volumes and
/// temporary directories created during repository analysis, ensuring
/// no residual data remains on the host system.
use std::fs;
use std::path::Path;

/// Cleans up Docker volume directory and build artifacts after analysis.
///
/// Removes the repository directory and all associated build artifacts
/// from the Docker volume to prevent disk space accumulation and ensure
/// clean analysis environments for subsequent runs.
///
/// # Arguments
/// * `root` - Path to the repository root directory to clean up
///
/// # Example
/// ```
/// cleanup_repo_volume(&Path::new("/tmp/audit-analysis/my-repo"));
/// ```
pub fn cleanup_repo_volume(root: &Path) -> Result<()> {
    if root.exists() {
        fs::remove_dir_all(root).expect(&format!("Failed to remove Docker volume at {:?}", root));
        println!("[INFO] Cleaned up Docker volume: {:?}", root);
    } else {
        println!("[WARN] No Docker volume found for cleanup at {:?}", root);
    }

    Ok(())
}
