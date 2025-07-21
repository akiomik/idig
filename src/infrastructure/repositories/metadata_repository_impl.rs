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
