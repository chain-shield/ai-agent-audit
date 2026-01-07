/// Phase 2: Parallel vulnerability detection across multiple AI agents
///
/// This phase orchestrates parallel security analysis using multiple AI agents
/// to discover potential vulnerabilities in smart contracts.
use crate::{
    config::{
        ADDITIONAL_CONTEXT_SECTION, CODE_SECTION, INVARIANT_OR_ACTOR_SECTION, INVARIANT_RUNS,
    },
    error::Result,
    llm_review::{
        agent::agent_enums::AIAgent,
        analysis::{
            context_state::{generate_audit_scope, get_metadata_context},
            semaphore::GENERAL_SEM,
        },
        dynamic_prompts::{
            self, actors,
            invariants::{self, generate_invariant_prompt, get_invariant_json},
            prompt_index,
        },
        threat_models::{
            issues::{IssuePrompt, IssueStructTrait},
            pattern_category::get_category_library_spec,
        },
    },
    prepare_code::git_clone::RepoPaths,
    reporting::save_file,
};

use log::info;
use serde::de::DeserializeOwned;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Executes the findings generation phase
///
/// Runs parallel security analysis across multiple AI agents to discover
/// potential vulnerabilities in the provided smart contract code.
pub async fn execute<T>(
    issue_prompt: IssuePrompt,
    code: &str,
    arc_agent: &Arc<AIAgent>,
    repo: &RepoPaths,
) -> Result<T>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
{
    let issue_title = match issue_prompt {
        IssuePrompt::Combined(_) => "vulnerability patterns",
        IssuePrompt::Invariant(_) => "invariants",
    };
    info!(
        "🔍 Phase 1: Generating {} from contract codebase...",
        issue_title
    );

    let mut handles = vec![];
    let all_patterns = Arc::new(Mutex::new(T::default()));

    let context = get_metadata_context(repo)
        .await
        .expect("could not extract context");

    let audit_scope = generate_audit_scope(repo).await?;
    let section_10_header = prompt_index::generated_section_header(
        "AUDIT SCOPE AND KEY INVARIANTS PROVIDED BY CLIENT",
        10,
    );

    let combined_context = if audit_scope.is_empty() {
        context
    } else {
        format!(
            r#"
{context}

{section_10_header}

{audit_scope}
        "#
        )
    };

    let codeblock = Arc::new(code.to_string());

    let added_content_from_brain = Arc::new(combined_context);
    let code_plus_context =
        generate_content_plus_context_block(&codeblock, &added_content_from_brain);

    // Simple local closure to DRY out spawn logic without extra generics
    let mut spawn_run = |prompt: Arc<String>| {
        let agent = Arc::clone(arc_agent);
        let sem = Arc::clone(&GENERAL_SEM);
        let shared_patterns = Arc::clone(&all_patterns);

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            if let Err(e) =
                run_security_prompt(agent, "security exploits", prompt, shared_patterns).await
            {
                log::error!("Prompt task failed: {e:#}");
            }
        }));
    };

    match issue_prompt {
        IssuePrompt::Combined((pattern_category, some_actors, some_invariants)) => {
            let (actor_context, actor_index) = if some_actors.is_some() {
                let actors = some_actors.clone().unwrap_or_default();
                actors::generate_formated_list_from_actor_data(
                    &actors.actors,
                    INVARIANT_OR_ACTOR_SECTION,
                )
            } else {
                (String::new(), String::new())
            };

            let (invariant_context, invariant_index) = if some_invariants.is_some() {
                let invariants = some_invariants.clone().unwrap_or_default();
                invariants::generate_full_list_of_invariant_findings(
                    &invariants,
                    INVARIANT_OR_ACTOR_SECTION,
                )
            } else {
                (String::new(), String::new())
            };

            for category in pattern_category.into_iter() {
                let category_spec =
                    get_category_library_spec(&category).expect("could not extract category spec");

                // construct prompt
                let (instruction_prompt, pattern_index) =
                    dynamic_prompts::findings::generate_pattern_category_to_findings_prompt(
                        &category, repo,
                    );
                let json_requirement_prompt =
                    dynamic_prompts::findings_template::get_post_json_requirement_for_multipattern(
                        &category_spec.issues,
                        "security vulnerability pattern",
                        repo,
                    );

                // generate table of contents for each prompt
                let prompt_index_actors =
                    prompt_index::generate_pattern_category_to_finding_discovery_prompt(
                        &pattern_index,
                        &actor_index,
                    );
                let prompt_index_invariants =
                    prompt_index::generate_pattern_category_to_finding_discovery_prompt(
                        &pattern_index,
                        &invariant_index,
                    );

                let prompt_actors = Arc::new(format!(
                    "{prompt_index_actors}{instruction_prompt}{actor_context}{code_plus_context}{json_requirement_prompt}"
                ));
                save_file::save_file_locally(&prompt_actors, &PathBuf::from("actor_prompt.md"))?;

                let prompt_invariant = Arc::new(format!(
                    "{prompt_index_invariants}{instruction_prompt}{invariant_context}{code_plus_context}{json_requirement_prompt}"
                ));
                save_file::save_file_locally(
                    &prompt_invariant,
                    &PathBuf::from("invariant_prompt.md"),
                )?;

                panic!("done with saving prompt files..");

                for run in 0..category_spec.runs {
                    if some_actors.is_some() {
                        info!(
                            "---- #{} LLM analysis Round for Finding with Actors----",
                            run + 1
                        );
                        spawn_run(Arc::clone(&prompt_actors));
                    }
                    if some_invariants.is_some() {
                        info!(
                            "---- #{} LLM analysis Round for Finding with Invariants----",
                            run + 1
                        );
                        spawn_run(Arc::clone(&prompt_invariant));
                    }
                }
            }
        }
        IssuePrompt::Invariant(invariants) => {
            let inv_prompt = Arc::new(generate_invariant_prompt(&invariants));
            let json_requirement_prompt = Arc::new(get_invariant_json(&invariants));
            let prompt = Arc::new(format!(
                "{inv_prompt}{code_plus_context}{json_requirement_prompt}"
            ));
            // info!("invariant prompt => {}", prompt);
            for _ in 0..INVARIANT_RUNS {
                spawn_run(Arc::clone(&prompt));
            }
        }
    }

    // Wait for ALL tasks to complete
    for handle in handles {
        handle.await?; // Will error if task panicked
    }

    let findings = all_patterns.lock().await;

    if !findings.issues().is_empty() {
        info!(
            "✅ Phase 1 complete: {} {} BEFORE deduping",
            issue_title,
            findings.issues().len()
        );
    }
    Ok(findings.clone())
}

/// Executes one LLM-prompt round and merges the returned findings into the shared findings collection
///
/// This function handles individual security analysis rounds, managing prompt generation,
/// LLM interaction, and result aggregation.
pub async fn run_security_prompt<T>(
    agent: Arc<AIAgent>,
    title: &str,
    prompt: Arc<String>,
    shared_patterns: Arc<Mutex<T>>,
) -> Result<()>
where
    T: 'static + IssueStructTrait + Send + Sync + Default + Clone + DeserializeOwned,
{
    // 2. Send to the right provider
    let patterns: T = agent.extract_with_retry(&prompt).await?;

    let issues_found = patterns.issues().len();
    info!("{} {} found!", issues_found, title);

    // 3. Merge results (if any) into the shared accumulator
    if issues_found > 0 {
        let mut guard = shared_patterns.lock().await;
        guard.issues_mut().extend_from_slice(patterns.issues());
    }

    Ok(())
}

/// Generates the combined content and context block for LLM analysis
///
/// Combines the contract code with additional context information
/// in a structured format for optimal LLM processing.
fn generate_content_plus_context_block(codeblock: &str, added_context: &str) -> String {
    let mut code_plus_context = String::new();
    let section_8_header =
        prompt_index::generated_section_header("SOLIDITY CODE TO REVIEW", CODE_SECTION);
    let section_9_header =
        prompt_index::generated_section_header("ADDITIONAL CONTEXT", ADDITIONAL_CONTEXT_SECTION);

    code_plus_context.push_str(&format!(
        r#"
{}
                "#,
        section_8_header
    ));
    code_plus_context.push_str(codeblock);

    code_plus_context.push_str(&format!(
        r#"

{}

        "#,
        section_9_header
    ));
    code_plus_context.push_str(&added_context);
    code_plus_context.push_str("\n\n");

    code_plus_context
}
