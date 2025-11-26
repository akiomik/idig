//! idig - A tool for extracting files from iPhone backups

use anyhow::Result;
use clap::Parser as _;
use idig::{
    Cli, Commands, DatabaseConnection, DisplayService, ExtractService, FileRepositoryImpl,
    ListService, MetadataRepositoryImpl, SearchParams, SearchService,
};
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let display_service = DisplayService::new();

    match cli.command {
        Commands::List { backups_root } => {
            // Create backup metadata repository and list service
            let backups_root_str = backups_root.to_string_lossy();
            let expanded_backups_root = shellexpand::tilde(&backups_root_str);
            let backups_path = PathBuf::from(expanded_backups_root.as_ref());
            let backup_repo = Arc::new(MetadataRepositoryImpl::new(backups_path));
            let backup_list_service = ListService::new(backup_repo);

            // List all backups
            let metadata_list = backup_list_service
                .list_backups()
                .await
                .map_err(|e| anyhow::anyhow!("Error listing backups: {e}"))?;

            display_service.display_metadata_list(&metadata_list);
        }
        Commands::Search {
            backup_dir,
            domain_exact,
            domain_contains,
            path_exact,
            path_contains,
            or,
        } => {
            // Database connection initialization
            let backup_dir_str = backup_dir.to_string_lossy();
            let expanded_backup_dir = shellexpand::tilde(&backup_dir_str);
            let backup_path = PathBuf::from(expanded_backup_dir.as_ref());
            let manifest_path = backup_path.join("Manifest.db");
            if !manifest_path.exists() {
                return Err(anyhow::anyhow!(
                    "Manifest.db not found in backup directory: {}",
                    backup_path.display()
                ));
            }

            let db_url = format!("sqlite://{}", manifest_path.display());
            let db = DatabaseConnection::new(&db_url).await?;
            let file_repo = FileRepositoryImpl::new(db);
            let search_service = SearchService::new();

            let params =
                SearchParams::new(domain_exact, domain_contains, path_exact, path_contains, or);

            let results = search_service.search(&file_repo, params).await?;
            display_service.display_search_results(results);
        }
        Commands::Extract {
            backup_dir,
            output,
            domain_exact,
            domain_contains,
            path_exact,
            path_contains,
            or,
        } => {
            // Database connection initialization
            let backup_dir_str = backup_dir.to_string_lossy();
            let expanded_backup_dir = shellexpand::tilde(&backup_dir_str);
            let backup_path = PathBuf::from(expanded_backup_dir.as_ref());
            let manifest_path = backup_path.join("Manifest.db");
            if !manifest_path.exists() {
                return Err(anyhow::anyhow!(
                    "Manifest.db not found in backup directory: {}",
                    backup_path.display()
                ));
            }

            let db_url = format!("sqlite://{}", manifest_path.display());
            let db = DatabaseConnection::new(&db_url).await?;
            let file_repo = FileRepositoryImpl::new(db);
            let extract_service = ExtractService::new();

            let params =
                SearchParams::new(domain_exact, domain_contains, path_exact, path_contains, or);

            let result = extract_service
                .extract(&file_repo, &backup_path.to_string_lossy(), &output, params)
                .await?;

            display_service.display_extract_results(&result);
        }
    }

    Ok(())
}
