/// This module provides access to the OpenAI tokenizer (BPE) used for text chunking.
/// It implements a thread-safe, lazily-initialized singleton pattern for the tokenizer.
use std::sync::OnceLock;
use tiktoken_rs::{CoreBPE, cl100k_base};

/// Lazily-initialized, thread-safe tokenizer instance
/// This is initialized only once when first accessed and then reused
static BPE_INSTANCE: OnceLock<CoreBPE> = OnceLock::new();

/// Returns a reference to the singleton BPE tokenizer instance.
///
/// This function provides access to the cl100k_base tokenizer used by OpenAI models
/// like GPT-4 and text-embedding-3. The tokenizer is initialized on first access
/// and then reused for subsequent calls, making it efficient for repeated use.
///
/// @return A static reference to the CoreBPE tokenizer instance
pub fn get_bpe() -> &'static CoreBPE {
    BPE_INSTANCE.get_or_init(|| cl100k_base().expect("Failed to load cl100k_base tokenizer"))
}
