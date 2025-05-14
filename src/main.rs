use ai_agent_audit::build_brain::{
    enbeddings::embed_files, enrichment, intake, slither_ffi, vector_db,
};
// crates/orchestrator/src/main.rs
use anyhow::Result;
use dotenvy::dotenv;
use qdrant_client::Qdrant;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // ────────────────────────────────
    // 1. Clone & pre-filter the repo
    // ────────────────────────────────
    let repo_url = std::env::args().nth(1).expect("repo url");
    let repo = intake::clone_and_filter(&repo_url)?;

    // ────────────────────────────────
    // 2a. Build with Forge (optional, but lets Slither parse correctly)
    // ────────────────────────────────
    enrichment::forge_build(&repo.root)?;

    // 2b. create a temp dir and ask slither_ffi to fill it with chunk files
    let tmp_dir = tempfile::tempdir()?;
    let slither_chunk_paths = slither_ffi::dump_chunks_to_dir(&repo.root, tmp_dir.path())?;

    // ────────────────────────────────
    // 3. Assemble the *full* file list to embed
    //    – original Solidity + docs  (repo.sol_files  ∪  repo.docs)
    //    – temp IR / storage files   (tmp_paths)
    // ────────────────────────────────
    let mut all_files: Vec<_> = repo
        .sol_files
        .into_iter()
        .chain(repo.docs.into_iter())
        .collect();
    all_files.extend(slither_chunk_paths);

    let embeddings = embed_files(&all_files).await?;

    // ────────────────────────────────
    // 4. Upsert into Qdrant
    // ────────────────────────────────
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?).build()?;
    vector_db::ensure_collection(&qdrant, "contract_chunks", 1536).await?;
    vector_db::upsert(&qdrant, "contract_chunks", &embeddings).await?;

    println!("✅ Ingest complete – {} chunks stored", embeddings.len());
    Ok(())
}
