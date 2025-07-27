use std::collections::HashSet;

use anyhow::Result;
use log::info;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{client::EmbeddingsClient, providers::openai::Client, vector_store::VectorStoreIndex};
// use rig_qdrant::QdrantVectorStore;  // Temporarily disabled due to version conflicts
/// AI agent implementations with vector-based context retrieval.
///
/// This module provides intelligent AI agents that combine static documentation
/// with dynamic vector search for contextual smart contract analysis.
use tiktoken_rs::cl100k_base;

use crate::build_brain::enbeddings::SourceChunk;
use crate::config::MAX_RAG_QUERY_CONTENT_LENGTH;
use crate::prepare_code::git_clone::RepoPaths;

pub async fn get_rag_for_security_query(_query_content: &str, _repo: &RepoPaths) -> Result<String> {
    log::warn!("🚫 get_rag_for_security_query called but temporarily disabled due to rig-qdrant version conflicts with rig-core 0.13.0");
    Err(anyhow::anyhow!("Vector search functionality temporarily disabled due to rig-qdrant version conflicts with rig-core 0.13.0"))
}
