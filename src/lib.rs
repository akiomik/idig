//! idig
//! ====
//!
//! A tool for extracting files from iPhone backups.

pub mod application;
pub mod cli;
pub mod domain;
pub mod infrastructure;

// Re-exports for convenience
pub use application::{
    DisplayService, ExtractError, ExtractResult, ExtractService, SearchParams, SearchService,
};
pub use cli::{Cli, Commands};
pub use domain::entities::{File, Metadata};
pub use domain::queries::{BasicQuery, CompositeQuery, FileQuery};
pub use domain::repositories::{FileRepository, MetadataRepository};
pub use domain::value_objects::{Domain, FileFlags, FileId, RelativePath};
pub use infrastructure::database::DatabaseConnection;
pub use infrastructure::repositories::FileRepositoryImpl;
