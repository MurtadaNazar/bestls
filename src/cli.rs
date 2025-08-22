use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Rust based LS command",
    long_about = r#"bestls is a Rust-powered file listing CLI tool.

Features:
- Outputs in table or JSON formats.
- Supports sorting by name, size, or modification date.
- Pretty-printed JSON output available.

Usage Examples:
  bestls -p ./src
  bestls --json --sort size
  bestls --json-pretty --sort date
"#
)]
pub struct Cli {
    #[arg(
        short = 'p',
        long = "path",
        value_name = "PATH",
        help = "Directory path to list files from. Defaults to current directory."
    )]
    pub path: Option<PathBuf>,

    #[arg(
        short = 'j',
        long = "json",
        help = "Output file list in compact JSON format.",
        default_value_t = false
    )]
    pub json: bool,

    #[arg(
        long = "json-pretty",
        help = "Output file list in pretty-printed JSON format.",
        default_value_t = false
    )]
    pub json_pretty: bool,

    #[arg(
        short = 's',
        long = "sort",
        value_enum,
        default_value = "name",
        help = "Sort files by the given attribute."
    )]
    pub sort_by: SortBy,

    #[arg(
        short = 'a',
        long = "all",
        help = "Include hidden files.",
        default_value_t = false
    )]
    pub all: bool,
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum SortBy {
    Name,
    Size,
    Date,
}
