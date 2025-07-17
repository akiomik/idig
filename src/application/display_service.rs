//! Display service for formatting and presenting search results

use crate::{ExtractResult, File};

/// Service for handling result display operations
#[non_exhaustive]
pub struct DisplayService;

impl DisplayService {
    /// Create a new display service instance
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Display search results to stdout
    #[inline]
    pub fn display_search_results(&self, results: Vec<File>) {
        if results.is_empty() {
            println!("No files found matching the search criteria.");
        } else {
            println!("Found {} file(s):", results.len());
            for file in results {
                println!(
                    "  ID: {} | Domain: {} | Path: {}",
                    file.file_id().value(),
                    file.domain().value(),
                    file.relative_path().value()
                );
            }
        }
    }

    /// Display extract results to stdout
    #[inline]
    pub fn display_extract_results(&self, result: &ExtractResult) {
        println!("Extraction completed:");
        println!("  Extracted: {} files", result.extracted_count);
        println!("  Skipped: {} files", result.skipped_count);

        if !result.errors.is_empty() {
            println!("  Errors: {} files", result.errors.len());
            for error in &result.errors {
                eprintln!(
                    "    Error extracting {}: {}",
                    error.relative_path, error.error
                );
            }
        }
    }

    /// Format search results as a string (useful for testing)
    #[must_use]
    #[inline]
    pub fn format_search_results(&self, results: Vec<File>) -> String {
        if results.is_empty() {
            "No files found matching the search criteria.".to_owned()
        } else {
            let mut output = format!("Found {} file(s):\n", results.len());
            for file in results {
                output.push_str(&format!(
                    "  ID: {} | Domain: {} | Path: {}\n",
                    file.file_id().value(),
                    file.domain().value(),
                    file.relative_path().value()
                ));
            }
            output.trim_end().to_owned()
        }
    }

    /// Format extract results as a string (useful for testing)
    #[must_use]
    #[inline]
    pub fn format_extract_results(&self, result: &ExtractResult) -> String {
        let mut output = format!(
            "Extraction completed:\n  Extracted: {} files\n  Skipped: {} files",
            result.extracted_count, result.skipped_count
        );

        if !result.errors.is_empty() {
            output.push_str(&format!("\n  Errors: {} files", result.errors.len()));
            for error in &result.errors {
                output.push_str(&format!(
                    "\n    Error extracting {}: {}",
                    error.relative_path, error.error
                ));
            }
        }

        output
    }
}

impl Default for DisplayService {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::{Domain, ExtractError, FileFlags, FileId, RelativePath};

    fn create_test_file(id: &str, domain: &str, path: &str) -> Result<File> {
        // Create a valid 40-character SHA1 hash by padding the id
        let padded_id = format!("{id:0<40}");
        Ok(File::new(
            FileId::new(&padded_id)?,
            Domain::new(domain.to_owned())?,
            RelativePath::new(path.to_owned())?,
            FileFlags::new(0),
            vec![], // empty file metadata
        ))
    }

    #[test]
    fn test_format_search_results_empty() {
        let service = DisplayService::new();
        let results = vec![];

        let output = service.format_search_results(results);
        assert_eq!(output, "No files found matching the search criteria.");
    }

    #[test]
    fn test_format_search_results_single_file() -> Result<()> {
        let service = DisplayService::new();
        let file = create_test_file("123", "com.apple.test", "Documents/test.txt")?;
        let results = vec![file];

        let output = service.format_search_results(results);
        let expected = "Found 1 file(s):\n  ID: 1230000000000000000000000000000000000000 | Domain: com.apple.test | Path: Documents/test.txt";
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_format_search_results_multiple_files() -> Result<()> {
        let service = DisplayService::new();
        let file1 = create_test_file("123", "com.apple.test", "Documents/test1.txt")?;
        let file2 = create_test_file("456", "com.apple.photos", "Library/photo.jpg")?;
        let results = vec![file1, file2];

        let output = service.format_search_results(results);
        let expected = "Found 2 file(s):\n  ID: 1230000000000000000000000000000000000000 | Domain: com.apple.test | Path: Documents/test1.txt\n  ID: 4560000000000000000000000000000000000000 | Domain: com.apple.photos | Path: Library/photo.jpg";
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_format_extract_results_success_only() {
        let service = DisplayService::new();
        let result = ExtractResult {
            extracted_count: 5,
            skipped_count: 2,
            errors: vec![],
        };

        let output = service.format_extract_results(&result);
        let expected = "Extraction completed:\n  Extracted: 5 files\n  Skipped: 2 files";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_extract_results_with_errors() {
        let service = DisplayService::new();
        let result = ExtractResult {
            extracted_count: 3,
            skipped_count: 1,
            errors: vec![
                ExtractError {
                    file_id: "abc123".to_owned(),
                    relative_path: "Documents/test.txt".to_owned(),
                    error: "Permission denied".to_owned(),
                },
                ExtractError {
                    file_id: "def456".to_owned(),
                    relative_path: "Photos/image.jpg".to_owned(),
                    error: "Disk full".to_owned(),
                },
            ],
        };

        let output = service.format_extract_results(&result);
        let expected = "Extraction completed:\n  Extracted: 3 files\n  Skipped: 1 files\n  Errors: 2 files\n    Error extracting Documents/test.txt: Permission denied\n    Error extracting Photos/image.jpg: Disk full";
        assert_eq!(output, expected);
    }
}
