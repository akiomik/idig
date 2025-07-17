use std::fmt;

/// `RelativePath` - Value Object representing a relative path within a backup
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativePath(String);

impl RelativePath {
    /// Creates a new `RelativePath`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path is an absolute path (starts with '/' or '\')
    ///
    /// Note: Empty paths are allowed as they represent files at the root level
    #[inline]
    pub fn new(path: String) -> anyhow::Result<Self> {
        if path.starts_with('/') || path.starts_with('\\') {
            return Err(anyhow::anyhow!("RelativePath cannot be an absolute path"));
        }

        Ok(Self(path))
    }

    #[must_use]
    #[inline]
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RelativePath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RelativePath> for String {
    #[inline]
    fn from(path: RelativePath) -> Self {
        path.0
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_relative_path_creation() -> Result<()> {
        let path = RelativePath::new("Documents/file.txt".to_owned())?;
        assert_eq!(path.value(), "Documents/file.txt");
        Ok(())
    }

    #[test]
    fn test_relative_path_absolute_path_rejected() {
        let absolute_path = "/absolute/path".to_owned();
        assert!(RelativePath::new(absolute_path).is_err());
    }

    #[test]
    fn test_relative_path_windows_absolute_rejected() {
        let windows_absolute = "\\absolute\\path".to_owned();
        assert!(RelativePath::new(windows_absolute).is_err());
    }

    #[test]
    fn test_relative_path_empty() -> Result<()> {
        let empty_path = String::new();
        let path = RelativePath::new(empty_path)?;
        assert_eq!(path.value(), "");
        Ok(())
    }
}
