//! Search service for handling file search operations

use crate::{BasicQuery, File, FileQuery, FileRepository};
use anyhow::Result;

/// Service for handling file search operations
#[non_exhaustive]
pub struct SearchService;

/// Search parameters for file queries
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SearchParams {
    pub domain_exact: Option<String>,
    pub domain_contains: Option<String>,
    pub path_exact: Option<String>,
    pub path_contains: Option<String>,
    pub use_or: bool,
}

impl SearchParams {
    /// Create new search parameters
    #[must_use]
    #[inline]
    pub const fn new(
        domain_exact: Option<String>,
        domain_contains: Option<String>,
        path_exact: Option<String>,
        path_contains: Option<String>,
        use_or: bool,
    ) -> Self {
        Self {
            domain_exact,
            domain_contains,
            path_exact,
            path_contains,
            use_or,
        }
    }
}

impl SearchService {
    /// Create a new search service instance
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Execute a search with the given parameters
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No search conditions are provided
    /// - The repository operation fails
    #[allow(
        clippy::future_not_send,
        reason = "Repository trait doesn't guarantee Send futures"
    )]
    #[inline]
    pub async fn search<R: FileRepository>(
        &self,
        file_repo: &R,
        params: SearchParams,
    ) -> Result<Vec<File>> {
        let query = Self::build_query(params)?;
        file_repo.search(query).await
    }

    /// Build a `FileQuery` from search parameters
    ///
    /// # Errors
    ///
    /// Returns an error if no search conditions are provided
    fn build_query(params: SearchParams) -> Result<FileQuery> {
        let mut conditions = Vec::new();

        if let Some(domain) = params.domain_exact {
            conditions.push(BasicQuery::DomainExact(domain));
        }

        if let Some(domain) = params.domain_contains {
            conditions.push(BasicQuery::DomainContains(domain));
        }

        if let Some(path) = params.path_exact {
            conditions.push(BasicQuery::PathExact(path));
        }

        if let Some(path) = params.path_contains {
            conditions.push(BasicQuery::PathContains(path));
        }

        if conditions.is_empty() {
            return Err(anyhow::anyhow!(
                "At least one search condition must be specified"
            ));
        }

        // Build query based on logic type
        let query = if conditions.len() == 1 {
            // Single condition - use Basic query
            // We know conditions has exactly one element at this point
            if let Some(condition) = conditions.into_iter().next() {
                FileQuery::Basic(condition)
            } else {
                return Err(anyhow::anyhow!(
                    "Internal error: expected exactly one condition"
                ));
            }
        } else if params.use_or {
            // Multiple conditions with OR logic
            FileQuery::any_of(conditions)
        } else {
            // Multiple conditions with AND logic (default)
            FileQuery::all_of(conditions)
        };

        Ok(query)
    }
}

impl Default for SearchService {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_query_single_condition() {
        let params = SearchParams::new(Some("com.apple.test".to_string()), None, None, None, false);

        let result = SearchService::build_query(params);
        assert!(result.is_ok());

        if let Ok(FileQuery::Basic(BasicQuery::DomainExact(domain))) = result {
            assert_eq!(domain, "com.apple.test");
        } else {
            panic!("Expected Basic query with DomainExact");
        }
    }

    #[test]
    fn test_build_query_multiple_conditions_and() {
        let params = SearchParams::new(
            Some("com.apple.test".to_string()),
            None,
            None,
            Some("Documents".to_string()),
            false,
        );

        let result = SearchService::build_query(params);
        assert!(result.is_ok());

        if let Ok(FileQuery::Composite(_)) = result {
            // This is expected for multiple conditions
        } else {
            panic!("Expected Composite query for multiple conditions");
        }
    }

    #[test]
    fn test_build_query_no_conditions() {
        let params = SearchParams::new(None, None, None, None, false);

        let result = SearchService::build_query(params);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("At least one search condition"));
    }

    #[test]
    fn test_build_query_multiple_conditions_or() {
        let params = SearchParams::new(
            Some("com.apple.test".to_string()),
            None,
            None,
            Some("Documents".to_string()),
            true,
        );

        let result = SearchService::build_query(params);
        assert!(result.is_ok());

        if let Ok(FileQuery::Composite(_)) = result {
            // This is expected for multiple conditions
        } else {
            panic!("Expected Composite query for multiple conditions");
        }
    }
}
