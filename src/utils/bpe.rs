use std::sync::OnceLock;
use tiktoken_rs::{cl100k_base, CoreBPE};

// Lazily-initialized, thread-safe tokenizer
static BPE_INSTANCE: OnceLock<CoreBPE> = OnceLock::new();

pub fn get_bpe() -> &'static CoreBPE {
    BPE_INSTANCE.get_or_init(|| cl100k_base().expect("Failed to load cl100k_base tokenizer"))
}
