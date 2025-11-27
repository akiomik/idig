use crate::domain::value_objects::{Domain, FileFlags, FileId, RelativePath};

/// File Entity - Represents a file in a backup
///
/// This entity encapsulates all the information about a file including its unique identifier,
/// domain context, path information, flags, and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// Unique file identifier (SHA1 hash)
    id: FileId,
    /// Application identifier
    domain: Domain,
    /// Relative path within a backup
    relative_path: RelativePath,
    /// File attribute flags
    flags: FileFlags,
    /// File metadata in plist format
    metadata: Vec<u8>,
}

impl File {
    /// Creates a new File entity with business logic applied
    ///
    /// Called from application layer or domain services.
    /// Future business rules (default flag setting, metadata validation, etc.) can be applied here.
    #[must_use]
    #[inline]
    pub const fn new(
        id: FileId,
        domain: Domain,
        relative_path: RelativePath,
        flags: FileFlags,
        metadata: Vec<u8>,
    ) -> Self {
        // Future business rules can be applied here
        // e.g., default flag setting, metadata validation, etc.
        Self {
            id,
            domain,
            relative_path,
            flags,
            metadata,
        }
    }

    /// Reconstructs a File entity from persisted data
    ///
    /// Called from repository implementations (infrastructure layer only).
    /// No business logic is applied since this is restoration from database.
    /// Data integrity is assumed to be already guaranteed.
    #[must_use]
    #[inline]
    pub const fn reconstruct(
        id: FileId,
        domain: Domain,
        relative_path: RelativePath,
        flags: FileFlags,
        metadata: Vec<u8>,
    ) -> Self {
        Self {
            id,
            domain,
            relative_path,
            flags,
            metadata,
        }
    }

    // Getters
    /// Returns the file ID
    #[must_use]
    #[inline]
    pub const fn id(&self) -> &FileId {
        &self.id
    }

    /// Returns the domain
    #[must_use]
    #[inline]
    pub const fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns the relative path
    #[must_use]
    #[inline]
    pub const fn relative_path(&self) -> &RelativePath {
        &self.relative_path
    }

    /// Returns the file flags
    #[must_use]
    #[inline]
    pub const fn flags(&self) -> &FileFlags {
        &self.flags
    }

    /// Returns the file metadata
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &[u8] {
        &self.metadata
    }

    // Business logic methods
    /// Updates the file flags
    #[inline]
    pub const fn update_flags(&mut self, new_flags: FileFlags) {
        self.flags = new_flags;
    }

    /// Updates the file metadata
    #[inline]
    pub fn update_metadata(&mut self, new_metadata: Vec<u8>) {
        self.metadata = new_metadata;
    }

    /// Checks if the file has a specific flag
    #[must_use]
    #[inline]
    pub const fn has_flag(&self, flag: FileFlags) -> bool {
        self.flags.contains(flag)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_file_entity_creation() -> Result<()> {
        let id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let file = File::new(
            id.clone(),
            domain.clone(),
            relative_path.clone(),
            flags.clone(),
            metadata.clone(),
        );

        assert_eq!(file.id(), &id);
        assert_eq!(file.domain(), &domain);
        assert_eq!(file.relative_path(), &relative_path);
        assert_eq!(file.flags(), &flags);
        assert_eq!(file.metadata(), &metadata);
        Ok(())
    }

    #[test]
    fn test_file_entity_reconstruct() -> Result<()> {
        let id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let file = File::reconstruct(
            id.clone(),
            domain.clone(),
            relative_path.clone(),
            flags.clone(),
            metadata.clone(),
        );

        assert_eq!(file.id(), &id);
        assert_eq!(file.domain(), &domain);
        assert_eq!(file.relative_path(), &relative_path);
        assert_eq!(file.flags(), &flags);
        assert_eq!(file.metadata(), &metadata);
        Ok(())
    }

    #[test]
    fn test_file_entity_update_flags() -> Result<()> {
        let id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let mut file = File::new(id, domain, relative_path, flags, metadata);

        let new_flags = FileFlags::DIRECTORY;
        file.update_flags(new_flags.clone());

        assert_eq!(file.flags(), &new_flags);
        assert!(file.has_flag(FileFlags::DIRECTORY));
        assert!(!file.has_flag(FileFlags::REGULAR_FILE));
        Ok(())
    }

    #[test]
    fn test_file_entity_update_metadata() -> Result<()> {
        let id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let mut file = File::new(id, domain, relative_path, flags, metadata);

        let new_metadata = b"updated metadata".to_vec();
        file.update_metadata(new_metadata.clone());

        assert_eq!(file.metadata(), &new_metadata);
        Ok(())
    }
}
