// src/main.rs
mod cli;
mod fsops;
mod table;

use clap::Parser;
use cli::{CLI, SortBy};
use fsops::get_files;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use table::print_table;

fn main() {
    let cli: CLI = CLI::parse();
    let path: PathBuf = cli.path.unwrap_or_else(|| PathBuf::from("."));

    match get_files(&path) {
        Ok(mut files) => {
            match cli.sort_by {
                SortBy::Name => files.sort_by(|a, b| a.name.cmp(&b.name)),
                SortBy::Size => files.sort_by(|a, b| a.len_bytes.cmp(&b.len_bytes)),
                SortBy::Date => files.sort_by(|a, b| a.modified.cmp(&b.modified)),
            }

            if cli.json_pretty {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&files)
                        .unwrap_or_else(|_| "cannot parse to JSON".into())
                );
            } else if cli.json {
                println!(
                    "{}",
                    serde_json::to_string(&files).unwrap_or_else(|_| "cannot parse to JSON".into())
                );
            } else {
                print_table(files);
            }
        }
        Err(e) => eprintln!("{}: {}", "Failed to read directory".red(), e),
    }
    println!("{}", path.display());
}
