//! Display service for formatting and presenting search results

use crate::File;

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
}

impl Default for DisplayService {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Domain, FileFlags, FileId, RelativePath};

    fn create_test_file(id: &str, domain: &str, path: &str) -> File {
        // Create a valid 40-character SHA1 hash by padding the id
        let padded_id = format!("{:0<40}", id);
        File::new(
            FileId::new(&padded_id).unwrap(),
            Domain::new(domain.to_string()).unwrap(),
            RelativePath::new(path.to_string()).unwrap(),
            FileFlags::new(0),
            vec![], // empty file metadata
        )
    }

    #[test]
    fn test_format_search_results_empty() {
        let service = DisplayService::new();
        let results = vec![];

        let output = service.format_search_results(results);
        assert_eq!(output, "No files found matching the search criteria.");
    }

    #[test]
    fn test_format_search_results_single_file() {
        let service = DisplayService::new();
        let file = create_test_file("123", "com.apple.test", "Documents/test.txt");
        let results = vec![file];

        let output = service.format_search_results(results);
        let expected = "Found 1 file(s):\n  ID: 1230000000000000000000000000000000000000 | Domain: com.apple.test | Path: Documents/test.txt";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_search_results_multiple_files() {
        let service = DisplayService::new();
        let file1 = create_test_file("123", "com.apple.test", "Documents/test1.txt");
        let file2 = create_test_file("456", "com.apple.photos", "Library/photo.jpg");
        let results = vec![file1, file2];

        let output = service.format_search_results(results);
        let expected = "Found 2 file(s):\n  ID: 1230000000000000000000000000000000000000 | Domain: com.apple.test | Path: Documents/test1.txt\n  ID: 4560000000000000000000000000000000000000 | Domain: com.apple.photos | Path: Library/photo.jpg";
        assert_eq!(output, expected);
    }
}
