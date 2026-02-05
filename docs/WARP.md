# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

`bestls` is a Rust-powered CLI tool that serves as a modern replacement for the traditional `ls` command. It provides colorful, human-readable file listings with support for multiple output formats (table, JSON), parallel metadata processing, and flexible sorting options.

## Development Commands

### Building and Testing
```bash
# Check code without building
cargo check

# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Run tests (currently no tests exist)
cargo test

# Run with cargo directly for development
cargo run -- [args]

# Example: List current directory with JSON output
cargo run -- --json-pretty
```

### Code Quality
```bash
# Format code (required before commits)
cargo fmt

# Lint code (required before commits, no warnings allowed)
cargo clippy

# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update
```

### Installation and Distribution
```bash
# Install locally for testing
cargo install --path .

# Generate shell completions
cargo run -- completion bash > ~/.local/share/bash-completion/completions/bestls
cargo run -- completion zsh > ~/.zfunc/_bestls
cargo run -- completion fish > ~/.config/fish/completions/bestls.fish
```

## Code Architecture

### Module Structure
The codebase follows a clean separation of concerns across four main modules:

- **`main.rs`**: Entry point and orchestration logic. Handles CLI parsing, file retrieval, sorting, and output format selection.
- **`cli.rs`**: Command-line interface definitions using `clap`. Defines the `Cli` struct with all arguments and the completion system.
- **`fsops.rs`**: File system operations and data structures. Contains `FileEntry` struct and parallel metadata processing using `rayon`.
- **`table.rs`**: Table formatting and display logic using the `tabled` crate with colorized output.

### Key Data Structures

#### `FileEntry` (fsops.rs)
The core data structure representing a file/directory with comprehensive metadata:
```rust
pub struct FileEntry {
    pub name: String,
    pub e_type: FileType,        // File, Directory, or Symlink
    pub len_bytes: u64,          // Raw size in bytes
    pub human_size: String,      // Human-readable size (KB, MB, GB)
    pub modified: String,        // Formatted modification timestamp
    pub permissions: String,     // Unix-style permissions (rwxrwxrwx)
    pub owner: String,           // File owner name
    pub group: String,           // File group name
}
```

#### Platform-Specific Features
- **Unix Systems**: Full permissions, owner/group resolution via `nix` crate
- **Windows**: Basic readonly/readwrite permissions, placeholder owner/group
- **Other Platforms**: "N/A" fallbacks for unsupported features

### Processing Pipeline
1. **CLI Parsing**: `clap` processes command-line arguments
2. **Directory Reading**: `fs::read_dir()` with hidden file filtering
3. **Parallel Processing**: `rayon` processes metadata for all entries simultaneously
4. **Sorting**: In-memory sorting by name, size, or modification date
5. **Output**: Either pretty table, compact JSON, or formatted JSON

### Key Dependencies
- **`clap`**: CLI argument parsing with derive macros
- **`rayon`**: Data parallelism for metadata fetching
- **`tabled`**: Table formatting with styles and colors
- **`serde`**: JSON serialization for `FileEntry`
- **`nix`** (Unix only): System user/group resolution
- **`chrono`**: Date/time formatting
- **`bytesize`**: Human-readable file size formatting

## Development Workflow

### Contributing Process (from CONTRIBUTING.md)
1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Run quality checks: `cargo test`, `cargo clippy`, `cargo fmt`
4. Commit with descriptive message
5. Push and create Pull Request

### Code Standards
- Follow official Rust style guide
- Use `cargo fmt` before committing (required)
- Ensure `cargo clippy` shows no warnings (required)
- Document public API elements with rustdoc
- Keep functions focused and reasonably sized

## Current State and Roadmap

### Current Version: 1.1.0
- Core listing functionality with parallel processing
- Multiple output formats (table, JSON)
- Sorting by name, size, or date
- Unix permissions and ownership display
- Shell completion generation
- **Note**: No unit tests currently exist

### Planned Features (from ROADMAP.md)
- **v1.2**: Tree view, depth limits, file filtering
- **v1.3**: Compact mode, customizable columns
- **v1.4**: Color themes, extension-based coloring
- **v2.0**: Plugin architecture

### Technical Debt
- Missing comprehensive test suite
- No benchmarks or performance testing
- No CI/CD setup yet

## Platform Considerations

### Unix-Specific Code
```rust
#[cfg(unix)]
// Permission handling, user/group resolution
```

### Cross-Platform Compatibility
The codebase uses conditional compilation to handle platform differences, particularly for file permissions and ownership information.

## File Structure Context
```
src/
├── main.rs      # Entry point and orchestration
├── cli.rs       # Command-line interface definitions  
├── fsops.rs     # File system operations and data structures
└── table.rs     # Table formatting and display
```

## Installation Methods
- **From crates.io**: `cargo install bestls`
- **From source**: `cargo build --release` (binary at `target/release/bestls`)
- **AUR Package**: Available in `pkg/` directory for Arch Linux users
