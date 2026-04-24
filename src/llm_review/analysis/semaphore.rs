use std::sync::Arc;
use tokio::sync::Semaphore;

const DEFAULT_MAX_CONCURRENTS_VERIFY: usize = 1;
const DEFAULT_MAX_CONCURRENTS_REVIEW: usize = 2;
const DEFAULT_MAX_CONCURRENTS_POC: usize = 1;
const DEFAULT_MAX_CONCURRENTS_GENERAL: usize = 8;

fn concurrency_limit_from_env(var: &str, default: usize) -> usize {
    std::env::var(var)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

pub static VERIFY_SEM: once_cell::sync::Lazy<Arc<Semaphore>> = once_cell::sync::Lazy::new(|| {
    Arc::new(Semaphore::new(concurrency_limit_from_env(
        "AI_AGENT_AUDIT_MAX_CONCURRENT_VERIFY",
        DEFAULT_MAX_CONCURRENTS_VERIFY,
    )))
});
pub static POC_SEM: once_cell::sync::Lazy<Arc<Semaphore>> = once_cell::sync::Lazy::new(|| {
    Arc::new(Semaphore::new(concurrency_limit_from_env(
        "AI_AGENT_AUDIT_MAX_CONCURRENT_POC",
        DEFAULT_MAX_CONCURRENTS_POC,
    )))
});
pub static CONTRACT_REVEW_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| {
        Arc::new(Semaphore::new(concurrency_limit_from_env(
            "AI_AGENT_AUDIT_MAX_CONCURRENT_REVIEW",
            DEFAULT_MAX_CONCURRENTS_REVIEW,
        )))
    });
pub static GENERAL_SEM: once_cell::sync::Lazy<Arc<Semaphore>> = once_cell::sync::Lazy::new(|| {
    Arc::new(Semaphore::new(concurrency_limit_from_env(
        "AI_AGENT_AUDIT_MAX_CONCURRENT_GENERAL",
        DEFAULT_MAX_CONCURRENTS_GENERAL,
    )))
});
