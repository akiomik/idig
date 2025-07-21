use crate::domain::entities::Metadata;
use crate::domain::repositories::MetadataRepository;
use anyhow::Result;
use std::sync::Arc;

/// Application service for listing backup metadata
///
/// This service handles the discovery and loading of backup metadata.
/// It delegates the actual metadata loading to the repository implementation.
#[derive(Debug)]
pub struct ListService<R: MetadataRepository> {
    repository: Arc<R>,
}

impl<R: MetadataRepository> ListService<R> {
    /// Creates a new `ListService` with the given repository
    #[inline]
    pub const fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Lists all available backups using the repository
    ///
    /// # Returns
    /// * `Result<Vec<Metadata>>` - List of backup metadata or an error
    ///
    /// # Errors
    /// Returns an error if the repository cannot access the data source
    #[inline]
    pub async fn list_backups(&self) -> Result<Vec<Metadata>>
    where
        R: Send + Sync,
    {
        self.repository.find_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::MetadataId;
    use anyhow::{Context as _, Result};
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    /// Mock implementation of `MetadataRepository` for unit testing
    #[derive(Debug, Clone)]
    struct MockMetadataRepository {
        /// Map from metadata ID to metadata
        metadata_by_id: HashMap<String, Metadata>,
        /// List of all metadata for `find_all`
        all_metadata: Vec<Metadata>,
        /// Whether to simulate errors
        should_error: bool,
        /// Error message to return
        error_message: String,
    }

    impl MockMetadataRepository {
        /// Creates a new mock repository
        fn new() -> Self {
            Self {
                metadata_by_id: HashMap::new(),
                all_metadata: Vec::new(),
                should_error: false,
                error_message: String::new(),
            }
        }

        /// Adds metadata that can be found by ID
        fn with_metadata_by_id(mut self, id: &str, metadata: Metadata) -> Self {
            self.metadata_by_id.insert(id.to_owned(), metadata);
            self
        }

        /// Sets the list of all metadata
        fn with_all_metadata(mut self, metadata_list: Vec<Metadata>) -> Self {
            self.all_metadata = metadata_list;
            self
        }

        /// Makes the repository return errors
        fn with_error(mut self, error_message: &str) -> Self {
            self.should_error = true;
            self.error_message = error_message.to_owned();
            self
        }
    }

    impl MetadataRepository for MockMetadataRepository {
        async fn find_by_id(&self, id: &MetadataId) -> Result<Metadata> {
            if self.should_error {
                return Err(anyhow::anyhow!("{}", self.error_message));
            }

            self.metadata_by_id
                .get(id.value())
                .cloned()
                .with_context(|| "Backup does not exist")
        }

        async fn find_all(&self) -> Result<Vec<Metadata>> {
            if self.should_error {
                return Err(anyhow::anyhow!("{}", self.error_message));
            }

            Ok(self.all_metadata.clone())
        }
    }

    /// Helper function to create test metadata
    fn create_test_metadata(id: &str, device_name: &str, product_name: &str) -> Result<Metadata> {
        let metadata_id = MetadataId::new(id)?;
        let last_backup_date =
            DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")?.with_timezone(&Utc);

        Ok(Metadata::new(
            metadata_id,
            device_name.to_owned(),
            product_name.to_owned(),
            last_backup_date,
        ))
    }

    // Pure unit tests (no file system dependencies)

    #[tokio::test]
    async fn test_list_backups_success() -> Result<()> {
        // Create test metadata
        let metadata1 = create_test_metadata("a1b2c3d4e5f67890123456789", "iPhone 15", "iPhone")?;
        let metadata2 = create_test_metadata("b2c3d4e5f6789012345678901", "iPad Pro", "iPad")?;
        let metadata3 = create_test_metadata("c3d4e5f6789012345678901ab", "iPhone 14", "iPhone")?;

        let all_metadata = vec![metadata1.clone(), metadata2.clone(), metadata3.clone()];

        // Set up mock repository
        let mock_repo = MockMetadataRepository::new().with_all_metadata(all_metadata);

        let service = ListService::new(Arc::new(mock_repo));

        // Test the service
        let results = service.list_backups().await?;

        // Verify results
        assert_eq!(results.len(), 3);

        // Check that all expected metadata is present
        let device_names: Vec<&str> = results.iter().map(Metadata::device_name).collect();
        assert!(device_names.contains(&"iPhone 15"));
        assert!(device_names.contains(&"iPad Pro"));
        assert!(device_names.contains(&"iPhone 14"));

        Ok(())
    }

    #[tokio::test]
    async fn test_list_backups_empty() -> Result<()> {
        // Set up mock repository with no metadata
        let mock_repo = MockMetadataRepository::new().with_all_metadata(vec![]);

        let service = ListService::new(Arc::new(mock_repo));

        // Test the service
        let results = service.list_backups().await?;

        // Verify results
        assert_eq!(results.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_list_backups_error() -> Result<()> {
        // Set up mock repository to return error
        let mock_repo = MockMetadataRepository::new().with_error("Database connection failed");

        let service = ListService::new(Arc::new(mock_repo));

        // Test the service
        let result = service.list_backups().await;

        // Verify error
        assert!(result.is_err());
        if let Err(error) = result {
            assert!(error.to_string().contains("Database connection failed"));
        }

        Ok(())
    }

    // Unit tests for the repository interaction (pure unit tests)
    #[tokio::test]
    async fn test_mock_repository_find_by_id() -> Result<()> {
        let metadata = create_test_metadata("a1b2c3d4e5f67890123456789", "Test Device", "iPhone")?;
        let metadata_id = MetadataId::new("a1b2c3d4e5f67890123456789")?;

        let mock_repo = MockMetadataRepository::new()
            .with_metadata_by_id("a1b2c3d4e5f67890123456789", metadata.clone());

        let result = mock_repo.find_by_id(&metadata_id).await?;
        assert_eq!(result.device_name(), "Test Device");
        assert_eq!(result.product_name(), "iPhone");

        Ok(())
    }

    #[tokio::test]
    async fn test_mock_repository_not_found() -> Result<()> {
        let mock_repo = MockMetadataRepository::new();
        let metadata_id = MetadataId::new("a1b2c3d4e5f67890123456789")?;

        let result_by_id = mock_repo.find_by_id(&metadata_id).await;
        assert!(result_by_id.is_err());

        Ok(())
    }
}
