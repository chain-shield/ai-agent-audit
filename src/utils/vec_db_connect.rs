// utils/connect.rs
// Temporarily disabled due to rig-qdrant version conflicts with rig-core 0.13.0
/*
use once_cell::sync::OnceCell;
use qdrant_client::{Qdrant, qdrant::QueryPointsBuilder};
use rig::{
    client::{EmbeddingsClient, ProviderClient},
    providers::openai::{Client, TEXT_EMBEDDING_3_SMALL},
    vector_store::VectorStoreIndexDyn,
};
use rig_qdrant::QdrantVectorStore;
use std::sync::Arc;

/// Build (once) and return an Arc<dyn VectorStoreIndexDyn>.
///
/// Synchronous because OnceCell's initializer must be sync.
pub fn vector_index() -> anyhow::Result<Arc<dyn VectorStoreIndexDyn>> {
    static ONCE: OnceCell<Arc<dyn VectorStoreIndexDyn>> = OnceCell::new();

    let idx = ONCE.get_or_try_init(|| {
        // 1. Qdrant client
        let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?)
            .build()
            .map_err(anyhow::Error::from)?;

        let openai = Client::from_env();
        let model = openai.embedding_model(TEXT_EMBEDDING_3_SMALL);

        // 2 ── Build the query-params object
        let qp = QueryPointsBuilder::new("contract_chunks") // collection name
            .with_payload(true) // pull "meta", etc.
            .build();
        // 3. Vector-store pointing at existing collection "contract_chunks"
        //    -> create the collection elsewhere (ingest step) or check/ensure here.
        let store = QdrantVectorStore::new(qdrant, model, qp);

        // 4. Erase to trait object
        Ok::<Arc<dyn VectorStoreIndexDyn>, anyhow::Error>(Arc::new(store))
    })?;

    Ok(idx.clone())
}
*/

use std::sync::Arc;

/// Temporary placeholder function while vector database is disabled
///
/// This function returns an error indicating that vector functionality is temporarily unavailable
/// due to rig-qdrant version conflicts with rig-core 0.13.0
pub fn vector_index() -> anyhow::Result<Arc<dyn std::any::Any + Send + Sync>> {
    Err(anyhow::anyhow!(
        "Vector database functionality temporarily disabled due to rig-qdrant version conflicts"
    ))
}
