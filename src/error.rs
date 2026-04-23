/// Centralized error types for the AI Agent Audit application.
///
/// This module provides a unified error handling system that consolidates
/// various error types from different modules into a cohesive hierarchy.
/// This improves debugging, error propagation, and overall system reliability.
use thiserror::Error;

/// Main error type for the AI Agent Audit application.
///
/// This enum encompasses all possible error conditions that can occur
/// during the audit process, providing specific error types for different
/// failure scenarios with descriptive messages and error chaining.
#[derive(Debug, Error)]
pub enum AuditError {
    /// Database operation failures (SQLite)
    #[error("Database operation failed: {message}")]
    Database {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Git operations and repository handling failures
    #[error("Git operation failed: {operation} - {message}")]
    Git {
        operation: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Docker container operations failures
    #[error("Docker operation failed: {command} - {message}")]
    Docker {
        command: String,
        message: String,
        exit_code: Option<i32>,
    },

    /// Slither static analysis failures
    #[error("Slither analysis failed: {printer} - {message}")]
    SlitherAnalysis {
        printer: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// LLM processing and API communication failures
    #[error("LLM processing failed: {provider} - {message}")]
    LlmProcessing {
        provider: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// File system operations failures
    #[error("File system operation failed: {path} - {message}")]
    FileSystem {
        path: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// JSON parsing and serialization failures
    #[error("JSON processing failed: {context} - {message}")]
    JsonProcessing {
        context: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Security validation failures
    #[error("Security validation failed: {check} - {message}")]
    Security { check: String, message: String },

    /// Configuration and environment setup failures
    #[error("Configuration error: {setting} - {message}")]
    Configuration { setting: String, message: String },

    /// Network operations failures
    #[error("Network operation failed: {url} - {message}")]
    Network {
        url: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Async task processing failures
    #[error("Async task failed: {task} - {message}")]
    AsyncTask {
        task: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Generic validation failures
    #[error("Validation failed: {field} - {message}")]
    Validation { field: String, message: String },
}

/// Result type alias for consistent error handling throughout the application.
pub type Result<T> = std::result::Result<T, AuditError>;

impl AuditError {
    /// Creates a new database error with context.
    pub fn database<E>(message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Database {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new git error with context.
    pub fn git<E>(operation: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Git {
            operation: operation.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new Docker error with context.
    pub fn docker(
        command: impl Into<String>,
        message: impl Into<String>,
        exit_code: Option<i32>,
    ) -> Self {
        Self::Docker {
            command: command.into(),
            message: message.into(),
            exit_code,
        }
    }

    /// Creates a new Slither analysis error with context.
    pub fn slither<E>(printer: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::SlitherAnalysis {
            printer: printer.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new LLM processing error with context.
    pub fn llm<E>(provider: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::LlmProcessing {
            provider: provider.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new file system error with context.
    pub fn file_system<E>(path: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::FileSystem {
            path: path.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new JSON processing error with context.
    pub fn json<E>(context: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::JsonProcessing {
            context: context.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new security validation error.
    pub fn security(check: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Security {
            check: check.into(),
            message: message.into(),
        }
    }

    /// Creates a new configuration error.
    pub fn configuration(setting: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Configuration {
            setting: setting.into(),
            message: message.into(),
        }
    }

    /// Creates a new network error with context.
    pub fn network<E>(url: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Network {
            url: url.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new async task error with context.
    pub fn async_task<E>(task: impl Into<String>, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::AsyncTask {
            task: task.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a new validation error.
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Conversion implementations for common error types
impl From<std::io::Error> for AuditError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystem {
            path: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for AuditError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonProcessing {
            context: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<rusqlite::Error> for AuditError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Database {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<reqwest::Error> for AuditError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network {
            url: err
                .url()
                .map(|u| u.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<tokio::task::JoinError> for AuditError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::AsyncTask {
            task: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<anyhow::Error> for AuditError {
    fn from(err: anyhow::Error) -> Self {
        Self::AsyncTask {
            task: "legacy_anyhow".to_string(),
            message: err.to_string(),
            source: Some(err.into()),
        }
    }
}

/// Convenience macros for creating specific error types
#[macro_export]
macro_rules! audit_error {
    (database, $msg:expr) => {
        $crate::error::AuditError::Database {
            message: $msg.to_string(),
            source: None,
        }
    };
    (git, $op:expr, $msg:expr) => {
        $crate::error::AuditError::Git {
            operation: $op.to_string(),
            message: $msg.to_string(),
            source: None,
        }
    };
    (docker, $cmd:expr, $msg:expr) => {
        $crate::error::AuditError::Docker {
            command: $cmd.to_string(),
            message: $msg.to_string(),
            exit_code: None,
        }
    };
    (security, $check:expr, $msg:expr) => {
        $crate::error::AuditError::Security {
            check: $check.to_string(),
            message: $msg.to_string(),
        }
    };
    (config, $setting:expr, $msg:expr) => {
        $crate::error::AuditError::Configuration {
            setting: $setting.to_string(),
            message: $msg.to_string(),
        }
    };
    (validation, $field:expr, $msg:expr) => {
        $crate::error::AuditError::Validation {
            field: $field.to_string(),
            message: $msg.to_string(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AuditError::security("url_validation", "Invalid URL format");
        assert!(err.to_string().contains("Security validation failed"));
        assert!(err.to_string().contains("url_validation"));
    }

    #[test]
    fn test_error_macro() {
        let err = audit_error!(validation, "repo_url", "URL is required");
        assert!(err.to_string().contains("Validation failed"));
        assert!(err.to_string().contains("repo_url"));
    }
}
