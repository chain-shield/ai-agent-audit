use ai_agent_audit::{
    config::init_config,
    llm_review::{
        agent_factory::{AgentConfig, AgentFactory, init_llm_clients},
        enums::{Severity, VulnerabilityType},
        findings::{Finding, Findings, PrivilegeLevel},
        phases::add_poc_findings::{self, PocStatus},
    },
    prepare_code::git_clone::{PocConfig, RepoPaths},
};
use dotenvy::dotenv;
use std::{env, fs, path::PathBuf, sync::Arc};

/// Returns true if Anthropic API key is present in the environment
fn anthropic_key_present() -> bool {
    env::var("ANTHROPIC_API_KEY")
        .ok()
        .filter(|v| !v.is_empty())
        .is_some()
}

/// Integration test for PoC retry loop using real puppy-raffle findings
///
/// This test specifically focuses on testing the retry loop to understand
/// why the LLM isn't fixing simple compilation errors like invalid hex literals.
///
/// Uses:
/// - Real findings from 4-puppy-raffle-audit/4-puppy-raffle-audit-3ff0f0-audit-report.md
/// - Real contract code from 4-puppy-raffle-audit/PuppyRaffle-token-count-2479.md
/// - Real metadata from 4-puppy-raffle-audit/metadata-4-puppy-raffle-audit-3ff0f0.md
/// - Existing Docker volume at /private/tmp/audit-analysis/4-puppy-raffle-audit-3ff0f0
#[tokio::test]
async fn test_poc_retry_loop_with_real_findings() {
    // Load env
    dotenv().ok();

    // Skip if no Anthropic key present
    if !anthropic_key_present() {
        eprintln!(
            "⚠️  Skipping test_poc_retry_loop_with_real_findings - no ANTHROPIC_API_KEY found"
        );
        return;
    }

    // Initialize logger
    let _ = env_logger::try_init();

    // Initialize configuration from environment
    init_config().expect("init_config() should succeed when Anthropic API key is present");

    // Initialize LLM clients
    init_llm_clients().expect("init_llm_clients() should succeed");

    // Load real contract code from file
    let contract_code = load_contract_code();
    println!("📄 Loaded contract code ({} chars)", contract_code.len());

    // Create findings from the audit report
    let findings = create_findings_from_audit_report();
    println!(
        "📋 Created {} findings from audit report",
        findings.findings.len()
    );

    // Setup repo paths pointing to existing Docker volume
    let repo = create_repo_paths_for_existing_volume();
    println!("📁 Repo root: {}", repo.root.display());
    println!("📝 PoC instructions: {}", repo.poc.instructions);

    // Verify the Docker volume exists
    if !repo.root.exists() {
        panic!(
            "❌ Docker volume does not exist at: {}\nPlease ensure the puppy-raffle audit has been run first.",
            repo.root.display()
        );
    }

    // Create Claude agent for PoC generation
    let agent_config = AgentConfig::new(Some(repo.clone()))
        .with_model("claude-3-7-sonnet-20250219")
        .with_temperature(0.3);

    let agent = Arc::new(
        AgentFactory::create_anthropic_agent(&agent_config).expect("Failed to build Claude agent"),
    );

    println!("🤖 Using Claude 3.7 Sonnet for PoC generation");
    println!("🚀 Starting PoC retry loop test...\n");

    // Execute the PoC generation phase
    let result = add_poc_findings::execute(findings, &contract_code, &agent, &repo).await;

    match result {
        Ok(findings_with_pocs) => {
            println!("\n✅ PoC generation workflow completed!");
            println!("📊 Detailed Results:\n");

            for (i, finding) in findings_with_pocs.findings.iter().enumerate() {
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("Finding #{}: {}", i + 1, finding.title);
                println!("  Severity: {:?}", finding.severity);
                println!("  Exploit Type: {:?}", finding.exploit_type);
                println!("  Contract: {}", finding.contract);
                println!("  Function: {}", finding.function);

                if let Some(status) = &finding.poc_test_status {
                    println!("  PoC Status: {:?}", status);

                    match status {
                        PocStatus::AllTestPass => println!("    ✅ All tests passing!"),
                        PocStatus::FailingTests => {
                            println!("    ⚠️  Tests failing after 5 attempts")
                        }
                        PocStatus::ErrorRunningTests => println!("    ❌ Error running tests"),
                        PocStatus::FindingIsInvalid => {
                            println!("    ℹ️  Finding marked as invalid")
                        }
                    }
                }

                if let Some(file) = &finding.poc_test_file {
                    println!("  PoC File: {}", file.display());

                    // Show the generated PoC code
                    if file.exists() {
                        if let Ok(poc_code) = fs::read_to_string(file) {
                            println!("\n  Generated PoC Code (first 30 lines):");
                            for (line_num, line) in poc_code.lines().take(30).enumerate() {
                                println!("    {:3} | {}", line_num + 1, line);
                            }
                        }
                    }
                }

                if let Some(cmd) = &finding.poc_test_command {
                    println!("\n  PoC Command: {}", cmd);
                }

                println!();
            }

            // Analyze results
            let total = findings_with_pocs.findings.len();
            let passing = findings_with_pocs
                .findings
                .iter()
                .filter(|f| matches!(f.poc_test_status, Some(PocStatus::AllTestPass)))
                .count();
            let failing = findings_with_pocs
                .findings
                .iter()
                .filter(|f| matches!(f.poc_test_status, Some(PocStatus::FailingTests)))
                .count();
            let errors = findings_with_pocs
                .findings
                .iter()
                .filter(|f| matches!(f.poc_test_status, Some(PocStatus::ErrorRunningTests)))
                .count();

            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📊 Summary:");
            println!("  Total findings: {}", total);
            println!("  ✅ Passing PoCs: {}", passing);
            println!("  ⚠️  Failing PoCs: {}", failing);
            println!("  ❌ Error PoCs: {}", errors);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

            // The test passes as long as the workflow doesn't crash
            // We're investigating WHY the LLM can't fix simple errors, not asserting success
            println!("✅ Test passed - workflow completed without crashing");
            println!("ℹ️  Note: This test is for investigation, not assertion of PoC success");
        }
        Err(e) => {
            panic!("❌ PoC generation workflow failed: {:?}", e);
        }
    }
}

/// Load the contract code from the markdown file
fn load_contract_code() -> String {
    let contract_file = PathBuf::from("4-puppy-raffle-audit/PuppyRaffle-token-count-2479.md");

    if !contract_file.exists() {
        panic!(
            "❌ Contract code file not found: {}\nPlease ensure the file exists in the workspace.",
            contract_file.display()
        );
    }

    fs::read_to_string(&contract_file)
        .unwrap_or_else(|e| panic!("Failed to read contract code file: {}", e))
}

/// Create findings from the audit report
fn create_findings_from_audit_report() -> Findings {
    // Create the two Medium findings from the audit report
    let finding1 = Finding {
        title: "Unbounded O(n^2) duplicate scan in PuppyRaffle.enterRaffle enables gas-based DoS of new raffle entries".to_string(),
        description: Some("enterRaffle pushes new players then validates no duplicates by scanning the entire players array with a nested loop, giving O(n^2) complexity over unbounded, user-inflated state. Any EOA can bloat players by adding many unique addresses. Once players grows near the per-tx block gas limit, any subsequent enterRaffle call will run out of gas during the duplicate scan, permanently DoSing further entries until the round is reset.".to_string()),
        severity: Severity::Medium,
        exploit_type: VulnerabilityType::GasGriefBlockLimit,
        contract: "PuppyRaffle".to_string(),
        function: "enterRaffle".to_string(),
        privilege: PrivilegeLevel::Permissionless,
        impact: Some("A permissionless attacker can bloat players[] with unique addresses and cause future enterRaffle calls to run out of gas due to the O(n^2) duplicate scan. This prevents any new entries from joining until the round resets in selectWinner(), halting prize pool growth and fee accrual.".to_string()),
        proof_of_concept: Some("1) Attacker repeatedly calls enterRaffle with batches of unique addresses they control, paying entranceFee per address, to grow players[]. 2) Since duplicate checking scans all pairs in players[], the gas cost grows O(n^2). 3) Once players[] is large enough that enterRaffle exceeds a typical gas cap, all further entry attempts revert OOG with similar gas limits, DoSing new entries until the raffle is reset.".to_string()),
        mitigation: Some("Replace the global O(n^2) duplicate scan with O(1) membership checks via a mapping and only check duplicates within the submitted batch.".to_string()),
        ..Default::default()
    };

    let finding2 = Finding {
        title: "Rounding dust from 80/20 split in PuppyRaffle.selectWinner DoSes withdrawFees and permanently locks protocol fees".to_string(),
        description: Some("selectWinner splits the pot using two separate integer divisions: (total*80)/100 and (total*20)/100. When totalAmountCollected is not divisible by 5, floor rounding creates a 1 wei dust remainder such that prizePool + fee < total. That dust stays on the contract, while totalFees tracks only the rounded-down fee. withdrawFees then requires address(this).balance == totalFees, so any dust makes this strict equality fail forever, bricking fee withdrawals and locking all accrued fees.".to_string()),
        severity: Severity::Medium,
        exploit_type: VulnerabilityType::RoundingError,
        contract: "PuppyRaffle".to_string(),
        function: "selectWinner".to_string(),
        privilege: PrivilegeLevel::Permissionless,
        impact: Some("Permanent DoS of fee withdrawal; protocol fees become irretrievably stuck as long as any rounding dust exists. A single round with non-multiple-of-5 totals bricks withdrawFees forever, locking all accrued fees.".to_string()),
        proof_of_concept: Some("1) Deploy PuppyRaffle with an entranceFee not divisible by 5 (e.g., 1 wei) and a short raffleDuration. 2) An attacker enters exactly 4 unique players paying 4 wei total. 3) After duration, anyone calls selectWinner. Prize = floor(4*80/100)=3, fee=floor(4*20/100)=0, leaving 1 wei dust on the contract. 4) totalFees increases by 0, but address(this).balance is 1, so withdrawFees reverts due to strict equality check.".to_string()),
        mitigation: Some("Use a single division to calculate one value and derive the other by subtraction to ensure prizePool + fee == total.".to_string()),
        ..Default::default()
    };

    Findings {
        findings: vec![finding1, finding2],
    }
}

/// Create repo paths pointing to the existing Docker volume
fn create_repo_paths_for_existing_volume() -> RepoPaths {
    let root = PathBuf::from("/private/tmp/audit-analysis/4-puppy-raffle-audit-3ff0f0");
    let repo_name = "4-puppy-raffle-audit".to_string();
    let test_folder = root.join(&repo_name).join("test");

    RepoPaths {
        project_id: "4-puppy-raffle-audit-3ff0f0".to_string(),
        root: root.clone(),
        repo_name: repo_name.clone(),
        source_code_folders: vec![root.join(&repo_name)],
        sol_files: vec![],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        docs: vec![],
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960".to_string(),
        poc: PocConfig {
            instructions: "Use Foundry/Forge for testing. Import from 'forge-std/Test.sol' for Foundry tests or 'ds-test/test.sol' for ds-test. The contract uses Solidity 0.7.6. Test folder is 'test/'.".to_string(),
            template: "forge-std".to_string(),
            test_folder,
        },
    }
}
