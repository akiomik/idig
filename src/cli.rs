//! CLI configuration and command definitions

use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "idig")]
#[command(about = "A tool for extracting files from iPhone backups")]
#[command(version)]
#[non_exhaustive]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[allow(
    clippy::exhaustive_enums,
    reason = "Commands enum is intentionally exhaustive for CLI definition"
)]
pub enum Commands {
    /// List all available backups
    #[clap(visible_alias = "ls")]
    List {
        /// Root directory containing multiple backup folders
        #[arg(long, value_hint = ValueHint::DirPath, default_value="~/Library/Application Support/MobileSync/Backup")]
        backups_root: PathBuf,
    },

    /// Search for files based on various criteria
    Search {
        /// iPhone backup directory path (containing Manifest.db)
        #[arg(short = 'b', long, value_hint = ValueHint::DirPath)]
        backup_dir: PathBuf,

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
    /// Extract files based on search criteria
    Extract {
        /// iPhone backup directory path (containing Manifest.db)
        #[arg(short = 'b', long, value_hint = ValueHint::DirPath)]
        backup_dir: PathBuf,

        /// Output directory for extracted files
        #[arg(short, long, value_hint = ValueHint::DirPath)]
        output: String,

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
