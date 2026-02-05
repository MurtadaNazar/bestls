# Roadmap – bestls

This document outlines the planned evolution of `bestls`.
For detailed developer tasks, see [TODO.md](./TODO.md).

---

## ✅ v1.x – Core Features

### v1.1 – Hidden Files & Permissions

- Show hidden files (`--all` / `-a`)
- Display Unix-style file permissions
- Show file owner/group information
- Refactor table output to include new columns

### ✅ v1.2 – Shell Completions (COMPLETED)

- ✅ Shell completions for Bash, Zsh, and Fish (`clap_complete`)
- ✅ `completion` subcommand for generating completions 
- ✅ Comprehensive installation instructions
- ✅ Integration with package managers (AUR)

### v1.3 – Tree View & Filtering

- Recursive listings with `--tree`
- Depth limits with `--depth <n>`
- File filters by extension or name
- Size-based filters (`--min-size`, `--max-size`)

### v1.4 – Output Customization

- Compact mode (`--compact`)
- Customizable columns
- Export output to file
- Multiple output formats (JSON, table)

### ✅ v1.5 – Color & Theme (COMPLETED)

- ✅ Color coding by extension
- ✅ Theme support via config file (`~/.config/bestls/config.toml`)
- ✅ Disable colors with `--no-color`
- ✅ Theme management CLI (`theme init`, `theme path`, `theme reset`)
- ✅ Default color mappings for 16+ file types

### v1.6 – Advanced Configuration

- Load defaults from config file
- Configurable sorting, columns, and colors
- Environment variable support
- Custom config file paths

---

## 🧪 Internal Improvements

- Unit tests and integration tests
- Benchmarks with Criterion
- Continuous Integration setup

---

## 🌐 Community & Ecosystem

- CONTRIBUTING guidelines
- GitHub issue templates & discussions
- AUR packaging (PKGBUILD)
- Improved crates.io metadata
- CHANGELOG for releases

---

## 🧱 v2.0 – Plugin Architecture

- Extensible plugin system
- Column/filter plugins
- Dynamic `.so` / `.dll` or subcommand modules

---

## 📦 Distribution

- Precompiled binaries in GitHub releases
- Homebrew formula
- Snap and AppImage packages
