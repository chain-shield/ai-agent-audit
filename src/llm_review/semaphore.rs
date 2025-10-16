use std::sync::Arc;
use tokio::sync::Semaphore;

// increase once hit higher openai teir
const MAX_CONCURRENTS_VERIFY: usize = 10;
const MAX_CONCURRENTS_REVIEW: usize = 3;

// at module scope or pass it in
pub static VERIFY_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_VERIFY))); // 5 in-flight calls max
pub static CONTRACT_REVEW_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_REVIEW))); // 5 in-flight calls max
