/// Integration test for the verification rounds system using Puppy Raffle findings
use ai_agent_audit::{
    config::AuditType,
    error::Result,
    llm_review::{
        agent::agent_factory::{AgentConfig, AgentFactory, LlmProvider},
        analysis::analysis_db::FindingsDb,
        phases::verify_rounds,
    },
    prepare_code::git_clone::RepoPaths,
};
use rig::providers::anthropic::CLAUDE_4_SONNET;
use std::{path::PathBuf, sync::Arc};

#[tokio::test]
#[ignore = "requires live Anthropic credentials and prepared local findings/code artifacts"]
async fn test_verify_rounds_puppy_raffle() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init()
        .ok();

    println!("\n🧪 Starting Verification Rounds Integration Test with Puppy Raffle\n");

    // Load and initialize configuration
    ai_agent_audit::config::init_config()?;

    // Initialize LLM clients
    ai_agent_audit::llm_review::agent::agent_factory::init_llm_clients()?;

    // Create minimal RepoPaths for testing
    let project_id = "4-puppy-raffle-audit-3ff0f0"; // From findings database
    println!("🔑 Project ID: {}", project_id);

    let repo = RepoPaths {
        github_url: "https://github.com/Cyfrin/4-puppy-raffle-audit".to_string(),
        project_id: project_id.to_string(),
        root: PathBuf::from("4-puppy-raffle-audit"),
        sol_files: vec![],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![],
        docs: vec![],
        repo_name: "4-puppy-raffle-audit".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "2a47715b30cf11ca82db148704e67652ad679cd8".to_string(),
        audit_type: AuditType::Code4rena,
    };

    println!("📂 Repository: {}", repo.repo_name);

    // Load findings from database
    println!("\n📊 Loading findings from database...");
    let findings_db = FindingsDb::open()?;
    let findings = findings_db.get_findings_by_project(&repo)?;

    println!(
        "✅ Loaded {} findings from database",
        findings.findings.len()
    );

    if findings.findings.is_empty() {
        println!(
            "⚠️  No findings found in database for project {}",
            repo.project_id
        );
        println!("   Make sure you've run the audit first to populate findings.db");
        return Ok(());
    }

    // Display findings summary
    println!("\n📋 Findings Summary:");
    for (i, finding) in findings.findings.iter().enumerate() {
        println!(
            "  {}. [{}] {} - {}::{}",
            i + 1,
            finding.severity,
            finding.title,
            finding.contract,
            finding.function
        );
    }

    // Load code and metadata
    println!("\n📖 Loading code and metadata...");
    // The metadata file is named: metadata-4-puppy-raffle-audit-3ff0f0.md
    let metadata_path = repo.root.join(format!("metadata-{}.md", repo.project_id));
    let metadata = std::fs::read_to_string(&metadata_path).unwrap_or_else(|_| {
        println!(
            "⚠️  Could not load metadata from {}",
            metadata_path.display()
        );
        String::new()
    });

    // Load main contract code
    let contract_path = repo
        .root
        .join("Contract-PuppyRaffle-NFTCollection-size-3700.md");
    let code = std::fs::read_to_string(&contract_path).unwrap_or_else(|_| {
        println!(
            "⚠️  Could not load contract from {}",
            contract_path.display()
        );
        String::new()
    });

    println!("✅ Loaded {} chars of code", code.len());
    println!("✅ Loaded {} chars of metadata", metadata.len());

    // Combine code and metadata for verification (instead of using context cache)
    let code_with_context = format!("{}\n\n# METADATA\n\n{}", code, metadata);

    // Create AI agent (Claude Sonnet 4.0)
    println!("\n🤖 Initializing Claude Sonnet 4.0 agent...");
    let agent_config = AgentConfig::new(Some(repo.clone()))
        .with_model(CLAUDE_4_SONNET)
        .with_temperature(1.0)
        .with_max_tokens(16_000); // Claude Sonnet 4.0 max is 64k, using 16k for safety

    let agent = Arc::new(AgentFactory::create_agent(
        LlmProvider::Anthropic,
        &agent_config,
    )?);
    println!("✅ Agent initialized");

    // Run verification rounds
    println!("\n🔍 Starting Verification Rounds...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let verified_findings =
        verify_rounds::execute_rounds(findings, &code_with_context, &agent, &repo).await?;

    // Display results
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 VERIFICATION RESULTS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut low_severity_count = 0;

    for (i, finding) in verified_findings.findings.iter().enumerate() {
        println!("{}. {}", i + 1, finding.title);
        println!("   Contract: {}::{}", finding.contract, finding.function);
        println!("   Severity: {}", finding.severity);

        if let Some(status) = &finding.status {
            let status_str: Vec<String> = status.iter().map(|s| s.to_string()).collect();
            println!("   Status: {}", status_str.join(", "));

            if status.iter().any(|s| matches!(s, ai_agent_audit::llm_review::phases::verify_rounds::FindingStatus::Valid)) {
                valid_count += 1;
            } else if status.iter().any(|s| matches!(s,
                ai_agent_audit::llm_review::phases::verify_rounds::FindingStatus::LowSeverityDueToLowImpact |
                ai_agent_audit::llm_review::phases::verify_rounds::FindingStatus::LowSeverityDueToRareLikelihood
            )) {
                low_severity_count += 1;
            } else {
                invalid_count += 1;
            }
        } else {
            println!("   Status: Valid (no issues found)");
            valid_count += 1;
        }

        if let Some(justification) = &finding.status_justification {
            println!("   Justification:\n{}", justification);
        }
        println!();
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Valid Findings: {}", valid_count);
    println!("⚠️  Low Severity: {}", low_severity_count);
    println!("❌ Invalid Findings: {}", invalid_count);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("✅ Integration test completed successfully!\n");

    Ok(())
}
