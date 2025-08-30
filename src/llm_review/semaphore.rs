use std::sync::Arc;
use tokio::sync::Semaphore;

// increase once hit higher openai teir
const MAX_CONCURRENTS: usize = 25;

// at module scope or pass it in
pub static VERIFY_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS))); // 5 in-flight calls max
pub static CONTRACT_REVEW_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS))); // 5 in-flight calls max
