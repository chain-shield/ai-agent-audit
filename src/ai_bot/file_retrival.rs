use crate::build_brain::enbeddings::SourceChunk;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::logging::print_first_four_lines;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::client::EmbeddingsClient;
use rig::providers::openai::{Client, TEXT_EMBEDDING_3_SMALL};
use rig::vector_store::VectorStoreIndex;
use rig::{completion::ToolDefinition, tool::Tool};
use rig_qdrant::QdrantVectorStore;
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
        log::info!(
            "🔧 FileRetrievalTool called with file_type='{}', query='{}'",
            args.file_type,
            args.query
        );

        // Validate file_type parameter
        if !["source", "test", "script", "library"].contains(&args.file_type.as_str()) {
            let error_msg = format!(
                "Invalid file_type: '{}'. Must be 'source', 'test', 'script', or 'library'.",
                args.file_type
            );
            log::error!("❌ FileRetrievalTool validation failed: {}", error_msg);
            return Err(RetrievalError::Other(anyhow::anyhow!(error_msg)));
        }

        // Sanitize query to prevent JSON parsing issues
        let sanitized_query = match sanitize_tool_input(&args.query) {
            Ok(query) => {
                if query != args.query {
                    log::info!("🧹 Query sanitized: '{}' -> '{}'", args.query, query);
                }
                query
            }
            Err(e) => {
                let error_msg = format!("Invalid query: {}", e);
                log::error!("❌ Query validation failed: {}", error_msg);
                return Err(RetrievalError::Other(anyhow::anyhow!(error_msg)));
            }
        };

        // Use tokio::task::spawn_blocking to move the non-Sync operation to a blocking context
        let qdrant_url = self.qdrant_url.clone();
        let openai_api_key = self.openai_api_key.clone();
        let repo = self.repo.clone();
        let query = sanitized_query; // Use sanitized query instead of raw input
        let file_type = args.file_type.clone();

        let result = tokio::task::spawn_blocking(move || {
            // Create a new tokio runtime for the blocking task
            let rt = tokio::runtime::Runtime::new().map_err(|e| {
                RetrievalError::Other(anyhow::anyhow!("Failed to create runtime: {}", e))
            })?;

            rt.block_on(async move {
                // Create vector store
                let qdrant = Qdrant::from_url(&qdrant_url).build()?;
                let openai = Client::new(&openai_api_key);
                let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

                let collection_name = format!("{}-contract_chunks", repo.unique_repo_hash());
                let qp = QueryPointsBuilder::new(&collection_name)
                    .with_payload(true)
                    .build();

                let vector_store = QdrantVectorStore::new(qdrant, model, qp);

                // Truncate query to fit embedding token limits
                let truncated_query = truncate_query_for_embedding(&query)
                    .map_err(|e| RetrievalError::Other(anyhow::anyhow!("Query truncation failed: {}", e)))?;

                if truncated_query.len() != query.len() {
                    log::info!("📏 Query truncated from {} to {} characters", query.len(), truncated_query.len());
                }

                // Perform search - get more results to combine related chunks
                let search_results = vector_store.top_n::<SourceChunk>(&truncated_query, 10).await?;
                log::info!("🔍 Vector search returned {} results", search_results.len());

                // Filter results by file type and score, then combine related chunks
                let mut file_chunks: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
                let mut total_results = 0;
                let mut score_filtered = 0;
                let mut type_filtered = 0;

                for (score, _doc_id, source_chunk) in &search_results {
                    total_results += 1;
                    log::debug!("📄 Result {}: score={:.3}, file_type='{}', metadata='{}'",
                               total_results, score, source_chunk.file_type, source_chunk.metadata);

                    // Only include results with good relevance scores
                    if *score <= 0.5 {
                        score_filtered += 1;
                        log::debug!("⚠️  Filtered out due to low score: {:.3}", score);
                        continue;
                    }

                    // Filter by file type using the direct field access
                    if source_chunk.file_type == file_type {
                        // Extract file name from metadata (format: "path/file.sol:chunk N")
                        let file_name = source_chunk.metadata
                            .split(':')
                            .next()
                            .unwrap_or(&source_chunk.metadata)
                            .to_string();

                        log::debug!("✅ Included result: {} chars from {}", source_chunk.text.len(), file_name);

                        file_chunks.entry(file_name)
                            .or_insert_with(Vec::new)
                            .push(source_chunk.text.clone());
                    } else {
                        type_filtered += 1;
                        log::debug!("⚠️  Filtered out due to file type mismatch: expected '{}', got '{}'",
                                   file_type, source_chunk.file_type);
                    }
                }

                // Combine chunks from the same files to provide more complete context
                let mut combined_content = Vec::new();
                for (file_name, chunks) in file_chunks {
                    let file_content = chunks.join("\n\n");
                    combined_content.push(format!("=== {} ===\n{}", file_name, file_content));
                }

                log::info!("📊 Filter results: {} total, {} score-filtered, {} type-filtered, {} files with chunks",
                          total_results, score_filtered, type_filtered, combined_content.len());

                let content = combined_content.join("\n\n");

                if content.is_empty() {
                    let error_msg = format!("No relevant content found for file type '{}' with query '{}' (searched {} results)",
                                           file_type, query, total_results);
                    log::warn!("❌ {}", error_msg);
                    return Err(RetrievalError::Other(anyhow::anyhow!(error_msg)));
                }

                log::info!("✅ FileRetrievalTool returning {} characters of content", content.len());
                print_first_four_lines(&content);

                // Sanitize content to prevent JSON parsing issues
                let sanitized_content = sanitize_content_for_json(&content);
                if sanitized_content.len() != content.len() {
                    log::info!("📝 Content sanitized: {} -> {} characters", content.len(), sanitized_content.len());
                }

                // Test JSON serialization to catch issues early
                let test_output = RetrieveOut { content: sanitized_content.clone() };
                match serde_json::to_string(&test_output) {
                    Ok(_) => {
                        log::debug!("✅ Content serialization test passed");
                    }
                    Err(e) => {
                        log::error!("❌ Content serialization test failed: {}", e);
                        log::error!("Problematic content preview: {}",
                                   sanitized_content.chars().take(500).collect::<String>());
                        return Err(RetrievalError::Other(anyhow::anyhow!(
                            "Content serialization failed: {}", e
                        )));
                    }
                }

                Ok(RetrieveOut { content: sanitized_content })
            })
        })
        .await
        .map_err(|e| RetrievalError::Other(anyhow::anyhow!("Task join error: {}", e)))??;

        Ok(result)
    }
}
