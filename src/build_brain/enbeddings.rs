/// This module handles the creation of embeddings for source code files.
/// It breaks down files into manageable chunks, tokenizes them, and uses OpenAI's
/// embedding model to create vector representations that can be stored in a vector database.
use anyhow::Result;
use log::info;
use rig::{
    client::EmbeddingsClient,
    embeddings::EmbeddingsBuilder,
    providers::openai::{self, Client},
    Embed,
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tiktoken_rs::CoreBPE;

use crate::utils::bpe::get_bpe; // OpenAI’s GPT-4 / text-embedding 3 vocab

/// Represents a chunk of source code to be embedded.
/// This struct is used to organize text content with its metadata
/// before sending it to the embedding model.
#[derive(Debug, Embed, Clone, Serialize, Deserialize)]
pub struct SourceChunk {
    #[embed] // Field Rig will vectorise
    pub text: String, // The actual text content to be embedded
    metadata: String, // we’ll keep this alongside the vector
}

/// Number of tokens in each chunk for embedding
const CHUNK_TOKENS: usize = 256;
/// Number of tokens to overlap between chunks to maintain context
const OVERLAP: usize = 32;
/// Number of documents to process in each batch when sending to the embedding model
const BATCH: usize = 30;
const MAX_CHUNK_LEN: usize = 4000; // OpenAI API supports ~8192 tokens, but leave headroom

/**
 * Processes a list of files and creates embeddings for their content.
 *
 * TODO - USE codellama:embed instead of openai embedding model
 * will need to either self host (need powerful computer) or self host on
 * on gcp ($250/month), this embedding is optimal for code
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
        for (i, chunk) in tokenize(bpe, &content).into_iter().enumerate() {
            let clean = chunk
                .replace('\0', "") // Remove null bytes
                .replace('\u{FFFD}', ""); // Remove replacement chars
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
                text: chunk,
                metadata: format!("{}:chunk {}", file.as_ref().display(), i),
            });
        }
    }

    info!("breaking out data into text chunks complete");
    // ------------------------------------------------------------------
    // 2. Pick an embedding model once
    // ------------------------------------------------------------------
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let openai = Client::new(&api_key);

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
