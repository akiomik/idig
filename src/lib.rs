//! idig
//! ====
//!
//! A tool for extracting files from iPhone backups.

pub mod domain;

// Re-exports for convenience
pub use domain::entities::File;
pub use domain::queries::FileQuery;
pub use domain::repositories::FileRepository;
pub use domain::value_objects::{Domain, FileFlags, FileId, RelativePath};
