/// File query for searching files based on various criteria
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FileQuery {
    Basic(BasicQuery),
    Composite(CompositeQuery),
}

/// Basic query conditions for file search
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BasicQuery {
    DomainExact(String),
    DomainContains(String),
    PathExact(String),
    PathContains(String),
}

/// Composite query conditions for combining multiple basic queries
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompositeQuery {
    AnyOf(Vec<BasicQuery>),
    AllOf(Vec<BasicQuery>),
}

impl FileQuery {
    /// Create a query for exact domain match
    #[must_use]
    #[inline]
    pub fn domain_exact(domain: impl Into<String>) -> Self {
        Self::Basic(BasicQuery::DomainExact(domain.into()))
    }

    /// Create a query for partial domain match
    #[must_use]
    #[inline]
    pub fn domain_contains(domain: impl Into<String>) -> Self {
        Self::Basic(BasicQuery::DomainContains(domain.into()))
    }

    /// Create a query for exact path match
    #[must_use]
    #[inline]
    pub fn path_exact(path: impl Into<String>) -> Self {
        Self::Basic(BasicQuery::PathExact(path.into()))
    }

    /// Create a query for partial path match
    #[must_use]
    #[inline]
    pub fn path_contains(path: impl Into<String>) -> Self {
        Self::Basic(BasicQuery::PathContains(path.into()))
    }

    /// Create a query that matches any of the given basic queries
    #[must_use]
    #[inline]
    pub const fn any_of(queries: Vec<BasicQuery>) -> Self {
        Self::Composite(CompositeQuery::AnyOf(queries))
    }

    /// Create a query that matches all of the given basic queries
    #[must_use]
    #[inline]
    pub const fn all_of(queries: Vec<BasicQuery>) -> Self {
        Self::Composite(CompositeQuery::AllOf(queries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_exact_query() {
        let query = FileQuery::domain_exact("AppDomain-com.apple.news");
        assert_eq!(
            query,
            FileQuery::Basic(BasicQuery::DomainExact(
                "AppDomain-com.apple.news".to_string()
            ))
        );
    }

    #[test]
    fn test_domain_contains_query() {
        let query = FileQuery::domain_contains("apple");
        assert_eq!(
            query,
            FileQuery::Basic(BasicQuery::DomainContains("apple".to_string()))
        );
    }

    #[test]
    fn test_path_exact_query() {
        let query = FileQuery::path_exact("Documents/file.txt");
        assert_eq!(
            query,
            FileQuery::Basic(BasicQuery::PathExact("Documents/file.txt".to_string()))
        );
    }

    #[test]
    fn test_path_contains_query() {
        let query = FileQuery::path_contains("Documents");
        assert_eq!(
            query,
            FileQuery::Basic(BasicQuery::PathContains("Documents".to_string()))
        );
    }

    #[test]
    fn test_any_of_query() {
        let basic_queries = vec![
            BasicQuery::DomainExact("AppDomain-com.apple.news".to_string()),
            BasicQuery::PathContains("Documents".to_string()),
        ];
        let query = FileQuery::any_of(basic_queries.clone());
        assert_eq!(
            query,
            FileQuery::Composite(CompositeQuery::AnyOf(basic_queries))
        );
    }
}
