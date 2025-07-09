/// Environment variable security utilities.
/// 
/// This module provides secure handling of environment variables including
/// validation, sanitization, and secure logging to prevent credential leakage.

use anyhow::{Context, Result};
use std::env;

/// Securely retrieves an API key from environment variables with validation.
/// 
/// # Security Features
/// - Validates key format and length
/// - Prevents logging of actual key values
/// - Returns sanitized error messages
pub fn get_api_key(env_var: &str) -> Result<String> {
    let key = env::var(env_var)
        .with_context(|| format!("Environment variable {} is not set", env_var))?;
    
    // Validate API key format
    if key.is_empty() {
        anyhow::bail!("API key {} is empty", env_var);
    }
    
    if key.len() < 10 {
        anyhow::bail!("API key {} appears to be too short", env_var);
    }
    
    if key.len() > 512 {
        anyhow::bail!("API key {} is suspiciously long", env_var);
    }
    
    // Check for obvious placeholder values
    let placeholder_values = ["your_api_key", "placeholder", "changeme", "test", "demo"];
    let key_lower = key.to_lowercase();
    if placeholder_values.iter().any(|&placeholder| key_lower.contains(placeholder)) {
        anyhow::bail!("API key {} appears to be a placeholder value", env_var);
    }
    
    Ok(key)
}

/// Securely retrieves a URL from environment variables with validation.
pub fn get_secure_url(env_var: &str) -> Result<String> {
    let url = env::var(env_var)
        .with_context(|| format!("Environment variable {} is not set", env_var))?;
    
    // Basic URL validation
    if !url.starts_with("http://") && !url.starts_with("https://") {
        anyhow::bail!("URL {} must start with http:// or https://", env_var);
    }
    
    if url.len() > 2048 {
        anyhow::bail!("URL {} is too long", env_var);
    }
    
    // Check for localhost/private IPs in production
    if env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        if url.contains("localhost") || url.contains("127.0.0.1") || url.contains("0.0.0.0") {
            anyhow::bail!("Localhost URLs not allowed in production environment");
        }
    }
    
    Ok(url)
}

/// Sanitizes a string for safe logging (removes sensitive information).
pub fn sanitize_for_logging(input: &str) -> String {
    if input.len() <= 8 {
        "*".repeat(input.len())
    } else {
        format!("{}***{}", &input[..4], &input[input.len()-4..])
    }
}

/// Validates that required environment variables are set without exposing values.
pub fn validate_required_env_vars() -> Result<()> {
    let required_vars = ["OPENAI_API_KEY", "QDRANT_URL"];
    let mut missing_vars = Vec::new();
    
    for var in &required_vars {
        if env::var(var).is_err() {
            missing_vars.push(*var);
        }
    }
    
    if !missing_vars.is_empty() {
        anyhow::bail!("Missing required environment variables: {}", missing_vars.join(", "));
    }
    
    Ok(())
}
