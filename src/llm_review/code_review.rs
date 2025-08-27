use crate::error::Result;
use crate::llm_review::code_review_v2::{enhance_codeblock, generate_ai_agents};
use crate::llm_review::invariants::ContractInvariants;
use crate::llm_review::{analysis_db::FindingsDb, context_state::generate_audit_scope};
use crate::prepare_code::git_clone::RepoPaths;
use crate::{enumerator::codeblock_db::CodeBlocksDb, llm_review::findings::Findings};
use log::info;
use std::path::PathBuf;

use super::phases;

/// Multi-LLM security analysis orchestration.
///
/// This module coordinates parallel security analysis across multiple LLM providers,
/// implements verification and deduplication workflows, and manages cost tracking.

/// Orchestrates comprehensive security analysis of smart contracts using multiple LLM providers.
///
/// This function performs parallel vulnerability detection across multiple AI agents,
/// implements verification and deduplication workflows, and returns categorized findings.
///
/// # Arguments
/// * `codeblocks_path` - Path to the database containing generated code blocks
///
/// # Returns
/// * `HashMap<String, Findings>` - Security findings organized by contract
/// * `Vec<ContractInvariants>` - Protocol invariant analysis results
pub async fn review_codebase_for_security_issues(
    codeblocks_path: &PathBuf,
    repo: &RepoPaths,
) -> Result<(Findings, Vec<ContractInvariants>)> {
    let mut all_security_issues = Findings {
        findings: Vec::new(),
    };
    let codeblocks_db = CodeBlocksDb::open(codeblocks_path)?;

    // grab all solidity contracts from database
    info!("grabbing contracts from db...");
    let contracts = codeblocks_db.get_all_contracts(repo)?;
    let audit_scope = generate_audit_scope(repo).await?;

    let (ai_verify_agent, ai_discovery_agents) = generate_ai_agents(repo).await?;

    let invariant_findings = Vec::<ContractInvariants>::new();

    let findings_db = FindingsDb::open()?;

    for (contract, codeblock) in contracts.into_iter() {
        info!("\n\n-------- contract {} ---------------\n\n", contract);

        // grab additional context from RAG
        // let rag_context = get_rag_for_security_query(&codeblock, repo).await?;
        // let audit_context = format!(
        //     "\n## CONTEXT \n\n {} \n\n {}",
        //     metadata_context, rag_context
        // );

        // // Phase 1: Pre-fetch strategic files once to avoid rate limits during parallel analysis
        // let prefetched_files =
        //     phases::prefetch_context::execute(&codeblock, repo, &ai_planning_agent).await?;

        // Combine original context with prefetched files
        // let metadata_context = if prefetched_files.is_empty() {
        //     metadata_context.to_string(Responsese)
        // } else {
        //     format!(
        //         "{}\n\n## Additional Protocol Files for More Context ------------------\n\n{}",
        //         metadata_context, prefetched_files
        //     )
        // };

        // Phase 1 generated enhanced codeblock
        let codeblock = enhance_codeblock(&contract, &codeblock, repo).await?;
        // info!("codeblock => {}", codeblock);

        // // Phase 2: Generate findings using parallel AI agents
        let raw_findings =
            phases::generate_findings::execute(&contract, &codeblock, &ai_discovery_agents, repo)
                .await?;

        if !raw_findings.findings.is_empty() {
            // Phase 3: Verify findings and remove false positives
            let mut verify_findings = raw_findings;
            // for j in 1..=VERIFY_RUNS { //  TOO STRICT?
            // info!("verify findings round {j}....................\n\n");
            verify_findings = phases::verify_findings::execute(
                verify_findings,
                &codeblock,
                &ai_verify_agent,
                repo,
            )
            .await?;
            // }

            let mut in_scope_findings = verify_findings;
            // if no scope provided - all findings in scope !
            // DO NOT use claude for scoping! too many false negatives
            if !audit_scope.is_empty() {
                // Phase 3a: Scope findings and remove out of scope ones
                in_scope_findings = phases::scope_findings::execute(
                    in_scope_findings,
                    &codeblock,
                    &ai_verify_agent,
                    repo,
                )
                .await?;
            }

            // Phase 4: Quality check and enhance findings
            let final_findings = phases::quality_check::execute(
                in_scope_findings,
                &codeblock,
                &ai_verify_agent,
                repo,
            )
            .await?;

            // Save findings to database before extending
            if let Err(e) = findings_db.insert_findings(&final_findings, repo) {
                log::warn!("Failed to save findings to database: {}", e);
            }

            all_security_issues.findings.extend(final_findings.findings);
        }
    }

    // dedup combined findings
    let deduped_security_bugs = all_security_issues.dedup().await?;

    Ok((deduped_security_bugs, invariant_findings))
}
