use anyhow::Result;
use rig::{
    embeddings::{Embedding, EmbeddingsBuilder},
    providers::openai::{self, Client},
    Embed, OneOrMany,
};
use std::{fs, path::Path};

#[derive(Embed)]
struct SourceChunk {
    #[embed] // Field Rig will vectorise
    text: String,
    metadata: String, // we’ll keep this alongside the vector
}

const CHUNK_TOKENS: usize = 256;
const OVERLAP: usize = 32;

pub async fn embed_files(paths: &[impl AsRef<Path>]) -> Result<Vec<(String, Vec<f32>)>> {
    // ------------------------------------------------------------------
    // 1. Slice every file into SourceChunk structs
    // ------------------------------------------------------------------
    let mut docs = Vec::<SourceChunk>::new();

    for file in paths {
        let content = fs::read_to_string(file.as_ref())?;
        for (i, chunk) in tokenize(&content).into_iter().enumerate() {
            docs.push(SourceChunk {
                text: chunk,
                metadata: format!("{}:chunk {}", file.as_ref().display(), i),
            });
        }
    }

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
    let embeddings = EmbeddingsBuilder::new(model)
        .documents(docs)? // accepts any IntoIterator<Item = impl Embed>
        .build() // returns Vec<DocumentEmbeddings<SourceChunk>>
        .await?;

    // ------------------------------------------------------------------
    // 4. Flatten → (metadata, vector) so the caller can upsert to Qdrant
    // ------------------------------------------------------------------
    Ok(embeddings
        .into_iter()
        .map(|(doc, emb): (SourceChunk, OneOrMany<Embedding>)| {
            // take the first embedding (there is always at least one)
            // let first_emb: Embedding = emb.into_iter().next().expect("non‑empty");
            let first_emb: Embedding = emb.first();

            // cast Vec<f64> -> Vec<f32> for Qdrant
            let vec_f32: Vec<f32> = first_emb.vec.into_iter().map(|v| v as f32).collect();

            (doc.metadata, vec_f32)
        })
        .collect())
}

/// Very naïve whitespace tokenizer – swap with tiktoken for prod
fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut i = 0;
    while i < words.len() {
        let end = usize::min(i + CHUNK_TOKENS, words.len());
        out.push(words[i..end].join(" "));
        i = end.saturating_sub(OVERLAP);
    }
    out
}
