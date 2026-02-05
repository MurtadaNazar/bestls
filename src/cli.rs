//! # Command Line Interface Module
//!
//! This module defines the command-line interface for **bestls** using the `clap` crate.
//! It provides a comprehensive CLI with support for various output formats, sorting options,
//! and shell completion generation.
//!
//! ## Key Components
//!
//! - [`Cli`]: Main command-line interface structure
//! - [`SortBy`]: Enumeration for sorting options
//! - [`Commands`][]: Subcommands (currently just completion generation)
//!
//! ## Features
//!
//! * **Flexible Output**: Table format (default), compact JSON, or pretty-printed JSON
//! * **Sorting Options**: Sort by file name, size, or modification date
//! * **Hidden Files**: Option to include or exclude hidden files (starting with '.')
//! * **Shell Completions**: Generate completions for bash, zsh, and fish shells
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```bash
//! # List current directory (default behavior)
//! bestls
//!
//! # List specific directory
//! bestls -p /home/user/documents
//!
//! # Include hidden files
//! bestls -a
//!
//! # Sort by file size
//! bestls --sort size
//!
//! # Output as pretty JSON
//! bestls --json-pretty
//! ```
//!
//! ### Shell Completion Generation
//!
//! ```bash
//! # Generate bash completions
//! bestls completion bash > ~/.local/share/bash-completion/completions/bestls
//!
//! # Generate zsh completions
//! bestls completion zsh > ~/.zfunc/_bestls
//!
//! # Generate fish completions
//! bestls completion fish > ~/.config/fish/completions/bestls.fish
//! ```

use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Shell};
use std::io;
use std::path::PathBuf;

/// Main command-line interface structure for bestls.
///
/// This struct defines all available command-line arguments and options using `clap`'s derive API.
/// It provides a comprehensive interface for file listing with various output formats and sorting options.
///
/// # Arguments
///
/// * `command` - Optional subcommand (currently only completion generation)
/// * `path` - Directory path to list (defaults to current directory)
/// * `json` - Output in compact JSON format
/// * `json_pretty` - Output in pretty-printed JSON format
/// * `sort_by` - Sort files by name, size, or modification date
/// * `all` - Include hidden files in the listing
///
/// # Examples
///
/// The CLI structure automatically handles argument parsing and validation:
///
/// ```rust
/// use clap::Parser;
/// use bestls::cli::Cli;
///
/// // Parse command-line arguments
/// let cli = Cli::parse();
///
/// // Access parsed values
/// let include_hidden = cli.all;
/// let path = cli.path.unwrap_or_else(|| std::path::PathBuf::from("."));
/// ```
///
/// # Shell Integration
///
/// When used as a CLI tool, the help output provides comprehensive usage information:
///
/// ```text
/// bestls is a Rust-powered file listing CLI tool.
///
/// Features:
/// - Outputs in table or JSON formats.
/// - Supports sorting by name, size, or modification date.
/// - Pretty-printed JSON output available.
/// - Shell completion generation support.
/// ```

/// Enumeration of available output formats.
///
/// This enum defines the different output formats supported by bestls.
/// It uses `clap`'s `ValueEnum` derive to automatically generate command-line
/// value parsing and validation.
///
/// # Variants
///
/// * `Table` - Pretty table format (default)
/// * `Json` - Compact JSON format
/// * `JsonPretty` - Pretty-printed JSON format
#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum OutputFormat {
    /// Pretty table format (default)
    #[value(name = "table")]
    Table,
    /// Compact JSON format
    #[value(name = "json")]
    Json,
    /// Pretty-printed JSON format
    #[value(name = "json-pretty")]
    JsonPretty,
}

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

    #[arg(
        long = "compact",
        help = "Output in compact single-column format.",
        default_value_t = false
    )]
    pub compact: bool,

    #[arg(
        long = "columns",
        value_name = "COLS",
        help = "Comma-separated columns to display: name,type,size,date,permissions,owner,group"
    )]
    pub columns: Option<String>,

    #[arg(
        long = "out",
        value_name = "FILE",
        help = "Export output to file instead of stdout."
    )]
    pub output_file: Option<std::path::PathBuf>,

    #[arg(
        long = "format",
        value_name = "FORMAT",
        value_enum,
        default_value = "table",
        help = "Output format: table, json, or json-pretty"
    )]
    pub format: OutputFormat,

    #[arg(
        long = "no-color",
        help = "Disable colored output.",
        default_value_t = false
    )]
    pub no_color: bool,

    #[arg(
        long = "tree",
        help = "Display directory tree (recursive listing).",
        default_value_t = false
    )]
    pub tree: bool,

    #[arg(
        long = "depth",
        value_name = "N",
        help = "Maximum depth for tree traversal (only with --tree)."
    )]
    pub depth: Option<usize>,

    #[arg(
        long = "filter-ext",
        value_name = "EXT",
        help = "Filter by file extension (e.g., rs,txt,md). Comma-separated list."
    )]
    pub filter_ext: Option<String>,

    #[arg(
        long = "filter-name",
        value_name = "PATTERN",
        help = "Filter by filename pattern (glob-style, e.g., '*.txt')."
    )]
    pub filter_name: Option<String>,

    #[arg(
        long = "min-size",
        value_name = "SIZE",
        help = "Filter files with minimum size (e.g., 1KB, 1MB, 100B)."
    )]
    pub min_size: Option<String>,

    #[arg(
        long = "max-size",
        value_name = "SIZE",
        help = "Filter files with maximum size (e.g., 1KB, 1MB, 100B)."
    )]
    pub max_size: Option<String>,
}

/// Enumeration of available sorting options for file listings.
///
/// This enum defines the different ways files can be sorted in the output.
/// It uses `clap`'s `ValueEnum` derive to automatically generate command-line
/// value parsing and validation.
///
/// # Variants
///
/// * `Name` - Sort files alphabetically by filename (default)
/// * `Size` - Sort files by size in bytes (smallest to largest)
/// * `Date` - Sort files by modification date (oldest to newest)
///
/// # Examples
///
/// ```rust
/// use bestls::cli::SortBy;
/// use clap::ValueEnum;
///
/// // Parse from command line value
/// let sort_option = SortBy::from_str("size", true).unwrap();
/// match sort_option {
///     SortBy::Name => println!("Sorting by name"),
///     SortBy::Size => println!("Sorting by size"),
///     SortBy::Date => println!("Sorting by date"),
/// }
/// ```
///
/// # CLI Usage
///
/// ```bash
/// # Sort by name (default)
/// bestls
///
/// # Sort by file size
/// bestls --sort size
///
/// # Sort by modification date
/// bestls --sort date
/// ```
#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum SortBy {
    /// Sort files alphabetically by filename
    Name,
    /// Sort files by size in bytes (smallest to largest)
    Size,
    /// Sort files by modification date (oldest to newest)
    Date,
}

/// Available subcommands for the bestls CLI.
///
/// Currently, the only subcommand is completion generation for various shells.
/// This enum uses `clap`'s derive API to automatically handle subcommand parsing
/// and help text generation.
///
/// # Subcommands
///
/// * `Completion` - Generate shell completion scripts
///
/// # Examples
///
/// ```bash
/// # Generate bash completions
/// bestls completion bash
///
/// # Generate zsh completions  
/// bestls completion zsh
///
/// # Generate fish completions
/// bestls completion fish
/// ```
///
/// # Installation
///
/// After generating completion scripts, install them to the appropriate location:
///
/// ```bash
/// # Bash
/// bestls completion bash > ~/.local/share/bash-completion/completions/bestls
///
/// # Zsh (ensure ~/.zfunc is in your $fpath)
/// bestls completion zsh > ~/.zfunc/_bestls
///
/// # Fish
/// bestls completion fish > ~/.config/fish/completions/bestls.fish
/// ```
#[derive(Debug, Parser)]
pub enum Commands {
    /// Generate shell completion scripts for bestls.
    ///
    /// This subcommand generates completion scripts for various shells,
    /// enabling tab completion for commands, options, and arguments.
    Completion {
        /// The target shell to generate completions for.
        ///
        /// Supported shells include bash, zsh, fish, powershell, and elvish.
        /// The generated script should be saved to the appropriate location
        /// for your shell's completion system.
        #[arg(value_enum)]
        shell: Shell,
    },
}

impl Cli {
    /// Generate and output shell completion scripts to stdout.
    ///
    /// This method creates completion scripts for the specified shell using `clap_complete`.
    /// The generated script provides tab completion for all commands, options, and their values.
    ///
    /// # Arguments
    ///
    /// * `shell` - The target shell type (bash, zsh, fish, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bestls::cli::Cli;
    /// use clap_complete::Shell;
    ///
    /// // Generate bash completions
    /// Cli::generate_completion(Shell::Bash);
    /// ```
    ///
    /// # Usage
    ///
    /// Typically called from the main function when the completion subcommand is used:
    ///
    /// ```bash
    /// # Generate and save bash completions
    /// bestls completion bash > ~/.local/share/bash-completion/completions/bestls
    /// ```
    ///
    /// # Output
    ///
    /// The completion script is written to stdout, allowing for easy redirection
    /// to the appropriate completion directory for your shell.
    pub fn generate_completion(shell: Shell) {
        let mut cmd = Self::command();
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());
    }
}
