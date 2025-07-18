use std::collections::HashSet;

use anyhow::Result;
use log::info;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::providers::openai::TEXT_EMBEDDING_3_SMALL;
use rig::{
    agent::{Agent, AgentBuilder},
    client::{CompletionClient, EmbeddingsClient},
    completion::Prompt,
    providers::openai::{Client, CompletionModel, GPT_4O},
    vector_store::VectorStoreIndex,
};
use rig_qdrant::QdrantVectorStore;
/// AI agent implementations with vector-based context retrieval.
///
/// This module provides intelligent AI agents that combine static documentation
/// with dynamic vector search for contextual smart contract analysis.
use tiktoken_rs::cl100k_base;

use crate::ai_bot::file_retrival::FileRetrievalTool;
use crate::build_brain::enbeddings::SourceChunk;
use crate::config::MAX_RAG_QUERY_CONTENT_LENGTH;
use crate::prepare_code::git_clone::RepoPaths;
use crate::utils::get_file_content::extract_content_from_docs;
use crate::utils::logging::print_first_four_lines;

/// Creates an AI audit agent with vector-based dynamic context retrieval and file search tools.
///
/// This agent combines static documentation context with dynamic vector search
/// and provides a file retrieval tool for targeted code analysis across different file types.
///
/// # Features
/// - Static documentation context from repository
/// - Dynamic vector search for relevant code chunks
/// - File retrieval tool with support for 4 file types:
///   - `source`: Main application code and smart contracts
///   - `test`: Test files and test cases
///   - `script`: Deployment and build scripts
///   - `library`: Library and utility code
///
/// # Arguments
/// * `repo` - Repository paths and metadata
///
/// # Returns
/// * `Agent<CompletionModel>` - Configured AI agent with vector search and file retrieval capabilities
///
/// # Example Usage
/// The agent can now intelligently search for specific types of code:
/// - "Find the main token contract implementation" → searches source files
/// - "Show me the test cases for transfer functions" → searches test files
/// - "Look at the deployment script configuration" → searches script files
/// - "Find utility functions in the library code" → searches library files
pub fn create_ai_audit_agent(repo: &RepoPaths) -> Result<Agent<CompletionModel>> {
    // Extract static documentation for base context
    let documentation = extract_content_from_docs(repo)?;

    // Initialize OpenAI client and model
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let gpt4o = openai.completion_model(GPT_4O);

    let solidity_auditor_preamble = "You are a world-class expert at smart contract auditing, renowned for your ability to find the most complex and trickiest security vulnerabilities in Solidity codebases.

You have access to a powerful file retrieval tool that can search through different types of code files:
- 'source': Main application code and smart contracts
- 'test': Test files and test cases
- 'script': Deployment and build scripts
- 'library': Library and utility code

Use the file retrieval tool strategically to:
1. Find relevant source code when analyzing vulnerabilities
2. Look at test files to understand expected behavior and edge cases
3. Check deployment scripts for configuration issues
4. Review library code for reusable components and dependencies

Always provide detailed explanations of any security issues you find, including the potential impact and recommended fixes.";

    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let qp = QueryPointsBuilder::new("contract_chunks") // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    let store = QdrantVectorStore::new(qdrant, model, qp);

    // let dynamic_context = vector_index()?;

    // Create the file retrieval tool
    let file_retrieval_tool = FileRetrievalTool::new(
        std::env::var("QDRANT_URL")?,
        std::env::var("OPENAI_API_KEY")?,
        repo.clone(),
    );

    let openai_audit_agent = AgentBuilder::new(gpt4o)
        .preamble(&solidity_auditor_preamble)
        .context(&documentation)
        .dynamic_context(5, store)
        .tool(file_retrieval_tool)
        .temperature(0.1)
        .build();

    Ok(openai_audit_agent)
}

pub async fn get_rag_for_security_query(query_content: &str, repo: &RepoPaths) -> Result<String> {
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
        .build()
        .map_err(anyhow::Error::from)?;

    // Tokenize the input
    let encoding = cl100k_base()?; // for OpenAI models
    let mut tokens = encoding.encode(query_content, HashSet::new());

    // Truncate tokens if needed
    if tokens.len() > MAX_RAG_QUERY_CONTENT_LENGTH {
        tokens.truncate(MAX_RAG_QUERY_CONTENT_LENGTH);
    }

    // Decode truncated tokens back into a string
    let query_content = encoding.decode(tokens)?;
    let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);
    let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

    /* 2 ── Build the query-params object */
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());
    let qp = QueryPointsBuilder::new(&vector_db_name) // collection name
        .with_payload(true) // pull "meta", etc.
        .build();
    // 3. Vector-store pointing at existing collection “contract_chunks”
    //    -> create the collection elsewhere (ingest step) or check/ensure here.
    info!("creating store...");
    let store = QdrantVectorStore::new(qdrant, model, qp);

    info!("retrieving relevant content from vector db");
    let relevant_docs: Vec<(f64, String, SourceChunk)> = store.top_n(&query_content, 3).await?;

    let dynamic_content = relevant_docs
        .iter()
        .map(|(_, _, source_chunk)| source_chunk.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    // info!("dynamic content");
    // print_first_four_lines(&dynamic_content);

    let final_context = format!("## ADDITIONAL CONTEXT: \n\n {}", dynamic_content);

    Ok(final_context)
}

/// Test function to verify AI agent tool usage and responses
///
/// This function creates an AI agent and tests it with various queries to verify:
/// 1. Tool usage (file retrieval)
/// 2. Dynamic context retrieval
/// 3. Response quality
pub async fn test_ai_agent_tools(repo: &RepoPaths) -> Result<()> {
    log::info!("🧪 Starting AI agent tool testing...");

    // Create the AI agent
    let agent = create_ai_audit_agent(repo)?;

    // Test queries to verify different tool usage patterns
    let test_queries = vec![
        ("Find the main contract implementation", "source"),
        ("Show me test cases for token transfers", "test"),
        ("Look at deployment scripts", "script"),
        ("Find utility functions in libraries", "library"),
        ("Analyze potential reentrancy vulnerabilities", "source"),
    ];

    for (query, expected_file_type) in test_queries {
        log::info!(
            "🔍 Testing query: '{}' (expecting {} files)",
            query,
            expected_file_type
        );
        log::info!("{}", "=".repeat(80));

        match agent.prompt(query).await {
            Ok(response) => {
                log::info!("✅ Agent response received ({} chars)", response.len());
                log::info!(
                    "📝 Response preview: {}",
                    response.chars().take(200).collect::<String>()
                );

                // Check if response mentions tool usage
                if response.to_lowercase().contains("file")
                    || response.to_lowercase().contains("search")
                {
                    log::info!("🔧 Response appears to mention file/search operations");
                } else {
                    log::warn!("⚠️  Response doesn't seem to mention tool usage");
                }
            }
            Err(e) => {
                log::error!("❌ Agent query failed: {}", e);
            }
        }

        log::info!(""); // Empty line for readability
    }

    log::info!("🧪 AI agent tool testing completed");
    Ok(())
}
