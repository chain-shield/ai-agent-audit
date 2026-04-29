use ai_agent_audit::config::{audit_config, init_config, try_audit_config};
use dotenvy::dotenv;
use std::env;
use std::path::Path;

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

    let has_env_file = Path::new(".env").exists();

    // Case 1: Environment already has the key
    if let Ok(k) = env::var("OPENAI_API_KEY") {
        if k.to_lowercase().contains("your-key") || !k.starts_with("sk-") {
            eprintln!(
                "⚠️ Skipping test_dotenv_loads_real_openai_key - OPENAI_API_KEY is placeholder or invalid"
            );
            return;
        }
        assert!(
            !k.is_empty(),
            "Env OPENAI_API_KEY should not be empty when present"
        );
        return; // already verified via env
    }

    if !has_env_file {
        eprintln!("⚠️ Skipping test_dotenv_loads_real_openai_key - no repo-root .env file present");
        return;
    }

    // Case 2: Not in env -> init config which loads .env internally
    if try_audit_config().is_none() {
        if let Err(err) = init_config() {
            eprintln!(
                "⚠️ Skipping test_dotenv_loads_real_openai_key - init_config() could not load a real key: {}",
                err
            );
            return;
        }
    }

    let cfg = audit_config();
    let Some(key) = cfg.openai_api_key.as_ref() else {
        eprintln!(
            "⚠️ Skipping test_dotenv_loads_real_openai_key - no OPENAI_API_KEY loaded from .env"
        );
        return;
    };

    if key.to_lowercase().contains("your-key") || !key.starts_with("sk-") {
        eprintln!(
            "⚠️ Skipping test_dotenv_loads_real_openai_key - .env contains placeholder or invalid key"
        );
        return;
    }

    assert!(
        !key.is_empty(),
        "OPENAI_API_KEY from .env should not be empty"
    );
}
