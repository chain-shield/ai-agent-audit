use ai_agent_audit::build_brain::{
    enbeddings::embed_files, enrichment, intake, slither_ffi, vector_db,
};
// crates/orchestrator/src/main.rs
use anyhow::Result;
use dotenvy::dotenv;
use log::{debug, info};
use qdrant_client::Qdrant;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // ────────────────────────────────
    // 1. Clone & pre-filter the repo
    // ────────────────────────────────
    let repo_url = std::env::args().nth(1).expect("repo url");
    info!("git cloning and extraction source code");
    let repo = intake::clone_and_filter(&repo_url)?;
    debug!("repo paths => {:?}", repo);

    // ────────────────────────────────
    // 2a. Build with Forge (optional, but lets Slither parse correctly)
    // ────────────────────────────────
    info!("compiling source code");
    enrichment::forge_build(&repo.root)?;

    // 2b. create a temp dir and ask slither_ffi to fill it with chunk files
    let tmp_dir = tempfile::tempdir()?;
    info!("generating slither ssa into txt files that contain function or storage var");
    let slither_chunk_paths = slither_ffi::dump_chunks_to_dir(&repo.root, tmp_dir.path())?;
    debug!("slither ssa files => {:?}", slither_chunk_paths);

    // ────────────────────────────────
    // 3. Assemble the *full* file list to embed
    //    – original Solidity + docs  (repo.sol_files  ∪  repo.docs)
    //    – temp IR / storage files   (tmp_paths)
    // ────────────────────────────────
    info!("combine all file locations for solidity + docs, IR fuctions, and storage into 1 vec");
    let mut all_files: Vec<_> = repo
        .sol_files
        .into_iter()
        .chain(repo.docs.into_iter())
        .collect();
    all_files.extend(slither_chunk_paths);

    info!("generating generate vector enbedding");
    let embeddings = embed_files(&all_files).await?;

    // ────────────────────────────────
    // 4. Upsert into Qdrant
    // ────────────────────────────────
    info!("connect to qdrant db");
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?).build()?;
    info!("create contract_chunks vector db (if does not exists");
    vector_db::ensure_collection(&qdrant, "contract_chunks", 1536).await?;
    info!("upsert embeddings");
    vector_db::upsert(&qdrant, "contract_chunks", &embeddings).await?;

    println!("✅ Ingest complete – {} chunks stored", embeddings.len());
    Ok(())
}
