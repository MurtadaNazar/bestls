//! # bestls - A Modern File Listing Tool
//!
//! **bestls** is a Rust-powered CLI tool that serves as a modern replacement for the traditional `ls` command.
//! It provides colorful, human-readable file listings with support for multiple output formats, parallel metadata
//! processing, and flexible sorting options.
//!
//! ## Features
//!
//! * **Multiple Output Formats**: Choose between pretty table, compact JSON, or formatted JSON output
//! * **Parallel Processing**: Uses `rayon` for concurrent metadata fetching to improve performance
//! * **Flexible Sorting**: Sort by name, size, or modification date
//! * **Cross-Platform**: Supports Unix permissions and ownership, with graceful fallbacks for other platforms
//! * **Shell Completions**: Generate completions for bash, zsh, and fish shells
//!
//! ## Architecture
//!
//! The codebase follows a clean separation of concerns across four main modules:
//!
//! - **`main`**: Entry point and orchestration logic
//! - **`cli`**: Command-line interface definitions using `clap`
//! - **`fsops`**: File system operations and data structures
//! - **`table`**: Table formatting and display logic
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```bash
//! # List current directory
//! bestls
//!
//! # List specific directory with hidden files
//! bestls -p /home/user -a
//!
//! # Output as pretty JSON sorted by size
//! bestls --json-pretty --sort size
//!
//! # Generate shell completions
//! bestls completion bash > ~/.local/share/bash-completion/completions/bestls
//! ```
//!
//! ### Programmatic Usage
//!
//! While primarily designed as a CLI tool, the core functionality can be used as a library:
//!
//! ```rust
//! use std::path::PathBuf;
//! use bestls::fsops::get_files;
//!
//! // Get file entries for current directory
//! let path = PathBuf::from(".");
//! let include_hidden = false;
//!
//! match get_files(&path, include_hidden) {
//!     Ok(files) => {
//!         println!("Found {} files", files.len());
//!         for file in files {
//!             println!("{}: {}", file.name, file.human_size);
//!         }
//!     }
//!     Err(e) => eprintln!("Error reading directory: {}", e),
//! }
//! ```

mod cli;
mod color;
mod fsops;
mod table;

use clap::Parser;
use cli::{Cli, Commands, OutputFormat, SortBy, ThemeSubcommand};
use color::{create_sample_config, load_theme};
use fsops::{
    get_files, get_files_recursive, matches_extension, matches_pattern, parse_size, FileEntry,
};
use glob::Pattern;
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use table::format_table;

/// Error type for filter configuration
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
enum ConfigError {
    InvalidGlobPattern(String),
    InvalidMinSize(String),
    InvalidMaxSize(String),
    SizeRangeInvalid(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidGlobPattern(e) => write!(f, "invalid glob pattern: {}", e),
            ConfigError::InvalidMinSize(e) => write!(f, "invalid --min-size value: {}", e),
            ConfigError::InvalidMaxSize(e) => write!(f, "invalid --max-size value: {}", e),
            ConfigError::SizeRangeInvalid(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Precomputed filter configuration to avoid repeated parsing per file
struct FilterConfig {
    exts: Option<Vec<String>>,
    name_pattern: Option<Pattern>,
    min_size: Option<u64>,
    max_size: Option<u64>,
}

impl FilterConfig {
    /// Build filter configuration from CLI options, parsing once
    /// Returns Err if any configuration is invalid
    fn from_cli(cli: &Cli) -> Result<Self, ConfigError> {
        // Pre-normalize extensions: lowercase and strip leading '.'
        let exts = cli.filter_ext.as_ref().map(|ext_filter| {
            ext_filter
                .split(',')
                .map(|s| s.trim().trim_start_matches('.').to_lowercase())
                .collect::<Vec<_>>()
        });

        // Compile and validate glob pattern once
        let name_pattern = match cli.filter_name.as_deref() {
            Some(pattern_str) => match Pattern::new(pattern_str) {
                Ok(pattern) => Some(pattern),
                Err(e) => {
                    return Err(ConfigError::InvalidGlobPattern(format!(
                        "invalid glob pattern '{}': {}",
                        pattern_str, e
                    )))
                }
            },
            None => None,
        };

        // Parse size strings once
        let min_size = if let Some(min_str) = cli.min_size.as_deref() {
            match parse_size(min_str) {
                Ok(size) => Some(size),
                Err(e) => return Err(ConfigError::InvalidMinSize(e.to_string())),
            }
        } else {
            None
        };

        let max_size = if let Some(max_str) = cli.max_size.as_deref() {
            match parse_size(max_str) {
                Ok(size) => Some(size),
                Err(e) => return Err(ConfigError::InvalidMaxSize(e.to_string())),
            }
        } else {
            None
        };

        // Validate that min_size <= max_size
        if let (Some(min), Some(max)) = (min_size, max_size) {
            if min > max {
                return Err(ConfigError::SizeRangeInvalid(format!(
                    "--min-size ({}) must be less than or equal to --max-size ({})",
                    min, max
                )));
            }
        }

        Ok(FilterConfig {
            exts,
            name_pattern,
            min_size,
            max_size,
        })
    }
}

/// Apply all filters to a file entry based on precomputed filter configuration
fn passes_filters(f: &FileEntry, cfg: &FilterConfig) -> bool {
    // Extension filter
    if let Some(ref exts) = cfg.exts {
        if !matches_extension(&f.name, exts) {
            return false;
        }
    }

    // Name pattern filter
    if let Some(ref name_pattern) = cfg.name_pattern {
        if !matches_pattern(&f.name, name_pattern) {
            return false;
        }
    }

    // Minimum size filter
    if let Some(min) = cfg.min_size {
        if f.len_bytes < min {
            return false;
        }
    }

    // Maximum size filter
    if let Some(max) = cfg.max_size {
        if f.len_bytes > max {
            return false;
        }
    }

    true
}

/// Load files from the specified path (tree or flat)
fn load_files(cli: &Cli, path: &Path, include_hidden: bool) -> std::io::Result<Vec<FileEntry>> {
    if cli.tree {
        get_files_recursive(path, include_hidden, cli.depth)
    } else {
        get_files(path, include_hidden)
    }
}

/// Handle theme management commands
fn handle_theme_command(subcommand: &ThemeSubcommand) {
    match subcommand {
        ThemeSubcommand::Init { show } => match create_sample_config() {
            Ok(path) => {
                println!("Theme config created at: {}", path.display());
                if *show {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => println!("\n{}", content),
                        Err(e) => eprintln!("Error reading config: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error creating config: {}", e),
        },
        ThemeSubcommand::Path => {
            if let Some(config_dir) = dirs::config_dir() {
                let config_path = config_dir.join("bestls").join("config.toml");
                println!("{}", config_path.display());
            } else {
                eprintln!("Could not determine config directory");
            }
        }
        ThemeSubcommand::Reset => {
            if let Some(config_dir) = dirs::config_dir() {
                let config_path = config_dir.join("bestls").join("config.toml");
                if config_path.exists() {
                    match std::fs::remove_file(&config_path) {
                        Ok(_) => println!("Theme reset to default (config file removed)"),
                        Err(e) => eprintln!("Error removing config: {}", e),
                    }
                } else {
                    println!("Theme already at default (no config file found)");
                }
            } else {
                eprintln!("Could not determine config directory");
            }
        }
    }
}

/// Main entry point for the bestls application.
///
/// This function orchestrates the entire file listing process:
/// 1. Parses command-line arguments using `clap`
/// 2. Handles shell completion generation if requested
/// 3. Retrieves file entries from the specified directory
/// 4. Applies all filters
/// 5. Sorts the entries according to the specified criteria
/// 6. Outputs the results in the requested format (table or JSON)
fn main() {
    let cli: Cli = Cli::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::Completion { shell } => {
                Cli::generate_completion(*shell);
                return;
            }
            Commands::Theme { subcommand } => {
                handle_theme_command(subcommand);
                return;
            }
        }
    }

    // Load theme for color output
    let theme = load_theme();

    let path: PathBuf = cli
        .path
        .as_deref()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let include_hidden: bool = cli.all;

    // Precompute filter configuration once
    let filter_cfg = match FilterConfig::from_cli(&cli) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(2);
        }
    };

    // Get files (tree or flat)
    let get_result = load_files(&cli, &path, include_hidden);

    match get_result {
        Ok(mut files) => {
            // Apply all configured filters
            files.retain(|f| passes_filters(f, &filter_cfg));

            // Apply sorting
            match cli.sort_by {
                SortBy::Name => {
                    files.sort_by(|a: &fsops::FileEntry, b: &fsops::FileEntry| a.name.cmp(&b.name))
                }
                SortBy::Size => files.sort_by(|a: &fsops::FileEntry, b: &fsops::FileEntry| {
                    a.len_bytes.cmp(&b.len_bytes)
                }),
                SortBy::Date => files.sort_by(|a: &fsops::FileEntry, b: &fsops::FileEntry| {
                    a.modified.cmp(&b.modified)
                }),
            }

            // Generate output based on effective format, normalizing legacy flags to a single source of truth
            let effective_format = cli.effective_format();
            let output = match effective_format {
                OutputFormat::Json => {
                    serde_json::to_string(&files).unwrap_or_else(|_| "cannot parse to JSON".into())
                }
                OutputFormat::JsonPretty => serde_json::to_string_pretty(&files)
                    .unwrap_or_else(|_| "cannot parse to JSON".into()),
                OutputFormat::Table => {
                    // Format table/compact output as string
                    format_table(
                        &files,
                        cli.columns.clone(),
                        cli.compact,
                        !cli.no_color,
                        Some(&theme),
                    )
                }
            };

            // Write output to file or stdout
            if let Some(file_path) = &cli.output_file {
                match File::create(file_path) {
                    Ok(mut file) => {
                        if let Err(e) = writeln!(file, "{}", output) {
                            eprintln!("{}: {}", "Failed to write to file".red(), e);
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: {}", "Failed to create output file".red(), e);
                    }
                }
            } else {
                println!("{}", output);
            }
        }
        Err(e) => eprintln!("{}: {}", "Failed to read directory".red(), e),
    }
}
