use crate::build_brain::enbeddings::SourceChunk;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::logging::print_first_four_lines;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
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
#[error("Retrieval error: {0}")]
pub enum RetrievalError {
    Qdrant(#[from] qdrant_client::QdrantError),
    VectorStore(#[from] rig::vector_store::VectorStoreError),
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
            description: "Retrieve content from source code, test, script, or library files based on type and query using semantic vector search. Use this tool to find specific code snippets, functions, or implementations.".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_type": {
                        "type": "string",
                        "enum": ["source", "test", "script", "library"],
                        "description": "Type of files to search: 'source' for main application code, 'test' for test files, 'script' for build/deployment scripts, 'library' for library/utility code in lib folders"
                    },
                    "query": {
                        "type": "string",
                        "description": "Search query - can be function names, class names, keywords, or natural language describing what you're looking for"
                    }
                },
                "required": ["file_type", "query"]
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

        // Use tokio::task::spawn_blocking to move the non-Sync operation to a blocking context
        let qdrant_url = self.qdrant_url.clone();
        let openai_api_key = self.openai_api_key.clone();
        let repo = self.repo.clone();
        let query = args.query.clone();
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

                // Perform search
                let search_results = vector_store.top_n::<SourceChunk>(&truncated_query, 5).await?;
                log::info!("🔍 Vector search returned {} results", search_results.len());

                // Filter results by file type and score, then extract content
                let mut filtered_results = Vec::new();
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
                        filtered_results.push(source_chunk.text.clone());
                        log::debug!("✅ Included result: {} chars", source_chunk.text.len());
                    } else {
                        type_filtered += 1;
                        log::debug!("⚠️  Filtered out due to file type mismatch: expected '{}', got '{}'",
                                   file_type, source_chunk.file_type);
                    }
                }

                log::info!("📊 Filter results: {} total, {} score-filtered, {} type-filtered, {} included",
                          total_results, score_filtered, type_filtered, filtered_results.len());

                let content = filtered_results.join("\n\n");

                if content.is_empty() {
                    let error_msg = format!("No relevant content found for file type '{}' with query '{}' (searched {} results)",
                                           file_type, query, total_results);
                    log::warn!("❌ {}", error_msg);
                    return Err(RetrievalError::Other(anyhow::anyhow!(error_msg)));
                }

                log::info!("✅ FileRetrievalTool returning {} characters of content", content.len());
                print_first_four_lines(&content);

                Ok(RetrieveOut { content })
            })
        })
        .await
        .map_err(|e| RetrievalError::Other(anyhow::anyhow!("Task join error: {}", e)))??;

        Ok(result)
    }
}
