/// This module provides functionality for interacting with the Qdrant vector database.
/// It handles creating collections and upserting vectors with their associated metadata.
use anyhow::Result;
use log::info;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, PointStruct, UpsertPointsBuilder,
    VectorParams, VectorsConfig,
};
use qdrant_client::Payload;
use qdrant_client::Qdrant;
use serde_json::json;

/// Ensures that a collection exists in the Qdrant database, creating it if missing.
///
/// This function creates a new collection with the specified name and dimension if it doesn't
/// already exist. It uses cosine distance for similarity calculations.
///
/// @param client - Reference to the Qdrant client
/// @param name - Name of the collection to create
/// @param dim - Dimension of the vectors to be stored in the collection
/// @return Result indicating success or failure
pub async fn ensure_collection(client: &Qdrant, name: &str, dim: u64) -> Result<()> {
    // check if collection already exists
    let already_exists = client.collection_exists(name).await?;

    if already_exists {
        info!("Collection {} already exists...no need to create", name);
        return Ok(());
    }

    // Build the request *by value* (no &CreateCollection -> eliminates the Into/From error)
    let req = CreateCollection {
        collection_name: name.to_owned(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: dim,
                distance: Distance::Cosine.into(), // Using cosine similarity for vector comparison
                ..Default::default()
            })),
        }),
        ..Default::default()
    };

    // Newer client expects `CreateCollection`, not `&CreateCollection`
    client.create_collection(req).await?;
    Ok(())
}

/// Upserts a batch of metadata and embedding vector tuples into a Qdrant collection.
///
/// This function takes a list of (metadata, vector) tuples and inserts or updates them
/// in the specified Qdrant collection. Each item's metadata is stored in the payload
/// under the key "meta".
///
/// @param client - Reference to the Qdrant client
/// @param collection - Name of the collection to upsert into
/// @param items - Slice of tuples containing metadata strings and their corresponding embedding vectors
/// @return Result indicating success or failure
pub async fn upsert(client: &Qdrant, collection: &str, items: &[(String, Vec<f32>)]) -> Result<()> {
    // Build PointStructs from the items
    let points: Vec<PointStruct> = items
        .iter()
        .enumerate()
        .map(|(i, (meta, vec))| {
            // Create payload by storing the metadata string under key "meta"
            let payload: Payload = json!({ "meta": meta })
                .try_into()
                .expect("could not process payload");
            // Create a new point with ID, vector, and payload
            PointStruct::new(i as u64, vec.clone(), payload)
        })
        .collect();

    // Use the builder-style API to create the upsert request
    let req = UpsertPointsBuilder::new(collection, points).wait(true); // wait=true mimics the old “blocking”

    // Execute the upsert operation
    client.upsert_points(req).await?;
    Ok(())
}
