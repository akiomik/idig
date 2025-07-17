//! CLI configuration and command definitions

use clap::{Parser, Subcommand, ValueHint};

#[derive(Parser)]
#[command(name = "idig")]
#[command(about = "A tool for extracting files from iPhone backups")]
#[command(version)]
#[non_exhaustive]
pub struct Cli {
    /// iPhone backup directory path (containing Manifest.db)
    #[arg(short = 'b', long, value_hint = ValueHint::DirPath)]
    pub backup_dir: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[allow(
    clippy::exhaustive_enums,
    reason = "Commands enum is intentionally exhaustive for CLI definition"
)]
pub enum Commands {
    /// Search for files based on various criteria
    Search {
        /// Exact domain match
        #[arg(long)]
        domain_exact: Option<String>,

        /// Partial domain match
        #[arg(long)]
        domain_contains: Option<String>,

        /// Exact path match
        #[arg(long)]
        path_exact: Option<String>,

        /// Partial path match
        #[arg(long)]
        path_contains: Option<String>,

        /// Use OR logic instead of AND (default is AND)
        #[arg(long)]
        or: bool,
    },
}
