use std::collections::HashSet;

use anyhow::Result;
use log::info;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{
    agent::{Agent, AgentBuilder},
    client::{CompletionClient, EmbeddingsClient},
    providers::openai::{Client, CompletionModel, GPT_4O},
    vector_store::VectorStoreIndex,
};
use rig_qdrant::QdrantVectorStore;
/// AI agent implementations with vector-based context retrieval.
///
/// This module provides intelligent AI agents that combine static documentation
/// with dynamic vector search for contextual smart contract analysis.
use tiktoken_rs::cl100k_base;

use crate::build_brain::enbeddings::SourceChunk;
use crate::config::MAX_RAG_QUERY_CONTENT_LENGTH;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_doc_file::extract_content_from_docs;
use crate::utils::logging::print_first_four_lines;

/// Creates an AI audit agent with vector-based dynamic context retrieval.
///
/// This agent combines static documentation context with dynamic vector search
/// to provide relevant code context for smart contract analysis queries.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `Agent<CompletionModel>` - Configured AI agent with vector search capabilities
pub fn create_ai_audit_agent(repo: &RepoPaths) -> Result<Agent<CompletionModel>> {
    // Extract static documentation for base context
    let documentation = extract_content_from_docs(repo)?;

    // Initialize OpenAI client and model
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let gpt4o = openai.completion_model(GPT_4O);

    // let solidity_auditor_preable = "Your are a world class expert at smart contract auditing, reknown for your ability to find the most complex and trickiest security vulnerabilities in solidity codebases.";

    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let qp = QueryPointsBuilder::new("contract_chunks") // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    let store = QdrantVectorStore::new(qdrant, model, qp);

    // let dynamic_context = vector_index()?;

    let openai_audit_agent = AgentBuilder::new(gpt4o)
        // .preamble(&solidity_auditor_preable)
        .context(&documentation)
        .dynamic_context(3, store)
        .temperature(0.1)
        .build();

    Ok(openai_audit_agent)
}

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
