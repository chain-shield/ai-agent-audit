use std::sync::Arc;
use tokio::sync::Semaphore;

// increase once hit higher openai teir
const MAX_CONCURRENTS_VERIFY: usize = 10;
const MAX_CONCURRENTS_REVIEW: usize = 3;
const MAX_CONCURRENTS_POC: usize = 1; // Run PoC tests atomically to avoid cross-file compilation errors
const MAX_CONCURRENTS_GENERAL: usize = 150;

// at module scope or pass it in
pub static VERIFY_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_VERIFY)));
pub static POC_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_POC)));
pub static CONTRACT_REVEW_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_REVIEW)));
pub static GENERAL_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_GENERAL)));
