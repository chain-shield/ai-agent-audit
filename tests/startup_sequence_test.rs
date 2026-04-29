use ai_agent_audit::config::init_config;
use ai_agent_audit::llm_review::agent::{
    agent_factory::{ensure_codex_chatgpt_auth, init_llm_clients},
    codex_app_server::cached_chatgpt_account,
};
use dotenvy::dotenv;

/// Mirrors main.rs startup sequence and verifies behavior when init_llm_clients()
/// and ensure_codex_chatgpt_auth() are called twice.
///
/// This reproduces the exact order:
///  - dotenv().ok()
///  - init_config()?
///  - env_logger::init()  (we use try_init() to avoid panics when tests run in parallel)
///  - init_llm_clients()? (first time should succeed)
///  - ensure_codex_chatgpt_auth()? (first time should succeed)
///  - init_llm_clients()? (second time should also succeed - idempotent)
///  - ensure_codex_chatgpt_auth()? (second time should also succeed - idempotent)
#[test]
fn test_main_style_startup_double_init_llm_clients() {
    // Load env
    dotenv().ok();

    if cached_chatgpt_account().ok().flatten().is_none() {
        eprintln!(
            "⚠️ Skipping test_main_style_startup_double_init_llm_clients - no cached ChatGPT/Codex auth found"
        );
        return;
    }

    // Initialize logger (avoid panic if another test already set the logger)
    let _ = env_logger::try_init();

    // Initialize configuration from environment
    init_config().expect("init_config() should succeed when at least one API key is present");

    // Initialize API-key-backed LLM clients - first call should succeed
    init_llm_clients().expect("first init_llm_clients() should succeed");

    // Initialize ChatGPT/Codex OAuth - first call should succeed
    ensure_codex_chatgpt_auth().expect("first ensure_codex_chatgpt_auth() should succeed");

    // Initialize API-key-backed LLM clients - second call should also succeed
    // The implementation checks if clients are already initialized and skips them
    let second = init_llm_clients();
    assert!(
        second.is_ok(),
        "second init_llm_clients() should succeed (idempotent)"
    );

    let second_oauth = ensure_codex_chatgpt_auth();
    assert!(
        second_oauth.is_ok(),
        "second ensure_codex_chatgpt_auth() should succeed (idempotent)"
    );
}
