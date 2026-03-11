use super::enbeddings::SourceChunk;
/// Vector database service for centralized Qdrant operations.
///
/// This module provides a high-level service interface for vector database operations,
/// eliminating duplication and providing consistent error handling across the codebase.
use crate::config::audit_config;
use crate::error::{AuditError, Result};
use crate::prepare_code::git_clone::RepoPaths;
use log::info;
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        CreateCollection, Distance, PointStruct, UpsertPointsBuilder, VectorParams, VectorsConfig,
        vectors_config::Config,
    },
};
use std::sync::Arc;

/// Vector database service providing centralized Qdrant operations.
pub struct VectorDbService {
    /// Qdrant client
    client: Qdrant,
    /// Default vector dimension
    vector_dimension: u64,
}

impl VectorDbService {
    /// Creates a new vector database service.
    pub async fn new() -> Result<Self> {
        let qdrant_url = std::env::var("QDRANT_URL")
            .map_err(|_| AuditError::configuration("QDRANT_URL", "Environment variable not set"))?;

        let client = Qdrant::from_url(&qdrant_url).build().map_err(|e| {
            AuditError::vector_db("connection", "Failed to connect to Qdrant database", e)
        })?;

        Ok(Self {
            client,
            vector_dimension: audit_config().vector_dimension,
        })
    }

    /// Creates a new service with custom configuration.
    pub async fn with_config(qdrant_url: &str, vector_dimension: u64) -> Result<Self> {
        let client = Qdrant::from_url(qdrant_url).build().map_err(|e| {
            AuditError::vector_db("connection", "Failed to connect to Qdrant database", e)
        })?;

        Ok(Self {
            client,
            vector_dimension,
        })
    }

    /// Generates a collection name for a repository.
    pub fn collection_name(&self, repo: &RepoPaths) -> String {
        format!("{}-contract_chunks", repo.unique_repo_hash())
    }

    /// Checks if a collection exists for the given repository.
    pub async fn collection_exists(&self, repo: &RepoPaths) -> Result<bool> {
        let collection_name = self.collection_name(repo);
        self.client
            .collection_exists(&collection_name)
            .await
            .map_err(|e| {
                AuditError::vector_db(
                    "collection_check",
                    format!("Failed to check if collection '{}' exists", collection_name),
                    e,
                )
            })
    }

    /// Creates a collection if it doesn't exist.
    pub async fn ensure_collection(&self, repo: &RepoPaths) -> Result<()> {
        let collection_name = self.collection_name(repo);

        if self.collection_exists(repo).await? {
            info!(
                "Collection {} already exists...no need to create",
                collection_name
            );
            return Ok(());
        }

        info!("Creating collection: {}", collection_name);

        let req = CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: self.vector_dimension,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        };

        self.client.create_collection(req).await.map_err(|e| {
            AuditError::vector_db(
                "collection_creation",
                format!("Failed to create collection '{}'", collection_name),
                e,
            )
        })?;

        Ok(())
    }

    /// Upserts embeddings into the collection.
    pub async fn upsert_embeddings(
        &self,
        repo: &RepoPaths,
        items: &[(SourceChunk, Vec<f32>)],
    ) -> Result<()> {
        let collection_name = self.collection_name(repo);

        info!(
            "Upserting {} embeddings to collection: {}",
            items.len(),
            collection_name
        );

        // Build PointStructs from the items
        let points: Vec<PointStruct> = items
            .iter()
            .enumerate()
            .map(|(i, (chunk, vec))| -> Result<PointStruct> {
                let payload: Payload = serde_json::to_value(chunk)
                    .map_err(|e| {
                        AuditError::json(
                            "chunk_serialization",
                            "Failed to serialize SourceChunk",
                            e,
                        )
                    })?
                    .try_into()
                    .map_err(|e| {
                        AuditError::json(
                            "payload_conversion",
                            "Failed to convert JSON to Payload",
                            e,
                        )
                    })?;

                Ok(PointStruct::new(i as u64, vec.clone(), payload))
            })
            .collect::<Result<Vec<_>>>()?;

        // Create and execute the upsert request
        let req = UpsertPointsBuilder::new(&collection_name, points).wait(true);

        self.client.upsert_points(req).await.map_err(|e| {
            AuditError::vector_db(
                "upsert",
                format!(
                    "Failed to upsert embeddings to collection '{}'",
                    collection_name
                ),
                e,
            )
        })?;

        info!("Successfully upserted {} embeddings", items.len());
        Ok(())
    }

    /// Deletes a collection for a repository.
    pub async fn delete_collection(&self, repo: &RepoPaths) -> Result<()> {
        let collection_name = self.collection_name(repo);

        if !self.collection_exists(repo).await? {
            info!(
                "Collection {} does not exist, nothing to delete",
                collection_name
            );
            return Ok(());
        }

        info!("Deleting collection: {}", collection_name);

        self.client
            .delete_collection(&collection_name)
            .await
            .map_err(|e| {
                AuditError::vector_db(
                    "collection_deletion",
                    format!("Failed to delete collection '{}'", collection_name),
                    e,
                )
            })?;

        Ok(())
    }

    /// Returns the underlying Qdrant client for advanced operations.
    pub fn client(&self) -> &Qdrant {
        &self.client
    }

    /// Returns the vector dimension used by this service.
    pub fn vector_dimension(&self) -> u64 {
        self.vector_dimension
    }
}

/// Shared vector database service instance.
use std::sync::OnceLock;
static VECTOR_SERVICE: OnceLock<Arc<VectorDbService>> = OnceLock::new();

/// Initializes the global vector database service.
pub async fn init_vector_service() -> Result<()> {
    let service = VectorDbService::new().await?;
    let arc_service = Arc::new(service);

    VECTOR_SERVICE.set(arc_service).map_err(|_| {
        AuditError::configuration("vector_service", "Vector service already initialized")
    })?;

    Ok(())
}

/// Returns a reference to the global vector database service.
///
/// # Panics
/// Panics if the service has not been initialized with `init_vector_service()`.
pub fn vector_service() -> &'static Arc<VectorDbService> {
    VECTOR_SERVICE
        .get()
        .expect("Vector service not initialized. Call init_vector_service() first.")
}

/// Returns a reference to the global vector database service, or None if not initialized.
pub fn try_vector_service() -> Option<&'static Arc<VectorDbService>> {
    VECTOR_SERVICE.get()
}
