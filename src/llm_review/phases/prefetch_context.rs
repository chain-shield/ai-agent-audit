/// Phase 1: AI-driven file selection and context prefetching
///
/// This phase uses AI agents to intelligently select the most strategically important
/// files for security analysis, reducing rate limits while maximizing context quality.
use crate::{
    ai_bot::file_picker::{FilePickerArgs, FilePickerTool, MAX_FILES_PER_CALL},
    config::MAX_FILE_RUNS,
    cost::cost_data::{TokenType, add_to_inference_cost_by_agent},
    error::Result,
    llm_review::{
        enums::AIAgent,
        prompt_support::{
            post_file_select_prompt::POST_FILE_SELECT, pre_file_select_prompt::PRE_FILE_SELECT,
        },
    },
    prepare_code::git_clone::RepoPaths,
};
use log::info;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// Files selected by AI for strategic context gathering
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SelectedFiles {
    pub files: Vec<String>,
}

/// Executes the prefetch context phase
///
/// Uses AI agents to intelligently select and fetch the most relevant files
/// for security analysis based on the contract code being audited.
pub async fn execute(code: &str, repo: &RepoPaths, agent: &Arc<AIAgent>) -> Result<String> {
    info!("🔍 Phase 1: Pre-fetching strategic files for analysis...");
    let mut handles = vec![];

    // Create a temporary file picker tool
    let file_picker = FilePickerTool::new(repo.clone());

    // Select the most important files for security auditing
    // Prioritize: main contracts, test files, and critical dependencies
    let available_files = &file_picker.available_files;

    let available_files_list = available_files
        .iter()
        .map(|f| format!("- {}", f))
        .collect::<Vec<_>>()
        .join("\n");

    // Create AI prompt for intelligent file selection
    let instructions = format!(
        "
        ## Context
        Below is the Solidity contract code and storage layout that you will audit.

        ## Goal
        From the list of available project files, select the **most strategically important files** that provide enough context for identifying security vulnerabilities.

        Prioritize:
        1. **Main Solidity contract files** related to core protocol logic.
        2. **Test files** that expose edge cases, invariants, or critical scenarios.
        3. **Deployment scripts** or configuration files that influence how contracts behave on-chain.
        4. **Key libraries** (especially those handling math, access control, or external integrations).

        ⚠️ You may select **up to {MAX_FILES_PER_CALL} files only**. Balance **completeness** with **token efficiency**.

        ## Available Files:
            {available_files_list}
        ");

    let prompt = Arc::new(format!(
        "{}\n{}\n\n## SOLIDITY CONTRACT + STORAGE TO REVIEW \n\n{}\n{}",
        PRE_FILE_SELECT, instructions, code, POST_FILE_SELECT
    ));

    let picked_files_hash: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::<String, usize>::new()));

    // Run multiple rounds to build consensus on file selection
    for i in 0..MAX_FILE_RUNS {
        info!("round {}: picking files to add to context", i + 1);
        let file_picker_prompt = Arc::clone(&prompt);
        let ai_agent = Arc::clone(&agent);
        let selected_files_hash = Arc::clone(&picked_files_hash);

        handles.push(tokio::spawn(async move {
            let result: Result<()> = async {
                add_to_inference_cost_by_agent(&file_picker_prompt, &ai_agent, TokenType::Input)
                    .await;

                let selected_files: SelectedFiles =
                    ai_agent.extract_with_retry(&file_picker_prompt).await?;

                let mut files_hash = selected_files_hash.lock().await;

                for file in selected_files.files {
                    files_hash
                        .entry(file)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
                Ok(())
            }
            .await;
            if let Err(e) = result {
                log::error!("Error picking files in round {}: {:?}", i, e);
            }
        }))
    }

    // Wait for all rounds to complete
    for h in handles {
        let _ = h.await;
    }

    // Extract consensus results
    let files_hash = Arc::try_unwrap(picked_files_hash)
        .expect("Arc still has multiple owners")
        .into_inner();

    let selected_files = top_n_files(files_hash, MAX_FILES_PER_CALL);

    if selected_files.is_empty() {
        info!("No additional files available for pre-fetching");
        return Ok(String::new());
    }

    // Fetch the selected files
    let args = FilePickerArgs {
        files: selected_files.clone(),
    };

    match file_picker.call(args).await {
        Ok(output) => {
            info!(
                "✅ Pre-fetched {} files: [{}]",
                selected_files.len(),
                selected_files.join(", ")
            );
            Ok(output.content)
        }
        Err(e) => {
            log::warn!("Failed to pre-fetch files: {}", e);
            Ok(String::new()) // Return empty string on failure, don't crash
        }
    }
}

/// Selects the top N files based on consensus voting
///
/// Files with higher vote counts are prioritized, ensuring the most
/// consistently selected files across multiple AI evaluation rounds.
fn top_n_files(files_hash: HashMap<String, usize>, max_files: usize) -> Vec<String> {
    let mut files: Vec<(String, usize)> = files_hash.into_iter().collect();

    // Sort by count descending (most voted files first)
    files.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // Take top N files
    files
        .into_iter()
        .take(max_files)
        .map(|(file, _)| file)
        .collect()
}
