/// This module provides an in-memory caching mechanism for markdown codeblocks
/// to avoid redundant database operations and improve performance when
/// processing the same seed files multiple times.
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::codeblock_db::MarkdownCodeblock;

/// Global in-memory cache for storing MarkdownCodeblock instances.
///
/// The cache is keyed by seed file paths and stores the corresponding
/// MarkdownCodeblock instances. This helps avoid redundant processing
/// and database operations when the same seed file is encountered multiple times.
///
/// The cache is thread-safe, using Arc and Mutex for concurrent access.
pub static CODEBLOCK_CACHE: Lazy<Arc<Mutex<HashMap<String, MarkdownCodeblock>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Retrieves a cached codeblock for a given seed file if it exists.
///
/// # Arguments
/// * `seed_file` - The path to the seed file used as the cache key
///
/// # Returns
/// * `Option<MarkdownCodeblock>` - The cached codeblock if found, None otherwise
pub async fn get_cached_codeblock(seed_file: &str) -> Option<MarkdownCodeblock> {
    let cache = Arc::clone(&CODEBLOCK_CACHE);
    let codeblock_cache = cache.lock().await;

    // Returns Some(codeblock) if found, and None if not found
    codeblock_cache.get(seed_file).cloned()
}

/// Stores a codeblock in the cache for a given seed file.
///
/// # Arguments
/// * `seed_file` - The path to the seed file used as the cache key
/// * `codeblock` - The MarkdownCodeblock instance to cache
pub async fn set_codeblock_cache(seed_file: &str, codeblock: &MarkdownCodeblock) {
    let cache = Arc::clone(&CODEBLOCK_CACHE);
    let mut codeblock_cache = cache.lock().await;

    codeblock_cache.insert(seed_file.to_string(), codeblock.clone());
}
