//! idig - A tool for extracting files from iPhone backups

use anyhow::Result;
use clap::Parser as _;
use idig::{
    Cli, Commands, DatabaseConnection, DisplayService, FileRepositoryImpl, SearchParams,
    SearchService,
};
use std::process::exit;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Database connection initialization
    let db_url = format!("sqlite://{}", cli.database);
    let db = DatabaseConnection::new(&db_url).await?;
    let file_repo = FileRepositoryImpl::new(db);
    let search_service = SearchService::new();
    let display_service = DisplayService::new();

    match cli.command {
        Commands::Search {
            domain_exact,
            domain_contains,
            path_exact,
            path_contains,
            or,
        } => {
            let params =
                SearchParams::new(domain_exact, domain_contains, path_exact, path_contains, or);

            match search_service.search(&file_repo, params).await {
                Ok(results) => {
                    display_service.display_search_results(results);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    exit(1);
                }
            }
        }
    }

    Ok(())
}
