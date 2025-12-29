use anyhow::Result;
/// Global context state management for AI analysis.
///
/// This module manages shared protocol metadata context that is generated once
/// and reused across all AI agents for consistent analysis. Provides thread-safe
/// access to protocol information including summaries and semantic data.
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    build_brain::{
        slither_ffi::{cache_key, get_all_files_src},
        summarize::{FileSummaryType, summarize_protocol, summarize_src_files},
    },
    cost::cost_data::get_token_count,
    llm_review::{
        analysis::pre_audit_analysis,
        dynamic_prompts::{actors, invariants},
    },
    prepare_code::git_clone::RepoPaths,
};

/// Multi-modal context containing actor and invariant analysis for a contract.
///
/// This context is generated once per contract during the pre-audit analysis phase
/// and cached for reuse across all vulnerability detection and verification phases.
/// It provides enriched context to AI agents by including:
///
/// - **Actors**: Formatted list of potential bad actors and their capabilities
/// - **Invariants**: Formatted list of verified protocol invariants
///
/// # Usage
///
/// The context is generated via [`generate_multi_modal_context`] and cached globally.
/// Subsequent calls to [`get_multi_modal_context`] retrieve the cached version,
/// avoiding redundant LLM calls.
///
/// # Example
///
/// ```rust,ignore
/// // Generate and cache context (called once per contract)
/// let context = generate_multi_modal_context(&codeblock, &contract, repo).await?;
///
/// // Retrieve cached context (called during pattern detection and verification)
/// let cached = get_multi_modal_context(&contract, repo).await;
/// ```
#[derive(Clone, Debug)]
pub struct MultiModalContext {
    /// Formatted list of actors and their capabilities relevant to the contract.
    ///
    /// This string contains a human-readable description of potential bad actors
    /// (e.g., malicious users, MEV bots, compromised admins) and their capabilities
    /// within the protocol context.
    pub actors: String,

    /// Formatted list of verified invariants for the contract.
    ///
    /// This string contains verified protocol invariants with their predicates,
    /// pre/post states, and relevant code locations. These invariants are used
    /// to guide vulnerability detection and verification.
    pub invariants: String,
}

/// Global metadata context shared across all AI agents

pub static PROMPT_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub static MULTI_MODAL_CONTEXT: Lazy<Arc<Mutex<HashMap<String, MultiModalContext>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub static METADATA_CONTEXT: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn get_context_key(repo: &RepoPaths) -> String {
    cache_key(&repo.root, "prompt_context", None)
}
/// Generates and caches protocol metadata context for AI analysis.
///
/// This function creates comprehensive context information including protocol
/// summaries, contract relationships, and semantic data that is shared across
/// all AI agents for consistent analysis.
///
/// # Arguments
/// * `repo` - Repository paths and metadata
/// * `semantics_path` - Path to semantic analysis database
pub async fn generate_and_save_metadata_context(repo: &RepoPaths) -> anyhow::Result<()> {
    let mut metadata = String::new();

    // full context
    let full_context = generate_context_for_code_review(repo).await?;

    let summaries = summarize_src_files(repo).await?;

    // NOTE: adding file summaries to full context ONLY to create protocol_summary
    // final metadata DOES NOT CONTAIN file summaries, because its too many tokens, and we no
    // longer need it for audit context
    let mut full_context_plus_summaries = full_context.clone();
    full_context_plus_summaries.push_str("\n## SUMMARY OF SOURCE CODE FILES\n\n");
    for summary in summaries {
        let file_type = summary.file_type.unwrap_or(FileSummaryType::OutOfScope);
        if file_type == FileSummaryType::Source {
            full_context_plus_summaries.push_str(&format!("### Summary of {}\n", summary.filename));
            full_context_plus_summaries.push_str(&summary.summary);
            full_context_plus_summaries.push_str("\n");
        }
    }

    let protocol_summary = summarize_protocol(repo, Some(&full_context_plus_summaries)).await?;

    metadata.push_str(&format!(
        "\n## PROTOCOL OVERVIEW:\n\n{}\n\n",
        protocol_summary
    ));

    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let mut metadata_cache = metadata_context.lock().await;

    // saving full context
    metadata_cache.insert(
        get_context_key(repo),
        format!("{}{}", metadata, full_context),
    );

    Ok(())
}

/// Retrieves the cached metadata context for AI analysis.
///
/// Returns the protocol metadata context that was previously generated and
/// cached for use across all AI agents.
///
/// # Returns
/// * `String` - Cached protocol metadata context
pub async fn get_metadata_context(repo: &RepoPaths) -> Option<String> {
    let key = get_context_key(repo);
    let metadata_context = Arc::clone(&METADATA_CONTEXT);
    let metadata_cache = metadata_context.lock().await;
    // Return cached output if exists
    metadata_cache.get(&key).cloned()
}

pub async fn generate_context_for_code_review(repo: &RepoPaths) -> Result<String> {
    let mut full_prompt_context = String::new();

    let src_file_list = get_all_files_src(repo)?;
    log::info!("src file list => {}", src_file_list);

    // prompt_context.push_str("\n## Slither Contract Summary\n");
    // prompt_context.push_str(&contract_summary);
    full_prompt_context.push_str("\n## Main List of Files in Project\n\n");
    full_prompt_context.push_str(&src_file_list);
    full_prompt_context.push_str("\n\n");

    // let docs = summarize::summarize_docs(repo, &full_prompt_context).await?;
    let documentation = repo.extract_content_from_docs()?;
    // adding FULL DOCS not doc_summaries
    full_prompt_context.push_str("\n ## DOCUMENTATION: \n\n ");
    full_prompt_context.push_str(&documentation);

    let lib_config_headers = repo.extract_lib_config_headers()?;
    full_prompt_context.push_str("\n ## PACKAGE.JSON HEADERS OF LIB PACKAGES: \n");
    full_prompt_context.push_str("\n Note: Check for important lib version info\n\n ");
    full_prompt_context.push_str("\n When code reviewing be mindful of which version of openzepplin, chainlink, etc the package version is using.\n\n ");
    full_prompt_context.push_str(&lib_config_headers);

    let config_files_content = repo.extract_content_from_config_files()?;
    // adding config files: foundry.toml, package.json, etc
    full_prompt_context.push_str("\n ## CONFIG FILES: \n");
    full_prompt_context.push_str("\n Note: Check for important package version info.\n\n ");
    full_prompt_context.push_str(&config_files_content);

    log::info!(
        "documentation full token count => {}",
        get_token_count(&documentation)
    );
    log::info!(
        "prompt context token count => {}",
        get_token_count(&full_prompt_context)
    );

    Ok(full_prompt_context)
}

pub async fn generate_audit_scope(repo: &RepoPaths) -> Result<String> {
    let key = cache_key(&repo.root, "audit_scope", None);

    // Return cached output if exists
    if let Some(cached) = METADATA_CONTEXT.lock().await.get(&key).cloned() {
        return Ok(cached);
    }

    let mut audit_scope = "#r 

        ## Privileged Roles 

        All Privileged Roles are TRUSTED by default unless listed as untrusted below.

        Errors and misuse committed by admin (or any other privileged role) are considered
        **governance risk, NOT vulnerabilities**.  They will get marked as Low or Informational.

        *Caveat*: If the admin can accidentally brick the protocol even while following spec (no malice or error) 
        — that can rise to Medium. 
        Example: a valid function like updateFee() can unintentionally revert all 
        deposits if called with a certain boundary value, even though the admin followed expected usage.
#".to_string();

    let protocol_specific_audit_scope = repo.extract_content_from_scope_file()?;

    audit_scope.push_str(&format!("\n\n {}", protocol_specific_audit_scope));

    // 1 . gather IR + storage  (re-use existing function)
    log::info!("get audit scope from file...");
    log::info!(
        "audit scope token count => {}",
        get_token_count(&audit_scope)
    );

    let cache = Arc::clone(&METADATA_CONTEXT);
    let mut context_cache = cache.lock().await;
    context_cache.insert(key, audit_scope.clone());
    Ok(audit_scope)
}

/// Generate cache key for multi-modal context (actors, invariants, etc.)
fn generate_multimodal_key(contract: &str, repo: &RepoPaths) -> String {
    format!("{}-{}", repo.root.to_string_lossy(), contract)
}

// generated orthogonal thread models: bad actors, invariants etc to add as supporting context for
// finding (and verifying) security vulnerability
pub async fn generate_multi_modal_context(
    codeblock: &str,
    contract: &str,
    repo: &RepoPaths,
) -> Result<MultiModalContext> {
    let key = generate_multimodal_key(contract, repo);
    let multimodal = Arc::clone(&MULTI_MODAL_CONTEXT);
    let multimodal_cache = multimodal.lock().await;

    if let Some(multimodal_context) = multimodal_cache.get(&key) {
        return Ok(multimodal_context.clone());
    }

    // Release lock before expensive operation
    drop(multimodal_cache);

    let actors = pre_audit_analysis::generate_actors(codeblock, repo).await?;
    let actors_capabilities = actors::generate_formated_list_from_actor_data(&actors.actors);

    let invariants = pre_audit_analysis::generate_invariants(codeblock, repo).await?;
    let invariant_list = invariants::generate_full_list_of_invariant_findings(&invariants);

    let multimodal_context = MultiModalContext {
        actors: actors_capabilities,
        invariants: invariant_list,
    };

    // CACHE RESULT
    let mut multimodal_cache = multimodal.lock().await;
    multimodal_cache.insert(key, multimodal_context.clone());
    Ok(multimodal_context)
}

pub async fn get_multi_modal_context(
    contract: &str,
    repo: &RepoPaths,
) -> Option<MultiModalContext> {
    let key = generate_multimodal_key(contract, repo);
    let multimodal = Arc::clone(&MULTI_MODAL_CONTEXT);
    let multimodal_cache = multimodal.lock().await;

    if let Some(multimodal_context) = multimodal_cache.get(&key) {
        Some(multimodal_context.clone())
    } else {
        None
    }
}
