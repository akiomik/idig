use anyhow::Result;
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _, QueryOrder as _};

use crate::domain::entities::File;
use crate::domain::queries::{BasicQuery, CompositeQuery, FileQuery};
use crate::domain::repositories::FileRepository;
use crate::infrastructure::database::{
    DatabaseConnection,
    entities::files::{Column, Entity},
};

/// Implementation of `FileRepository` using `SeaORM`
pub struct FileRepositoryImpl {
    /// Database connection
    db: DatabaseConnection,
}

impl FileRepositoryImpl {
    /// Creates a new `FileRepositoryImpl`
    #[must_use]
    #[inline]
    pub const fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn apply_basic_query(
        query: sea_orm::Select<Entity>,
        basic_query: BasicQuery,
    ) -> sea_orm::Select<Entity> {
        match basic_query {
            BasicQuery::DomainExact(domain) => query.filter(Column::Domain.eq(domain)),
            BasicQuery::DomainContains(domain) => query.filter(Column::Domain.contains(&domain)),
            BasicQuery::PathExact(path) => query.filter(Column::RelativePath.eq(path)),
            BasicQuery::PathContains(path) => query.filter(Column::RelativePath.contains(&path)),
        }
    }

    fn apply_composite_query(
        query: sea_orm::Select<Entity>,
        composite_query: CompositeQuery,
    ) -> sea_orm::Select<Entity> {
        match composite_query {
            CompositeQuery::AnyOf(basic_queries) => {
                if basic_queries.is_empty() {
                    return query;
                }

                let mut condition = None;
                for basic_query in basic_queries {
                    let basic_condition = match basic_query {
                        BasicQuery::DomainExact(domain) => Column::Domain.eq(domain),
                        BasicQuery::DomainContains(domain) => Column::Domain.contains(&domain),
                        BasicQuery::PathExact(path) => Column::RelativePath.eq(path),
                        BasicQuery::PathContains(path) => Column::RelativePath.contains(&path),
                    };

                    condition = match condition {
                        None => Some(basic_condition),
                        Some(existing) => Some(existing.or(basic_condition)),
                    };
                }

                if let Some(final_condition) = condition {
                    query.filter(final_condition)
                } else {
                    query
                }
            }
            CompositeQuery::AllOf(basic_queries) => {
                if basic_queries.is_empty() {
                    return query;
                }

                let mut result_query = query;
                for basic_query in basic_queries {
                    let basic_condition = match basic_query {
                        BasicQuery::DomainExact(domain) => Column::Domain.eq(domain),
                        BasicQuery::DomainContains(domain) => Column::Domain.contains(&domain),
                        BasicQuery::PathExact(path) => Column::RelativePath.eq(path),
                        BasicQuery::PathContains(path) => Column::RelativePath.contains(&path),
                    };
                    result_query = result_query.filter(basic_condition);
                }
                result_query
            }
        }
    }
}

impl FileRepository for FileRepositoryImpl {
    #[inline]
    async fn search(&self, query: FileQuery) -> Result<Vec<File>> {
        let mut db_query = Entity::find();

        // Apply query conditions
        db_query = match query {
            FileQuery::Basic(basic_query) => Self::apply_basic_query(db_query, basic_query),
            FileQuery::Composite(composite_query) => {
                Self::apply_composite_query(db_query, composite_query)
            }
        };

        // Add sorting by domain and relative path
        db_query = db_query
            .order_by_asc(Column::Domain)
            .order_by_asc(Column::RelativePath);

        // Execute query and convert to domain entities
        let models = db_query.all(self.db.get_connection()).await?;
        let mut files = Vec::with_capacity(models.len());

        for model in models {
            files.push(model.to_domain()?);
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::queries::BasicQuery;
    use crate::infrastructure::database::entities::files::ActiveModel;
    use anyhow::Context as _;
    use sea_orm::{ActiveModelTrait as _, ConnectionTrait as _, Database, Set};

    async fn setup_test_db() -> Result<DatabaseConnection> {
        // Use in-memory SQLite database for testing
        let db = Database::connect("sqlite::memory:").await?;

        // Create table
        let schema = sea_orm::Schema::new(sea_orm::DatabaseBackend::Sqlite);
        let stmt = schema.create_table_from_entity(Entity);
        db.execute(db.get_database_backend().build(&stmt)).await?;

        Ok(DatabaseConnection::new_from_connection(db))
    }

    async fn insert_test_data(db: &DatabaseConnection) -> Result<()> {
        // Insert test data (using valid 40-character SHA1 hashes)
        let test_files = vec![
            ActiveModel {
                file_id: Set("356a192b7913b04c54574d18c28d46e6395428ab".to_owned()),
                domain: Set("com.apple.news".to_owned()),
                relative_path: Set("Documents/news.txt".to_owned()),
                flags: Set(1),
                file: Set(b"news content".to_vec()),
            },
            ActiveModel {
                file_id: Set("da4b9237bacccdf19c0760cab7aec4a8359010b0".to_owned()),
                domain: Set("com.apple.photos".to_owned()),
                relative_path: Set("Pictures/photo.jpg".to_owned()),
                flags: Set(2),
                file: Set(b"photo content".to_vec()),
            },
            ActiveModel {
                file_id: Set("77de68daecd823babbb58edb1c8e14d7106e83bb".to_owned()),
                domain: Set("com.example.app".to_owned()),
                relative_path: Set("Documents/example.txt".to_owned()),
                flags: Set(3),
                file: Set(b"example content".to_vec()),
            },
        ];

        for file in test_files {
            file.insert(db.get_connection()).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_search_domain_exact() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::domain_exact("com.apple.news");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].domain().value(), "com.apple.news");
        assert_eq!(results[0].relative_path().value(), "Documents/news.txt");

        Ok(())
    }

    #[tokio::test]
    async fn test_search_domain_contains() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::domain_contains("apple");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 2);
        // Results should contain both apple.news and apple.photos
        let domains: Vec<&str> = results.iter().map(|f| f.domain().value()).collect();
        assert!(domains.contains(&"com.apple.news"));
        assert!(domains.contains(&"com.apple.photos"));

        Ok(())
    }

    #[tokio::test]
    async fn test_search_path_exact() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::path_exact("Documents/news.txt");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].relative_path().value(), "Documents/news.txt");
        assert_eq!(results[0].domain().value(), "com.apple.news");

        Ok(())
    }

    #[tokio::test]
    async fn test_search_path_contains() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::path_contains("Documents");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 2);
        // Should find both files in Documents folder
        let paths: Vec<&str> = results.iter().map(|f| f.relative_path().value()).collect();
        assert!(paths.contains(&"Documents/news.txt"));
        assert!(paths.contains(&"Documents/example.txt"));

        Ok(())
    }

    #[tokio::test]
    async fn test_search_any_of() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::any_of(vec![
            BasicQuery::DomainExact("com.apple.news".to_owned()),
            BasicQuery::PathContains("Pictures".to_owned()),
        ]);
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 2);
        // Should find news file (by domain) and photo file (by path)
        let file_ids: Vec<&str> = results.iter().map(|f| f.id().value()).collect();
        assert!(file_ids.contains(&"356a192b7913b04c54574d18c28d46e6395428ab"));
        assert!(file_ids.contains(&"da4b9237bacccdf19c0760cab7aec4a8359010b0"));

        Ok(())
    }

    #[tokio::test]
    async fn test_search_no_results() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::domain_exact("com.nonexistent.app");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_empty_any_of() -> Result<()> {
        let db = setup_test_db().await?;
        insert_test_data(&db).await?;
        let repo = FileRepositoryImpl::new(db);

        let query = FileQuery::any_of(vec![]);
        let results = repo.search(query).await?;

        // Empty AnyOf should return all records
        assert_eq!(results.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_search_with_empty_relative_paths() -> Result<()> {
        let db = setup_test_db().await?;

        // Insert test data including records with empty relative paths
        let test_files = vec![
            ActiveModel {
                file_id: Set("356a192b7913b04c54574d18c28d46e6395428ab".to_owned()),
                domain: Set("com.apple.news".to_owned()),
                relative_path: Set("Documents/news.txt".to_owned()),
                flags: Set(1),
                file: Set(b"news content".to_vec()),
            },
            // File with empty relative path (common in iPhone backups)
            ActiveModel {
                file_id: Set("da4b9237bacccdf19c0760cab7aec4a8359010b0".to_owned()),
                domain: Set("AppDomain-com.apple.photos".to_owned()),
                relative_path: Set(String::new()), // Empty path
                flags: Set(2),
                file: Set(b"photo content".to_vec()),
            },
        ];

        for file in test_files {
            file.insert(db.get_connection()).await?;
        }

        let repo = FileRepositoryImpl::new(db);

        // Search for domains containing "apple" - should include file with empty path
        let query = FileQuery::domain_contains("apple");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 2);

        // Find the file that originally had empty path
        let empty_path_file = results
            .iter()
            .find(|f| f.domain().value() == "AppDomain-com.apple.photos")
            .context("Should find the file with originally empty path")?;

        // Verify that empty path remains empty
        assert_eq!(empty_path_file.relative_path().value(), "");

        Ok(())
    }

    #[tokio::test]
    async fn test_search_exact_domain_with_empty_path() -> Result<()> {
        let db = setup_test_db().await?;

        // Insert file with empty relative path
        let test_file = ActiveModel {
            file_id: Set("da4b9237bacccdf19c0760cab7aec4a8359010b0".to_owned()),
            domain: Set("AppDomain-com.apple.photos".to_owned()),
            relative_path: Set(String::new()), // Empty path
            flags: Set(2),
            file: Set(b"photo content".to_vec()),
        };
        test_file.insert(db.get_connection()).await?;

        let repo = FileRepositoryImpl::new(db);

        // Search for exact domain that has empty path
        let query = FileQuery::domain_exact("AppDomain-com.apple.photos");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].domain().value(), "AppDomain-com.apple.photos");
        assert_eq!(results[0].relative_path().value(), ""); // Empty path remains empty

        Ok(())
    }

    #[tokio::test]
    async fn test_search_path_exact_empty() -> Result<()> {
        let db = setup_test_db().await?;

        // Insert file with empty relative path
        let test_file = ActiveModel {
            file_id: Set("da4b9237bacccdf19c0760cab7aec4a8359010b0".to_owned()),
            domain: Set("AppDomain-com.apple.photos".to_owned()),
            relative_path: Set(String::new()), // Empty path
            flags: Set(2),
            file: Set(b"photo content".to_vec()),
        };
        test_file.insert(db.get_connection()).await?;

        let repo = FileRepositoryImpl::new(db);

        // Search for exact empty path
        let query = FileQuery::path_exact("");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].domain().value(), "AppDomain-com.apple.photos");
        assert_eq!(results[0].relative_path().value(), ""); // Empty path

        Ok(())
    }

    #[tokio::test]
    async fn test_search_sorting_order() -> Result<()> {
        let db = setup_test_db().await?;

        // Insert test data with different domains and paths to test sorting
        let test_files = vec![
            ActiveModel {
                file_id: Set("356a192b7913b04c54574d18c28d46e6395428ab".to_owned()),
                domain: Set("com.example.app".to_owned()),
                relative_path: Set("Documents/file2.txt".to_owned()),
                flags: Set(1),
                file: Set(b"content".to_vec()),
            },
            ActiveModel {
                file_id: Set("da4b9237bacccdf19c0760cab7aec4a8359010b0".to_owned()),
                domain: Set("com.apple.photos".to_owned()),
                relative_path: Set("Pictures/photo.jpg".to_owned()),
                flags: Set(2),
                file: Set(b"content".to_vec()),
            },
            ActiveModel {
                file_id: Set("77de68daecd823babbb58edb1c8e14d7106e83bb".to_owned()),
                domain: Set("com.apple.photos".to_owned()),
                relative_path: Set("Documents/file1.txt".to_owned()),
                flags: Set(3),
                file: Set(b"content".to_vec()),
            },
            ActiveModel {
                file_id: Set("629e88b8f2b2f0c8b6f8c8f2b2f0c8b6f8c8f2b2".to_owned()),
                domain: Set("com.apple.news".to_owned()),
                relative_path: Set(String::new()), // Empty path should come first
                flags: Set(4),
                file: Set(b"content".to_vec()),
            },
        ];

        for file in test_files {
            file.insert(db.get_connection()).await?;
        }

        let repo = FileRepositoryImpl::new(db);

        // Search for all files containing "com" to get multiple results
        let query = FileQuery::domain_contains("com");
        let results = repo.search(query).await?;

        assert_eq!(results.len(), 4);

        // Verify sorting: first by domain (ascending), then by relative path (ascending)
        // Expected order:
        // 1. com.apple.news, "" (empty path)
        // 2. com.apple.photos, "Documents/file1.txt"
        // 3. com.apple.photos, "Pictures/photo.jpg"
        // 4. com.example.app, "Documents/file2.txt"

        assert_eq!(results[0].domain().value(), "com.apple.news");
        assert_eq!(results[0].relative_path().value(), "");

        assert_eq!(results[1].domain().value(), "com.apple.photos");
        assert_eq!(results[1].relative_path().value(), "Documents/file1.txt");

        assert_eq!(results[2].domain().value(), "com.apple.photos");
        assert_eq!(results[2].relative_path().value(), "Pictures/photo.jpg");

        assert_eq!(results[3].domain().value(), "com.example.app");
        assert_eq!(results[3].relative_path().value(), "Documents/file2.txt");

        Ok(())
    }
}
