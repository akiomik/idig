//! Display service for formatting and presenting search results

use crate::{ExtractResult, File};
use tabled::{Table, Tabled, settings::Style};

/// Represents a file for table display
#[derive(Tabled)]
struct FileTableRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Domain")]
    domain: String,
    #[tabled(rename = "Path")]
    path: String,
}

/// Represents extraction statistics for table display
#[derive(Tabled)]
struct ExtractionStatsRow {
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Count")]
    count: usize,
}

/// Represents extraction errors for table display
#[derive(Tabled)]
struct ExtractionErrorRow {
    #[tabled(rename = "File Path")]
    path: String,
    #[tabled(rename = "Error")]
    error: String,
}

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
            let table_rows: Vec<FileTableRow> = results
                .into_iter()
                .map(|file| FileTableRow {
                    id: file.file_id().value().to_owned(),
                    domain: file.domain().value().to_owned(),
                    path: file.relative_path().value().to_owned(),
                })
                .collect();

            let mut table = Table::new(table_rows);
            table.with(Style::rounded());
            println!("{table}");
        }
    }

    /// Display extract results to stdout
    #[inline]
    pub fn display_extract_results(&self, result: &ExtractResult) {
        println!("Extraction completed:");

        // Display statistics table
        let stats_rows = vec![
            ExtractionStatsRow {
                status: "Extracted".to_owned(),
                count: result.extracted_count,
            },
            ExtractionStatsRow {
                status: "Skipped".to_owned(),
                count: result.skipped_count,
            },
            ExtractionStatsRow {
                status: "Errors".to_owned(),
                count: result.errors.len(),
            },
        ];

        let mut stats_table = Table::new(stats_rows);
        stats_table.with(Style::rounded());
        println!("{stats_table}");

        // Display errors table if there are any errors
        if !result.errors.is_empty() {
            println!("\nError details:");
            let error_rows: Vec<ExtractionErrorRow> = result
                .errors
                .iter()
                .map(|error| ExtractionErrorRow {
                    path: error.relative_path.clone(),
                    error: error.error.clone(),
                })
                .collect();

            let mut error_table = Table::new(error_rows);
            error_table.with(Style::rounded());
            println!("{error_table}");
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
            let table_rows: Vec<FileTableRow> = results
                .into_iter()
                .map(|file| FileTableRow {
                    id: file.file_id().value().to_owned(),
                    domain: file.domain().value().to_owned(),
                    path: file.relative_path().value().to_owned(),
                })
                .collect();

            let mut table = Table::new(table_rows);
            table.with(Style::rounded());
            output.push_str(&table.to_string());
            output
        }
    }

    /// Format extract results as a string (useful for testing)
    #[must_use]
    #[inline]
    pub fn format_extract_results(&self, result: &ExtractResult) -> String {
        let mut output = "Extraction completed:\n".to_owned();

        // Format statistics table
        let stats_rows = vec![
            ExtractionStatsRow {
                status: "Extracted".to_owned(),
                count: result.extracted_count,
            },
            ExtractionStatsRow {
                status: "Skipped".to_owned(),
                count: result.skipped_count,
            },
            ExtractionStatsRow {
                status: "Errors".to_owned(),
                count: result.errors.len(),
            },
        ];

        let mut stats_table = Table::new(stats_rows);
        stats_table.with(Style::rounded());
        output.push_str(&stats_table.to_string());

        // Format errors table if there are any errors
        if !result.errors.is_empty() {
            output.push_str("\nError details:\n");
            let error_rows: Vec<ExtractionErrorRow> = result
                .errors
                .iter()
                .map(|error| ExtractionErrorRow {
                    path: error.relative_path.clone(),
                    error: error.error.clone(),
                })
                .collect();

            let mut error_table = Table::new(error_rows);
            error_table.with(Style::rounded());
            output.push_str(&error_table.to_string());
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
        // Test that the output contains the expected data in table format
        assert!(output.contains("Found 1 file(s):"));
        assert!(output.contains("1230000000000000000000000000000000000000"));
        assert!(output.contains("com.apple.test"));
        assert!(output.contains("Documents/test.txt"));
        assert!(output.contains("ID"));
        assert!(output.contains("Domain"));
        assert!(output.contains("Path"));
        Ok(())
    }

    #[test]
    fn test_format_search_results_multiple_files() -> Result<()> {
        let service = DisplayService::new();
        let file1 = create_test_file("123", "com.apple.test", "Documents/test1.txt")?;
        let file2 = create_test_file("456", "com.apple.photos", "Library/photo.jpg")?;
        let results = vec![file1, file2];

        let output = service.format_search_results(results);
        // Test that the output contains the expected data in table format
        assert!(output.contains("Found 2 file(s):"));
        assert!(output.contains("1230000000000000000000000000000000000000"));
        assert!(output.contains("4560000000000000000000000000000000000000"));
        assert!(output.contains("com.apple.test"));
        assert!(output.contains("com.apple.photos"));
        assert!(output.contains("Documents/test1.txt"));
        assert!(output.contains("Library/photo.jpg"));
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
        // Test that the output contains the expected data in table format
        assert!(output.contains("Extraction completed:"));
        assert!(output.contains("Extracted"));
        assert!(output.contains('5'));
        assert!(output.contains("Skipped"));
        assert!(output.contains('2'));
        assert!(output.contains("Errors"));
        assert!(output.contains('0'));
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
        // Test that the output contains the expected data in table format
        assert!(output.contains("Extraction completed:"));
        assert!(output.contains('3'));
        assert!(output.contains('1'));
        assert!(output.contains('2'));
        assert!(output.contains("Error details:"));
        assert!(output.contains("Documents/test.txt"));
        assert!(output.contains("Permission denied"));
        assert!(output.contains("Photos/image.jpg"));
        assert!(output.contains("Disk full"));
    }
}
