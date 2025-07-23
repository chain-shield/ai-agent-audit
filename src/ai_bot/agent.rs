use std::collections::HashSet;

use anyhow::Result;
use log::info;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{client::EmbeddingsClient, providers::openai::Client, vector_store::VectorStoreIndex};
use rig_qdrant::QdrantVectorStore;
/// AI agent implementations with vector-based context retrieval.
///
/// This module provides intelligent AI agents that combine static documentation
/// with dynamic vector search for contextual smart contract analysis.
use tiktoken_rs::cl100k_base;

use crate::build_brain::enbeddings::SourceChunk;
use crate::config::MAX_RAG_QUERY_CONTENT_LENGTH;
use crate::prepare_code::git_clone::RepoPaths;

pub async fn get_rag_for_security_query(query_content: &str, repo: &RepoPaths) -> Result<String> {
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    // Tokenize the input
    let encoding = cl100k_base()?; // for OpenAI models
    let mut tokens = encoding.encode(query_content, HashSet::new());

    // Truncate tokens if needed
    if tokens.len() > MAX_RAG_QUERY_CONTENT_LENGTH {
        tokens.truncate(MAX_RAG_QUERY_CONTENT_LENGTH);
    }

    // Decode truncated tokens back into a string
    let query_content = encoding.decode(tokens)?;
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());
    let qp = QueryPointsBuilder::new(&vector_db_name) // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    info!("creating store...");
    let store = QdrantVectorStore::new(qdrant, model, qp);

    info!("retrieving relevant content from vector db");
    let relevant_docs: Vec<(f64, String, SourceChunk)> = store.top_n(&query_content, 3).await?;

    let dynamic_content = relevant_docs
        .iter()
        .map(|(_, _, source_chunk)| source_chunk.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    // info!("dynamic content");
    // print_first_four_lines(&dynamic_content);

    let final_context = format!("## ADDITIONAL CONTEXT: \n\n {}", dynamic_content);

    Ok(final_context)
}
