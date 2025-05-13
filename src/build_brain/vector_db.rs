

// crates/vector_db/src/lib.rs
use anyhow::Result;
use qdrant_client::Payload;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, PointStruct, VectorParams, VectorsConfig, UpsertPointsBuilder,
};
use serde_json::json;

/// Ensure that a collection exists (creates it if missing).
pub async fn ensure_collection(client: &Qdrant, name: &str, dim: u64) -> Result<()> {
    // Build the request *by value* (no &CreateCollection -> eliminates the Into/From error)
    let req = CreateCollection {
        collection_name: name.to_owned(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: dim,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    };

    // Newer client expects `CreateCollection`, not `&CreateCollection`
    client.create_collection(req).await?;
    Ok(())
}

/// Upsert a batch of `(meta, embedding)` tuples.
pub async fn upsert(
    client: &Qdrant,
    collection: &str,
    items: &[(String, Vec<f32>)],
) -> Result<()> {
    // Build PointStructs
    let points: Vec<PointStruct> = items
        .iter()
        .enumerate()
        .map(|(i, (meta, vec))| {
            // payload — just stick the metadata string under key "meta"
            let payload: Payload = json!({ "meta": meta }).try_into().expect("could not process payload");
            PointStruct::new(i as u64, vec.clone(), payload)
        })
        .collect();

    // New builder‑style API
    let req = UpsertPointsBuilder::new(collection, points).wait(true); // wait=true mimics the old “blocking”

    client.upsert_points(req).await?;
    Ok(())
}
