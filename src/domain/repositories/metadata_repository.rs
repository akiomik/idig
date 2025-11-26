use anyhow::Result;

use crate::domain::entities::Metadata;
use crate::domain::value_objects::MetadataId;

/// Repository trait for `Metadata` operations
///
/// This trait defines the contract for loading backup metadata from various sources,
/// following the Repository pattern in Domain-Driven Design.
pub trait MetadataRepository {
    /// Loads backup metadata by its unique identifier
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the backup metadata
    ///
    /// # Returns
    /// * `Result<Metadata>` - The loaded backup metadata or an error
    ///
    /// # Errors
    /// Returns an error if there's an issue accessing the data source or if the metadata is not found
    #[allow(
        async_fn_in_trait,
        reason = "This trait is only used internally and Send bounds are handled explicitly"
    )]
    async fn find_by_id(&self, id: &MetadataId) -> Result<Metadata>;

    /// Lists all available backup metadata
    ///
    /// # Returns
    /// * `Result<Vec<Metadata>>` - List of all backup metadata or an error
    ///
    /// # Errors
    /// Returns an error if there's an issue accessing the data source
    #[allow(
        async_fn_in_trait,
        reason = "This trait is only used internally and Send bounds are handled explicitly"
    )]
    async fn find_all(&self) -> Result<Vec<Metadata>>;
}
