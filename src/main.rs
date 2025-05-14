use ai_agent_audit::build_brain::{enbeddings::embed_files, enrichment, intake, vector_db};
// crates/orchestrator/src/main.rs
use anyhow::Result;
use dotenvy::dotenv;
use qdrant_client::Qdrant;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // 1. clone repo (pass in via CLI arg or env)
    let repo_url = std::env::args().nth(1).expect("repo url");
    let repo = intake::clone_and_filter(&repo_url)?;

    // 2. (optional) compile for metadata
    enrichment::forge_build(&repo.root)?;

    // 3. embed Solidity + docs
    let files: Vec<_> = repo
        .sol_files
        .into_iter()
        .chain(repo.docs.into_iter())
        .collect();
    let embeddings = embed_files(&files).await?;

    // 4. upsert to Qdrant
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?).build()?;
    vector_db::ensure_collection(&qdrant, "contract_chunks", 1536).await?; // OpenAI 3-small dim
    vector_db::upsert(&qdrant, "contract_chunks", &embeddings).await?;

    println!("✅ Ingest complete – {} chunks stored", embeddings.len());
    Ok(())
}
