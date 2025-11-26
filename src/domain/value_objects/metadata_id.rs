use std::fmt;

/// `MetadatId` - Value Object representing a unique id of a backup
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetadataId(String);

impl MetadataId {
    /// Creates a new `MetadataId`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The string is empty
    /// - The string is not exactly 25 characters long
    /// - The string contains non-alphanumeric characters, except the hyphen (-)
    #[inline]
    pub fn new(id: &str) -> anyhow::Result<Self> {
        if id.is_empty() {
            return Err(anyhow::anyhow!("MetadataId cannot be empty"));
        }

        // SHA1 hash is a 25-character hexadecimal string
        if id.len() != 25 {
            return Err(anyhow::anyhow!("MetadataId must be 25 characters long"));
        }

        if !id.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(anyhow::anyhow!(
                "MetadataId must contain only alphanumeric characters or the hyphen"
            ));
        }

        Ok(Self(id.to_lowercase()))
    }

    /// Returns the string value of the `MetadataId`
    #[must_use]
    #[inline]
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MetadataId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<MetadataId> for String {
    #[inline]
    fn from(metadata_id: MetadataId) -> Self {
        metadata_id.0
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_metadata_id_creation() -> Result<()> {
        let valid_id = "a1b2c3d4e5f67890123456789";
        let metadata_id = MetadataId::new(valid_id)?;

        assert_eq!(metadata_id.value(), "a1b2c3d4e5f67890123456789");

        Ok(())
    }

    #[test]
    fn test_metadata_id_invalid_length() {
        let invalid_length = "short";

        assert!(MetadataId::new(invalid_length).is_err());
    }

    #[test]
    fn test_metadata_id_invalid_characters() {
        let invalid_chars = "_1b2c3d4e5f67890123456789"; // '_' is not allowed
        //
        assert!(MetadataId::new(invalid_chars).is_err());
    }

    #[test]
    fn test_metadata_id_empty() {
        let empty_id = "";

        assert!(MetadataId::new(empty_id).is_err());
    }

    #[test]
    fn test_metadata_id_case_normalization() -> Result<()> {
        let uppercase_id = "A1B2C3D4E5F67890123456789";
        let metadata_id = MetadataId::new(uppercase_id)?;

        assert_eq!(metadata_id.value(), "a1b2c3d4e5f67890123456789");

        Ok(())
    }
}
