use crate::config::{
    MAX_CONCURRENTS_GENERAL, MAX_CONCURRENTS_POC, MAX_CONCURRENTS_REVIEW, MAX_CONCURRENTS_VERIFY,
};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub static VERIFY_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_VERIFY)));
pub static POC_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_POC)));
pub static CONTRACT_REVEW_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_REVIEW)));
pub static GENERAL_SEM: once_cell::sync::Lazy<Arc<Semaphore>> =
    once_cell::sync::Lazy::new(|| Arc::new(Semaphore::new(MAX_CONCURRENTS_GENERAL)));
