use anyhow::Result;
use log::info;
use rig::{
    embeddings::EmbeddingsBuilder,
    providers::openai::{self, Client},
    Embed,
};
use std::{fs, path::Path};
use tiktoken_rs::CoreBPE;

use crate::utils::bpe::get_bpe; // OpenAI’s GPT-4 / text-embedding 3 vocab

#[derive(Embed, Clone)]
struct SourceChunk {
    #[embed] // Field Rig will vectorise
    text: String,
    metadata: String, // we’ll keep this alongside the vector
}

const CHUNK_TOKENS: usize = 256;
const OVERLAP: usize = 32;
const BATCH: usize = 30;

/**
*  TODO - USE codellama:embed instead of openai embedding model
*  will need to either self host (need powerful computer) or self host on
*  on gcp ($250/month), this embedding is optimal for code
*
*
*
* */
pub async fn embed_files(paths: &[impl AsRef<Path>]) -> Result<Vec<(String, Vec<f32>)>> {
    // ------------------------------------------------------------------
    // 1. Slice every file into SourceChunk structs
    // ------------------------------------------------------------------
    let mut docs = Vec::<SourceChunk>::new();

    info!("looping through all files and breaking into chunks");
    let bpe = get_bpe();
    for file in paths {
        let content = fs::read_to_string(file.as_ref())?;
        for (i, chunk) in tokenize(bpe, &content).into_iter().enumerate() {
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
    let mut all_vecs = Vec::<(String, Vec<f32>)>::new();

    // info!("using openai to embed in {}-item batches…", BATCH);
    for docs_slice in docs.chunks(BATCH) {
        info!("Batch size: {}", docs_slice.len());
        // for (i, doc) in docs_slice.iter().enumerate() {
        //     info!(
        //         "Chunk {}: text='{}', metadata='{}'",
        //         i, doc.text, doc.metadata
        //     );
        // }
        let batch = EmbeddingsBuilder::new(model.clone())
            .documents(docs_slice.to_vec())? // slice → Vec
            .build()
            .await?;

        all_vecs.extend(batch.into_iter().filter_map(|(doc, emb)| {
            let v = emb.first().vec;
            if v.is_empty() {
                return None;
            } // guard against 0-dim
            Some((doc.metadata, v.into_iter().map(|x| x as f32).collect()))
        }));
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
