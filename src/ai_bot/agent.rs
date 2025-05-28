use anyhow::Result;
use qdrant_client::{qdrant::QueryPointsBuilder, Qdrant};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{
    agent::{Agent, AgentBuilder},
    providers::openai::{Client, CompletionModel, GPT_4O},
    vector_store::VectorStoreIndex, // trait
};
use rig_qdrant::QdrantVectorStore;
use std::path::PathBuf;

use crate::utils::{get_doc_file::extract_content_from_docs, vec_db_connect::vector_index};

pub fn create_ai_audit_agent(repo_root: PathBuf) -> Result<Agent<CompletionModel>> {
    let documentation = extract_content_from_docs(&repo_root)?;

    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);

    let gpt4o = openai.completion_model(GPT_4O);

    let solidity_auditor_preable = "Your are a world class expert at smart contract auditing, reknown for your ability to find the most complex and trickiest security vulnerabilities in solidity codebases.";

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
        .preamble(&solidity_auditor_preable)
        .context(&documentation)
        .dynamic_context(3, store)
        .temperature(0.1)
        .build();

    Ok(openai_audit_agent)
}
