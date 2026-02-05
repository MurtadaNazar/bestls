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
use fsops::{get_files, get_files_recursive, parse_size, matches_extension, matches_pattern};
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use table::print_table;

/// Main entry point for the bestls application.
/// 
/// This function orchestrates the entire file listing process:
/// 1. Parses command-line arguments using `clap`
/// 2. Handles shell completion generation if requested
/// 3. Retrieves file entries from the specified directory
/// 4. Sorts the entries according to the specified criteria
/// 5. Outputs the results in the requested format (table or JSON)
/// 
/// # Process Flow
/// 
/// 1. **Argument Parsing**: Uses `clap` to parse and validate command-line arguments
/// 2. **Completion Check**: If completion subcommand is provided, generates shell completions and exits
/// 3. **Directory Processing**: Calls [`fsops::get_files`] to retrieve and process file metadata
/// 4. **Sorting**: Applies the requested sorting algorithm (name, size, or date)
/// 5. **Output**: Formats and displays results based on the chosen output format
/// 
/// # Examples
/// 
/// The main function handles various CLI invocations:
/// 
/// ```bash
/// # List current directory with default table output
/// bestls
/// 
/// # List specific directory as pretty JSON, sorted by size
/// bestls -p /home/user --json-pretty --sort size
/// 
/// # Include hidden files and sort by modification date
/// bestls -a --sort date
/// ```
/// 
/// # Error Handling
/// 
/// If directory reading fails, the function prints a colorized error message using `owo_colors`
/// and continues execution. This allows the program to gracefully handle permission errors
/// or invalid paths without crashing.
fn main() {
    let cli: Cli = Cli::parse();

    if let Some(Commands::Completion { shell }) = cli.command {
        Cli::generate_completion(shell);
        return;
    }

    let path: PathBuf = cli.path.unwrap_or_else(|| PathBuf::from("."));
    let include_hidden: bool = cli.all;

    // Get files (tree or flat)
    let get_result = if cli.tree {
        get_files_recursive(&path, include_hidden, cli.depth)
    } else {
        get_files(&path, include_hidden)
    };

    match get_result {
        Ok(mut files) => {
            // Apply filters
            files.retain(|f| {
                // Extension filter
                if let Some(ref ext_filter) = cli.filter_ext {
                    let exts: Vec<String> = ext_filter
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    if !matches_extension(&f.name, &exts) {
                        return false;
                    }
                }
                
                // Name pattern filter
                if let Some(ref name_pattern) = cli.filter_name {
                    if !matches_pattern(&f.name, name_pattern) {
                        return false;
                    }
                }
                
                // Size filters
                if let Some(ref min) = cli.min_size {
                    if let Some(min_bytes) = parse_size(min) {
                        if f.len_bytes < min_bytes {
                            return false;
                        }
                    }
                }
                
                if let Some(ref max) = cli.max_size {
                    if let Some(max_bytes) = parse_size(max) {
                        if f.len_bytes > max_bytes {
                            return false;
                        }
                    }
                }
                
                true
            });

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

            // Generate and output results
            let mut output = String::new();
            match cli.format {
                OutputFormat::Json => {
                    output = serde_json::to_string(&files)
                        .unwrap_or_else(|_| "cannot parse to JSON".into());
                }
                OutputFormat::JsonPretty => {
                    output = serde_json::to_string_pretty(&files)
                        .unwrap_or_else(|_| "cannot parse to JSON".into());
                }
                OutputFormat::Table => {
                    // Handle legacy json/json_pretty flags for backward compatibility
                    if cli.json_pretty {
                        output = serde_json::to_string_pretty(&files)
                            .unwrap_or_else(|_| "cannot parse to JSON".into());
                    } else if cli.json {
                        output = serde_json::to_string(&files)
                            .unwrap_or_else(|_| "cannot parse to JSON".into());
                    } else {
                        print_table(files, cli.columns.clone(), cli.compact, !cli.no_color);
                        output.clear(); // Skip file writing for table output (prints directly)
                    }
                }
            }

            // Write to file if output_file specified (only for JSON/string output)
            if !output.is_empty() {
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
        }
        Err(e) => eprintln!("{}: {}", "Failed to read directory".red(), e),
    }
}
