use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Shell};
use std::io;
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
- Shell completion generation support.

Usage Examples:
  bestls -p ./src
  bestls --json --sort size
  bestls --json-pretty --sort date
  bestls completion bash > ~/.local/share/bash-completion/completions/bestls
"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
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

#[derive(Debug, Parser)]
pub enum Commands {
    /// Generate shell completions
    Completion {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

impl Cli {
    /// Generate shell completions
    pub fn generate_completion(shell: Shell) {
        let mut cmd = Self::command();
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());
    }
}
