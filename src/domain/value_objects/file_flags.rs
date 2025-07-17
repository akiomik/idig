use std::fmt;

/// `FileFlags` - Value Object representing file attribute flags
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileFlags(i32);

impl FileFlags {
    /// Creates a new `FileFlags` instance
    #[must_use]
    #[inline]
    pub const fn new(flags: i32) -> Self {
        Self(flags)
    }

    /// Returns the integer value of the flags
    #[must_use]
    #[inline]
    pub const fn value(&self) -> i32 {
        self.0
    }

    /// Checks if a specific flag is set
    #[must_use]
    #[inline]
    pub const fn has_flag(&self, flag: i32) -> bool {
        (self.0 & flag) != 0
    }

    #[inline]
    pub fn set_flag(&mut self, flag: i32) {
        self.0 |= flag;
    }

    #[inline]
    pub fn unset_flag(&mut self, flag: i32) {
        self.0 &= !flag;
    }
}

impl fmt::Display for FileFlags {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<FileFlags> for i32 {
    #[inline]
    fn from(flags: FileFlags) -> Self {
        flags.0
    }
}

impl From<i32> for FileFlags {
    #[inline]
    fn from(flags: i32) -> Self {
        Self(flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_flags() {
        let mut flags = FileFlags::new(0x01);
        assert!(flags.has_flag(0x01));
        assert!(!flags.has_flag(0x02));

        flags.set_flag(0x02);
        assert!(flags.has_flag(0x02));

        flags.unset_flag(0x01);
        assert!(!flags.has_flag(0x01));
    }

    #[test]
    fn test_file_flags_multiple() {
        let mut flags = FileFlags::new(0x00);

        flags.set_flag(0x01);
        flags.set_flag(0x04);
        flags.set_flag(0x08);

        assert!(flags.has_flag(0x01));
        assert!(!flags.has_flag(0x02));
        assert!(flags.has_flag(0x04));
        assert!(flags.has_flag(0x08));

        assert_eq!(flags.value(), 0x01 | 0x04 | 0x08);
    }

    #[test]
    fn test_file_flags_from_i32() {
        let flags: FileFlags = 0x0F.into();
        assert_eq!(flags.value(), 0x0F);
        assert!(flags.has_flag(0x01));
        assert!(flags.has_flag(0x02));
        assert!(flags.has_flag(0x04));
        assert!(flags.has_flag(0x08));
    }
}
