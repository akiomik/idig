use std::fmt;

/// `FileID` - Value Object representing a SHA1 hash
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileId(String);

impl FileId {
    /// Creates a new `FileId` from a SHA1 hash string
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The string is empty
    /// - The string is not exactly 40 characters long
    /// - The string contains non-hexadecimal characters
    #[inline]
    pub fn new(id: &str) -> anyhow::Result<Self> {
        if id.is_empty() {
            return Err(anyhow::anyhow!("FileId cannot be empty"));
        }

        // SHA1 hash is a 40-character hexadecimal string
        if id.len() != 40 {
            return Err(anyhow::anyhow!(
                "FileId must be 40 characters long (SHA1 hash)"
            ));
        }

        if !id.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "FileId must contain only hexadecimal characters"
            ));
        }

        Ok(Self(id.to_lowercase()))
    }

    /// Returns the string value of the `FileId`
    #[must_use]
    #[inline]
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FileId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<FileId> for String {
    #[inline]
    fn from(file_id: FileId) -> Self {
        file_id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_id_creation() {
        let valid_sha1 = "a1b2c3d4e5f6789012345678901234567890abcd";
        let file_id = FileId::new(valid_sha1).unwrap();
        assert_eq!(file_id.value(), "a1b2c3d4e5f6789012345678901234567890abcd");
    }

    #[test]
    fn test_file_id_invalid_length() {
        let invalid_sha1 = "short";
        assert!(FileId::new(invalid_sha1).is_err());
    }

    #[test]
    fn test_file_id_invalid_characters() {
        let invalid_sha1 = "g1b2c3d4e5f6789012345678901234567890abcd"; // 'g' is not hex
        assert!(FileId::new(invalid_sha1).is_err());
    }

    #[test]
    fn test_file_id_empty() {
        let empty_id = "";
        assert!(FileId::new(empty_id).is_err());
    }

    #[test]
    fn test_file_id_case_normalization() {
        let uppercase_sha1 = "A1B2C3D4E5F6789012345678901234567890ABCD";
        let file_id = FileId::new(uppercase_sha1).unwrap();
        assert_eq!(file_id.value(), "a1b2c3d4e5f6789012345678901234567890abcd");
    }
}
