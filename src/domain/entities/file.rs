use crate::domain::value_objects::{Domain, FileFlags, FileId, RelativePath};

/// File Entity - Represents a file in the domain model
///
/// This entity encapsulates all the information about a file including its unique identifier,
/// domain context, path information, flags, and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// Unique file identifier (SHA1 hash)
    file_id: FileId,
    /// Application identifier
    domain: Domain,
    /// Relative path within a backup
    relative_path: RelativePath,
    /// File attribute flags
    flags: FileFlags,
    /// File metadata in plist format
    file_metadata: Vec<u8>,
}

impl File {
    /// Creates a new File entity with business logic applied
    ///
    /// Called from application layer or domain services.
    /// Future business rules (default flag setting, metadata validation, etc.) can be applied here.
    #[must_use]
    #[inline]
    pub const fn new(
        file_id: FileId,
        domain: Domain,
        relative_path: RelativePath,
        flags: FileFlags,
        file_metadata: Vec<u8>,
    ) -> Self {
        // Future business rules can be applied here
        // e.g., default flag setting, metadata validation, etc.
        Self {
            file_id,
            domain,
            relative_path,
            flags,
            file_metadata,
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
        file_id: FileId,
        domain: Domain,
        relative_path: RelativePath,
        flags: FileFlags,
        file_metadata: Vec<u8>,
    ) -> Self {
        Self {
            file_id,
            domain,
            relative_path,
            flags,
            file_metadata,
        }
    }

    // Getters
    /// Returns the file ID
    #[must_use]
    #[inline]
    pub const fn file_id(&self) -> &FileId {
        &self.file_id
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
    pub fn file_metadata(&self) -> &[u8] {
        &self.file_metadata
    }

    // Business logic methods
    /// Updates the file flags
    #[inline]
    pub fn update_flags(&mut self, new_flags: FileFlags) {
        self.flags = new_flags;
    }

    /// Updates the file metadata
    #[inline]
    pub fn update_metadata(&mut self, new_metadata: Vec<u8>) {
        self.file_metadata = new_metadata;
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
        let file_id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let file = File::new(
            file_id.clone(),
            domain.clone(),
            relative_path.clone(),
            flags.clone(),
            metadata.clone(),
        );

        assert_eq!(file.file_id(), &file_id);
        assert_eq!(file.domain(), &domain);
        assert_eq!(file.relative_path(), &relative_path);
        assert_eq!(file.flags(), &flags);
        assert_eq!(file.file_metadata(), &metadata);
        Ok(())
    }

    #[test]
    fn test_file_entity_reconstruct() -> Result<()> {
        let file_id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let file = File::reconstruct(
            file_id.clone(),
            domain.clone(),
            relative_path.clone(),
            flags.clone(),
            metadata.clone(),
        );

        assert_eq!(file.file_id(), &file_id);
        assert_eq!(file.domain(), &domain);
        assert_eq!(file.relative_path(), &relative_path);
        assert_eq!(file.flags(), &flags);
        assert_eq!(file.file_metadata(), &metadata);
        Ok(())
    }

    #[test]
    fn test_file_entity_update_flags() -> Result<()> {
        let file_id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let mut file = File::new(file_id, domain, relative_path, flags, metadata);

        let new_flags = FileFlags::DIRECTORY;
        file.update_flags(new_flags.clone());

        assert_eq!(file.flags(), &new_flags);
        assert!(file.has_flag(FileFlags::DIRECTORY));
        assert!(!file.has_flag(FileFlags::REGULAR_FILE));
        Ok(())
    }

    #[test]
    fn test_file_entity_update_metadata() -> Result<()> {
        let file_id = FileId::new("a1b2c3d4e5f6789012345678901234567890abcd")?;
        let domain = Domain::new("AppDomain-com.apple.news".to_owned())?;
        let relative_path = RelativePath::new("Documents/test.txt".to_owned())?;
        let flags = FileFlags::REGULAR_FILE;
        let metadata = b"test metadata".to_vec();

        let mut file = File::new(file_id, domain, relative_path, flags, metadata);

        let new_metadata = b"updated metadata".to_vec();
        file.update_metadata(new_metadata.clone());

        assert_eq!(file.file_metadata(), &new_metadata);
        Ok(())
    }
}
