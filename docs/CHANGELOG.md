# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2026-02-05

### Added

- **Tree view functionality** with `--tree` flag for hierarchical directory display
- **Recursive file listing** with optional depth limiting via `--depth`
- **File filtering system** with multiple filter options:
  - `--filter-ext` for extension-based filtering (case-insensitive)
  - `--filter-name` for glob pattern matching on filenames
  - `--min-size` and `--max-size` for size-based filtering
- **Custom output formatting**:
  - `--compact` flag for single-column output
  - `--columns` parameter for future column selection
  - Multiple output formats: table (default), JSON, and pretty JSON
- **Rich error types** with `SizeParseError` for better error handling
- Comprehensive documentation in `docs/` directory
- Project structure reorganization with docs, examples, and implementation guides

### Enhanced

- **Extension filtering** is now case-insensitive for better UX
- **Performance optimization**: Pre-computed filter configuration to reduce per-file parsing overhead
- **Error handling**: Size parsing now returns `Result` type instead of `Option`, allowing non-CLI reuse
- **Recursive directory traversal** with proper error logging for subdirectory failures
- CLI argument validation and helpful error messages
- Updated README with new features and comprehensive examples

### Changed

- Moved documentation files to `docs/` directory:
  - `CHANGELOG.md` → `docs/CHANGELOG.md`
  - `ROADMAP.md` → `docs/ROADMAP.md`
  - `TODO.md` → `docs/TODO.md`
  - Added `CONTRIBUTING.md`, `IMPLEMENTATION_SUMMARY.md`, `EXAMPLES.md`, `VERSION_POLICY.md`, `REVIEW_CHECKLIST.md`, `WARP.md`
- Refactored `FilterConfig` struct for pre-parsing and normalization
- Extended `FileEntry` data structure for comprehensive metadata
- Renamed and reorganized CLI options for clarity

### Fixed

- **Overflow handling** in `parse_size`: Added checks for negative values and overflow conditions
- Duplicate metadata calls eliminated in recursive directory traversal
- Trailing whitespace formatting issues resolved
- Broken documentation links fixed (relative paths in `docs/CONTRIBUTING.md`)
- Unused column parsing computation removed to avoid confusion

### Technical

- Introduced `SizeParseError` enum for flexible error reporting
- Implemented `Display` and `Error` traits for proper error propagation
- Optimized extension filtering with pre-normalization in `FilterConfig`
- Added parallel processing for metadata fetching with `rayon`
- Improved recursion error handling with logging instead of silent failures
- Enhanced code organization with better separation of concerns
- Deprecated `format_compact` function in favor of `format_table(..., compact = true, ...)`
- Added `#[allow(dead_code)]` attributes for future-use functions

### Dependencies

- Maintained compatibility with existing dependencies
- No new external dependencies added

## [1.2.0] - 2025-08-22

### Added

- Shell completion generation support for Bash, Zsh, and Fish
- New `completion` subcommand to generate shell completions
- Integration with `clap_complete` for robust completion support
- Updated CLI help with completion usage examples
- Added `dirs` dependency for future configuration file support

### Enhanced

- Arch Linux package (PKGBUILD) with shell completion installation
- Comprehensive installation instructions for completions
- Updated documentation with completion setup guide

### Technical

- Added `clap_complete` dependency for shell completion functionality  
- Implemented `Commands` enum for subcommand structure
- Added completion generation methods to CLI module
- Enhanced command-line interface with completion subcommand

## [1.1.0] - 2025-08-22

### Added

- `--all` / `-a` flag to display hidden files
- Unix-style file permissions display in the output
- File owner and group information display (via `users` crate)
- Enhanced table formatting with new columns

### Changed

- Refactored table output to accommodate new columns
- Improved metadata fetching to include ownership information
- Updated README with new features and examples

### Technical

- Integrated with Unix file permission system
- Added user/group resolution functionality
- Optimized table rendering for new column layout

## [1.0.0] - 2025-08-22

### Added

- Initial release of bestls
- Colorful table output with human-readable file sizes
- Multiple output formats (table, JSON, pretty JSON)
- Parallel metadata fetching with Rayon
- Flexible sorting options (name, size, date)
- Basic command-line interface
- Installation support via cargo

[1.2.0]: https://github.com/MurtadaNazar/bestls/compare/v1.1.1...v1.2.0
[1.1.0]: https://github.com/MurtadaNazar/bestls/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/MurtadaNazar/bestls/releases/tag/v1.0.0
