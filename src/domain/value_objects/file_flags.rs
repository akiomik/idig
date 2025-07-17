use bitflags::bitflags;

bitflags! {
    /// `FileFlags` - Value Object representing file attribute flags
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct FileFlags: i32 {
        // Basic file type flags
        const REGULAR_FILE = 1;
        const DIRECTORY = 2;
        const SYMBOLIC_LINK = 4;

        // Additional attribute flags
        const HIDDEN = 8;
        const SYSTEM = 16;
        const ARCHIVE = 32;
        const READ_ONLY = 64;

        // Special flags
        const COMPRESSED = 128;
        const ENCRYPTED = 256;
        const SPARSE = 512;
    }
}

impl FileFlags {
    // Convenience methods for checking specific file types
    /// Checks if this is a regular file
    #[must_use]
    #[inline]
    pub const fn is_regular_file(&self) -> bool {
        self.contains(Self::REGULAR_FILE)
    }

    /// Checks if this is a directory
    #[must_use]
    #[inline]
    pub const fn is_directory(&self) -> bool {
        self.contains(Self::DIRECTORY)
    }

    /// Checks if this is a symbolic link
    #[must_use]
    #[inline]
    pub const fn is_symbolic_link(&self) -> bool {
        self.contains(Self::SYMBOLIC_LINK)
    }

    /// Checks if this is a hidden file
    #[must_use]
    #[inline]
    pub const fn is_hidden(&self) -> bool {
        self.contains(Self::HIDDEN)
    }

    /// Checks if this is a system file
    #[must_use]
    #[inline]
    pub const fn is_system(&self) -> bool {
        self.contains(Self::SYSTEM)
    }

    /// Checks if this is an archive file
    #[must_use]
    #[inline]
    pub const fn is_archive(&self) -> bool {
        self.contains(Self::ARCHIVE)
    }

    /// Checks if this is a read-only file
    #[must_use]
    #[inline]
    pub const fn is_read_only(&self) -> bool {
        self.contains(Self::READ_ONLY)
    }

    /// Checks if this is a compressed file
    #[must_use]
    #[inline]
    pub const fn is_compressed(&self) -> bool {
        self.contains(Self::COMPRESSED)
    }

    /// Checks if this is an encrypted file
    #[must_use]
    #[inline]
    pub const fn is_encrypted(&self) -> bool {
        self.contains(Self::ENCRYPTED)
    }

    /// Checks if this is a sparse file
    #[must_use]
    #[inline]
    pub const fn is_sparse(&self) -> bool {
        self.contains(Self::SPARSE)
    }
}

impl From<i32> for FileFlags {
    #[inline]
    fn from(flags: i32) -> Self {
        Self::from_bits_truncate(flags)
    }
}

impl From<FileFlags> for i32 {
    #[inline]
    fn from(flags: FileFlags) -> Self {
        flags.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_flags() {
        let mut flags = FileFlags::REGULAR_FILE;
        assert!(flags.contains(FileFlags::REGULAR_FILE));
        assert!(!flags.contains(FileFlags::DIRECTORY));

        flags.insert(FileFlags::DIRECTORY);
        assert!(flags.contains(FileFlags::DIRECTORY));

        flags.remove(FileFlags::REGULAR_FILE);
        assert!(!flags.contains(FileFlags::REGULAR_FILE));
    }

    #[test]
    fn test_file_flags_multiple() {
        let mut flags = FileFlags::empty();

        flags.insert(FileFlags::REGULAR_FILE);
        flags.insert(FileFlags::SYMBOLIC_LINK);
        flags.insert(FileFlags::HIDDEN);

        assert!(flags.contains(FileFlags::REGULAR_FILE));
        assert!(!flags.contains(FileFlags::DIRECTORY));
        assert!(flags.contains(FileFlags::SYMBOLIC_LINK));
        assert!(flags.contains(FileFlags::HIDDEN));

        assert_eq!(flags.bits(), 1 | 4 | 8);
    }

    #[test]
    fn test_file_flags_from_i32() {
        let flags: FileFlags = 0x0F.into();
        assert_eq!(flags.bits(), 0x0F);
        assert!(flags.contains(FileFlags::REGULAR_FILE));
        assert!(flags.contains(FileFlags::DIRECTORY));
        assert!(flags.contains(FileFlags::SYMBOLIC_LINK));
        assert!(flags.contains(FileFlags::HIDDEN));
    }

    #[test]
    fn test_file_type_constants() {
        assert_eq!(FileFlags::REGULAR_FILE.bits(), 0b00_0000_0001);
        assert_eq!(FileFlags::DIRECTORY.bits(), 0b00_0000_0010);
        assert_eq!(FileFlags::SYMBOLIC_LINK.bits(), 0b00_0000_0100);
        assert_eq!(FileFlags::HIDDEN.bits(), 0b00_0000_1000);
        assert_eq!(FileFlags::SYSTEM.bits(), 0b00_0001_0000);
        assert_eq!(FileFlags::ARCHIVE.bits(), 0b00_0010_0000);
        assert_eq!(FileFlags::READ_ONLY.bits(), 0b00_0100_0000);
        assert_eq!(FileFlags::COMPRESSED.bits(), 0b00_1000_0000);
        assert_eq!(FileFlags::ENCRYPTED.bits(), 0b01_0000_0000);
        assert_eq!(FileFlags::SPARSE.bits(), 0b10_0000_0000);
    }

    #[test]
    fn test_file_type_checks() {
        let regular_file = FileFlags::REGULAR_FILE;
        assert!(regular_file.is_regular_file());
        assert!(!regular_file.is_directory());
        assert!(!regular_file.is_symbolic_link());

        let directory = FileFlags::DIRECTORY;
        assert!(!directory.is_regular_file());
        assert!(directory.is_directory());
        assert!(!directory.is_symbolic_link());

        let symlink = FileFlags::SYMBOLIC_LINK;
        assert!(!symlink.is_regular_file());
        assert!(!symlink.is_directory());
        assert!(symlink.is_symbolic_link());
    }

    #[test]
    fn test_attribute_checks() {
        let mut flags = FileFlags::REGULAR_FILE;

        // Test individual attributes
        flags.insert(FileFlags::HIDDEN);
        assert!(flags.is_hidden());
        assert!(!flags.is_system());

        flags.insert(FileFlags::READ_ONLY);
        assert!(flags.is_read_only());
        assert!(flags.is_hidden());

        flags.insert(FileFlags::COMPRESSED);
        assert!(flags.is_compressed());

        flags.insert(FileFlags::ENCRYPTED);
        assert!(flags.is_encrypted());

        flags.insert(FileFlags::SPARSE);
        assert!(flags.is_sparse());
    }

    #[test]
    fn test_bitflags_operations() {
        let combined = FileFlags::DIRECTORY | FileFlags::HIDDEN | FileFlags::READ_ONLY;

        assert!(combined.is_directory());
        assert!(combined.is_hidden());
        assert!(combined.is_read_only());
        assert!(!combined.is_regular_file());
        assert!(!combined.is_compressed());

        assert_eq!(combined.bits(), 2 | 8 | 64); // 74
    }

    #[test]
    fn test_bitflags_intersection() {
        let flags1 = FileFlags::DIRECTORY | FileFlags::HIDDEN;
        let flags2 = FileFlags::HIDDEN | FileFlags::READ_ONLY;

        let intersection = flags1 & flags2;
        assert!(intersection.is_hidden());
        assert!(!intersection.is_directory());
        assert!(!intersection.is_read_only());
    }

    #[test]
    fn test_bitflags_difference() {
        let mut flags = FileFlags::DIRECTORY | FileFlags::HIDDEN | FileFlags::READ_ONLY;
        flags.remove(FileFlags::HIDDEN);

        assert!(flags.is_directory());
        assert!(!flags.is_hidden());
        assert!(flags.is_read_only());
    }

    #[test]
    fn test_empty() {
        let empty = FileFlags::empty();
        assert_eq!(empty.bits(), 0);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_all() {
        let all = FileFlags::all();
        assert!(all.is_regular_file());
        assert!(all.is_directory());
        assert!(all.is_symbolic_link());
        assert!(all.is_hidden());
        assert!(all.is_system());
        assert!(all.is_archive());
        assert!(all.is_read_only());
        assert!(all.is_compressed());
        assert!(all.is_encrypted());
        assert!(all.is_sparse());
    }
}
