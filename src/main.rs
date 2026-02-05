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
mod fsops;
mod table;

use clap::Parser;
use cli::{Cli, Commands, OutputFormat, SortBy};
use fsops::{
    get_files, get_files_recursive, matches_extension, matches_pattern, parse_size, FileEntry,
};
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use table::format_table;

/// Precomputed filter configuration to avoid repeated parsing per file
struct FilterConfig {
    exts: Option<Vec<String>>,
    name_pattern: Option<String>,
    min_size: Option<u64>,
    max_size: Option<u64>,
}

impl FilterConfig {
    /// Build filter configuration from CLI options, parsing once
    fn from_cli(cli: &Cli) -> Self {
        let exts = cli.filter_ext.as_ref().map(|ext_filter| {
            ext_filter
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        });

        FilterConfig {
            exts,
            name_pattern: cli.filter_name.clone(),
            min_size: cli.min_size.as_deref().and_then(parse_size),
            max_size: cli.max_size.as_deref().and_then(parse_size),
        }
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

    if let Some(Commands::Completion { shell }) = cli.command {
        Cli::generate_completion(shell);
        return;
    }

    let path: PathBuf = cli
        .path
        .as_deref()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let include_hidden: bool = cli.all;

    // Precompute filter configuration once
    let filter_cfg = FilterConfig::from_cli(&cli);

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

            // Generate output based on selected format
            let output = match cli.format {
                OutputFormat::Json => {
                    serde_json::to_string(&files).unwrap_or_else(|_| "cannot parse to JSON".into())
                }
                OutputFormat::JsonPretty => serde_json::to_string_pretty(&files)
                    .unwrap_or_else(|_| "cannot parse to JSON".into()),
                OutputFormat::Table => {
                    // Handle legacy json/json_pretty flags for backward compatibility
                    if cli.json_pretty {
                        serde_json::to_string_pretty(&files)
                            .unwrap_or_else(|_| "cannot parse to JSON".into())
                    } else if cli.json {
                        serde_json::to_string(&files)
                            .unwrap_or_else(|_| "cannot parse to JSON".into())
                    } else {
                        // Format table/compact output as string
                        format_table(&files, cli.columns.clone(), cli.compact, !cli.no_color)
                    }
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
