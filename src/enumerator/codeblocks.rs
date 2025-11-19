use crate::build_brain::graph_db::SmartContractFunction;
use crate::build_brain::inheritance_map::{self, resolve_contract_file};
use crate::build_brain::summarize_db::get_file_summary_from_db;
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
    ImportDependencies, detect_scripts_connected_to_contract, detect_source_code_dependencies,
    is_standard_interface_name, is_standard_library_contract_name, should_exclude_this_library,
};
use crate::enumerator::utils::{
    SolFileType, get_hashmap_of_contract_to_functions, get_token_count_of_function_ir,
};
use crate::llm_review::contract::contract_category::ContractCategory;
use crate::llm_review::contract::contract_file_map::{
    get_file_from_contract, get_file_from_lib_contract,
};
use crate::llm_review::utils::contract_in_scope::contract_scope_and_type;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::display_file::display_file;
use tokio::fs;

use anyhow::Result;
use log::{info, warn};
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
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
        let (is_contract_in_scope, _) = contract_scope_and_type(&main_contract, repo).await?;

        if !is_contract_in_scope {
            info!(
                "{} is not in scope , so no creating codeblock",
                main_contract
            );
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
        let parents_of_main = match inheritance_map::get_parents(
            &main_contract,
            SolFileType::Standard,
            repo,
        )
        .await
        {
            Ok(parents) => parents,
            Err(e) => {
                warn!(
                    "Could not find parents for main contract '{}': {}. Continuing without parents...",
                    main_contract, e
                );
                Vec::new()
            }
        };
        for (parent, file) in &parents_of_main {
            if !is_standard_interface_name(parent) && !is_standard_library_contract_name(parent) {
                contracts_with_parents.insert((parent.clone(), file.clone()));
            }
        }

        // Add called contracts and their parents
        for contract in &contracts {
            // Skip if it's the main contract (already added above)
            if contract == &main_contract {
                continue;
            }

            // Add direct parents (1 level up) for called contracts
            let parents_plus_file =
                match inheritance_map::get_parents(contract, SolFileType::Standard, repo).await {
                    Ok(parents) => parents,
                    Err(e) => {
                        warn!(
                            "Could not find parents for contract '{}': {}. Skipping parents...",
                            contract, e
                        );
                        Vec::new()
                    }
                };
            for (parent, file) in &parents_plus_file {
                // info!("inserting {} (parent of {})", parent, contract);
                if !is_standard_interface_name(parent) && !is_standard_library_contract_name(parent)
                {
                    contracts_with_parents.insert((parent.clone(), file.clone()));
                }
            }
        }

        // Detect contracts and interfaces from source code that Slither's call graph misses
        // This includes: constructor calls, interface casts, imports, type declarations
        // We analyze BOTH the main contract AND all called contracts for comprehensive coverage

        // First, analyze the main contract
        let ImportDependencies {
            lib_files: mut main_lib_files,
            source_files: mut main_source_files,
            interfaces: mut main_interfaces,
            interface_implementations: mut main_interface_implementations,
        } = detect_source_code_dependencies(&main_contract, repo).await?;

        // warn!(
        //     "🔍 DEBUG: contracts_with_depth has {} contracts",
        //     contracts_with_depth.len()
        // );
        //
        // warn!(
        //     "🔍 DEBUG: Total source dependencies from main contract: {}",
        //     main_source_files.len()
        // );
        // warn!(
        //     "🔍 DEBUG: Total lib dependencies from main contract: {}",
        //     main_lib_files.len()
        // );
        // warn!(
        //     "🔍 DEBUG: Total interfaces from main contract: {}",
        //     main_interfaces.len()
        // );

        for contract in &contracts_with_depth {
            let ImportDependencies {
                lib_files,
                source_files,
                interfaces,
                interface_implementations,
            } = detect_source_code_dependencies(contract, repo).await?;
            main_source_files.extend(source_files);
            main_lib_files.extend(lib_files);
            main_interfaces.extend(interfaces);
            main_interface_implementations.extend(interface_implementations);
        }

        // warn!(
        //     "🔍 DEBUG: Total source dependencies after analyzing depth contracts: {}",
        //     main_source_files.len()
        // );
        // warn!(
        //     "🔍 DEBUG: Total lib dependencies after analyzing depth contracts: {}",
        //     main_lib_files.len()
        // );
        // warn!(
        //     "🔍 DEBUG: Total interfaces after analyzing depth contracts: {}",
        //     main_interfaces.len()
        // );
        // warn!(
        //     "🔍 DEBUG: Total interface implementations detected: {}",
        //     main_interface_implementations.len()
        // );

        if !main_interface_implementations.is_empty() {
            warn!(
                "🔍 DEBUG: Interface implementations: {}",
                main_interface_implementations
                    .keys()
                    .map(|k| k.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        // ── 3.  Assemble final Markdown body with TOKEN BUDGET ENFORCEMENT ────────────────────────────
        let mut markdown_codeblock_for_llm = String::new();
        let mut current_token_count = 0_usize;

        // track files as they are added to codeblock, to prevent dups
        let mut unique_files = HashSet::new();

        // Add main contract code (CRITICAL - always include)
        let (main_contract_code, contract_file) =
            get_contract_file_content(&main_contract, None, repo).await?;
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
        unique_files.insert(contract_file);

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
                prioritized_contracts.push((contract.clone(), None, "called"));
            }
        }

        // Priority 2: Add parents (MEDIUM PRIORITY - usually standard libraries)
        for (parent, parent_file) in &contracts_with_parents {
            if *parent != main_contract && !contracts.contains(parent) {
                prioritized_contracts.push((parent.clone(), Some(parent_file.clone()), "parent"));
            }
        }

        // DEBUG: Log what contracts were detected
        // info!("🔍 DEBUG: Total contracts detected: {}", contracts.len());
        // info!("🔍 DEBUG: contracts = {:?}", contracts);
        // info!(
        //     "🔍 DEBUG: contracts_with_parents = {:?}",
        //     contracts_with_parents
        //         .iter()
        //         .map(|(c, _)| c.to_string())
        //         .collect::<Vec<_>>()
        //         .join(", ")
        // );
        // info!(
        //     "🔍 DEBUG: contracts_with_depth = {:?}",
        //     contracts_with_depth
        // );

        // Process contracts in priority order (called → parents)
        // The prioritized_contracts vector is already ordered correctly from above
        let mut contracts_added = 0;
        let mut contracts_skipped_no_file = 0;
        let mut contracts_skipped_too_small = 0;
        let mut contracts_skipped_budget = 0;

        for (contract, file_option, contract_type) in prioritized_contracts {
            let (contract_code, contract_file) =
                get_contract_file_content(&contract, file_option, repo).await?;
            // Skip if no content (could not resolve file)
            if contract_code.trim().is_empty() || unique_files.contains(&contract_file) {
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
                unique_files.insert(contract_file); // ✅ Insert only after successfully adding
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

        // Add source files (imported libraries, utility files, etc.)
        let mut source_files_added = 0;
        let mut source_files_skipped_dup = 0;
        let mut source_files_skipped_read_error = 0;
        let mut source_files_skipped_budget = 0;

        for source_file in &main_source_files {
            // Skip duplicates
            if unique_files.contains(source_file) {
                source_files_skipped_dup += 1;
                continue;
            }

            // Read file content
            let source_content = match fs::read_to_string(source_file).await {
                Ok(content) => content,
                Err(e) => {
                    info!(
                        "⏭️ Could not read source file {} for dependency: {}",
                        display_file(source_file, repo),
                        e
                    );
                    source_files_skipped_read_error += 1;
                    continue;
                }
            };

            let source_section = format!("{}\n", source_content);
            let source_tokens = get_token_count(&source_section);

            let new_total = current_token_count + source_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping 'source file: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    display_file(source_file, repo),
                    source_tokens,
                    new_total,
                    token_budget
                );
                source_files_skipped_budget += 1;
            } else {
                info!(
                    "✅ Adding 'source file: {}' ({} tokens) - total: {}/{} tokens",
                    display_file(source_file, repo),
                    source_tokens,
                    new_total,
                    token_budget
                );
                markdown_codeblock_for_llm.push_str(&source_section);
                current_token_count = new_total;
                unique_files.insert(source_file.clone());
                source_files_added += 1;
            }
        }

        info!(
            "📊 Source files: {} added, {} skipped (duplicate: {}, read error: {}, budget: {})",
            source_files_added,
            source_files_skipped_dup
                + source_files_skipped_read_error
                + source_files_skipped_budget,
            source_files_skipped_dup,
            source_files_skipped_read_error,
            source_files_skipped_budget
        );

        let supporting_lib_header =
            "\n## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS\n";
        markdown_codeblock_for_llm.push_str(supporting_lib_header);
        current_token_count += get_token_count(supporting_lib_header);

        // adding interfaces
        for (interface_name, interface_file) in &main_interfaces {
            if unique_files.contains(interface_file) {
                continue;
            }

            // Skip excluded libraries (OpenZeppelin, forge-std, etc.)
            if let Some(file_str) = interface_file.to_str() {
                if should_exclude_this_library(file_str) {
                    info!(
                        "⏭️ Skipping excluded library interface '{}' at {}",
                        interface_name,
                        display_file(&interface_file, repo)
                    );
                    continue;
                }
            }

            info!(
                "Adding Interface (or root implimentation) File: {}....",
                display_file(&interface_file, repo)
            );
            let interface_content = match fs::read_to_string(interface_file).await {
                Ok(content) => content,
                Err(e) => {
                    info!(
                        "could not read file {} for lib dependency detection: {}",
                        display_file(&interface_file, repo),
                        e
                    );
                    continue;
                }
            };

            let interface_section = format!("{}\n", interface_content);
            let interface_tokens = get_token_count(&interface_section);

            let new_total = current_token_count + interface_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping 'interface/child: {} : {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    interface_name,
                    display_file(&interface_file, repo),
                    interface_tokens,
                    new_total,
                    token_budget
                );
            } else {
                info!(
                    "✅ Adding 'interface/child: {}' ({} tokens) - total: {}/{} tokens",
                    display_file(&interface_file, repo),
                    interface_tokens,
                    new_total,
                    token_budget
                );
                markdown_codeblock_for_llm.push_str(&interface_section);
                current_token_count = new_total;
                unique_files.insert(interface_file.clone());
            }
        }

        // Add interface implementations (contracts that implement detected interfaces)
        // This is CRITICAL for security analysis - when code uses an interface (e.g., IPriceOracle),
        // we need to analyze ALL possible implementations (e.g., CovenantCurator, PythOracle)
        let mut impl_added = 0;
        let mut impl_skipped_dup = 0;
        let mut impl_skipped_read_error = 0;
        let mut impl_skipped_budget = 0;

        for (impl_name, impl_file) in &main_interface_implementations {
            // Skip duplicates
            if unique_files.contains(impl_file) {
                impl_skipped_dup += 1;
                continue;
            }

            // Read file content
            let impl_content = match fs::read_to_string(impl_file).await {
                Ok(content) => content,
                Err(e) => {
                    info!(
                        "⏭️ Could not read interface implementation {}: {} for dependency: {}",
                        impl_name,
                        display_file(impl_file, repo),
                        e
                    );
                    impl_skipped_read_error += 1;
                    continue;
                }
            };

            let impl_section = format!("{}\n", impl_content);
            let impl_tokens = get_token_count(&impl_section);

            let new_total = current_token_count + impl_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping 'interface implementation: {}: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    impl_name,
                    display_file(impl_file, repo),
                    impl_tokens,
                    new_total,
                    token_budget
                );
                impl_skipped_budget += 1;
            } else {
                info!(
                    "✅ Adding 'interface implementation: {}: {}' ({} tokens) - total: {}/{} tokens",
                    impl_name,
                    display_file(impl_file, repo),
                    impl_tokens,
                    new_total,
                    token_budget
                );
                markdown_codeblock_for_llm.push_str(&impl_section);
                current_token_count = new_total;
                unique_files.insert(impl_file.clone());
                impl_added += 1;
            }
        }

        info!(
            "📊 Interface implementations: {} added, {} skipped (duplicate: {}, read error: {}, budget: {})",
            impl_added,
            impl_skipped_dup + impl_skipped_read_error + impl_skipped_budget,
            impl_skipped_dup,
            impl_skipped_read_error,
            impl_skipped_budget
        );

        let supporting_lib_header = "\n## SUPPORTING CONTEXT: EXTERNAL LIBRARIES\n";
        markdown_codeblock_for_llm.push_str(supporting_lib_header);
        current_token_count += get_token_count(supporting_lib_header);

        // add external libary filse
        for lib_file in &main_lib_files {
            if unique_files.contains(lib_file) {
                continue;
            }

            info!(
                "Adding External Library File: {}....",
                display_file(&lib_file, repo),
            );
            let lib_content = match fs::read_to_string(lib_file).await {
                Ok(content) => content,
                Err(e) => {
                    info!(
                        "could not read file {} for lib dependency detection: {}",
                        display_file(&lib_file, repo),
                        e
                    );
                    continue;
                }
            };

            let lib_section = format!("{}\n", lib_content);
            let lib_tokens = get_token_count(&lib_section);

            let new_total = current_token_count + lib_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping 'external lib: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    display_file(&lib_file, repo),
                    lib_tokens,
                    new_total,
                    token_budget
                );
            } else {
                info!(
                    "✅ Adding 'external lib: {}' ({} tokens) - total: {}/{} tokens",
                    display_file(&lib_file, repo),
                    lib_tokens,
                    new_total,
                    token_budget
                );
                markdown_codeblock_for_llm.push_str(&lib_section);
                current_token_count = new_total;
                unique_files.insert(lib_file.clone());
            }
        }

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
                        display_file(&script, repo),
                        e
                    );
                    continue;
                }
            };

            let script_section = format!("{}\n", script_content);
            let script_tokens = get_token_count(&script_section);

            let new_total = current_token_count + script_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping 'script: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    display_file(&script, repo),
                    script_tokens,
                    new_total,
                    token_budget
                );
            } else {
                info!(
                    "✅ Adding 'script: {}' ({} tokens) - total: {}/{} tokens",
                    display_file(&script, repo),
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

        let contract_category =
            extract_contract_category_from_contract(&main_contract, repo).await?;
        let codeblock = MarkdownCodeblock {
            id: Uuid::new_v4().to_string(),
            project_id: repo.project_id.clone(),
            contract: main_contract.clone(),
            contract_category: contract_category.unwrap_or_default(),
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

pub async fn extract_contract_category_from_contract(
    contract: &str,
    repo: &RepoPaths,
) -> Result<Option<ContractCategory>> {
    // Try Standard (source) files first
    let file = match resolve_contract_file(contract, SolFileType::Standard, repo).await? {
        Some(f) => Some(f),
        None => {
            // If not found in source, try LibFolder
            resolve_contract_file(contract, SolFileType::LibFolder, repo).await?
        }
    };

    let file_path = match file {
        Some(f) => f,
        None => return Ok(None),
    };

    // Convert absolute path to relative path (same format as stored in database)
    // Database stores paths like: "2025-11-sequence/src/TrailsRouterShim.sol"
    let filename = file_path
        .strip_prefix(&repo.root)
        .unwrap_or(&file_path)
        .to_string_lossy()
        .to_string();

    let contract_category =
        get_file_summary_from_db(&filename, repo)?.and_then(|f| f.contract_category);

    Ok(contract_category)
}

pub async fn get_contract_file_content(
    contract: &str,
    option_file: Option<PathBuf>,
    repo: &RepoPaths,
) -> Result<(String, PathBuf)> {
    let file_path = option_file.unwrap_or({
        // Try source files first, then library files
        let file = match get_file_from_contract(contract, repo).await {
            Some((filename, _)) => filename,
            None => {
                // Try library files
                match get_file_from_lib_contract(contract, repo).await {
                    Some((filename, _)) => filename,
                    None => {
                        info!("could not find file for contract {}", contract);
                        PathBuf::new()
                    }
                }
            }
        };
        file
    });

    if file_path.as_os_str().is_empty() {
        return Ok((String::new(), PathBuf::new()));
    }
    let file_content = fs::read_to_string(&file_path).await?;
    Ok((file_content, file_path))
}
