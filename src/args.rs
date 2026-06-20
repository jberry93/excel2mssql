//! Define the arguments for the CLI

use clap::Parser;

/// Arguments for the CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// SQL Server connection string
    #[arg(short, long)]
    pub connection_string: String,

    /// Table name from SQL Server database
    #[arg(short, long)]
    pub table: String,

    /// Path to Excel file we want to use
    #[arg(short, long)]
    pub excel: String,

    /// Determine which sheet we wish to ingest
    #[arg(short, long)]
    pub sheet: Option<String>,

    /// Run without insertions
    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,
}
