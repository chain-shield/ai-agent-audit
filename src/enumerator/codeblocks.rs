use crate::build_brain::callgraph;
use crate::build_brain::graph_db::SmartContractFunction;
use crate::cost::cost_data::get_token_count;
/// Intelligent code slicing for focused AI analysis.
///
/// This module generates contextual code blocks by traversing call graphs and
/// assembling relevant code, IR, and storage information within token budgets
/// for optimal LLM analysis.
use crate::enumerator::codeblock_cache::{get_cached_codeblock, set_codeblock_cache};
use crate::enumerator::codeblock_db::MarkdownCodeblock;
use crate::enumerator::extract_ir::robust_extract_fn_metadata_from_func_id;
use crate::enumerator::parse_solidity::{
    detect_scripts_connected_to_contract, detect_source_code_dependencies,
    is_standard_interface_name, is_standard_library_contract_name,
};
use crate::enumerator::utils::{
    get_hashmap_of_contract_to_functions, get_token_count_of_function_ir,
};
use crate::llm_review::contract_file_map::get_file_from_contract;
use crate::llm_review::utils::contract_in_scope::is_contract_in_scope;
use crate::prepare_code::git_clone::RepoPaths;
use tokio::fs;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

/// Generates contextual code blocks for each contract using call graph traversal.
///
/// This function creates focused code slices by:
/// 1. Checking cache to avoid redundant processing
/// 2. Performing breadth-first search through call graphs up to max_depth
/// 3. Respecting token budgets for LLM context limits
/// 4. Assembling markdown with storage layouts and SlithIR representations
/// 5. Caching results for efficient reprocessing
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantic_db` - Database containing call graph and function data
/// * `codeblock_db` - Database for storing generated code blocks
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `Result<()>` - Success or error
pub async fn generate_codeblock_from_codebase(
    repo: &RepoPaths,
    semantic_db: &Connection,
    codeblock_db: &CodeBlocksDb,
    max_depth: usize,
    token_budget: usize,
) -> Result<()> {
    log::info!("getting contract to func mapping");
    let contract_to_func_map = get_hashmap_of_contract_to_functions(repo, semantic_db).await?;

    for (main_contract, functions_of_contract) in contract_to_func_map {
        // check contract in inscope!
        if !is_contract_in_scope(&main_contract, repo).await? {
            continue;
        }

        // Check if codeblock already generated
        log::info!("contract => {:#?}", main_contract);
        log::info!("fn count of contract => {:#?}", functions_of_contract.len());
        if let Some(_) = get_cached_codeblock(&main_contract).await {
            // Save seed-to-codeblock mapping in the database
            continue;
        };
        // 2. BFS until depth / token budget
        let mut frontier: VecDeque<(SmartContractFunction, usize)> = VecDeque::new();
        for func in functions_of_contract {
            frontier.push_back((func, 0_usize))
        }
        let mut visited = HashSet::new();
        let mut contracts = HashSet::new();
        let mut contracts_with_depth = HashSet::new();
        let mut token_count = 0_usize;

        while let Some((func, depth)) = frontier.pop_front() {
            if !visited.insert(func.id.clone()) {
                continue;
            }

            //keep track of unique contract traversed in BPS
            if !is_standard_interface_name(&func.contract)
                && !is_standard_library_contract_name(&func.contract)
            {
                contracts.insert(func.contract.clone());
                if depth > 0 && depth <= 2 {
                    contracts_with_depth.insert(func.contract.clone());
                }
            }

            // get token count of new fn + IR + storage
            let token_count_fn_ir_storage = get_token_count_of_function_ir(&func, repo).await?;
            // info!("token_count_fn_ir_storage => {}", token_count_fn_ir_storage);

            // check budget, make sure not exceeding token context window
            if token_count + token_count_fn_ir_storage > token_budget {
                break; // budget exhausted
            }

            // update token count
            token_count += token_count_fn_ir_storage;

            // info!("token_count => {}", token_count);
            if depth < max_depth {
                let mut statement = semantic_db
                    .prepare("SELECT callee FROM edges WHERE caller = ?1 AND project_id = ?2;")?;
                let rows =
                    statement.query_map([&func.id, &repo.project_id], |r| r.get::<_, String>(0))?;
                for callee in rows.flatten() {
                    if let Some(callee_fn) = robust_extract_fn_metadata_from_func_id(
                        &callee,
                        &func,
                        &main_contract,
                        semantic_db,
                        repo,
                    )? {
                        frontier.push_back((callee_fn, depth + 1));
                    }
                }
            }
        }

        // -- 2. get collection of all inherited and called contracts
        let mut contracts_with_parents = HashSet::new();

        // Add parents of main contract (up to 1 level)
        let parents_of_main = callgraph::get_parents(&main_contract, repo).await?;
        for parent in &parents_of_main {
            if !is_standard_interface_name(parent) && !is_standard_library_contract_name(parent) {
                contracts_with_parents.insert(parent.clone());
            }
        }

        // Add called contracts and their parents
        for contract in &contracts {
            // Skip if it's the main contract (already added above)
            if contract == &main_contract {
                continue;
            }

            // Add direct parents (1 level up) for called contracts
            let parents = callgraph::get_parents(contract, repo).await?;
            for parent in &parents {
                // info!("inserting {} (parent of {})", parent, contract);
                if !is_standard_interface_name(parent) && !is_standard_library_contract_name(parent)
                {
                    contracts_with_parents.insert(parent.clone());
                }
            }
        }

        // Detect contracts and interfaces from source code that Slither's call graph misses
        // This includes: constructor calls, interface casts, imports, type declarations
        // We analyze BOTH the main contract AND all called contracts for comprehensive coverage

        // First, analyze the main contract
        let mut main_source_contracts =
            detect_source_code_dependencies(&main_contract, repo).await?;

        info!(
            "🔍 DEBUG: Source dependencies for main contract '{}': {} contracts",
            main_contract,
            main_source_contracts.len()
        );
        info!(
            "🔍 DEBUG: contracts_with_depth has {} contracts",
            contracts_with_depth.len()
        );

        for contract in &contracts_with_depth {
            let source = detect_source_code_dependencies(contract, repo).await?;
            info!(
                "🔍 DEBUG: Source dependencies for '{}': {} contracts",
                contract,
                source.len()
            );
            main_source_contracts.extend(source);
        }

        info!(
            "🔍 DEBUG: Total source dependencies after analyzing depth contracts: {}",
            main_source_contracts.len()
        );

        for contract_name in &main_source_contracts {
            info!(
                "adding source-detected contract {} into main and called contract",
                contract_name,
            );
            contracts.insert(contract_name.clone());
        }

        // ── 3.  Assemble final Markdown body with TOKEN BUDGET ENFORCEMENT ────────────────────────────
        let mut markdown_codeblock_for_llm = String::new();
        let mut current_token_count = 0_usize;

        // Add main contract code (CRITICAL - always include)
        let main_contract_code = get_contract_file_content(&main_contract, repo).await?;
        let main_section = format!(
            "\n## *MAIN TARGET CONTRACT* TO REVIEW\n\n{}",
            main_contract_code
        );
        let main_tokens = get_token_count(&main_section);

        info!(
            "✅ Adding 'Main Contract: {}' ({} tokens) - CRITICAL",
            main_contract, main_tokens
        );
        markdown_codeblock_for_llm.push_str(&main_section);
        current_token_count += main_tokens;

        if current_token_count > token_budget {
            info!(
                "⚠️ Main contract alone ({} tokens) exceeds budget ({} tokens)",
                current_token_count, token_budget
            );
        }
        markdown_codeblock_for_llm.push_str("\nEND OF MAIN TARGET CONTRACT\n");

        // Add parent and called contracts (prioritized by importance)
        let supporting_header = "\n## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES\n";
        markdown_codeblock_for_llm.push_str(supporting_header);
        current_token_count += get_token_count(supporting_header);

        // Prioritize contracts by importance:
        // 1. HIGH: Called contracts (user flow, attack surface)
        // 2. MEDIUM: Parent contracts (standard libraries)
        let mut prioritized_contracts = Vec::new();

        // Priority 1: Add called contracts (HIGH PRIORITY - user flow, attack surface)
        for contract in &contracts {
            if *contract != main_contract {
                prioritized_contracts.push((contract.clone(), "called"));
            }
        }

        // Priority 2: Add parents (MEDIUM PRIORITY - usually standard libraries)
        for parent in &contracts_with_parents {
            if *parent != main_contract && !contracts.contains(parent) {
                prioritized_contracts.push((parent.clone(), "parent"));
            }
        }

        // DEBUG: Log what contracts were detected
        info!("🔍 DEBUG: Total contracts detected: {}", contracts.len());
        info!("🔍 DEBUG: contracts = {:?}", contracts);
        info!(
            "🔍 DEBUG: contracts_with_parents = {:?}",
            contracts_with_parents
        );
        info!(
            "🔍 DEBUG: contracts_with_depth = {:?}",
            contracts_with_depth
        );

        // Process contracts in priority order (called → parents)
        // The prioritized_contracts vector is already ordered correctly from above
        let mut contracts_added = 0;
        let mut contracts_skipped_no_file = 0;
        let mut contracts_skipped_too_small = 0;
        let mut contracts_skipped_budget = 0;

        for (contract, contract_type) in prioritized_contracts {
            let contract_code = get_contract_file_content(&contract, repo).await?;
            // Skip if no content (could not resolve file)
            if contract_code.trim().is_empty() {
                info!(
                    "⏭️ Skipping '{} contract: {}' - no file or empty content",
                    contract_type, contract
                );
                contracts_skipped_no_file += 1;
                continue;
            }

            let contract_section = format!("{}\n", contract_code);
            let section_tokens = get_token_count(&contract_section);

            // Enforce minimum section size to avoid 1-token noise
            if section_tokens < 5 {
                info!(
                    "⏭️ Skipping '{} contract: {}' ({} tokens) - below minimum (5 tokens)",
                    contract_type, contract, section_tokens
                );
                contracts_skipped_too_small += 1;
                continue;
            }

            let new_total = current_token_count + section_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping '{} contract: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    contract_type, contract, section_tokens, new_total, token_budget
                );
                contracts_skipped_budget += 1;
            } else {
                info!(
                    "✅ Adding '{} contract: {}' ({} tokens) - total: {}/{} tokens",
                    contract_type, contract, section_tokens, new_total, token_budget
                );
                markdown_codeblock_for_llm.push_str(&contract_section);
                current_token_count = new_total;
                contracts_added += 1;
            }
        }

        info!(
            "📊 Supporting contracts: {} added, {} skipped (no file: {}, too small: {}, budget: {})",
            contracts_added,
            contracts_skipped_no_file + contracts_skipped_too_small + contracts_skipped_budget,
            contracts_skipped_no_file,
            contracts_skipped_too_small,
            contracts_skipped_budget
        );

        markdown_codeblock_for_llm.push_str("\nEND OF SUPPORTING CONTRACTS AND INTERFACES\n\n");
        markdown_codeblock_for_llm.push_str("\nDEPLOYMENT SCRIPTS\n\n");

        // Add relevant deploy scripts
        let contract_scripts = detect_scripts_connected_to_contract(&main_contract, repo).await?;

        for script in &contract_scripts {
            let script_content = match fs::read_to_string(script).await {
                Ok(content) => content,
                Err(e) => {
                    info!(
                        "could not read file {} for dependency detection: {}",
                        script.display(),
                        e
                    );
                    continue;
                }
            };

            let script_section = format!("{}\n", script_content);
            let script_tokens = get_token_count(&script_section);

            let new_total = current_token_count + script_tokens;

            let script_file_relative = script.strip_prefix(&repo.root.join(&repo.repo_name))?;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping 'script: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    script_file_relative.display(),
                    script_tokens,
                    new_total,
                    token_budget
                );
            } else {
                info!(
                    "✅ Adding 'script: {}' ({} tokens) - total: {}/{} tokens",
                    script_file_relative.display(),
                    script_tokens,
                    new_total,
                    token_budget
                );
                markdown_codeblock_for_llm.push_str(&script_section);
                current_token_count = new_total;
            }
        }

        // Final token count verification
        let final_token_count = get_token_count(&markdown_codeblock_for_llm);
        info!(
            "📊 FINAL codeblock for '{}': {} tokens (budget: {} tokens, {}%)",
            main_contract,
            final_token_count,
            token_budget,
            (final_token_count * 100) / token_budget
        );

        if final_token_count > token_budget {
            info!(
                "⚠️ WARNING: Final codeblock ({} tokens) exceeds budget ({} tokens) by {} tokens",
                final_token_count,
                token_budget,
                final_token_count - token_budget
            );
        }

        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: main_contract.clone(),
            tokens: final_token_count, // Use actual token count, not BFS token count
            content: markdown_codeblock_for_llm,
        };
        // 4. store
        codeblock_db.insert_codeblock(&codeblock)?;

        // save to cache
        set_codeblock_cache(&main_contract, &codeblock).await;
    }

    Ok(())
}

pub async fn get_contract_file_content(contract: &str, repo: &RepoPaths) -> Result<String> {
    let file = match get_file_from_contract(contract, repo).await {
        Some(filename) => filename,
        None => {
            info!("could not find file for contract {}", contract);
            return Ok(String::new());
        }
    };

    let file_content = fs::read_to_string(&file).await?;
    Ok(file_content)
}
