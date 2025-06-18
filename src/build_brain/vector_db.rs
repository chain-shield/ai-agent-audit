use std::path::{Path, PathBuf};

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

use crate::build_brain::enbeddings::embed_files;
use crate::build_brain::slither_ffi;

use super::enbeddings::SourceChunk;
use super::git_clone::RepoPaths;

pub async fn generate_slither_chucks_and_save_all_metadata_to_vector_db(
    repo: &RepoPaths,
    semantic_db: &Path,
) -> anyhow::Result<()> {
    // check if vector db for this repo already exists
    if does_qdrant_vector_db_for_this_repo_already_exist(&repo).await? {
        return Ok(());
    }
    // 2b. Create a temp dir and ask slither_ffi to fill it with chunk files
    // Create a temporary directory to store the Slither analysis results
    let tmp_dir = tempfile::tempdir()?;
    info!("generating slither ssa into txt files that contain function or storage var");

    // TODO - UNPAUSE AFTER DONE TESTING
    // Extract IR and storage information using Slither and write to text files
    let slither_chunk_paths = slither_ffi::save_code_metadata_and_analysis_to_txt_files(
        &repo.root,
        tmp_dir.path(),
        &semantic_db,
    )
    .await?;
    info!("slither ssa file count => {}", slither_chunk_paths.len());
    // ────────────────────────────────
    // 4. Assemble the *full* file list to embed
    //    – original Solidity + docs  (repo.sol_files  ∪  repo.docs)
    //    – temp IR / storage files   (tmp_paths)
    // ────────────────────────────────
    info!("combine all file locations for solidity + docs, IR functions, and storage into 1 vec");
    // Combine all file paths into a single vector:
    // - Solidity source files
    // - Documentation files
    // - Slither analysis result files
    // TODO - UNPAUSE AFTER DONE TESTING
    let mut all_files: Vec<_> = repo.docs.clone().into_iter().collect();
    all_files.extend(slither_chunk_paths);
    info!("all files => {:?}", all_files.len());

    //embed all files and upsert to qdrant vector db for later dynamic retrival
    // TODO - UNPAUSE AFTER DONE TESTING
    generate_enbeddings_and_save_to_qdrant_vector_db(&all_files, &repo).await?;
    Ok(())
}

pub async fn generate_enbeddings_and_save_to_qdrant_vector_db(
    all_files: &[PathBuf],
    repo: &RepoPaths,
) -> Result<()> {
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());

    info!("connect to qdrant db");

    // Build Qdrant client configuration and connect to the database
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?).build()?;

    let already_exists = qdrant.collection_exists(&vector_db_name).await?;

    // if vector db already exits for this repo no need to re-upsert
    if already_exists {
        return Ok(());
    }

    // Create the collection if it doesn't exist
    info!("create contract_chunks vector db (if does not exist)");
    ensure_collection(&qdrant, &vector_db_name, 1536).await?;

    // Generate vector embeddings for all files
    info!("generating vector embedding");
    let embeddings = embed_files(&all_files).await?;
    // ────────────────────────────────
    // 4. Upsert into Qdrant
    // ────────────────────────────────
    // Upsert the embeddings into the Qdrant collection
    info!("upsert embeddings");
    upsert(&qdrant, &vector_db_name, &embeddings).await?;

    // Print completion message
    println!("✅ Ingest complete – {} chunks stored", embeddings.len());
    Ok(())
}

pub async fn does_qdrant_vector_db_for_this_repo_already_exist(
    repo: &RepoPaths,
) -> anyhow::Result<bool> {
    let vector_db_name = format!("{}-contract_chunks", repo.unique_repo_hash());

    info!("connect to qdrant db");
    // Build Qdrant client configuration and connect to the database
    let qdrant = Qdrant::from_url(&std::env::var("QDRANT_URL")?).build()?;

    Ok(qdrant.collection_exists(&vector_db_name).await?)
}

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
pub async fn upsert(
    client: &Qdrant,
    collection: &str,
    items: &[(SourceChunk, Vec<f32>)],
) -> Result<()> {
    // Build PointStructs from the items
    let points: Vec<PointStruct> = items
        .iter()
        .enumerate()
        .map(|(i, (chunk, vec))| {
            // 🚩 flatten: payload IS the SourceChunk
            let payload: Payload = serde_json::to_value(chunk)
                .expect("serialise SourceChunk")
                .try_into()
                .expect("to Payload");
            // // Create payload by storing the metadata string under key "meta"
            // let payload: Payload = serde_json::to_value(chunck)
            //     .try_into()
            //     .expect("could not process payload");
            // // Create a new point with ID, vector, and payload
            PointStruct::new(i as u64, vec.clone(), payload)
        })
        .collect();

    // Use the builder-style API to create the upsert request
    let req = UpsertPointsBuilder::new(collection, points).wait(true); // wait=true mimics the old “blocking”

    // Execute the upsert operation
    client.upsert_points(req).await?;
    Ok(())
}
