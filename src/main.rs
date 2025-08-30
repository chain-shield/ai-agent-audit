use ai_agent_audit::{
    // test_rig,
    build_brain::{enrichment, summarize::summarize_src_files, vector_db},
    cli_args::parse,
    config::{audit_config, init_config},
    cost::cost_data::get_total_inference_cost,
    enumerator::codeblock_maker,
    error::Result,
    llm_review::{
        agent_factory::init_llm_clients,
        code_review_v2,
        context_state::{self},
    },
    prepare_code::{self},
    reporting::{
        audit::{self},
        contract_data, save_file,
    },
};
use clap::Parser;
use dotenvy::dotenv;
use log::info;
/// The main entry point for the AI Agent Audit tool.
///
/// This application performs comprehensive smart contract security audits by:
/// 1. Cloning and building repositories (Foundry/Hardhat) in Docker containers
/// 2. Extracting call graphs, IR, and storage layouts using Slither
/// 3. Generating contextual code slices for focused AI analysis
/// 5. Running multi-LLM security analysis across 19+ vulnerability categories
/// 5. Creating vector embeddings and storing in Qdrant for semantic search
/// 6. Generating professional audit reports with findings and cost tracking
/// The main async function that orchestrates the entire process.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize configuration from environment
    init_config()?;

    // Initialize the logger
    env_logger::init();

    // Initialize LLM clients
    init_llm_clients()?;

    // parse command line args
    // Cli struct contains all info we need to execute audit
    let cli = parse::Cli::parse();

    // Exit early for testing
    // println!("🔬 Test completed - exiting early for analysis");
    // return Ok(());

    info!("git cloning and extraction source code");

    // Clone repository in Docker container and build with Foundry/Hardhat
    let repo = prepare_code::git_clone::clone_and_filter_git_repo(&cli)?;
    info!("repo root => {:?}", &repo.root);
    info!("repo name => {:?}", &repo.repo_name);
    info!("repo source folder => {:?}", &repo.source_code_folder);
    info!("repo tests => {:?}", &repo.test_files);
    info!("repo scripts => {:?}", &repo.script_files);
    info!("repo config files => {:?}", &repo.config_files);
    info!("repo docs => {:?}", &repo.docs);
    info!("excluded folders => {:?}", &repo.excluded_folders);

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
        audit_config().max_depth,
        audit_config().token_budget,
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
    let security_issues =
        code_review_v2::review_codebase_for_security_issues_v2(&codeblocks_db, &repo).await?;

    // ────────────────────────────────
    // 6. Report Generation
    // ────────────────────────────────
    // Generate comprehensive audit report (paid version)
    let audit_report =
        audit::generated_audit_report(&security_issues, &repo, audit::ReportType::Pattern).await?;

    // ────────────────────────────────
    // 7. File Export & Cleanup
    // ────────────────────────────────
    // Save all reports and analysis data to markdown files
    save_file::save_audit_report(&audit_report, &repo)?;
    contract_data::save_contract_and_fn_ir(&codeblocks_db, &repo)?;
    contract_data::save_metadata(&repo).await?;

    // Display total inference cost across all LLM providers
    let total_cost = get_total_inference_cost().await;
    info!("Total Inference Cost ===> {}", total_cost);

    // save all repoPaths and context to db
    prepare_code::repo_data::save_repo_data_to_db(&repo).await?;
    // Clean up Docker volumes
    // cleanup_repo_volume(&repo.root)?;

    Ok(())
}
