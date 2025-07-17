use crate::domain::entities::File;
use crate::domain::queries::FileQuery;
use anyhow::Result;

/// `FileRepository` trait - Interface for file repository operations
#[allow(
    async_fn_in_trait,
    reason = "Using native async fn in trait for better ergonomics"
)]
pub trait FileRepository: Send + Sync {
    /// Search files
    async fn search(&self, query: FileQuery) -> Result<Vec<File>>;
}
