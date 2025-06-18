/// The main entry point for the AI Agent Audit tool.
///
/// This application analyzes Solidity smart contracts by:
/// 1. Cloning a repository containing smart contracts
/// 2. Building the contracts with Forge
/// 3. Extracting IR and storage information using Slither
/// 4. Creating embeddings for the source code and analysis results
/// 5. Storing the embeddings in a Qdrant vector database for semantic search
use ai_agent_audit::{
    build_brain::{enrichment, git_clone, slither_ffi, vector_db},
    enumerator::codeblock_maker,
    llm_review::code_review,
    reporting::{audit, contract_data, save_file},
};
use anyhow::Result;
use dotenvy::dotenv;
use log::info;

const MAX_DEPTH: usize = 3; // depth of 3 is security researcher standard
const TOKEN_BUDGET: usize = 150_000;

/// The main async function that orchestrates the entire process.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Initialize the logger
    env_logger::init();

    // ────────────────────────────────
    // 1. Clone & pre-filter the repo
    // ────────────────────────────────
    // Get the repository URL from the command line arguments
    let repo_url = std::env::args().nth(1).expect("repo url");
    info!("git cloning and extraction source code");
    // Clone the repository and filter out irrelevant files
    let repo = git_clone::clone_and_filter_git_repo(&repo_url)?;
    info!("repo paths => {:?}", repo);

    // ────────────────────────────────
    // 2a. Build with Forge (optional, but lets Slither parse correctly)
    // ────────────────────────────────
    info!("compiling source code");
    // Build the Solidity contracts using Forge
    enrichment::forge_build(&repo.root)?;

    // let prompt = generate_abridged_slither_metadata_prompt_context(&repo.root).await?;

    // save call graph to database
    let semantic_db = enrichment::build_semantics_db_from_call_graph(&repo.root).await?;
    info!("Call-graph DB at {}", semantic_db.display());

    // 2b. Create a temp dir and ask slither_ffi to fill it with chunk files
    // Create a temporary directory to store the Slither analysis results
    let tmp_dir = tempfile::tempdir()?;
    info!("generating slither ssa into txt files that contain function or storage var");

    // TODO - UNPAUSE AFTER DONE TESTING
    // Extract IR and storage information using Slither and write to text files
    let slither_chunk_paths = slither_ffi::save_code_metadata_and_analysis_to_txt_files(
        &repo.root,
        tmp_dir.path(),
        &semantic_db,
    )
    .await?;
    info!("slither ssa file count => {}", slither_chunk_paths.len());

    // ────────────────────────────────
    // 3. Static-analysis (Slither detectors)
    info!("generating codeblock for each contract in repo");
    let codeblocks_db = codeblock_maker::generate_and_save_codeblocks_for_each_contract(
        &repo.root,
        &semantic_db,
        MAX_DEPTH,
        TOKEN_BUDGET,
    )
    .await?;
    info!("Slices at {}", codeblocks_db.display());

    // ────────────────────────────────
    // 4. Assemble the *full* file list to embed
    //    – original Solidity + docs  (repo.sol_files  ∪  repo.docs)
    //    – temp IR / storage files   (tmp_paths)
    // ────────────────────────────────
    info!("combine all file locations for solidity + docs, IR functions, and storage into 1 vec");
    // Combine all file paths into a single vector:
    // - Solidity source files
    // - Documentation files
    // - Slither analysis result files
    // TODO - UNPAUSE AFTER DONE TESTING
    let mut all_files: Vec<_> = repo.docs.clone().into_iter().collect();
    all_files.extend(slither_chunk_paths);
    info!("all files => {:?}", all_files.len());

    //embed all files and upsert to qdrant vector db for later dynamic retrival
    // TODO - UNPAUSE AFTER DONE TESTING
    vector_db::generate_enbeddings_and_save_to_qdrant_vector_db(&all_files, &repo).await?;

    let (security_issues, invariants) =
        code_review::review_codebase_for_security_issues(&repo.root, &codeblocks_db, &semantic_db)
            .await?;

    let audit_report =
        audit::generated_audit_report(security_issues, invariants, &repo, &semantic_db).await?;

    // save audit report, contract IRs, and metadata to md files
    save_file::save_audit_report(&audit_report, &repo)?;
    contract_data::save_contract_and_fn_ir(&codeblocks_db, &repo)?;

    Ok(())
}
