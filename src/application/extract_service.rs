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
        backup_dir: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
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
        let output_dir = output_dir.as_ref();
        fs::create_dir_all(output_dir).with_context(|| {
            format!(
                "Failed to create output directory: {}",
                output_dir.display()
            )
        })?;

        let backup_dir = backup_dir.as_ref();
        for file in files {
            match Self::extract_single_file(&file, backup_dir, output_dir) {
                Ok(true) => {
                    result.extracted_count = result.extracted_count.saturating_add(1);
                }
                Ok(false) => {
                    result.skipped_count = result.skipped_count.saturating_add(1);
                }
                Err(e) => result.errors.push(ExtractError {
                    file_id: file.id().to_string(),
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
    fn extract_single_file(file: &File, backup_dir: &Path, output_dir: &Path) -> Result<bool> {
        let file_id_str = file.id().to_string();

        // Construct source path: backup_dir/XX/fileID (where XX is first 2 chars of fileID)
        let prefix = &file_id_str[0..2];
        let source_path = backup_dir.join(prefix).join(&file_id_str);

        // Skip if source file doesn't exist
        if !source_path.exists() {
            return Ok(false);
        }

        // Construct destination path preserving relative path structure
        let dest_path = output_dir.join(file.relative_path().to_string());

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
    use crate::domain::value_objects::{Domain, FileFlags, FileId, RelativePath};
    use anyhow::Result;
    use assert_fs::TempDir;
    use assert_fs::prelude::*;
    use predicates::path;
    use pretty_assertions::assert_eq;

    // Mock repository for testing
    struct MockFileRepository {
        files: Vec<File>,
        should_fail: bool,
    }

    impl MockFileRepository {
        const fn new(files: Vec<File>) -> Self {
            Self {
                files,
                should_fail: false,
            }
        }

        const fn new_failing() -> Self {
            Self {
                files: vec![],
                should_fail: true,
            }
        }
    }

    impl FileRepository for MockFileRepository {
        async fn search(&self, _query: FileQuery) -> Result<Vec<File>> {
            if self.should_fail {
                return Err(anyhow::anyhow!("Mock repository failure"));
            }
            Ok(self.files.clone())
        }
    }

    fn create_test_file() -> Result<File> {
        let file_id = FileId::new("da39a3ee5e6b4b0d3255bfef95601890afd80709")?;
        let domain = Domain::new("AppDomain-com.apple.test".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        Ok(File::new(file_id, domain, relative_path, flags, metadata))
    }

    fn create_test_file_with_params(
        file_id_str: &str,
        domain_str: &str,
        relative_path_str: &str,
    ) -> Result<File> {
        let file_id = FileId::new(file_id_str)?;
        let domain = Domain::new(domain_str.to_owned())?;
        let relative_path = RelativePath::new(relative_path_str.to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        Ok(File::new(file_id, domain, relative_path, flags, metadata))
    }

    #[tokio::test]
    async fn test_extract_service_new() {
        let service = ExtractService::new();
        let service2 = ExtractService::default();

        // Both should be unit structs and create successfully
        // We can't directly compare unit structs, but we can test that they behave the same
        assert!(matches!(service, ExtractService));
        assert!(matches!(service2, ExtractService));
    }

    #[tokio::test]
    async fn test_extract_service_no_files() -> Result<()> {
        let service = ExtractService::new();
        let repo = MockFileRepository::new(vec![]);
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let temp_backup = TempDir::new()?;
        let temp_output = TempDir::new()?;

        let result = service
            .extract(&repo, temp_backup.path(), temp_output.path(), params)
            .await?;

        assert_eq!(result.extracted_count, 0);
        assert_eq!(result.skipped_count, 0);
        assert!(result.errors.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_extract_service_repository_error() -> Result<()> {
        let service = ExtractService::new();
        let repo = MockFileRepository::new_failing();
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let temp_backup = TempDir::new()?;
        let temp_output = TempDir::new()?;

        let result = service
            .extract(&repo, temp_backup.path(), temp_output.path(), params)
            .await;

        assert!(result.is_err());
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Failed to search for files"));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_extract_service_file_not_found_skipped() -> Result<()> {
        let service = ExtractService::new();
        let test_file = create_test_file()?;
        let repo = MockFileRepository::new(vec![test_file]);
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let temp_backup = TempDir::new()?;
        let temp_output = TempDir::new()?;

        // Don't create the source file - it should be skipped

        let result = service
            .extract(&repo, temp_backup.path(), temp_output.path(), params)
            .await?;

        assert_eq!(result.extracted_count, 0);
        assert_eq!(result.skipped_count, 1);
        assert!(result.errors.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_extract_service_successful_extraction() -> Result<()> {
        let service = ExtractService::new();
        let test_file = create_test_file()?;
        let repo = MockFileRepository::new(vec![test_file.clone()]);
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let temp_backup = TempDir::new()?;
        let temp_output = TempDir::new()?;

        // Create the source file in backup directory structure
        let file_id_str = test_file.id().to_string();
        let prefix = &file_id_str[0..2];
        temp_backup
            .child(prefix)
            .child(&file_id_str)
            .write_str("test file content")?;

        let result = service
            .extract(&repo, temp_backup.path(), temp_output.path(), params)
            .await?;

        assert_eq!(result.extracted_count, 1);
        assert_eq!(result.skipped_count, 0);
        assert!(result.errors.is_empty());

        // Verify the file was copied to the correct destination
        temp_output
            .child("Documents")
            .child("test.txt")
            .assert("test file content");

        Ok(())
    }

    #[tokio::test]
    async fn test_extract_service_multiple_files_mixed_results() -> Result<()> {
        let service = ExtractService::new();

        // Create multiple test files
        let file1 = create_test_file_with_params(
            "da39a3ee5e6b4b0d3255bfef95601890afd80709",
            "AppDomain-com.apple.test1",
            "Documents/file1.txt",
        )?;
        let file2 = create_test_file_with_params(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4",
            "AppDomain-com.apple.test2",
            "Photos/image.jpg",
        )?;
        let file3 = create_test_file_with_params(
            "356a192b7913b04c54574d18c28d46e6395428ab",
            "AppDomain-com.apple.test3",
            "Music/song.mp3",
        )?;

        let repo = MockFileRepository::new(vec![file1.clone(), file2.clone(), file3.clone()]);
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let temp_backup = TempDir::new()?;
        let temp_output = TempDir::new()?;

        // Create source files for file1 and file3 only (file2 will be skipped)
        for file in [&file1, &file3] {
            let file_id_str = file.id().to_string();
            let prefix = &file_id_str[0..2];
            temp_backup
                .child(prefix)
                .child(&file_id_str)
                .write_str(&format!("content for {file_id_str}"))?;
        }

        let result = service
            .extract(&repo, temp_backup.path(), temp_output.path(), params)
            .await?;

        assert_eq!(result.extracted_count, 2);
        assert_eq!(result.skipped_count, 1);
        assert!(result.errors.is_empty());

        // Verify the extracted files exist
        temp_output
            .child("Documents")
            .child("file1.txt")
            .assert(path::exists());
        temp_output
            .child("Music")
            .child("song.mp3")
            .assert(path::exists());
        temp_output
            .child("Photos")
            .child("image.jpg")
            .assert(path::missing());

        Ok(())
    }

    #[tokio::test]
    async fn test_extract_service_nested_directory_structure() -> Result<()> {
        let service = ExtractService::new();
        let test_file = create_test_file_with_params(
            "da627651e4e11a916ba8a9cd3f235e9a25a3b58e",
            "AppDomain-com.apple.test",
            "Documents/Projects/MyApp/src/main.rs",
        )?;
        let repo = MockFileRepository::new(vec![test_file.clone()]);
        let params = SearchParams::new(Some("test.domain".to_owned()), None, None, None, false);

        let temp_backup = TempDir::new()?;
        let temp_output = TempDir::new()?;

        // Create the source file
        let file_id_str = test_file.id().to_string();
        let prefix = &file_id_str[0..2];
        temp_backup
            .child(prefix)
            .child(&file_id_str)
            .write_str("fn main() { println!(\"Hello!\"); }")?;

        let result = service
            .extract(&repo, temp_backup.path(), temp_output.path(), params)
            .await?;

        assert_eq!(result.extracted_count, 1);
        assert_eq!(result.skipped_count, 0);
        assert!(result.errors.is_empty());

        // Verify nested directory structure is created correctly
        temp_output
            .child("Documents")
            .child("Projects")
            .child("MyApp")
            .child("src")
            .child("main.rs")
            .assert("fn main() { println!(\"Hello!\"); }");

        Ok(())
    }

    #[test]
    fn test_extract_result_equality() {
        let result1 = ExtractResult {
            extracted_count: 1,
            skipped_count: 2,
            errors: vec![ExtractError {
                file_id: "test123".to_owned(),
                relative_path: "test/path.txt".to_owned(),
                error: "Test error".to_owned(),
            }],
        };

        let result2 = ExtractResult {
            extracted_count: 1,
            skipped_count: 2,
            errors: vec![ExtractError {
                file_id: "test123".to_owned(),
                relative_path: "test/path.txt".to_owned(),
                error: "Test error".to_owned(),
            }],
        };

        assert_eq!(result1, result2);

        // Test Clone
        let cloned = result1;
        assert_eq!(cloned, result2);
    }

    #[test]
    fn test_extract_error_equality() {
        let error1 = ExtractError {
            file_id: "abc123".to_owned(),
            relative_path: "Documents/test.txt".to_owned(),
            error: "Permission denied".to_owned(),
        };

        let error2 = ExtractError {
            file_id: "abc123".to_owned(),
            relative_path: "Documents/test.txt".to_owned(),
            error: "Permission denied".to_owned(),
        };

        assert_eq!(error1, error2);

        // Test Clone
        let cloned = error1;
        assert_eq!(cloned, error2);
    }
}
