/// Vector embeddings generation for semantic search.
///
/// This module creates high-quality vector embeddings from source code and analysis
/// results using OpenAI's text-embedding-3-small model. Handles intelligent text
/// chunking with overlap to maintain context for optimal semantic search performance.
use anyhow::Result;
use log::info;
use rig::{
    Embed,
    client::EmbeddingsClient,
    embeddings::EmbeddingsBuilder,
    providers::openai::{self, Client},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tiktoken_rs::CoreBPE;

use crate::utils::bpe::get_bpe; // OpenAI’s GPT-4 / text-embedding 3 vocab

/// Represents a chunk of source code with metadata for vector embedding.
///
/// This struct organizes text content and associated metadata for embedding
/// generation, enabling semantic search with rich context information.
#[derive(Debug, Embed, Clone, Serialize, Deserialize)]
pub struct SourceChunk {
    #[embed] // Field Rig will vectorise
    pub text: String, // The actual text content to be embedded
    pub metadata: String,  // we’ll keep this alongside the vector
    pub file_type: String, // Added to store "source", "test", or "script"
}

/// Number of tokens in each chunk for optimal embedding quality
const CHUNK_TOKENS: usize = 1024;
/// Number of tokens to overlap between chunks to maintain context continuity
const OVERLAP: usize = 128;
/// Batch size for embedding API requests to optimize throughput
const BATCH: usize = 30;
/// Maximum chunk length in characters (OpenAI supports ~8192 tokens, leaving headroom)
const MAX_CHUNK_LEN: usize = 12000;

/// Infers file_type based on file path or naming conventions.
/// Returns "source", "test", "script", or "library" based on path patterns.
fn infer_file_type(path: &Path) -> String {
    // Convert to lowercase for case-insensitive matching
    let path_str = path.to_string_lossy().to_lowercase();
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    // Check for library patterns first (most specific)
    if path_str.contains("/lib/")
        || path_str.contains("\\lib\\")
        || path_str.contains("/libs/")
        || path_str.contains("\\libs\\")
        || path_str.contains("/library/")
        || path_str.contains("\\library\\")
        || path_str.contains("/libraries/")
        || path_str.contains("\\libraries\\")
        || path_str.contains("/node_modules/")
        || path_str.contains("\\node_modules\\")
        || path_str.starts_with("lib/")
        || path_str.starts_with("libs/")
        || path_str.starts_with("library/")
        || path_str.starts_with("libraries/")
        || path_str.starts_with("node_modules/")
    {
        return "library".to_string();
    }

    // Check for test patterns
    if path_str.contains("/test") ||
       path_str.contains("\\test") ||
       path_str.contains("/tests/") ||
       path_str.contains("\\tests\\") ||
       path_str.starts_with("test/") ||
       path_str.starts_with("tests/") ||
       file_name.contains("test") ||
       file_name.ends_with(".t.sol") ||  // Solidity test files
       file_name.ends_with("_test.rs") ||
       file_name.ends_with("_test.py")
    {
        return "test".to_string();
    }

    // Check for script patterns
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if ext == "sh"
            || ext == "bash"
            || ext == "zsh"
            || ext == "bat"
            || ext == "cmd"
            || ext == "ps1"
            || (ext == "py"
                && (file_name.contains("script")
                    || file_name.contains("deploy")
                    || file_name.contains("build")
                    || file_name.contains("setup")))
        {
            return "script".to_string();
        }
    }

    // Check for Solidity script files
    if file_name.ends_with(".s.sol") {
        return "script".to_string();
    }

    // Check for script directories
    if path_str.contains("/script")
        || path_str.contains("\\script")
        || path_str.contains("/scripts/")
        || path_str.contains("\\scripts\\")
        || path_str.contains("/bin/")
        || path_str.contains("\\bin\\")
        || path_str.contains("/tools/")
        || path_str.contains("\\tools\\")
        || path_str.starts_with("script/")
        || path_str.starts_with("scripts/")
        || path_str.starts_with("bin/")
        || path_str.starts_with("tools/")
    {
        return "script".to_string();
    }

    // Default to source for everything else
    "source".to_string()
}

/**
 * Processes a list of files and creates embeddings for their content.
 *
 * @param paths - Array of file paths to process
 * @return Result containing a vector of tuples with metadata and embedding vectors
 */
pub async fn embed_files(paths: &[impl AsRef<Path>]) -> Result<Vec<(SourceChunk, Vec<f32>)>> {
    // ------------------------------------------------------------------
    // 1. Slice every file into SourceChunk structs
    // ------------------------------------------------------------------
    let mut docs = Vec::<SourceChunk>::new();

    info!("looping through all files and breaking into chunks");
    let bpe = get_bpe();
    for file in paths {
        let content = fs::read_to_string(file.as_ref())?;
        let file_type = infer_file_type(file.as_ref()); // Infer file_type
        for (i, chunk) in tokenize(bpe, &content).into_iter().enumerate() {
            let clean = chunk
                .replace(['\0', '\u{FFFD}'], ""); // Remove null bytes and replacement chars
            let clean = clean.trim();

            if clean.is_empty() {
                log::warn!("Skipping empty chunk from {}", file.as_ref().display());
                continue;
            }
            if clean.len() > MAX_CHUNK_LEN {
                log::warn!(
                    "Skipping oversized chunk ({} chars) from {}",
                    clean.len(),
                    file.as_ref().display()
                );
                continue;
            }
            docs.push(SourceChunk {
                text: clean.to_string(),
                metadata: format!("{}:chunk {}", file.as_ref().display(), i),
                file_type: file_type.clone(),
            });
        }
    }

    info!("breaking out data into text chunks complete");
    // ------------------------------------------------------------------
    // 2. Pick an embedding model once
    // ------------------------------------------------------------------
    let openai = Client::from_env();

    // 1536‑dim “storage‑optimised” v3 model
    let model = openai.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

    // ------------------------------------------------------------------
    // 3. Build embeddings in one RPC batch
    //    EmbeddingsBuilder<M, D>::new(model) infers both generics
    // ------------------------------------------------------------------
    // ------------------------------------------------------------------
    // 4. Flatten → (metadata, vector) so the caller can upsert to Qdrant
    // ------------------------------------------------------------------
    let mut all_vecs = Vec::<(SourceChunk, Vec<f32>)>::new();

    // info!("using openai to embed in {}-item batches…", BATCH);
    for docs_slice in docs.chunks(BATCH) {
        // info!("Batch size: {}", docs_slice.len());
        // for (i, doc) in docs_slice.iter().enumerate() {
        //     info!(
        //         "Chunk {}: text='{}', metadata='{}'",
        //         i, doc.text, doc.metadata
        //     );
        // }
        let batch_result = EmbeddingsBuilder::new(model.clone())
            .documents(docs_slice.to_vec())? // slice → Vec
            .build()
            .await;

        match batch_result {
            Ok(batch) => {
                all_vecs.extend(batch.into_iter().filter_map(|(doc, emb)| {
                    let v = emb.first().vec;
                    if v.is_empty() {
                        return None;
                    }
                    Some((doc, v.into_iter().map(|x| x as f32).collect()))
                }));
            }
            Err(e) => {
                log::error!("Embedding batch failed: {:#}", e);
                // Optionally retry, skip or abort here
            }
        };
    }
    Ok(all_vecs)
}

/// Split `s` into fixed-width token windows with `OVERLAP` tokens of context.
fn tokenize(bpe: &CoreBPE, s: &str) -> Vec<String> {
    let tokens = bpe.encode_with_special_tokens(s);

    let mut out = Vec::new();
    let mut start = 0;

    while start < tokens.len() {
        let end = usize::min(start + CHUNK_TOKENS, tokens.len());
        let token_slice = tokens[start..end].to_vec();
        let chunk = bpe.decode(token_slice).unwrap_or_default();
        out.push(chunk);

        if end == tokens.len() {
            break; // reached the tail – exit
        }
        start += CHUNK_TOKENS - OVERLAP; // always moves forward
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_infer_file_type() {
        // Test library files
        assert_eq!(infer_file_type(Path::new("src/lib/utils.sol")), "library");

        assert_eq!(
            infer_file_type(Path::new("contracts/libs/SafeMath.sol")),
            "library"
        );
        assert_eq!(infer_file_type(Path::new("src\\lib\\Token.sol")), "library");
        assert_eq!(
            infer_file_type(Path::new("libraries/OpenZeppelin.sol")),
            "library"
        );

        // Test test files
        assert_eq!(infer_file_type(Path::new("test/Token.t.sol")), "test");
        assert_eq!(infer_file_type(Path::new("tests/integration.sol")), "test");
        assert_eq!(infer_file_type(Path::new("src/test/unit_test.sol")), "test");
        assert_eq!(infer_file_type(Path::new("TokenTest.sol")), "test");

        // Test script files
        assert_eq!(infer_file_type(Path::new("script/Deploy.s.sol")), "script");
        assert_eq!(
            infer_file_type(Path::new("scripts/migration.sol")),
            "script"
        );
        assert_eq!(infer_file_type(Path::new("deploy.sh")), "script");
        assert_eq!(infer_file_type(Path::new("setup.py")), "script");

        // Test source files (default)
        assert_eq!(infer_file_type(Path::new("src/Token.sol")), "source");
        assert_eq!(
            infer_file_type(Path::new("contracts/MyContract.sol")),
            "source"
        );
        assert_eq!(infer_file_type(Path::new("README.md")), "source");
    }
}
