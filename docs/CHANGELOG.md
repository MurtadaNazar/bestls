# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
