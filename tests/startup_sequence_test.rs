use ai_agent_audit::config::init_config;
use ai_agent_audit::llm_review::agent::agent_factory::init_llm_clients;
use dotenvy::dotenv;
use std::env;

/// Returns true if at least one LLM API key is present in the environment
fn any_llm_key_present() -> bool {
    [
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "GEMINI_API_KEY",
        "DEEPSEEK_API_KEY",
    ]
    .into_iter()
    .any(|k| env::var(k).ok().filter(|v| !v.is_empty()).is_some())
}

/// Mirrors main.rs startup sequence and verifies behavior when init_llm_clients() is called twice.
///
/// This reproduces the exact order:
///  - dotenv().ok()
///  - init_config()?
///  - env_logger::init()  (we use try_init() to avoid panics when tests run in parallel)
///  - init_llm_clients()? (first time should succeed)
///  - init_llm_clients()? (second time should fail with already-initialized error)
#[test]
fn test_main_style_startup_double_init_llm_clients() {
    // Load env
    dotenv().ok();

    // Skip if no keys present (init_config() would fail validation otherwise)
    if !any_llm_key_present() {
        eprintln!(
            "⚠️ Skipping test_main_style_startup_double_init_llm_clients - no LLM API key found"
        );
        return;
    }

    // Initialize logger (avoid panic if another test already set the logger)
    let _ = env_logger::try_init();

    // Initialize configuration from environment
    init_config().expect("init_config() should succeed when at least one API key is present");

    // Initialize LLM clients - first call should succeed
    init_llm_clients().expect("first init_llm_clients() should succeed");

    // Initialize LLM clients - second call should fail due to OnceLock already set
    let second = init_llm_clients();
    assert!(
        second.is_err(),
        "second init_llm_clients() should return Err"
    );

    let err = second.err().unwrap();
    let msg = format!("{}", err);

    // The error should indicate the client was already initialized (for the first available provider)
    assert!(
        msg.to_lowercase().contains("already initialized"),
        "expected an 'already initialized' style error, got: {}",
        msg
    );
}
