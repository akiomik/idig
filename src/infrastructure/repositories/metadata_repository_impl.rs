use crate::domain::entities::Metadata;
use crate::domain::repositories::MetadataRepository;
use crate::domain::value_objects::MetadataId;
use crate::infrastructure::plist::entities::BackupInfo;
use anyhow::{Context as _, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Implementation of `MetadataRepository` for file-based backup storage
///
/// This implementation handles reading and parsing iPhone backup Info.plist files
/// from the file system using the plist crate for deserialization.
#[derive(Debug)]
#[non_exhaustive]
pub struct MetadataRepositoryImpl {
    /// Root directory containing backup folders
    backup_root: PathBuf,
}

impl MetadataRepositoryImpl {
    /// Creates a new instance with a specific backup root directory
    #[must_use]
    #[inline]
    pub fn new<P: AsRef<Path>>(backup_root: P) -> Self {
        Self {
            backup_root: backup_root.as_ref().to_path_buf(),
        }
    }

    /// Loads metadata from a plist file content
    fn load_from_plist_content(plist_content: &[u8]) -> Result<Metadata> {
        // Parse the plist content
        let plist_info: BackupInfo =
            plist::from_bytes(plist_content).with_context(|| "Failed to parse plist content")?;

        // Convert to domain entity
        plist_info.to_domain()
    }
}

impl MetadataRepository for MetadataRepositoryImpl {
    #[inline]
    async fn find_by_id(&self, id: &MetadataId) -> Result<Metadata> {
        if !self.backup_root.exists() {
            return Err(anyhow::anyhow!(
                "Backup root directory does not exist: {}",
                self.backup_root.display()
            ));
        }

        // Look for a directory with this ID
        let backup_dir = self.backup_root.join(id.value());
        if !backup_dir.exists() {
            return Err(anyhow::anyhow!(
                "Backup directory does not exist: {}",
                backup_dir.display()
            ));
        }

        self.find_by_backup_directory(&backup_dir).await
    }

    #[inline]
    async fn find_all(&self) -> Result<Vec<Metadata>> {
        if !self.backup_root.exists() {
            return Err(anyhow::anyhow!(
                "Backup root directory does not exist: {}",
                self.backup_root.display()
            ));
        }

        let mut entries = fs::read_dir(&self.backup_root).await.with_context(|| {
            format!(
                "Failed to read backup directory: {}",
                self.backup_root.display()
            )
        })?;

        let mut metadata_list = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            // Skip non-directories
            if !path.is_dir() {
                continue;
            }

            // Try to load metadata from this directory
            if let Ok(metadata) = self.find_by_backup_directory(&path).await {
                metadata_list.push(metadata);
            }
        }

        Ok(metadata_list)
    }
}

impl MetadataRepositoryImpl {
    /// Private method to find metadata by backup directory
    /// This is an implementation detail and not part of the public interface
    async fn find_by_backup_directory<P: AsRef<Path> + Send>(
        &self,
        backup_directory: P,
    ) -> Result<Metadata> {
        let backup_directory = backup_directory.as_ref();

        if !backup_directory.is_dir() {
            return Err(anyhow::anyhow!(
                "Path is not a directory: {}",
                backup_directory.display()
            ));
        }

        let info_plist_path = backup_directory.join("Info.plist");
        if !info_plist_path.exists() {
            return Err(anyhow::anyhow!(
                "Info.plist file not found: {}",
                info_plist_path.display()
            ));
        }

        // Read and parse the plist file
        let plist_content = fs::read(&info_plist_path).await.with_context(|| {
            format!(
                "Failed to read Info.plist file: {}",
                info_plist_path.display()
            )
        })?;

        Self::load_from_plist_content(&plist_content).with_context(|| {
            format!(
                "Failed to parse Info.plist file: {}",
                info_plist_path.display()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use assert_fs::TempDir;
    use assert_fs::prelude::*;
    use pretty_assertions::assert_eq;

    // Helper to create a valid Info.plist content
    fn create_valid_plist_content() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Device Name</key>
    <string>iPhone 15 Pro</string>
    <key>Last Backup Date</key>
    <string>2024-01-15T10:30:00Z</string>
    <key>Product Name</key>
    <string>iPhone16,1</string>
    <key>Unique Identifier</key>
    <string>a1b2c3d4e5f67890123456789</string>
</dict>
</plist>"#.to_owned()
    }

    // Helper to create an invalid plist content
    fn create_invalid_plist_content() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Device Name</key>
    <string>iPhone 15 Pro</string>
    <key>Last Backup Date</key>
    <string>invalid-date</string>
    <key>Product Name</key>
    <string>iPhone16,1</string>
    <key>Unique Identifier</key>
    <string>invalid-id</string>
</dict>
</plist>"#.to_owned()
    }

    // Helper to create a malformed plist content
    fn create_malformed_plist_content() -> String {
        "This is not a valid plist file".to_owned()
    }

    #[test]
    fn test_metadata_repository_impl_new() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let repo = MetadataRepositoryImpl::new(temp_dir.path());

        // Verify the backup_root is set correctly
        assert_eq!(repo.backup_root, temp_dir.path().to_path_buf());
        Ok(())
    }

    #[test]
    fn test_load_from_plist_content_success() -> Result<()> {
        let plist_content = create_valid_plist_content();
        let metadata = MetadataRepositoryImpl::load_from_plist_content(plist_content.as_bytes())?;

        assert_eq!(metadata.id().value(), "a1b2c3d4e5f67890123456789");
        assert_eq!(metadata.device_name(), "iPhone 15 Pro");
        assert_eq!(metadata.product_name(), "iPhone16,1");

        Ok(())
    }

    #[test]
    fn test_load_from_plist_content_malformed_plist() {
        let malformed_content = create_malformed_plist_content();
        let result = MetadataRepositoryImpl::load_from_plist_content(malformed_content.as_bytes());

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Failed to parse plist content"));
        }
    }

    #[test]
    fn test_load_from_plist_content_invalid_metadata() {
        let invalid_content = create_invalid_plist_content();
        let result = MetadataRepositoryImpl::load_from_plist_content(invalid_content.as_bytes());

        assert!(result.is_err());
        // The error should come from BackupInfo::to_domain validation
    }

    #[tokio::test]
    async fn test_find_by_id_success() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let metadata_id = "a1b2c3d4e5f67890123456789";

        // Create backup directory structure
        temp_dir
            .child(metadata_id)
            .child("Info.plist")
            .write_str(&create_valid_plist_content())?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let id = MetadataId::new(metadata_id)?;
        let metadata = repo.find_by_id(&id).await?;

        assert_eq!(metadata.id().value(), metadata_id);
        assert_eq!(metadata.device_name(), "iPhone 15 Pro");
        assert_eq!(metadata.product_name(), "iPhone16,1");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id_backup_root_not_exists() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf(); // Copy the path before closing
        temp_dir.close()?; // Remove the directory

        let repo = MetadataRepositoryImpl::new(temp_path);
        let id = MetadataId::new("a1b2c3d4e5f67890123456789")?;
        let result = repo.find_by_id(&id).await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Backup root directory does not exist"));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id_backup_dir_not_exists() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let metadata_id = "a1b2c3d4e5f67890123456789";

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let id = MetadataId::new(metadata_id)?;
        let result = repo.find_by_id(&id).await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Backup directory does not exist"));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id_info_plist_not_exists() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let metadata_id = "a1b2c3d4e5f67890123456789";

        // Create backup directory but no Info.plist
        temp_dir.child(metadata_id).create_dir_all()?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let id = MetadataId::new(metadata_id)?;
        let result = repo.find_by_id(&id).await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Info.plist file not found"));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id_invalid_plist() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let metadata_id = "a1b2c3d4e5f67890123456789";

        // Create backup directory with invalid plist
        temp_dir
            .child(metadata_id)
            .child("Info.plist")
            .write_str(&create_malformed_plist_content())?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let id = MetadataId::new(metadata_id)?;
        let result = repo.find_by_id(&id).await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Failed to parse Info.plist file"));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all_success() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create multiple backup directories
        let backup1_id = "a1b2c3d4e5f67890123456789";
        let backup2_id = "b2c3d4e5f67890123456789ab";

        temp_dir
            .child(backup1_id)
            .child("Info.plist")
            .write_str(&create_valid_plist_content())?;

        // Create second backup with different device name
        let plist2 = create_valid_plist_content()
            .replace("iPhone 15 Pro", "iPad Pro")
            .replace("a1b2c3d4e5f67890123456789", backup2_id);

        temp_dir
            .child(backup2_id)
            .child("Info.plist")
            .write_str(&plist2)?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let metadata_list = repo.find_all().await?;

        assert_eq!(metadata_list.len(), 2);

        // Sort by device name for predictable testing
        let mut sorted_metadata = metadata_list;
        sorted_metadata.sort_by(|a, b| a.device_name().cmp(b.device_name()));

        assert_eq!(sorted_metadata[0].device_name(), "iPad Pro");
        assert_eq!(sorted_metadata[1].device_name(), "iPhone 15 Pro");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all_with_invalid_directories() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create valid backup
        let valid_backup_id = "a1b2c3d4e5f67890123456789";
        temp_dir
            .child(valid_backup_id)
            .child("Info.plist")
            .write_str(&create_valid_plist_content())?;

        // Create invalid backup (no Info.plist)
        temp_dir.child("invalid_backup").create_dir_all()?;

        // Create regular file (should be skipped)
        temp_dir
            .child("regular_file.txt")
            .write_str("not a directory")?;

        // Create backup with invalid plist
        temp_dir
            .child("invalid_plist_backup")
            .child("Info.plist")
            .write_str(&create_malformed_plist_content())?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let metadata_list = repo.find_all().await?;

        // Should only return the valid backup
        assert_eq!(metadata_list.len(), 1);
        assert_eq!(metadata_list[0].device_name(), "iPhone 15 Pro");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let metadata_list = repo.find_all().await?;

        assert!(metadata_list.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_find_all_backup_root_not_exists() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_path_buf(); // Copy the path before closing
        temp_dir.close()?; // Remove the directory

        let repo = MetadataRepositoryImpl::new(temp_path);
        let result = repo.find_all().await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Backup root directory does not exist"));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_backup_directory_success() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let backup_dir = temp_dir.child("test_backup");

        backup_dir
            .child("Info.plist")
            .write_str(&create_valid_plist_content())?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let metadata = repo.find_by_backup_directory(backup_dir.path()).await?;

        assert_eq!(metadata.device_name(), "iPhone 15 Pro");

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_backup_directory_not_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.child("not_a_directory.txt");
        file_path.write_str("this is a file")?;

        let repo = MetadataRepositoryImpl::new(temp_dir.path());
        let result = repo.find_by_backup_directory(file_path.path()).await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Path is not a directory"));
        }

        Ok(())
    }
}
