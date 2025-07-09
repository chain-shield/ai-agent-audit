/// The main entry point for the AI Agent Audit tool.
///
/// This application performs comprehensive smart contract security audits by:
/// 1. Cloning and building repositories (Foundry/Hardhat) in Docker containers
/// 2. Extracting call graphs, IR, and storage layouts using Slither
/// 3. Generating contextual code slices for focused AI analysis
/// 4. Running multi-LLM security analysis across 19+ vulnerability categories
/// 5. Creating vector embeddings and storing in Qdrant for semantic search
/// 6. Generating professional audit reports with findings and cost tracking
use ai_agent_audit::{
    build_brain::{enrichment, vector_db},
    cost::cost_data::get_total_inference_cost,
    enumerator::codeblock_maker,
    llm_review::{
        code_review,
        context_state::{self},
    },
    prepare_code,
    reporting::{
        audit::{self, ReportType},
        contract_data, save_file,
    },
    utils::delete_docker_volumes::cleanup_repo_volume,
};
use anyhow::Result;
use dotenvy::dotenv;
use log::info;

/// Maximum call graph traversal depth for code slice generation
const MAX_DEPTH: usize = 3; // depth of 3 is security researcher standard

/// Maximum token budget per code block to stay within LLM context limits
const TOKEN_BUDGET: usize = 150_000;

/// The main async function that orchestrates the entire process.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Initialize the logger
    env_logger::init();

    // ────────────────────────────────
    // 1. Repository Preparation
    // ────────────────────────────────
    let repo_url = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Repository URL is required as first argument"))?;

    // Validate URL format and length before processing
    if repo_url.len() > 2048 {
        anyhow::bail!("Repository URL is too long (max 2048 characters)");
    }

    if !repo_url.starts_with("https://") && !repo_url.starts_with("http://") {
        anyhow::bail!("Only HTTP/HTTPS repository URLs are supported");
    }

    info!("Processing repository: {}", repo_url);
    info!("git cloning and extraction source code");

    // Clone repository in Docker container and build with Foundry/Hardhat
    let repo = prepare_code::git_clone::clone_and_filter_git_repo(&repo_url)?;
    info!("repo root => {:?}", &repo.root);
    info!("repo name => {:?}", &repo.repo_name);

    // ────────────────────────────────
    // 2. Static Analysis & Graph Generation
    // ────────────────────────────────
    // Build semantic database with call graphs and inheritance data
    let semantics_db = enrichment::build_semantics_db_from_call_graph(repo.clone()).await?;
    info!("Call-graph DB at {}", semantics_db.display());

    // Generate and cache protocol metadata context for AI analysis
    info!("generating metadata context...");
    context_state::generate_and_save_metadata_context(&repo, &semantics_db).await?;

    // ────────────────────────────────
    // 3. Code Slice Generation
    // ────────────────────────────────
    info!("generating codeblock for each contract in repo");
    // Create contextual code slices using call graph traversal
    let codeblocks_db = codeblock_maker::generate_and_save_codeblocks_for_each_contract(
        &repo,
        &semantics_db,
        MAX_DEPTH,
        TOKEN_BUDGET,
    )
    .await?;
    info!("Slices at {}", codeblocks_db.display());

    // ────────────────────────────────
    // 4. Vector Database Population
    // ────────────────────────────────
    // Create embeddings and store in Qdrant for semantic search
    vector_db::generate_slither_chucks_and_save_all_metadata_to_vector_db(&repo, &semantics_db)
        .await?;

    // ────────────────────────────────
    // 5. AI Security Analysis
    // ────────────────────────────────
    // Run multi-LLM security analysis across vulnerability categories
    let (security_issues, invariants) =
        code_review::review_codebase_for_security_issues(&codeblocks_db).await?;

    // ────────────────────────────────
    // 6. Report Generation
    // ────────────────────────────────
    // Generate comprehensive audit report (paid version)
    let audit_report = audit::generated_audit_report(
        &security_issues,
        &invariants,
        &repo,
        &semantics_db,
        ReportType::Paid,
    )
    .await?;

    // Generate limited audit report (free version)
    let free_audit_report = audit::generated_audit_report(
        &security_issues,
        &invariants,
        &repo,
        &semantics_db,
        ReportType::Free,
    )
    .await?;

    // ────────────────────────────────
    // 7. File Export & Cleanup
    // ────────────────────────────────
    // Save all reports and analysis data to markdown files
    save_file::save_audit_report(&audit_report, &repo, ReportType::Paid)?;
    save_file::save_audit_report(&free_audit_report, &repo, ReportType::Free)?;
    contract_data::save_contract_and_fn_ir(&codeblocks_db, &repo)?;
    contract_data::save_metadata(&semantics_db, &repo).await?;

    // Display total inference cost across all LLM providers
    let total_cost = get_total_inference_cost().await;
    info!("Total Inference Cost ===> {}", total_cost);

    // Clean up Docker volumes
    cleanup_repo_volume(&repo.root)?;

    Ok(())
}
