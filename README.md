# bestls

[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub Discussions](https://img.shields.io/github/discussions/MurtadaNazar/bestls?color=blue&logo=github)](https://github.com/MurtadaNazar/bestls/discussions)
[![Version: 1.3.0](https://img.shields.io/badge/version-1.3.0-blue.svg)](docs/CHANGELOG.md)

A fast, colorful, and Rust-powered replacement for the traditional `ls` command.

`bestls` provides human-readable file listings in both pretty tables and JSON formats, with powerful filtering, tree view, and modern CLI conveniences built for speed and usability.

## ✨ Features

- 🎨 **Colorful output** - Beautiful colored tables with optional color controls
- 📊 **Multiple formats** - Tables, compact JSON, pretty JSON, or single-column compact mode
- ⚡ **Blazing fast** - Parallel metadata fetching with Rayon
- 📏 **Human-readable** - File sizes in KB, MB, GB format with conversions
- 🔧 **Flexible sorting** - Sort by name, size, or modification date
- 🌳 **Tree view** - Recursive directory traversal with depth control
- 🔍 **Smart filtering** - Filter by extension, pattern, and file size
- 👁️ **Hidden files** - View hidden files with `--all` flag
- 🔒 **File permissions** - Unix-style permission display (rwxrwxrwx)
- 👥 **Owner info** - File owner and group information
- 📤 **Export data** - Save results to files, JSON formats for automation
- 🔄 **Shell completion** - Tab-completion for Bash, Zsh, and Fish
- 🪶 **Lightweight** - Single binary with no runtime dependencies

## 📦 Installation

### From crates.io

```bash
cargo install bestls
```

### From source

```bash
git clone https://github.com/MurtadaNazar/bestls.git
cd bestls
cargo build --release
```

The binary will be available at `target/release/bestls`.

## 🚀 Quick Start

### Basic Usage

```bash
# List current directory
bestls

# List specific directory
bestls -p /path/to/directory

# Include hidden files
bestls -a

# Sort by size
bestls --sort size
```

### Filtering & Tree View

```bash
# Show only Rust files
bestls --filter-ext rs

# Recursive tree view (limit depth to 2)
bestls --tree --depth 2

# Find files between 1KB and 10MB
bestls --min-size 1KB --max-size 10MB

# Pattern matching
bestls --filter-name "*.md"
```

### Output Options

```bash
# Compact single-column output
bestls --compact

# Export as JSON
bestls --format json --out results.json

# Pretty JSON
bestls --format json-pretty

# No colors (for piping/scripts)
bestls --no-color
```

### Advanced Examples

```bash
# Find large Rust files sorted by size
bestls -p src --filter-ext rs --min-size 5KB --sort size

# Export directory structure as JSON
bestls --tree --depth 3 --format json --out structure.json

# Filter markdown files and export
bestls --filter-ext md --json-pretty --out docs_list.json
```

### Shell Completions

Enable tab-completion in your shell:

```bash
# For Bash
bestls completion bash > ~/.local/share/bash-completion/completions/bestls

# For Zsh (add to .zshrc: fpath=(~/.zfunc $fpath))
bestls completion zsh > ~/.zfunc/_bestls

# For Fish
bestls completion fish > ~/.config/fish/completions/bestls.fish
```

## 📖 Documentation

- **[EXAMPLES.md](docs/EXAMPLES.md)** - Comprehensive usage examples and workflows
- **[CHANGELOG.md](docs/CHANGELOG.md)** - Version history and release notes
- **[CONTRIBUTING.md](docs/CONTRIBUTING.md)** - Guidelines for contributors
- **[ROADMAP.md](docs/ROADMAP.md)** - Planned features and development roadmap
- **[VERSION_POLICY.md](docs/VERSION_POLICY.md)** - Version management and release process
- **[IMPLEMENTATION_SUMMARY.md](docs/IMPLEMENTATION_SUMMARY.md)** - Technical implementation details

## 🛠️ Command Line Options

### Core Options

| Option      | Short | Description                 |
| ----------- | ----- | --------------------------- |
| `--path`    | `-p`  | Directory path to list      |
| `--sort`    | `-s`  | Sort by: `name`, `size`, `date` |
| `--all`     | `-a`  | Show hidden files (starting with .) |
| `--help`    | `-h`  | Show help information       |
| `--version` | `-V`  | Show version information    |

### Filtering Options

| Option          | Description                                   |
| --------------- | --------------------------------------------- |
| `--tree`        | Recursive directory listing                   |
| `--depth N`     | Maximum recursion depth                       |
| `--filter-ext`  | Filter by extensions (comma-separated)        |
| `--filter-name` | Filter by filename pattern (glob-style)       |
| `--min-size`    | Minimum file size (e.g., 1KB, 1MB)            |
| `--max-size`    | Maximum file size (e.g., 10MB, 1GB)           |

### Output Options

| Option          | Description                          |
| --------------- | ------------------------------------ |
| `--format`      | Output format: `table`, `json`, `json-pretty` |
| `--compact`     | Single-column output mode            |
| `--columns`     | Select visible columns               |
| `--out`         | Export output to file                |
| `--no-color`    | Disable colored output               |
| `--json`        | Compact JSON (legacy)                |
| `--json-pretty` | Pretty JSON (legacy)                 |

### Subcommands

| Command     | Description                      |
| ----------- | -------------------------------- |
| `completion`| Generate shell completions       |

## 🏗️ Building from Source

### Prerequisites

- Rust 1.85.0 or later
- Cargo package manager

### Build Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/MurtadaNazar/bestls.git
   cd bestls
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. (Optional) Install globally:

   ```bash
   cargo install --path .
   ```

## 💬 Community

Join our community discussions! We'd love to hear from you:

- 💡 **[Ideas & Feature Requests](https://github.com/MurtadaNazar/bestls/discussions)** - Share your ideas for new features
- ❓ **[Q&A](https://github.com/MurtadaNazar/bestls/discussions)** - Get help and ask questions
- 🛠️ **[Show and Tell](https://github.com/MurtadaNazar/bestls/discussions)** - Share how you're using bestls
- 📢 **[Announcements](https://github.com/MurtadaNazar/bestls/discussions)** - Stay updated with the latest news

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

For more details, see [CONTRIBUTING.md](docs/CONTRIBUTING.md).

## 📄 License

This project is licensed under:

- **MIT License** - See [LICENSE](LICENSE) for details

## 👨‍💻 Author

### Murtaza Nazar

- Email: [mkm9284@gmail.com](mailto:mkm9284@gmail.com)
- GitHub: [@MurtadaNazar](https://github.com/MurtadaNazar)

## 🔗 Links

- [Repository](https://github.com/MurtadaNazar/bestls)
- [Issues](https://github.com/MurtadaNazar/bestls/issues)
- [Releases](https://github.com/MurtadaNazar/bestls/releases)
