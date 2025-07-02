/// The main entry point for the AI Agent Audit tool.
///
/// This application analyzes Solidity smart contracts by:
/// 1. Cloning a repository containing smart contracts
/// 2. Building the contracts with Forge
/// 3. Extracting IR and storage information using Slither
/// 4. Creating embeddings for the source code and analysis results
/// 5. Storing the embeddings in a Qdrant vector database for semantic search
use ai_agent_audit::{
    build_brain::{enrichment, git_clone, vector_db},
    cost::cost_data::get_total_inference_cost,
    enumerator::codeblock_maker,
    llm_review::{
        code_review,
        context_state::{self},
    },
    reporting::{
        audit::{self, ReportType},
        contract_data, save_file,
    },
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

    // save call graph to database
    let semantics_db = enrichment::build_semantics_db_from_call_graph(&repo.root).await?;
    info!("Call-graph DB at {}", semantics_db.display());

    // save metadata context for protocol to global state
    info!("generating metadata context...");
    context_state::generate_and_save_metadata_context(&repo.root, &semantics_db).await?;

    // ────────────────────────────────
    // 3. Static-analysis (Slither detectors)
    info!("generating codeblock for each contract in repo");
    let codeblocks_db = codeblock_maker::generate_and_save_codeblocks_for_each_contract(
        &repo.root,
        &semantics_db,
        MAX_DEPTH,
        TOKEN_BUDGET,
    )
    .await?;
    info!("Slices at {}", codeblocks_db.display());

    vector_db::generate_slither_chucks_and_save_all_metadata_to_vector_db(&repo, &semantics_db)
        .await?;

    let (security_issues, invariants) =
        code_review::review_codebase_for_security_issues(&codeblocks_db).await?;

    let audit_report = audit::generated_audit_report(
        security_issues,
        invariants,
        &repo,
        &semantics_db,
        ReportType::Paid,
    )
    .await?;

    // save audit report, contract IRs, and metadata to md files
    save_file::save_audit_report(&audit_report, &repo)?;
    contract_data::save_contract_and_fn_ir(&codeblocks_db, &repo)?;
    contract_data::save_metadata(&semantics_db, &repo).await?;

    // total cost
    let total_cost = get_total_inference_cost().await;
    info!("Total Inference Cost ===> {}", total_cost);

    Ok(())
}
