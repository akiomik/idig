//! idig - A tool for extracting files from iPhone backups

use anyhow::Result;
use clap::Parser as _;
use idig::{
    Cli, Commands, DatabaseConnection, DisplayService, ExtractService, FileRepositoryImpl,
    SearchParams, SearchService,
};
use std::{path::Path, process::exit};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Database connection initialization
    let manifest_path = Path::new(&cli.backup_dir).join("Manifest.db");
    if !manifest_path.exists() {
        eprintln!(
            "Error: Manifest.db not found in backup directory: {}",
            cli.backup_dir
        );
        exit(1);
    }
    let db_url = format!("sqlite://{}", manifest_path.display());
    let db = DatabaseConnection::new(&db_url).await?;
    let file_repo = FileRepositoryImpl::new(db);
    let search_service = SearchService::new();
    let extract_service = ExtractService::new();
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
        Commands::Extract {
            output,
            domain_exact,
            domain_contains,
            path_exact,
            path_contains,
            or,
        } => {
            let params =
                SearchParams::new(domain_exact, domain_contains, path_exact, path_contains, or);

            match extract_service
                .extract(&file_repo, &cli.backup_dir, &output, params)
                .await
            {
                Ok(result) => {
                    println!("Extraction completed:");
                    println!("  Extracted: {} files", result.extracted_count);
                    println!("  Skipped: {} files", result.skipped_count);

                    if !result.errors.is_empty() {
                        println!("  Errors: {} files", result.errors.len());
                        for error in &result.errors {
                            eprintln!(
                                "    Error extracting {}: {}",
                                error.relative_path, error.error
                            );
                        }
                    }
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
