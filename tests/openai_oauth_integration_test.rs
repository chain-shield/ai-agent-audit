use ai_agent_audit::build_brain::summarize::FileSummary;
use ai_agent_audit::config::{
    OPENAI_MODEL, OPENAI_REASONING_EFFORT, OPENAI_SUMMARY_MODEL, OPENAI_SUMMARY_REASONING_EFFORT,
    init_config,
};
use ai_agent_audit::llm_review::agent::{
    agent_factory::{AgentConfig, AgentFactory, ensure_codex_chatgpt_auth, init_llm_clients},
    codex_app_server::cached_chatgpt_account,
};
use ai_agent_audit::llm_review::contract::contract_category::ContractCategory;
use ai_agent_audit::llm_review::findings::findings::PrivilegeLevel;
use ai_agent_audit::llm_review::threat_models::{
    actors::{Actors, RoleType},
    invariants::{ContractInvariants, InvariantStatus, InvariantType},
    patterns::{Patterns, VulnerabilityPattern},
};
use dotenvy::dotenv;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Once;
use std::time::Duration;
use tokio::time::timeout;

static OAUTH_STARTUP: Once = Once::new();

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct SimpleOAuthResponse {
    id: Option<String>,
    answer: u32,
    confidence: String,
}

fn ensure_oauth_runtime_initialized() {
    dotenv().ok();
    let _ = env_logger::try_init();

    OAUTH_STARTUP.call_once(|| {
        let _ = init_config();

        // Startup should initialize API-key-backed providers separately from OAuth.
        init_llm_clients().expect("startup provider initialization should succeed");

        // Startup should authenticate once and cache the ChatGPT session for later calls.
        ensure_codex_chatgpt_auth().expect("startup OAuth initialization should succeed");
        ensure_codex_chatgpt_auth()
            .expect("cached startup OAuth initialization should be reusable");
    });
}

#[tokio::test]
#[ignore = "requires live ChatGPT/Codex OAuth"]
async fn test_openai_codex_chatgpt_oauth_prompt_and_extract() {
    ensure_oauth_runtime_initialized();

    let account = cached_chatgpt_account()
        .expect("should be able to read cached ChatGPT account")
        .expect("startup should leave a cached ChatGPT account");

    println!(
        "Authenticated ChatGPT account: {} ({})",
        account.email, account.plan_type
    );

    let agent_config = AgentConfig::new(None)
        .with_model(OPENAI_MODEL)
        .with_preamble("You are a precise test assistant.")
        .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT);

    let agent =
        AgentFactory::create_openai_agent(&agent_config).expect("should create OpenAI agent");

    let prompt_response = agent
        .prompt("Reply with READY and nothing else.")
        .await
        .expect("prompt should succeed through ChatGPT OAuth");

    assert_eq!(prompt_response.trim(), "READY");

    let extraction_prompt = r#"
Return valid JSON with:
- id: string or null
- answer: integer
- confidence: string

Question: what is 2 + 2?
Set id to null.
"#;

    let extracted: SimpleOAuthResponse = agent
        .extract_with_retry(extraction_prompt)
        .await
        .expect("structured extraction should succeed through ChatGPT OAuth");

    assert!(extracted.id.is_none(), "id should deserialize from null");
    assert_eq!(extracted.answer, 4);
    assert!(
        !extracted.confidence.trim().is_empty(),
        "confidence should not be empty"
    );
}

#[tokio::test]
#[ignore = "requires live ChatGPT/Codex OAuth"]
async fn test_openai_codex_handles_real_app_schemas_with_optional_nested_fields() {
    ensure_oauth_runtime_initialized();

    timeout(Duration::from_secs(300), async {
        let agent_config = AgentConfig::new(None)
            .with_model(OPENAI_MODEL)
            .with_preamble("You are a precise test assistant.")
            .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT);

        let agent =
            AgentFactory::create_openai_agent(&agent_config).expect("should create OpenAI agent");

        let actors_prompt = r#"
	Return valid JSON for the `Actors` schema with exactly one actor.
	Set `id` to null.
	Use:
	- name: "Unprivileged caller"
	- role_type: "UnprivilegedUser"
	- description: "Permissionless user interacting with the protocol."
	- capabilities: ["Call public functions"]
	"#;

        let actors: Actors = agent
            .extract_with_retry(actors_prompt)
            .await
            .expect("Actors extraction should succeed through ChatGPT OAuth");

        assert_eq!(actors.actors.len(), 1);
        assert!(actors.actors[0].id.is_none(), "actor id should be nullable");
        assert_eq!(actors.actors[0].name, "Unprivileged caller");
        assert_eq!(actors.actors[0].role_type, RoleType::UnprivilegedUser);

        let invariants_prompt = r#"
	Return valid JSON for the `ContractInvariants` schema with exactly one invariant.
	Set `id`, `pre_state`, `post_state`, and `impact` to null.
	Use:
	- inv_type: "Permission"
	- contract: "Vault"
	- function: "deposit"
	- predicate: "Only authorized actors can pause withdrawals."
	- desc: "The pause control should remain restricted to privileged governance."
	- checks: ["onlyOwner", "role validation"]
	- status: "Holds"
	"#;

        let invariants: ContractInvariants = agent
            .extract_with_retry(invariants_prompt)
            .await
            .expect("ContractInvariants extraction should succeed through ChatGPT OAuth");

        assert_eq!(invariants.invariants.len(), 1);
        assert!(invariants.invariants[0].id.is_none());
        assert_eq!(invariants.invariants[0].inv_type, InvariantType::Permission);
        assert_eq!(invariants.invariants[0].status, InvariantStatus::Holds);
        assert!(invariants.invariants[0].pre_state.is_none());
        assert!(invariants.invariants[0].post_state.is_none());
        assert!(invariants.invariants[0].impact.is_none());

        let patterns_prompt = r#"
	Return valid JSON for the `Patterns` schema with exactly one pattern.
	Set `impact` to null.
	Use:
	- issue_type: "Reentrancy"
	- title: "External callback before state sync"
	- contract: "Vault"
	- function: "withdraw"
	- description: "External control is returned before internal balances are finalized."
	- static_signals: ["external call before balance update"]
	- assets_at_risk: ["vault reserves"]
	- privilege: "Permissionless"
	"#;

        let patterns: Patterns = agent
            .extract_with_retry(patterns_prompt)
            .await
            .expect("Patterns extraction should succeed through ChatGPT OAuth");

        assert_eq!(patterns.patterns.len(), 1);
        assert_eq!(
            patterns.patterns[0].issue_type,
            VulnerabilityPattern::Reentrancy
        );
        assert_eq!(
            patterns.patterns[0].privilege.to_string(),
            PrivilegeLevel::Permissionless.to_string()
        );
        assert!(patterns.patterns[0].impact.is_none());
    })
    .await
    .expect("structured OAuth extracts should not stall indefinitely");
}

#[tokio::test]
#[ignore = "requires live ChatGPT/Codex OAuth"]
async fn test_openai_codex_repeated_calls_clear_context_and_handle_parallel_prompts() {
    ensure_oauth_runtime_initialized();

    timeout(Duration::from_secs(300), async {
        let agent_config = AgentConfig::new(None)
            .with_model(OPENAI_MODEL)
            .with_preamble("You are a precise test assistant.")
            .with_openai_reasoning_effort(OPENAI_REASONING_EFFORT);

        let agent =
            AgentFactory::create_openai_agent(&agent_config).expect("should create OpenAI agent");

        let first = agent
            .prompt("Reply exactly with MEMORY-SET and nothing else.")
            .await
            .expect("first prompt should succeed");
        assert_eq!(first.trim(), "MEMORY-SET");

        let second = agent
            .prompt("Reply exactly with SECOND-CALL and nothing else.")
            .await
            .expect("second prompt should succeed");
        assert_eq!(second.trim(), "SECOND-CALL");

        let mut handles = Vec::new();
        for token in ["ALPHA", "BRAVO", "CHARLIE", "DELTA"] {
            let agent_config = agent_config.clone();
            let prompt = format!("Reply exactly with {token} and nothing else.");
            handles.push(tokio::spawn(async move {
                let agent = AgentFactory::create_openai_agent(&agent_config)
                    .expect("parallel prompt agent creation should succeed");
                let response = agent
                    .prompt(&prompt)
                    .await
                    .expect("parallel prompt should succeed");
                (token, response)
            }));
        }

        for handle in handles {
            let (token, response) = handle.await.expect("parallel task should join");
            assert_eq!(response.trim(), token);
        }
    })
    .await
    .expect("parallel OAuth prompts should not stall indefinitely");
}

#[tokio::test]
#[ignore = "requires live ChatGPT/Codex OAuth"]
async fn test_openai_codex_low_effort_summary_style_extract_still_works() {
    ensure_oauth_runtime_initialized();

    let agent_config = AgentConfig::new(None)
        .with_model(OPENAI_SUMMARY_MODEL)
        .with_preamble("You are a precise summarization test assistant.")
        .with_openai_reasoning_effort(OPENAI_SUMMARY_REASONING_EFFORT);

    let agent =
        AgentFactory::create_openai_agent(&agent_config).expect("should create OpenAI agent");

    let summary_prompt = r#"
Return valid JSON for the `FileSummary` schema.
Use:
- summary: "Vault contract that accepts ERC20 deposits, mints proportional shares, and lets an admin pause withdrawals."
- contract_category: "VaultShareBased"
"#;

    let summary: FileSummary = agent
        .extract_with_retry(summary_prompt)
        .await
        .expect("low-effort summary extraction should succeed through ChatGPT OAuth");

    assert_eq!(summary.contract_category, ContractCategory::VaultShareBased);
    assert!(
        summary.summary.contains("Vault contract"),
        "summary text should deserialize correctly"
    );
}
