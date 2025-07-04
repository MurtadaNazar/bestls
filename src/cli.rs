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
pub struct CLI {
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
        help = "Sort files by the given attribute.",
        value_parser = ["name", "size", "date"]
    )]
    pub sort_by: SortBy,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortBy {
    Name,
    Size,
    Date,
}
