use crate::prepare_code::git_clone::RepoPaths;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tiktoken_rs::cl100k_base;

/// Maximum tokens for file content (to prevent huge payloads)
const MAX_FILE_TOKENS: usize = 5_000;

/// Maximum number of file picker calls per tool instance
const MAX_FILE_PICKER_CALLS: usize = 1;

/// Maximum number of files that can be picked in a single call
pub const MAX_FILES_PER_CALL: usize = 5;

/// Truncates content to fit within token limits
fn truncate_content_for_tokens(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoding = cl100k_base()?;
    let mut tokens = encoding.encode_with_special_tokens(content);

    if tokens.len() <= MAX_FILE_TOKENS {
        return Ok(content.to_string());
    }

    log::warn!(
        "File content too long ({} tokens), truncating to {} tokens",
        tokens.len(),
        MAX_FILE_TOKENS
    );
    tokens.truncate(MAX_FILE_TOKENS);

    let truncated = encoding.decode(tokens)?;
    Ok(format!(
        "{}\n\n[... content truncated due to size ...]",
        truncated
    ))
}

#[derive(Deserialize, Clone)]
pub struct FilePickerArgs {
    pub files: Vec<String>, // Array of file paths to retrieve (max MAX_FILES_PER_CALL)
}

#[derive(Serialize, Debug)]
pub struct FilePickerOut {
    pub content: String,
}

#[derive(Debug, Error)]
pub enum FilePickerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Too many files requested: {0} (max {1})")]
    TooManyFiles(usize, usize),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Too many tool calls: {0} (max {1})")]
    TooManyCalls(usize, usize),
}

pub struct FilePickerTool {
    pub repo: RepoPaths,
    pub available_files: Vec<String>, // List of available files
    call_count: Arc<Mutex<usize>>,    // Track number of calls
}

impl FilePickerTool {
    pub fn new(repo: RepoPaths) -> Self {
        let available_files = Self::collect_files(&repo);
        Self {
            repo,
            available_files,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    fn collect_files(repo: &RepoPaths) -> Vec<String> {
        let mut files = Vec::new();

        // Add Solidity files
        for sol_file in &repo.sol_files {
            if let Ok(relative_path) = sol_file.strip_prefix(&repo.root) {
                let lib_folder = format!("{}/lib", repo.repo_name);
                if !relative_path.starts_with(lib_folder) {
                    files.push(relative_path.to_string_lossy().to_string());
                }
            }
        }

        // Add documentation files (excluding README.md since it's already in context)
        for doc_file in &repo.docs {
            if let Ok(relative_path) = doc_file.strip_prefix(&repo.root) {
                let path_str = relative_path.to_string_lossy().to_string();
                // Skip README files since they're already provided in the context window
                if !path_str.to_lowercase().contains("readme") {
                    files.push(path_str);
                }
            }
        }

        files.sort();
        files
    }
}

impl Tool for FilePickerTool {
    const NAME: &'static str = "pick_files";
    type Args = FilePickerArgs;
    type Output = FilePickerOut;
    type Error = FilePickerError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let file_list = self.available_files.join("\n");

        ToolDefinition {
            name: Self::NAME.to_string(),
            description: format!(
                "Pick up to {} specific files to read their full content. IMPORTANT: Limited to {} calls total. NOTE: README files are excluded as they're already in context. Available files:\n{}",
                MAX_FILES_PER_CALL, MAX_FILE_PICKER_CALLS, file_list
            ),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "files": {
                        "type": "array",
                        "items": {"type": "string"},
                        "maxItems": MAX_FILES_PER_CALL,
                        "description": format!("Array of file paths to retrieve (max {})", MAX_FILES_PER_CALL)
                    }
                },
                "required": ["files"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Check call limit
        {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
            if *count > MAX_FILE_PICKER_CALLS {
                log::warn!(
                    "🚫 File picker call limit exceeded: {} (max {})",
                    *count,
                    MAX_FILE_PICKER_CALLS
                );
                // Return helpful message instead of error to avoid bailing the app
                return Ok(FilePickerOut {
                    content: format!(
                        "⚠️ File picker call limit reached ({} calls used). No more files can be retrieved. Please proceed with your analysis using the information already gathered.",
                        MAX_FILE_PICKER_CALLS
                    ),
                });
            }
            log::info!(
                "📁 File picker call #{} of {}",
                *count,
                MAX_FILE_PICKER_CALLS
            );
        }

        if args.files.len() > MAX_FILES_PER_CALL {
            return Err(FilePickerError::TooManyFiles(
                args.files.len(),
                MAX_FILES_PER_CALL,
            ));
        }

        let mut content_parts = Vec::new();

        for file_path in &args.files {
            let full_path = self.repo.root.join(file_path);

            if !full_path.exists() {
                return Err(FilePickerError::FileNotFound(file_path.clone()));
            }

            let content = fs::read_to_string(&full_path)?;

            // Truncate content if it's too large
            let truncated_content = match truncate_content_for_tokens(&content) {
                Ok(truncated) => truncated,
                Err(e) => {
                    log::warn!("Failed to truncate content for {}: {}", file_path, e);
                    content // Use original content if truncation fails
                }
            };

            content_parts.push(format!("=== {} ===\n{}", file_path, truncated_content));
        }

        let combined_content = content_parts.join("\n\n");

        // Calculate total tokens for logging
        let total_tokens = match cl100k_base() {
            Ok(encoding) => encoding.encode_with_special_tokens(&combined_content).len(),
            Err(_) => combined_content.len() / 4, // Rough estimate if tokenizer fails
        };

        log::info!(
            "📁 FilePickerTool returning files: [{}], {} tokens total",
            args.files.join(", "),
            total_tokens
        );

        Ok(FilePickerOut {
            content: combined_content,
        })
    }
}
