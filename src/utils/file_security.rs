/// File system security utilities.
///
/// This module provides secure file operations including path validation,
/// symlink detection, and safe file reading with size limits.
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum file size for reading (10MB)
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum path length to prevent buffer overflow attacks
const MAX_PATH_LENGTH: usize = 4096;

/// Validates that a path is safe to access (no path traversal, symlinks, etc.)
pub fn validate_safe_path(path: &Path, allowed_base: &Path) -> Result<()> {
    // Convert to absolute paths for comparison
    let abs_path = path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {:?}", path))?;
    let abs_base = allowed_base
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize base path: {:?}", allowed_base))?;

    // Check if path is within allowed base directory
    if !abs_path.starts_with(&abs_base) {
        anyhow::bail!(
            "Path traversal attempt detected: {:?} is outside {:?}",
            abs_path,
            abs_base
        );
    }

    // Check path length
    if abs_path.to_string_lossy().len() > MAX_PATH_LENGTH {
        anyhow::bail!(
            "Path is too long: {} characters",
            abs_path.to_string_lossy().len()
        );
    }

    // Check for symlinks in the path components
    let mut current = abs_path.as_path();
    while let Some(parent) = current.parent() {
        if parent == abs_base {
            break;
        }

        let metadata = fs::symlink_metadata(current)
            .with_context(|| format!("Failed to read metadata for: {:?}", current))?;

        if metadata.file_type().is_symlink() {
            anyhow::bail!("Symlink detected in path: {:?}", current);
        }

        current = parent;
    }

    Ok(())
}

/// Safely reads a file with size limits and validation
pub fn safe_read_file(path: &Path, allowed_base: &Path) -> Result<String> {
    // Validate path safety
    validate_safe_path(path, allowed_base)?;

    // Check file metadata
    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to read file metadata: {:?}", path))?;

    // Check if it's actually a file
    if !metadata.is_file() {
        anyhow::bail!("Path is not a regular file: {:?}", path);
    }

    // Check file size
    if metadata.len() > MAX_FILE_SIZE {
        anyhow::bail!(
            "File is too large: {} bytes (max: {} bytes)",
            metadata.len(),
            MAX_FILE_SIZE
        );
    }

    // Read file content
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {:?}", path))
}

/// Validates file extension against allowed list
pub fn validate_file_extension(path: &Path, allowed_extensions: &[&str]) -> Result<()> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow::anyhow!("File has no extension: {:?}", path))?;

    if !allowed_extensions.contains(&extension) {
        anyhow::bail!(
            "File extension '{}' not allowed. Allowed: {:?}",
            extension,
            allowed_extensions
        );
    }

    Ok(())
}

/// Creates a secure temporary directory with restricted permissions
pub fn create_secure_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .context("Failed to create temporary directory")?;

    let path = temp_dir.keep();

    // Set restrictive permissions (owner only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o700); // rwx------
        fs::set_permissions(&path, perms)?;
    }

    Ok(path)
}

/// Validates and sanitizes a Git repository URL to prevent command injection.
///
/// # Security
/// This function prevents command injection by validating URL format and
/// rejecting URLs with shell metacharacters or suspicious patterns.
pub fn validate_repo_url(url: &str) -> Result<()> {
    // Basic URL format validation
    let url_regex = Regex::new(r"^https?://[a-zA-Z0-9.-]+/[a-zA-Z0-9._/-]+(?:\.git)?/?$")
        .context("Failed to compile URL regex")?;

    if !url_regex.is_match(url) {
        anyhow::bail!("Invalid repository URL format: {}", url);
    }

    // Check for shell metacharacters that could enable command injection
    let dangerous_chars = [
        '&', '|', ';', '`', '$', '(', ')', '{', '}', '<', '>', '"', '\'', '\\',
    ];
    if url.chars().any(|c| dangerous_chars.contains(&c)) {
        anyhow::bail!(
            "Repository URL contains potentially dangerous characters: {}",
            url
        );
    }

    // Reject URLs that are too long (potential buffer overflow)
    if url.len() > 2048 {
        anyhow::bail!("Repository URL is too long: {} characters", url.len());
    }

    Ok(())
}

/// Sanitizes repository name to prevent path traversal and injection attacks.
pub fn sanitize_repo_name(name: &str) -> String {
    // Remove any path traversal attempts and dangerous characters
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(250) // Limit length
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}

/// Sanitizes filename to prevent directory traversal and special characters
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .take(255) // Limit filename length
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}
