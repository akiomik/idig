//! Extract service for copying files from iPhone backups

use crate::SearchParams;
use crate::domain::entities::File;
use crate::domain::repositories::FileRepository;
use anyhow::{Context as _, Result};
use std::fs;
use std::path::Path;

/// Service for extracting files from iPhone backups
#[non_exhaustive]
pub struct ExtractService;

impl ExtractService {
    /// Creates a new `ExtractService`
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Extracts files based on search parameters
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Search fails
    /// - File system operations fail
    /// - Source files are not found
    #[allow(
        clippy::future_not_send,
        reason = "Repository trait doesn't guarantee Send futures"
    )]
    #[inline]
    pub async fn extract<R: FileRepository>(
        &self,
        repository: &R,
        backup_dir: &str,
        output_dir: &str,
        params: SearchParams,
    ) -> Result<ExtractResult> {
        // Search for files matching the criteria
        let query = params.build_query()?;
        let files = repository
            .search(query)
            .await
            .context("Failed to search for files")?;

        if files.is_empty() {
            return Ok(ExtractResult {
                extracted_count: 0,
                skipped_count: 0,
                errors: Vec::new(),
            });
        }

        let mut result = ExtractResult {
            extracted_count: 0,
            skipped_count: 0,
            errors: Vec::new(),
        };

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {output_dir}"))?;

        for file in files {
            match Self::extract_single_file(&file, backup_dir, output_dir) {
                Ok(true) => {
                    result.extracted_count = result.extracted_count.saturating_add(1);
                }
                Ok(false) => {
                    result.skipped_count = result.skipped_count.saturating_add(1);
                }
                Err(e) => result.errors.push(ExtractError {
                    file_id: file.file_id().to_string(),
                    relative_path: file.relative_path().to_string(),
                    error: e.to_string(),
                }),
            }
        }

        Ok(result)
    }

    /// Extracts a single file
    ///
    /// Returns Ok(true) if extracted, Ok(false) if skipped, Err if failed
    fn extract_single_file(file: &File, backup_dir: &str, output_dir: &str) -> Result<bool> {
        let file_id_str = file.file_id().to_string();

        // Construct source path: backup_dir/XX/fileID (where XX is first 2 chars of fileID)
        let prefix = &file_id_str[0..2];
        let source_path = Path::new(backup_dir).join(prefix).join(&file_id_str);

        // Skip if source file doesn't exist
        if !source_path.exists() {
            return Ok(false);
        }

        // Construct destination path preserving relative path structure
        let dest_path = Path::new(output_dir).join(file.relative_path().to_string());

        // Create parent directories if they don't exist
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create parent directory: {}", parent.display())
            })?;
        }

        // Copy the file
        fs::copy(&source_path, &dest_path).with_context(|| {
            format!(
                "Failed to copy file from {} to {}",
                source_path.display(),
                dest_path.display()
            )
        })?;

        Ok(true)
    }
}

impl Default for ExtractService {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an extraction operation
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ExtractResult {
    /// Number of files successfully extracted
    pub extracted_count: usize,
    /// Number of files skipped (source not found)
    pub skipped_count: usize,
    /// Errors encountered during extraction
    pub errors: Vec<ExtractError>,
}

/// Error information for a failed file extraction
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ExtractError {
    /// File ID that failed to extract
    pub file_id: String,
    /// Relative path of the file
    pub relative_path: String,
    /// Error message
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::File;
    use crate::domain::queries::FileQuery;
    use crate::domain::repositories::FileRepository;

    // Mock repository for testing
    struct MockFileRepository {
        files: Vec<File>,
    }

    impl FileRepository for MockFileRepository {
        async fn search(&self, _query: FileQuery) -> Result<Vec<File>> {
            Ok(self.files.clone())
        }
    }

    #[tokio::test]
    async fn test_extract_service_no_files() -> Result<()> {
        let service = ExtractService::new();
        let repo = MockFileRepository { files: vec![] };
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let result = service.extract(&repo, "/backup", "/output", params).await?;

        assert_eq!(result.extracted_count, 0);
        assert_eq!(result.skipped_count, 0);
        assert!(result.errors.is_empty());
        Ok(())
    }
}
