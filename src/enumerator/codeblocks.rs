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
use crate::enumerator::utils::{
    get_function_metadata_from_contract_plus_fn, get_function_metadata_from_id,
    get_hashmap_of_contract_to_functions, get_token_count_of_function_ir,
};
use crate::llm_review::contract_file_map::{get_file_from_contract, get_file_from_interface};
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_fn_name::{
    get_function_name_from_func_id, get_function_name_from_interface, string_starts_with_char,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Mutex;
use tokio::fs;

use anyhow::Result;
use log::info;
use rusqlite::Connection;
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

use super::codeblock_db::CodeBlocksDb;

/// Global cache for internal_call_edge_map results.
/// Key format: "ir_block_hash:contract_name"
static INTERNAL_CALL_EDGE_CACHE: Lazy<Mutex<HashMap<String, HashMap<String, (String, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Global cache for source code dependency detection results.
/// Key format: "repo_hash:contract_name"
/// Value: (HashSet<contracts>, HashSet<interfaces>)
static SOURCE_DEPENDENCY_CACHE: Lazy<Mutex<HashMap<String, (HashSet<String>, HashSet<String>)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Decide if we should include grandparent contracts.
/// Skip if it's a known standard library to reduce token count.
fn should_include_grandparent(parent: &str) -> bool {
    const SKIP_LIST: &[&str] = &[
        "Context",
        "ERC165",
        "IERC165",
        "Ownable",
        "AccessControl",
        "ReentrancyGuard",
        "Pausable",
        "ERC20",
        "ERC721",
        "ERC4626",
        "ERC1155",
        "Initializable",
        "UUPSUpgradeable",
    ];

    !SKIP_LIST.iter().any(|skip| parent.contains(skip))
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

    for (main_contract, functions_of_contract) in contract_to_func_map {
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
        let mut all_funcs_connected_to_contract = Vec::<SmartContractFunction>::new();
        let mut token_count = 0_usize;

        while let Some((func, depth)) = frontier.pop_front() {
            if !visited.insert(func.id.clone()) {
                continue;
            }

            //keep track of unique contract traversed in BPS
            contracts.insert(func.contract.clone());
            // info!("contract {} / func {} added...", func.contract, func.name);

            all_funcs_connected_to_contract.push(func.clone());

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

        // Add main contract itself (might not be in contracts set if no callees)
        contracts_with_parents.insert(main_contract.clone());

        // Add parents of main contract (up to 2 levels)
        let parents_of_main = callgraph::get_parents(&main_contract, repo).await?;
        for parent in &parents_of_main {
            contracts_with_parents.insert(parent.clone());
        }

        // Detect contracts and interfaces from source code that Slither's call graph misses
        // This includes: constructor calls, interface casts, imports, type declarations
        // We analyze BOTH the main contract AND all called contracts for comprehensive coverage

        // First, analyze the main contract
        let (main_source_contracts, main_source_interfaces) =
            detect_source_code_dependencies(&main_contract, repo).await?;

        for contract_name in &main_source_contracts {
            // info!(
            //     "adding source-detected contract {} from main contract {}",
            //     contract_name, main_contract
            // );
            contracts.insert(contract_name.clone());
        }

        for interface_name in &main_source_interfaces {
            // info!(
            //     "adding source-detected interface {} from main contract {}",
            //     interface_name, main_contract
            // );
            contracts_with_parents.insert(interface_name.clone());
        }

        // Add called contracts and their parents
        for contract in &contracts {
            // Skip if it's the main contract (already added above)
            if contract == &main_contract {
                continue;
            }

            // Add the called contract itself
            contracts_with_parents.insert(contract.clone());

            // Add direct parents (1 level up) for called contracts
            let parents = callgraph::get_parents(contract, repo).await?;
            for parent in &parents {
                // info!("inserting {} (parent of {})", parent, contract);
                contracts_with_parents.insert(parent.clone());
            }
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
        let supporting_header = "\n## SUPPORTING CONTEXT: PARENT AND CALLED CONTRACTS\n";
        markdown_codeblock_for_llm.push_str(supporting_header);
        current_token_count += get_token_count(supporting_header);

        // Get scoped files for priority checking
        let scoped_files = repo.extract_scoped_files()?;
        let has_scoped_files = !scoped_files.is_empty();

        // Prioritize contracts by importance:
        // 1. HIGHEST: Scoped contracts (explicitly marked for audit)
        // 2. HIGH: Called contracts (user flow, attack surface)
        // 3. MEDIUM: Parent contracts (standard libraries)
        let mut prioritized_contracts = Vec::new();
        let parents_of_main = callgraph::get_parents(&main_contract, repo).await?;

        // Priority 1: Add scoped contracts first (HIGHEST PRIORITY - explicitly in audit scope)
        if has_scoped_files {
            for contract in &contracts_with_parents {
                if *contract != main_contract
                    && is_in_scoped_files(contract, repo, &scoped_files).await
                {
                    prioritized_contracts.push((contract.clone(), "scoped"));
                }
            }
        }

        // Priority 2: Add called contracts (HIGH PRIORITY - user flow, attack surface)
        for contract in &contracts {
            if *contract != main_contract
                && !parents_of_main.contains(contract)
                && !is_in_scoped_files(contract, repo, &scoped_files).await
            // Skip if already added as scoped
            {
                prioritized_contracts.push((contract.clone(), "called"));
            }
        }

        // Priority 3: Add parents (MEDIUM PRIORITY - usually standard libraries)
        for parent in &contracts_with_parents {
            if *parent != main_contract
                && !is_in_scoped_files(parent, repo, &scoped_files).await
                && !contracts.contains(parent)
            {
                prioritized_contracts.push((parent.clone(), "parent"));
            }
        }

        // Process contracts in priority order (scoped → called → parents)
        // The prioritized_contracts vector is already ordered correctly from above
        let mut contracts_added = 0;
        let mut contracts_skipped = 0;
        for (contract, contract_type) in prioritized_contracts {
            let contract_code = get_contract_file_content(&contract, repo).await?;
            // Skip if no content (could not resolve file)
            if contract_code.trim().is_empty() {
                info!(
                    "⏭️ Skipping '{} contract: {}' - no file or empty content",
                    contract_type, contract
                );
                contracts_skipped += 1;
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
                contracts_skipped += 1;
                continue;
            }

            let new_total = current_token_count + section_tokens;

            if new_total > token_budget {
                info!(
                    "⏭️ Skipping '{} contract: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                    contract_type, contract, section_tokens, new_total, token_budget
                );
                contracts_skipped += 1;
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
            "📊 Supporting contracts: {} added, {} skipped due to budget",
            contracts_added, contracts_skipped
        );

        // Add interface contracts (lowest priority)
        let interfaces_header = "\n## INTERFACES\n\n";
        markdown_codeblock_for_llm.push_str(interfaces_header);
        current_token_count += get_token_count(interfaces_header);

        let mut interfaces_added = HashSet::new();
        let mut interfaces_added_count = 0;
        let mut interfaces_skipped_count = 0;

        for contract in &contracts_with_parents {
            // Check if contract name starts with 'I' (interface naming convention)
            if contract.starts_with('I') && interfaces_added.insert(contract.clone()) {
                // Cheap check first: standard by name
                if is_standard_interface_name(&contract) {
                    info!("⏭️ Skipping standard interface: {}", contract);
                    interfaces_skipped_count += 1;
                    continue;
                }

                // Then check by known library path (requires a mapping lookup)
                if let Some(iface_path) = get_file_from_interface(&contract, repo).await {
                    if path_is_standard_lib(&iface_path) {
                        info!("⏭️ Skipping standard interface: {}", contract);
                        interfaces_skipped_count += 1;
                        continue;
                    }
                }

                let interface_code = get_interface_file_content(&contract, repo).await?;
                if !interface_code.is_empty() {
                    let interface_section = format!("{}\n", interface_code);
                    let section_tokens = get_token_count(&interface_section);

                    // Minimum section size gate for interfaces
                    if section_tokens < 5 {
                        info!(
                            "⏭️ Skipping 'Interface: {}' ({} tokens) - below minimum (5 tokens)",
                            contract, section_tokens
                        );
                        interfaces_skipped_count += 1;
                        continue;
                    }

                    let new_total = current_token_count + section_tokens;

                    if new_total > token_budget {
                        info!(
                            "⏭️ Skipping 'Interface: {}' ({} tokens) - would exceed budget ({}/{} tokens)",
                            contract, section_tokens, new_total, token_budget
                        );
                        interfaces_skipped_count += 1;
                    } else {
                        info!(
                            "✅ Adding 'Interface: {}' ({} tokens) - total: {}/{} tokens",
                            contract, section_tokens, new_total, token_budget
                        );
                        markdown_codeblock_for_llm.push_str(&interface_section);
                        current_token_count = new_total;
                        interfaces_added_count += 1;
                    }
                }
            }
        }

        markdown_codeblock_for_llm.push_str("\nEND OF SUPPORTING CONTRACTS AND INTERFACES\n");

        info!(
            "📊 Interfaces: {} added, {} skipped due to budget",
            interfaces_added_count, interfaces_skipped_count
        );

        // // list storage vars
        // for contract in &contracts {
        //     let storage_var_ir = generate_code_slice_for_storage(contract, repo).await?;
        //     // loop through and add all functions of contract
        //     markdown_codeblock_for_llm.push_str(&storage_var_ir);
        //     markdown_codeblock_for_llm.push('\n');
        // }
        // // let mut library_calls = Vec::<LibCall>::new();
        // // info!(
        // //     "ALL CONNECTED FUNCTIONS COUNT: {}",
        // //     all_funcs_connected_to_contract.len()
        // // );
        // for func in &all_funcs_connected_to_contract {
        //     let function_ir_code = generate_codeblock_for_function(func, repo).await?;
        //     // check function IR for LIBRARY_CALL
        //     // library_calls.extend(collect_library_calls(&function_ir_code));
        //     markdown_codeblock_for_llm.push_str(&function_ir_code);
        //     markdown_codeblock_for_llm.push('\n');
        // }
        //
        // Deduplicate library calls (by library + canonical_sig)
        // let total_occurrences = library_calls.len();
        // let mut seen_pairs = _HashSet::new();
        // library_calls.retain(|c| seen_pairs.insert((c.library.clone(), c.canonical_sig.clone())));
        // info!(
        //     "library calls found: occurrences={}, unique_functions={}",
        //     total_occurrences,
        //     library_calls.len()
        // );

        // // extract IR for library calls if any unique calls remain
        // if !library_calls.is_empty() {
        //     let code_for_library_calls = generate_library_funcs_markdown(&library_calls).await;
        //     // Only append if we actually have emitted function bodies
        //     if code_for_library_calls.contains("************ CODE FOR ") {
        //         info!(
        //             "adding {} library function code snippets to codeblock",
        //             library_calls.len()
        //         );
        //         print_first_n_lines(50, &code_for_library_calls);
        //         // visual separation before library section
        //         markdown_codeblock_for_llm.push_str("\n\n---\n\n");
        //         markdown_codeblock_for_llm.push_str(&code_for_library_calls);
        //     } else {
        //         info!("no library code available to append (no matches in mapping)");
        //     }
        // } else {
        //     info!("no library calls detected in IR; skipping library section");
        // }

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

        // if codeblock.contract.contains("LaunchpadV2Pair") {
        //     info!("CONTRACT => {}", codeblock.contract);
        //     info!("codeblock => {}", codeblock.content);
        // }
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

pub async fn get_interface_file_content(interface: &str, repo: &RepoPaths) -> Result<String> {
    let file = match get_file_from_interface(interface, repo).await {
        Some(filename) => filename,
        None => {
            info!("could not find file for interface {}", interface);
            return Ok(String::new());
        }
    };

    let file_content = fs::read_to_string(&file).await?;
    Ok(file_content)
}

/// Strip comments (//, /* */) and string literals from Solidity source to reduce regex false positives

/// Heuristic: treat well-known standard interfaces as skippable
fn is_standard_interface_name(name: &str) -> bool {
    matches!(
        name,
        "IERC20"
            | "IERC20Metadata"
            | "IERC20Permit"
            | "IERC165"
            | "IERC721"
            | "IERC721Metadata"
            | "IERC721Enumerable"
            | "IERC1155"
            | "IERC4626"
    )
}

fn path_is_standard_lib(path: &std::path::Path) -> bool {
    let p = path.to_string_lossy().to_ascii_lowercase();
    p.contains("openzeppelin") || p.contains("solmate") || p.contains("forge-std")
}

fn strip_comments_and_strings(src: &str) -> String {
    // Compile-once regexes
    static RE_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)/\*.*?\*/").unwrap());
    static RE_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)//[^\n]*").unwrap());
    static RE_DQ: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?s)\"(?:\\.|[^\"\\])*\""#).unwrap());
    static RE_SQ: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)'(?:\\.|[^'\\])*'").unwrap());

    // Remove strings first so comment markers inside strings are ignored
    let mut out = RE_DQ.replace_all(src, "\"\"").into_owned();
    out = RE_SQ.replace_all(&out, "''").into_owned();

    // Remove block and line comments
    out = RE_BLOCK.replace_all(&out, " ").into_owned();
    out = RE_LINE.replace_all(&out, " ").into_owned();

    out
}

/// Check if a contract's source file is in the scoped files list
async fn is_in_scoped_files(
    contract: &str,
    repo: &RepoPaths,
    scoped_files: &Vec<std::path::PathBuf>,
) -> bool {
    if scoped_files.is_empty() {
        return false;
    }
    match get_file_from_contract(contract, repo).await {
        Some(file) => scoped_files.contains(&file),
        None => false,
    }
}

/// Detect contracts and interfaces referenced in source code.
///
/// Slither's call graph does NOT include:
/// - Constructor calls: `new ContractName(...)`
/// - Interface casts: `IERC20(address)`
/// - Type declarations: `IERC20 public token`
/// - Import statements: `import { IERC20 } from "..."`
///
/// This function parses source code to find all these patterns.
///
/// # Arguments
/// * `contract` - The contract name to analyze
/// * `repo` - Repository paths
///
/// # Returns
/// * `(HashSet<String>, HashSet<String>)` - (contracts, interfaces) found in source
pub async fn detect_source_code_dependencies(
    contract: &str,
    repo: &RepoPaths,
) -> Result<(HashSet<String>, HashSet<String>)> {
    // Create cache key: "repo_hash:contract_name"
    let cache_key = format!("{}:{}", repo.unique_repo_hash(), contract);

    // Check cache first
    {
        let cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
        if let Some(cached_result) = cache.get(&cache_key) {
            info!(
                "cache hit for source dependencies of contract {} (repo: {})",
                contract,
                repo.unique_repo_hash()
            );
            return Ok(cached_result.clone());
        }
    }

    info!(
        "cache miss for source dependencies of contract {} (repo: {}), analyzing...",
        contract,
        repo.unique_repo_hash()
    );

    let mut contracts = HashSet::new();
    let mut interfaces = HashSet::new();

    // Get the source file for this contract
    let file = match get_file_from_contract(contract, repo).await {
        Some(f) => f,
        None => {
            info!(
                "could not find file for contract {} to detect dependencies",
                contract
            );
            // Cache the empty result to avoid re-attempting failed lookups
            let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
            cache.insert(cache_key, (contracts.clone(), interfaces.clone()));
            return Ok((contracts, interfaces));
        }
    };

    // Read the source code
    let source_code = match fs::read_to_string(&file).await {
        Ok(content) => content,
        Err(e) => {
            info!(
                "could not read file {} for dependency detection: {}",
                file.display(),
                e
            );
            return Ok((contracts, interfaces));
        }
    };
    // Strip comments and string literals to avoid false positives (e.g., "XOR (^)" in comments)
    let source_code = strip_comments_and_strings(&source_code);

    // 1. Constructor calls: new ContractName(...)
    let new_contract_regex = Regex::new(r"\bnew\s+([A-Z][A-Za-z0-9_]*)\s*\(").unwrap();
    for cap in new_contract_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let contract_name = name.as_str().to_string();
            // info!(
            //     "detected constructor call: new {}(...) in {}",
            //     contract_name, contract
            // );
            contracts.insert(contract_name);
        }
    }

    // 2. Interface/Contract casts: InterfaceName(address)
    // Matches: IERC20(token), IUniswapV2Pair(pair), ImmutableCreate2Factory(addr), etc.
    // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
    let type_cast_regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s*\([^)]*\)").unwrap();
    for cap in type_cast_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            // Filter out common Solidity keywords and built-in types
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected type cast (interface or contract): {}(...) in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!("detected contract cast: {}(...) in {}", type_name, contract);
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 3. Import statements: import { ContractName } from "..."
    // Note: Handles multiple comma-separated imports correctly via split(',')
    // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
    let import_regex = Regex::new(r#"import\s*\{([^}]+)\}\s*from"#).unwrap();
    for cap in import_regex.captures_iter(&source_code) {
        if let Some(imports) = cap.get(1) {
            for import in imports.as_str().split(',') {
                let name = import.trim();
                if name.starts_with('I')
                    && name.len() > 1
                    && name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected import (interface or contract): {} in {}",
                    //     name, contract
                    // );
                    interfaces.insert(name.to_string()); // Try as interface
                    contracts.insert(name.to_string()); // Also try as contract
                } else if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                    // info!("detected contract import: {} in {}", name, contract);
                    contracts.insert(name.to_string());
                }
            }
        }
    }

    // 4. Type declarations: ContractName/InterfaceName public/private/internal variable
    // Matches: IERC20 public token, TSwapPool private pool, ImmutableCreate2Factory immutable factory, etc.
    // Variable names: start with [a-zA-Z_], can contain [a-zA-Z0-9_]*
    // If starts with 'I', could be interface OR contract → add to BOTH (defensive)
    let type_decl_regex = Regex::new(
        r"\b([A-Z][A-Za-z0-9_]*)\s+(?:public|private|internal|immutable|constant)\s+[a-zA-Z_][a-zA-Z0-9_]*",
    )
    .unwrap();
    for cap in type_decl_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected type declaration (interface or contract): {} in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract type declaration: {} in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 5. Array types: ContractName[] or InterfaceName[size]
    // Matches: IVault[] public vaults, TSwapPool[10] pools, etc.
    // Detects BOTH interfaces and contracts
    let array_type_regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s*\[\s*\d*\s*\]").unwrap();
    for cap in array_type_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected array type (interface or contract): {}[] in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract array type: {}[] in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 6. Mapping value types: mapping(... => ContractName/InterfaceName)
    // Matches: mapping(address => IStrategy), mapping(uint => TSwapPool), etc.
    // Detects BOTH interfaces and contracts
    let mapping_value_regex =
        Regex::new(r"mapping\s*\([^)]+\s*=>\s*([A-Z][A-Za-z0-9_]*)\s*\)").unwrap();
    for cap in mapping_value_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected mapping value (interface or contract): mapping(...=> {}) in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract mapping value: mapping(...=> {}) in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 7. Mapping key types: mapping(ContractName/InterfaceName => ...)
    // Matches: mapping(IToken => uint256), mapping(TSwapPool => address), etc.
    // Detects BOTH interfaces and contracts
    let mapping_key_regex = Regex::new(r"mapping\s*\(\s*([A-Z][A-Za-z0-9_]*)\s*=>").unwrap();
    for cap in mapping_key_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected mapping key (interface or contract): mapping({} => ...) in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract mapping key: mapping({} => ...) in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 8. Function parameters: function foo(ContractName/InterfaceName param, ...)
    // Matches ALL parameters: function swap(IERC20 tokenIn, IPool pool, TSwapPool factory)
    // First extracts function signatures, then parses parameters within them
    // This avoids false positives from variable declarations and struct fields
    let function_sig_regex =
        Regex::new(r"function\s+[a-zA-Z_][a-zA-Z0-9_]*\s*\(([^)]*)\)").unwrap();
    let param_type_regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s+[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    for func_match in function_sig_regex.captures_iter(&source_code) {
        if let Some(params) = func_match.get(1) {
            // Now parse parameter types within this function signature only
            for param_cap in param_type_regex.captures_iter(params.as_str()) {
                if let Some(name) = param_cap.get(1) {
                    let type_name = name.as_str();
                    if !is_solidity_builtin(type_name) {
                        if type_name.starts_with('I')
                            && type_name.len() > 1
                            && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                        {
                            // info!(
                            //     "detected function parameter (interface or contract): {} in {}",
                            //     type_name, contract
                            // );
                            interfaces.insert(type_name.to_string()); // Try as interface
                            contracts.insert(type_name.to_string()); // Also try as contract
                        } else {
                            // info!(
                            //     "detected contract function parameter: {} in {}",
                            //     type_name, contract
                            // );
                            contracts.insert(type_name.to_string());
                        }
                    }
                }
            }
        }
    }

    // 9. Function return types: returns (ContractName/InterfaceName)
    // Matches: returns (IVault vault), returns (TSwapPool), etc.
    // Detects BOTH interfaces and contracts
    let return_type_regex = Regex::new(r"returns\s*\([^)]*\b([A-Z][A-Za-z0-9_]*)\b").unwrap();
    for cap in return_type_regex.captures_iter(&source_code) {
        if let Some(name) = cap.get(1) {
            let type_name = name.as_str();
            if !is_solidity_builtin(type_name) {
                if type_name.starts_with('I')
                    && type_name.len() > 1
                    && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                {
                    // info!(
                    //     "detected return type (interface or contract): {} in {}",
                    //     type_name, contract
                    // );
                    interfaces.insert(type_name.to_string()); // Try as interface
                    contracts.insert(type_name.to_string()); // Also try as contract
                } else {
                    // info!(
                    //     "detected contract return type: {} in {}",
                    //     type_name, contract
                    // );
                    contracts.insert(type_name.to_string());
                }
            }
        }
    }

    // 10. Struct fields: struct Data { ContractName/InterfaceName field; }
    // Matches ALL fields with semicolons: struct PoolData { IERC20 token; TSwapPool pool; }
    // First extracts full struct definitions, then parses fields within them
    // Uses semicolon to properly delimit field boundaries and handles multi-line structs
    let struct_block_regex = Regex::new(r"struct\s+[A-Z][A-Za-z0-9_]*\s*\{([^}]+)\}").unwrap();
    let struct_field_regex =
        Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\s+[a-zA-Z_][a-zA-Z0-9_]*\s*;").unwrap();
    for struct_match in struct_block_regex.captures_iter(&source_code) {
        if let Some(struct_body) = struct_match.get(1) {
            // Now find all field types within this struct body
            for field_cap in struct_field_regex.captures_iter(struct_body.as_str()) {
                if let Some(name) = field_cap.get(1) {
                    let type_name = name.as_str();
                    if !is_solidity_builtin(type_name) {
                        if type_name.starts_with('I')
                            && type_name.len() > 1
                            && type_name.chars().nth(1).map_or(false, |c| c.is_uppercase())
                        {
                            // info!(
                            //     "detected struct field (interface or contract): {} in {}",
                            //     type_name, contract
                            // );
                            interfaces.insert(type_name.to_string()); // Try as interface
                            contracts.insert(type_name.to_string()); // Also try as contract
                        } else {
                            // info!(
                            //     "detected contract struct field: {} in {}",
                            //     type_name, contract
                            // );
                            contracts.insert(type_name.to_string());
                        }
                    }
                }
            }
        }
    }

    // Validate discovered names against known mappings to avoid false positives
    let mut filtered_contracts: HashSet<String> = HashSet::new();
    for c in &contracts {
        if get_file_from_contract(c, repo).await.is_some() {
            filtered_contracts.insert(c.clone());
        } else {
            // info!(
            //     "ignoring source dependency candidate '{}' (no contract file found)",
            //     c
            // );
        }
    }

    let mut filtered_interfaces: HashSet<String> = HashSet::new();
    for i in &interfaces {
        if get_file_from_interface(i, repo).await.is_some() {
            filtered_interfaces.insert(i.clone());
        } else {
            // info!(
            //     "ignoring source dependency candidate '{}' (no interface file found)",
            //     i
            // );
        }
    }

    // Cache the result before returning
    {
        let mut cache = SOURCE_DEPENDENCY_CACHE.lock().unwrap();
        cache.insert(
            cache_key,
            (filtered_contracts.clone(), filtered_interfaces.clone()),
        );
    }

    info!(
        "cached source dependencies for contract {} (repo: {}): {} contracts, {} interfaces",
        contract,
        repo.unique_repo_hash(),
        filtered_contracts.len(),
        filtered_interfaces.len()
    );

    Ok((filtered_contracts, filtered_interfaces))
}

/// Check if a type name is a Solidity built-in type or common library type
///
/// Covers all Solidity built-in types and common false positives:
/// - Value types: address, bool, string, bytes (lowercase in Solidity)
/// - Integer types: uint8-uint256, int8-int256 (lowercase in Solidity)
/// - Fixed bytes: bytes1-bytes32 (lowercase in Solidity)
/// - Common libraries: Math, SafeMath, Strings, etc. (PascalCase)
///
/// Note: Solidity built-in types are lowercase, but our regex captures PascalCase types.
/// We filter by checking if the LOWERCASE version is a built-in type.
fn is_solidity_builtin(name: &str) -> bool {
    let name_lower = name.to_lowercase();

    // Exact matches for common types (check lowercase version)
    if matches!(
        name_lower.as_str(),
        // Solidity built-in types (always lowercase in actual code)
        "string" | "bytes" | "address" | "uint" | "int" | "bool" |
        // Memory location keywords
        "memory" | "storage" | "calldata"
    ) {
        return true;
    }

    // Common OpenZeppelin/library names (PascalCase - check original name)
    if matches!(
        name,
        "Math"
            | "SafeMath"
            | "Strings"
            | "Arrays"
            | "EnumerableSet"
            | "EnumerableMap"
            | "Counters"
            | "SafeCast"
            | "SignedMath"
            | "Checkpoints"
            | "Context"
            | "Ownable"
    ) {
        return true;
    }

    // Pattern matches for sized types (check lowercase version)
    // uint8, uint16, uint24, ..., uint256
    if name_lower.starts_with("uint") && name_lower.len() > 4 {
        if let Ok(_) = name_lower[4..].parse::<u16>() {
            return true;
        }
    }

    // int8, int16, int24, ..., int256
    if name_lower.starts_with("int") && name_lower.len() > 3 {
        if let Ok(_) = name_lower[3..].parse::<u16>() {
            return true;
        }
    }

    // bytes1, bytes2, ..., bytes32
    if name_lower.starts_with("bytes") && name_lower.len() > 5 {
        if let Ok(n) = name_lower[5..].parse::<u8>() {
            return n >= 1 && n <= 32;
        }
    }

    false
}

pub fn robust_extract_fn_metadata_from_func_id(
    func_id: &str,
    parent_func: &SmartContractFunction,
    contract: &str,
    semantic_db: &Connection,
    repo: &RepoPaths,
) -> Result<Option<SmartContractFunction>> {
    let mut callee_fn =
        get_function_metadata_from_id(func_id, repo, semantic_db)?.unwrap_or_default();

    // if unwarp was not successful
    if callee_fn.id.is_empty() {
        // info!("could not find caller to {}", &func_id);

        let (new_contract, fn_sig) =
            check_internal_calls_to_find_contract_of_fn_call(func_id, &contract, &parent_func.ir)
                .unwrap_or_default();

        if new_contract.is_empty() {
            return Ok(None);
        }

        // info!(
        //     "in INTERNAL_CALL found contract: {}, and fn sig {}",
        //     new_contract, fn_sig
        // );

        callee_fn =
            get_function_metadata_from_contract_plus_fn(&new_contract, &fn_sig, semantic_db)?
                .unwrap_or_default();

        // info!(
        //     "found IR for fn sig with token count: {}",
        //     get_token_count(&callee_fn.ir)
        // );

        if callee_fn.id.is_empty() {
            return Ok(None);
        }
    }

    // handle functions called by contract interfaces (starts with 'I')
    let updated_callee_fn = if callee_fn.ir.is_empty()
        && string_starts_with_char(&callee_fn.contract, 'I')
    {
        find_fn_metadata_from_contract_interface_call(&callee_fn, semantic_db)?.unwrap_or(callee_fn)
    } else {
        callee_fn
    };

    // handle edge case where contract cannot be found in protocol but Mock exists
    let final_update_callee_fn = if updated_callee_fn.ir.is_empty() {
        find_fn_metadata_from_potential_contract_mock(&updated_callee_fn, semantic_db)?
            .unwrap_or(updated_callee_fn)
    } else {
        updated_callee_fn
    };

    Ok(Some(final_update_callee_fn))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallEdge {
    /// Internal call; `contract` is None if unqualified (assume current contract)
    Internal {
        contract: Option<String>,
        fn_sig: String,
    },
}

// if contract name in function is I<contract_name> or II<contract_name>
// extract contract_name and see if can use contract_name + function name to find IR
fn find_fn_metadata_from_contract_interface_call(
    func: &SmartContractFunction,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    if string_starts_with_char(&func.contract, 'I') {
        // info!(
        //     "looking for contract name {} from interface {}...",
        //     func.contract, func.name
        // );
        let mut new_contract = func.contract.clone();
        new_contract.remove(0);
        let func_metadata =
            get_function_metadata_from_contract_plus_fn(&new_contract, &func.name, semantic_db)?
                .unwrap_or_default();

        if func_metadata.id.is_empty() {
            // recursive call incase contract name has MULTIPLE I's (ie. II<contract_name>)
            // info!(
            //     "no interface connected name found for {}, with fn interface: {}",
            //     new_contract, func.name
            // );
            let next_func = SmartContractFunction {
                contract: new_contract,
                ..func.clone()
            };
            find_fn_metadata_from_contract_interface_call(&next_func, semantic_db)
        } else {
            // info!(
            //     "contract found for {} with IR of token count: {}",
            //     new_contract,
            //     get_token_count(&func_metadata.ir)
            // );
            Ok(Some(func_metadata))
        }
    } else {
        // EDGE CASE: check if mock function exits
        let func_metadata = find_fn_metadata_from_potential_contract_mock(func, semantic_db)?;
        Ok(func_metadata)
    }
}

// see if <contract_name>Mock exists
fn find_fn_metadata_from_potential_contract_mock(
    func: &SmartContractFunction,
    semantic_db: &Connection,
) -> Result<Option<SmartContractFunction>> {
    let new_contract = format!("{}{}", func.contract, "Mock");

    // info!(
    //     "looking for mock of contract {} with fn interface: {}...",
    //     func.contract, func.name
    // );
    let func_metadata =
        get_function_metadata_from_contract_plus_fn(&new_contract, &func.name, semantic_db)?
            .unwrap_or_default();

    if func_metadata.id.is_empty() {
        return Ok(None);
    }

    info!(
        "mock contract found for {} with ir of token count: {}",
        new_contract,
        get_token_count(&func_metadata.ir)
    );
    Ok(Some(func_metadata))
}

pub fn check_internal_calls_to_find_contract_of_fn_call(
    func_id: &str,
    contract: &str,
    ir_of_parent_fn: &str,
) -> Option<(String, String)> {
    // extract function name from func_id (i.e. 124_func => func)
    let fn_name = get_function_name_from_func_id(func_id);

    // get internal call fn -> contract mapping
    let fn_to_contract_map = internal_call_edge_map(ir_of_parent_fn, contract);
    fn_to_contract_map.get(&fn_name).cloned()
}

/// Parse a single SlithIR SSA line for INTERNAL_(DYNAMIC_)CALL and LIBRARY_CALL.
/// Returns a normalized call edge with canonical signature "name(type1,type2)".
pub fn parse_call_edge_line(line: &str) -> Option<CallEdge> {
    // INTERNAL_CALL with qualifier: Contract.fn(types)(args...)
    static RE_INTERNAL_QUAL: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"INTERNAL_(?:DYNAMIC_)?CALL\s*,\s*([A-Za-z_]\w*)\.([A-Za-z_]\w*)\(([^)]*)\)"#)
            .unwrap()
    });

    // INTERNAL_CALL without qualifier: fn(types)(args...)  (assume same contract)
    static RE_INTERNAL_UNQUAL: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"INTERNAL_(?:DYNAMIC_)?CALL\s*,\s*([A-Za-z_]\w*)\(([^)]*)\)"#).unwrap()
    });

    if let Some(c) = RE_INTERNAL_QUAL.captures(line) {
        let contract = c.get(1).unwrap().as_str().to_string();
        let name = c.get(2).unwrap().as_str();
        let params = c.get(3).map(|m| m.as_str()).unwrap_or("").trim();
        let canonical_sig = format!("{}({})", name, params);
        return Some(CallEdge::Internal {
            contract: Some(contract),
            fn_sig: canonical_sig,
        });
    }

    if let Some(c) = RE_INTERNAL_UNQUAL.captures(line) {
        let name = c.get(1).unwrap().as_str();
        let params = c.get(2).map(|m| m.as_str()).unwrap_or("").trim();
        let canonical_sig = format!("{}({})", name, params);
        return Some(CallEdge::Internal {
            contract: None,
            fn_sig: canonical_sig,
        });
    }

    None
}

/// Collect call edges from a whole IR block; supply `current_contract` to resolve unqualified INTERNAL_CALLs.
/// Results are cached globally to avoid redundant parsing of the same IR blocks.
pub fn internal_call_edge_map(
    ir_block: &str,
    current_contract: &str,
) -> HashMap<String, (String, String)> {
    // Create a cache key from IR block hash and contract name
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    ir_block.hash(&mut hasher);
    current_contract.hash(&mut hasher);
    let cache_key = format!("{}:{}", hasher.finish(), current_contract);

    // Check cache first
    {
        let cache = INTERNAL_CALL_EDGE_CACHE.lock().unwrap();
        if let Some(cached_result) = cache.get(&cache_key) {
            return cached_result.clone();
        }
    }

    // Cache miss - compute the result
    let mut out = HashMap::new();
    for line in ir_block.lines() {
        if let Some(edge) = parse_call_edge_line(line) {
            match edge {
                CallEdge::Internal { contract, fn_sig } => {
                    let callee_contract = contract.unwrap_or_else(|| current_contract.to_string());
                    let fn_name = get_function_name_from_interface(&fn_sig);
                    out.insert(fn_name, (callee_contract, fn_sig));
                }
            }
        }
    }

    // Store in cache
    {
        let mut cache = INTERNAL_CALL_EDGE_CACHE.lock().unwrap();
        cache.insert(cache_key, out.clone());
    }

    out
}
