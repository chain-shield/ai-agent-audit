use crate::build_brain::graph_db::SmartContractFunction;
use crate::build_brain::inheritance_map::{self, resolve_contract_file};
use crate::build_brain::summarize_db::get_file_summary_from_db;
use crate::config::{CHAINSHIELD_DB_FOLDER, CODEBLOCK_DB};
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
    is_standard_interface_name, is_standard_library_contract_name, should_exclude_this_library,
    ImportDependencies,
};
use crate::enumerator::utils::{
    get_hashmap_of_contract_to_functions, get_token_count_of_function_ir, SolFileType,
};
use crate::llm_review::contract::contract_category::ContractCategory;
use crate::llm_review::contract::contract_file_map::{
    get_file_from_contract, get_file_from_lib_contract,
};
use crate::llm_review::dynamic_prompts::prompt_index;
use crate::llm_review::utils::contract_in_scope::contract_scope_and_type;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::display_file::display_file;
use tokio::fs;

use anyhow::Result;
use log::{info, warn};
use rusqlite::Connection;
use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

/// Generates and saves contextual code blocks for all contracts in a repository.
///
/// This function orchestrates the code block generation process by:
/// 1. Deleting old codeblock database from previous run
/// 2. Opening semantic and codeblock databases
/// 3. Generating focused code slices for each contract using call graph traversal
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_db` - Path to semantic analysis database
/// * `max_depth` - Maximum call graph traversal depth
/// * `token_budget` - Maximum tokens per code block
///
/// # Returns
/// * `PathBuf` - Path to the generated codeblock database
pub async fn generate_and_save_codeblocks_for_each_contract(
    repo: &RepoPaths,
    semantics_db: &Path,
    max_depth: usize,
    token_budget: usize,
) -> Result<PathBuf> {
    log::info!("connecting to databases..");

    // Open semantic database
    let semantic_conn = Connection::open(semantics_db)?;

    // Create codeblock database path
    let codeblock_path =
        Path::new(&format!("{}/{}", CHAINSHIELD_DB_FOLDER, CODEBLOCK_DB)).to_path_buf();

    // Delete the codeblock database from previous run to ensure fresh data
    if codeblock_path.exists() {
        log::info!("Deleting old codeblock database from previous run");
        std::fs::remove_file(&codeblock_path)?;
    }

    // Open codeblock database
    let codeblock_db = CodeBlocksDb::open(&codeblock_path)?;

    // Generate codeblocks
    generate_codeblock_from_codebase(repo, &semantic_conn, &codeblock_db, max_depth, token_budget)
        .await?;

    Ok(codeblock_path)
}

/// Check if Slither analysis data is available for codeblock generation.
///
/// This function checks if the contract-to-function map is populated, which indicates
/// that Slither successfully analyzed the codebase and populated the semantic database.
/// If empty, falls back to import-only traversal mode.
///
/// # Arguments
/// * `contract_to_func_map` - HashMap of contracts to their functions from semantic DB
///
/// # Returns
/// * `bool` - True if Slither data is available, false otherwise
fn check_slither_data_available(
    contract_to_func_map: &std::collections::HashMap<String, Vec<SmartContractFunction>>,
) -> bool {
    let slither_available = !contract_to_func_map.is_empty();

    if !slither_available {
        log::warn!("⚠️  Slither call graph unavailable. Falling back to import-only traversal.");
    }

    slither_available
}

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

    // Check if Slither data is available for BFS traversal
    let slither_available = check_slither_data_available(&contract_to_func_map);

    // Determine which contracts to process
    let contracts_to_process: Vec<String> = if slither_available {
        contract_to_func_map.keys().cloned().collect()
    } else {
        // Fallback: get all in-scope contracts
        use crate::enumerator::utils::contracts_in_source_folder;
        contracts_in_source_folder(repo).await?
    };

    for main_contract in contracts_to_process {
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
        if let Some(_) = get_cached_codeblock(&main_contract).await {
            // Save seed-to-codeblock mapping in the database
            continue;
        };

        // 2. Discover contracts via BFS (Slither) or import traversal (fallback)
        let (contracts, contracts_with_depth, _token_count) = if slither_available {
            // BFS traversal using Slither call graph
            let functions_of_contract = contract_to_func_map.get(&main_contract).unwrap();
            log::info!("fn count of contract => {:#?}", functions_of_contract.len());

            let mut frontier: VecDeque<(SmartContractFunction, usize)> = VecDeque::new();
            for func in functions_of_contract {
                frontier.push_back((func.clone(), 0_usize))
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
                    let mut statement = semantic_db.prepare(
                        "SELECT callee FROM edges WHERE caller = ?1 AND project_id = ?2;",
                    )?;
                    let rows = statement
                        .query_map([&func.id, &repo.project_id], |r| r.get::<_, String>(0))?;
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

            (contracts, contracts_with_depth, token_count)
        } else {
            // Import-only fallback (use max_depth - 1 to reduce codeblock size)
            log::warn!(
                "Using import-only traversal for contract '{}'",
                main_contract
            );
            generate_contracts_via_import_traversal(&main_contract, repo, max_depth, token_budget)
                .await?
        };

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

        // Track file paths for CODE INDEX
        let mut main_contract_files: Vec<String> = Vec::new();
        let mut supporting_contract_files: Vec<String> = Vec::new();
        let mut source_files_list: Vec<String> = Vec::new();
        let mut interface_files_list: Vec<String> = Vec::new();
        let mut impl_files_list: Vec<String> = Vec::new();
        let mut lib_files_list: Vec<String> = Vec::new();
        let mut script_files_list: Vec<String> = Vec::new();

        // Add main contract code (CRITICAL - always include)
        let (main_contract_code, contract_file) =
            get_contract_file_content(&main_contract, None, repo).await?;
        let section_8_2_header = prompt_index::generated_sub_header("MAIN TARGET CONTRACT", 8, 2);
        let main_section = format!(
            r#"

{}

<file path="{}">
```solidity
{}
```
</file>"#,
            section_8_2_header,
            display_file(&contract_file, repo),
            main_contract_code
        );
        let main_tokens = get_token_count(&main_section);

        info!(
            "✅ Adding 'Main Contract: {}' ({} tokens) - CRITICAL",
            main_contract, main_tokens
        );
        markdown_codeblock_for_llm.push_str(&main_section);
        current_token_count += main_tokens;
        main_contract_files.push(display_file(&contract_file, repo));
        unique_files.insert(contract_file);

        if current_token_count > token_budget {
            info!(
                "⚠️ Main contract alone ({} tokens) exceeds budget ({} tokens)",
                current_token_count, token_budget
            );
        }

        // Add parent and called contracts (prioritized by importance)
        let supporting_header = prompt_index::generated_sub_header(
            "SUPPORTING CONTRACTS, LIBRARIES & INTERFACES",
            8,
            3,
        );
        markdown_codeblock_for_llm.push_str(&format!(
            r#"

{}
        "#,
            supporting_header
        ));
        current_token_count += get_token_count(&supporting_header);

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

            let contract_section = format!(
                "<file path=\"{}\">\n```solidity\n{}\n```\n</file>\n",
                display_file(&contract_file, repo),
                contract_code
            );
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
                supporting_contract_files.push(display_file(&contract_file, repo));
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

            let source_section = format!(
                "<file path=\"{}\">\n```solidity\n{}\n```\n</file>\n",
                display_file(source_file, repo),
                source_content
            );
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
                source_files_list.push(display_file(source_file, repo));
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
            prompt_index::generated_sub_header("INTERFACES AND ROOT IMPLEMENTATIONS", 8, 4);
        markdown_codeblock_for_llm.push_str(&format!(
            r#"
{}
            "#,
            supporting_lib_header
        ));
        current_token_count += get_token_count(&supporting_lib_header);

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

            let interface_section = format!(
                "<file path=\"{}\">\n```solidity\n{}\n```\n</file>\n",
                display_file(&interface_file, repo),
                interface_content
            );
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
                interface_files_list.push(display_file(&interface_file, repo));
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

            let impl_section = format!(
                "<file path=\"{}\">\n```solidity\n{}\n```\n</file>\n",
                display_file(impl_file, repo),
                impl_content
            );
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
                impl_files_list.push(display_file(impl_file, repo));
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

        let supporting_lib_header = prompt_index::generated_sub_header("EXTERNAL LIBRARIES", 8, 5);
        markdown_codeblock_for_llm.push_str(&format!(
            r#"
{}
            "#,
            supporting_lib_header
        ));
        current_token_count += get_token_count(&supporting_lib_header);

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

            let lib_section = format!(
                "<file path=\"{}\">\n```solidity\n{}\n```\n</file>\n",
                display_file(&lib_file, repo),
                lib_content
            );
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
                lib_files_list.push(display_file(&lib_file, repo));
                unique_files.insert(lib_file.clone());
            }
        }

        let section_8_6_header = prompt_index::generated_sub_header("DEPLOYMENT SCRIPTS", 8, 6);
        markdown_codeblock_for_llm.push_str(&format!(
            r#"
{}
            "#,
            section_8_6_header
        ));

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

            let script_section = format!(
                "<file path=\"{}\">\n```solidity\n{}\n```\n</file>\n",
                display_file(&script, repo),
                script_content
            );
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
                script_files_list.push(display_file(&script, repo));
            }
        }

        // ── 4. Generate CODE INDEX (Section 8.1) ────────────────────────────
        let code_index = generate_code_index(
            &main_contract_files,
            &supporting_contract_files,
            &source_files_list,
            &interface_files_list,
            &impl_files_list,
            &lib_files_list,
            &script_files_list,
        );

        // Prepend CODE INDEX to the beginning of the markdown
        let mut final_markdown = String::new();
        final_markdown.push_str(&code_index);
        final_markdown.push_str(&markdown_codeblock_for_llm);
        markdown_codeblock_for_llm = final_markdown;

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

/// Extract all contract names from a Solidity file.
///
/// This function parses a Solidity file and extracts contract names based on the following rules:
///
/// **Includes:**
/// - Concrete contracts
/// - Abstract contracts
/// - Libraries in source code folders (e.g., /src/libraries/) - matches BFS behavior
///
/// **Excludes:**
/// - Interfaces (contract_type == Interface)
/// - Libraries in lib/ folders (standard libraries like OpenZeppelin, forge-std)
/// - Standard library contracts (checked via `is_standard_library_contract_name()`)
/// - Mock/test contracts (path contains "/mocks/" or "/test/", or name contains "mock")
///
/// # Arguments
/// * `file` - Path to Solidity file
/// * `repo` - Repository paths
///
/// # Returns
/// * `Vec<String>` - List of contract names found in the file
///
/// # Errors
/// Returns an error if the file cannot be read
async fn extract_contracts_from_file(file: &PathBuf, repo: &RepoPaths) -> Result<Vec<String>> {
    use crate::enumerator::utils::SOLIDITY_REGEXES;

    // Read file content
    let content = match fs::read_to_string(file).await {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Failed to read file {:?}: {}", file, e);
            return Ok(Vec::new());
        }
    };

    // Check if file is in source code folder
    let is_source_file = repo
        .source_code_folders
        .iter()
        .any(|src| file.starts_with(src));

    // Check if file is in mock/test directory
    let file_str = file.to_string_lossy();
    let is_mock_or_test =
        file_str.contains("/mocks/") || file_str.contains("/test/") || file_str.contains("/tests/");

    if is_mock_or_test {
        log::debug!("Skipping mock/test file: {:?}", file);
        return Ok(Vec::new());
    }

    let mut contracts = Vec::new();

    // Extract all contract declarations using regex
    for cap in SOLIDITY_REGEXES.contract_decl.captures_iter(&content) {
        let declaration_type = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let contract_name = cap.get(2).map(|m| m.as_str()).unwrap_or("");

        if contract_name.is_empty() {
            continue;
        }

        // Skip if contract name contains "mock" (case-insensitive)
        if contract_name.to_lowercase().contains("mock") {
            log::debug!("Skipping mock contract: {}", contract_name);
            continue;
        }

        // Skip standard library contracts
        if is_standard_library_contract_name(contract_name) {
            log::debug!("Skipping standard library contract: {}", contract_name);
            continue;
        }

        // Determine if this is an interface or library
        let is_interface = declaration_type.contains("interface");
        let is_library = declaration_type.contains("library");

        // Skip interfaces
        if is_interface {
            log::debug!("Skipping interface: {}", contract_name);
            continue;
        }

        // Skip libraries UNLESS they are in source code folders
        if is_library && !is_source_file {
            log::debug!("Skipping library in lib/ folder: {}", contract_name);
            continue;
        }

        // Include this contract
        log::debug!(
            "Including contract '{}' from {:?} (type: {}, is_source: {})",
            contract_name,
            file,
            declaration_type,
            is_source_file
        );
        contracts.push(contract_name.to_string());
    }

    Ok(contracts)
}

/// Generate contract sets using import-only traversal (fallback when Slither fails).
///
/// This function replaces the BFS call graph traversal (lines 85-140 in generate_codeblock_from_codebase)
/// when Slither is unavailable or fails. It discovers contracts by traversing import
/// dependencies up to `max_depth` levels deep.
///
/// # Algorithm
/// 1. Level 0: Start with main_contract
/// 2. For each level from 1 to max_depth:
///    - Analyze imports of contracts from previous level
///    - Discover contracts at current level
/// 3. Respect token budget at each level (currently deferred to assembly phase)
/// 4. Track visited files to avoid cycles
///
/// # Arguments
/// * `main_contract` - The contract to analyze
/// * `repo` - Repository paths
/// * `max_depth` - Maximum import traversal depth (default 3, configurable)
/// * `token_budget` - Maximum tokens per code block (currently unused, handled in assembly)
///
/// # Returns
/// * `(contracts, contracts_with_depth, token_count)` where:
///   - `contracts`: All unique contracts discovered (excluding libs/interfaces)
///   - `contracts_with_depth`: Contracts at depth 1-2 (for compatibility with BFS)
///   - `token_count`: 0 (placeholder, calculated later in assembly phase)
///
/// # Errors
/// Returns an error if the main contract file cannot be found
async fn generate_contracts_via_import_traversal(
    main_contract: &str,
    repo: &RepoPaths,
    max_depth: usize,
    _token_budget: usize, // Currently unused, token budget enforced in assembly phase
) -> Result<(HashSet<String>, HashSet<String>, usize)> {
    log::info!(
        "🔄 Import-only traversal for '{}' (max_depth: {})",
        main_contract,
        max_depth
    );

    // Initialize levels: level[0] = main contract, level[1..max_depth] = discovered contracts
    let mut levels: Vec<HashSet<String>> = vec![HashSet::new(); max_depth + 1];
    levels[0].insert(main_contract.to_string());

    let mut visited_files = HashSet::new();

    // Traverse each level
    for current_depth in 0..max_depth {
        let contracts_at_current_level = levels[current_depth].clone();

        if contracts_at_current_level.is_empty() {
            log::debug!(
                "No contracts at depth {}, stopping traversal",
                current_depth
            );
            break;
        }

        log::debug!(
            "Processing {} contracts at depth {}",
            contracts_at_current_level.len(),
            current_depth
        );

        for contract in &contracts_at_current_level {
            // Get file for contract (try source first, then lib)
            let file_opt = match get_file_from_contract(contract, repo).await {
                Some(f) => Some(f),
                None => get_file_from_lib_contract(contract, repo).await,
            };

            let (file, _contract_type) = match file_opt {
                Some(f) => f,
                None => {
                    log::warn!(
                        "Could not find file for contract '{}' at depth {}",
                        contract,
                        current_depth
                    );
                    continue;
                }
            };

            // Skip if already visited (cycle detection)
            if visited_files.contains(&file) {
                log::debug!(
                    "Skipping already visited file {:?} for contract '{}'",
                    file,
                    contract
                );
                continue;
            }

            visited_files.insert(file.clone());

            // Get import dependencies for this contract
            let import_deps = match detect_source_code_dependencies(contract, repo).await {
                Ok(deps) => deps,
                Err(e) => {
                    log::warn!(
                        "Failed to detect dependencies for contract '{}': {}",
                        contract,
                        e
                    );
                    continue;
                }
            };

            // Extract contracts from source files
            for source_file in &import_deps.source_files {
                if visited_files.contains(source_file) {
                    continue;
                }

                let contracts_in_file = match extract_contracts_from_file(source_file, repo).await {
                    Ok(c) => c,
                    Err(e) => {
                        log::warn!("Failed to extract contracts from {:?}: {}", source_file, e);
                        continue;
                    }
                };

                for discovered_contract in contracts_in_file {
                    // Skip standard interfaces and libraries
                    if is_standard_interface_name(&discovered_contract)
                        || is_standard_library_contract_name(&discovered_contract)
                    {
                        continue;
                    }

                    log::debug!(
                        "Discovered contract '{}' at depth {} from {:?}",
                        discovered_contract,
                        current_depth + 1,
                        source_file
                    );
                    levels[current_depth + 1].insert(discovered_contract);
                }
            }

            // Note: Interface implementations are NOT added here because they will be
            // discovered later in the assembly phase (lines 325-336) when we call
            // detect_source_code_dependencies() on all contracts in contracts_with_depth
        }
    }

    // Assemble results
    let contracts: HashSet<String> = levels.iter().flatten().cloned().collect();

    // contracts_with_depth = depth 1-2 (matches BFS behavior at lines 105-107)
    let contracts_with_depth: HashSet<String> = levels[1].union(&levels[2]).cloned().collect();

    log::info!(
        "✅ Import-only traversal complete: {} total contracts, {} at depth 1-2",
        contracts.len(),
        contracts_with_depth.len()
    );

    // Token count is 0 (placeholder) - will be calculated during assembly phase
    Ok((contracts, contracts_with_depth, 0))
}

/// Generates a CODE INDEX table of contents for all files included in the codeblock.
///
/// This function creates a structured index showing which files are included in each section,
/// making it easier for LLMs to navigate the codeblock and understand the structure.
///
/// # Arguments
/// * `main_contract_files` - Files in Section 8.2 (Main Target Contract)
/// * `supporting_contract_files` - Files in Section 8.3 (Supporting Contracts)
/// * `source_files_list` - Files in Section 8.3 (Source Files)
/// * `interface_files_list` - Files in Section 8.4 (Interfaces)
/// * `impl_files_list` - Files in Section 8.4 (Interface Implementations)
/// * `lib_files_list` - Files in Section 8.5 (External Libraries)
/// * `script_files_list` - Files in Section 8.6 (Deployment Scripts)
///
/// # Returns
/// * `String` - Formatted CODE INDEX markdown
fn generate_code_index(
    main_contract_files: &[String],
    supporting_contract_files: &[String],
    source_files_list: &[String],
    interface_files_list: &[String],
    impl_files_list: &[String],
    lib_files_list: &[String],
    script_files_list: &[String],
) -> String {
    let section_8_1_header =
        prompt_index::generated_sub_header("CODE INDEX (read this first)", 8, 1);

    let mut index = format!(
        r#"

{}

### CODE INDEX (read this first)

"#,
        section_8_1_header
    );

    // Section 8.2: Main Target Contract
    if !main_contract_files.is_empty() {
        index.push_str("- `Section 8.2: Main Target Contract:` ");
        index.push_str(&main_contract_files.join(", "));
        index.push_str("\n\n");
    }

    // Section 8.3: Supporting Contracts, Libraries & Interfaces
    let mut section_8_3_files = Vec::new();
    section_8_3_files.extend(supporting_contract_files.iter().cloned());
    section_8_3_files.extend(source_files_list.iter().cloned());

    if !section_8_3_files.is_empty() {
        index.push_str("- `Section 8.3: Supporting Contracts, Libraries & Interfaces:` ");
        index.push_str(&section_8_3_files.join(", "));
        index.push_str("\n\n");
    }

    // Section 8.4: Interfaces and Root Implementations
    let mut section_8_4_files = Vec::new();
    section_8_4_files.extend(interface_files_list.iter().cloned());
    section_8_4_files.extend(impl_files_list.iter().cloned());

    if !section_8_4_files.is_empty() {
        index.push_str("- `Section 8.4: Interfaces and Root Implementations:` ");
        index.push_str(&section_8_4_files.join(", "));
        index.push_str("\n\n");
    }

    // Section 8.5: External Libraries
    if !lib_files_list.is_empty() {
        index.push_str("- `Section 8.5: External Libraries:` ");
        index.push_str(&lib_files_list.join(", "));
        index.push_str("\n\n");
    }

    // Section 8.6: Deployment Scripts
    if !script_files_list.is_empty() {
        index.push_str("- `Section 8.6: Deployment Scripts:` ");
        index.push_str(&script_files_list.join(", "));
        index.push_str("\n\n");
    }

    index
}
