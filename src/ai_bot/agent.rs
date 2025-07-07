use anyhow::Result;
use log::info;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{
    agent::{Agent, AgentBuilder},
    client::{CompletionClient, EmbeddingsClient},
    providers::openai::{Client, CompletionModel, GPT_4O},
    vector_store::VectorStoreIndex, // trait
};
use rig_qdrant::QdrantVectorStore;

use crate::build_brain::enbeddings::SourceChunk;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_doc_file::extract_content_from_docs;
use crate::utils::logging::print_first_four_lines;

pub fn create_ai_audit_agent(repo: &RepoPaths) -> Result<Agent<CompletionModel>> {
    let documentation = extract_content_from_docs(repo)?;

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

// TODO - restore once token limit increased
pub async fn get_context_for_security_query(
    query_content: &str,
    repo: &RepoPaths,
) -> Result<String> {
    let documentation = extract_content_from_docs(repo)?;

    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

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

    info!("relevant docs => {:#?}", relevant_docs);

    let dynamic_content = relevant_docs
        .iter()
        .map(|(_, _, source_chunk)| source_chunk.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    info!("dynamic content");
    print_first_four_lines(&dynamic_content);

    let final_context = format!(
        "\n ## DOCUMENTATION: \n\n {} \n\n ## ADDITIONAL CONTEXT: \n\n {}",
        documentation, dynamic_content
    );

    Ok(final_context)
}
