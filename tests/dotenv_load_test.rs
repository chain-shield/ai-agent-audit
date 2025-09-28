use ai_agent_audit::config::{audit_config, init_config, try_audit_config};
use dotenvy::dotenv;
use std::env;

/// This test verifies that a real OPENAI_API_KEY is loaded from the repo-root .env file.
///
/// Behavior:
/// - If OPENAI_API_KEY is already set in the process env, we simply assert it's real.
/// - If it's NOT set, we rely on init_config() (which calls dotenv() internally)
///   to load the .env, and then assert that audit_config() has a real key.
///
/// Note: This test does not remove or overwrite your environment. It only reads.
#[test]
fn test_dotenv_loads_real_openai_key() {
    // Optional: load .env early for visibility; init_config() also does this.
    dotenv().ok();

    // Case 1: Environment already has the key
    if let Ok(k) = env::var("OPENAI_API_KEY") {
        assert!(
            !k.to_lowercase().contains("your-key"),
            "Env OPENAI_API_KEY looks like a placeholder; update your .env or env variable"
        );
        assert!(
            k.starts_with("sk-"),
            "Env OPENAI_API_KEY should start with 'sk-' (looks invalid)"
        );
        return; // already verified via env
    }

    // Case 2: Not in env -> init config which loads .env internally
    if try_audit_config().is_none() {
        init_config().expect("init_config() should load .env and validate a real key");
    }

    let cfg = audit_config();
    let key = cfg
        .openai_api_key
        .as_ref()
        .expect("OPENAI_API_KEY should be present after init_config() loads .env");

    assert!(
        !key.to_lowercase().contains("your-key"),
        "OPENAI_API_KEY from .env looks like a placeholder; update your .env"
    );
    assert!(
        key.starts_with("sk-"),
        "OPENAI_API_KEY from .env should start with 'sk-' (looks invalid)"
    );
}
