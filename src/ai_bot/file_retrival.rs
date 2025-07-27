use crate::build_brain::enbeddings::SourceChunk;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::logging::print_first_four_lines;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::client::EmbeddingsClient;
use rig::providers::openai::{Client, TEXT_EMBEDDING_3_SMALL};
use rig::vector_store::VectorStoreIndex;
use rig::{completion::ToolDefinition, tool::Tool};
// use rig_qdrant::QdrantVectorStore;  // Temporarily disabled due to version conflicts
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tiktoken_rs::cl100k_base;

/// Maximum tokens for embedding queries (OpenAI embedding models have 8192 token limit)
const MAX_EMBEDDING_TOKENS: usize = 8000; // Leave some headroom

/// Truncates a query string to fit within embedding token limits
fn truncate_query_for_embedding(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encoding = cl100k_base()?;
    let mut tokens = encoding.encode_with_special_tokens(query);

    if tokens.len() <= MAX_EMBEDDING_TOKENS {
        return Ok(query.to_string());
    }

    log::warn!(
        "Query too long ({} tokens), truncating to {} tokens",
        tokens.len(),
        MAX_EMBEDDING_TOKENS
    );
    tokens.truncate(MAX_EMBEDDING_TOKENS);

    let truncated = encoding.decode(tokens)?;
    Ok(truncated)
}

/// Sanitizes and validates tool input arguments to prevent JSON parsing issues
fn sanitize_tool_input(query: &str) -> Result<String, String> {
    // Check length
    if query.len() > 200 {
        return Err(format!("Query too long: {} chars (max 200)", query.len()));
    }

    // Remove problematic characters that can break JSON parsing
    let sanitized = query
        .replace('"', "'") // Replace double quotes with single quotes
        .replace('\\', "/") // Replace backslashes with forward slashes
        .replace('\n', " ") // Replace newlines with spaces
        .replace('\r', " ") // Replace carriage returns with spaces
        .replace('\t', " ") // Replace tabs with spaces
        .replace('\0', "") // Remove null characters
        .chars()
        .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .collect::<String>();

    // Trim and collapse multiple spaces
    let sanitized = sanitized.split_whitespace().collect::<Vec<_>>().join(" ");

    if sanitized.is_empty() {
        return Err("Query is empty after sanitization".to_string());
    }

    Ok(sanitized)
}

/// Sanitizes content to ensure it can be safely serialized as JSON
fn sanitize_content_for_json(content: &str) -> String {
    // Limit content size to prevent huge JSON payloads
    const MAX_CONTENT_CHARS: usize = 80000; // ~50KB limit

    let mut sanitized = if content.len() > MAX_CONTENT_CHARS {
        log::warn!(
            "Content too large ({} chars), truncating to {} chars",
            content.len(),
            MAX_CONTENT_CHARS
        );
        let truncated = content.chars().take(MAX_CONTENT_CHARS).collect::<String>();
        format!("{}\n\n[... content truncated due to size ...]", truncated)
    } else {
        content.to_string()
    };

    // Replace problematic characters that can break JSON
    sanitized = sanitized
        .replace('\r', "\\r") // Replace carriage returns
        .replace('\t', "    ") // Replace tabs with spaces
        .replace('\0', ""); // Remove null characters

    // Ensure the content doesn't contain unescaped quotes or backslashes
    // (serde_json should handle this, but let's be extra safe)
    sanitized
}

#[derive(Deserialize)]
pub struct RetrieveArgs {
    file_type: String, // e.g., "source", "test", "script"
    query: String,     // e.g., function name or keyword
}

#[derive(Serialize)]
pub struct RetrieveOut {
    content: String,
}

#[derive(Debug, Error)]
pub enum RetrievalError {
    #[error("Qdrant error: {0}")]
    Qdrant(#[from] qdrant_client::QdrantError),
    #[error("Vector store error: {0}")]
    VectorStore(#[from] rig::vector_store::VectorStoreError),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub struct FileRetrievalTool {
    qdrant_url: String,
    openai_api_key: String,
    repo: RepoPaths, // For dynamic collection name
}

impl FileRetrievalTool {
    pub fn new(qdrant_url: String, openai_api_key: String, repo: RepoPaths) -> Self {
        Self {
            qdrant_url,
            openai_api_key,
            repo,
        }
    }
}

impl Tool for FileRetrievalTool {
    const NAME: &'static str = "retrieve_file_content";
    type Args = RetrieveArgs;
    type Output = RetrieveOut;
    type Error = RetrievalError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Retrieve content from source code, test, script, or library files using semantic search. IMPORTANT: Keep queries short and simple (under 50 words) to avoid JSON parsing issues.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_type": {
                        "type": "string",
                        "enum": ["source", "test", "script", "library"],
                        "description": "Type of files to search"
                    },
                    "query": {
                        "type": "string",
                        "maxLength": 200,
                        "pattern": "^[a-zA-Z0-9\\s\\-_\\.]+$",
                        "description": "Short search query using simple keywords only (e.g. 'transfer function', 'access control', 'mint token'). Avoid quotes, special characters, and long descriptions."
                    }
                },
                "required": ["file_type", "query"],
                "additionalProperties": false
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        log::warn!(
            "🚫 FileRetrievalTool called but temporarily disabled due to rig-qdrant version conflicts with rig-core 0.13.0. Args: file_type='{}', query='{}'",
            args.file_type,
            args.query
        );
        
        Err(RetrievalError::Other(anyhow::anyhow!(
            "File retrieval functionality temporarily disabled due to rig-qdrant version conflicts with rig-core 0.13.0"
        )))
    }
}
