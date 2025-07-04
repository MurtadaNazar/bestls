# bestls

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)

A fast, colorful, and Rust-powered replacement for the traditional `ls` command.

`bestls` provides human-readable file listings in both pretty tables and JSON formats, with sorting and other modern CLI conveniences built for speed and usability.

## ✨ Features

- 🎨 **Colorful output** - Beautiful colored tables for easy reading
- 📊 **Multiple formats** - Output as tables, compact JSON, or pretty JSON
- ⚡ **Blazing fast** - Parallel metadata fetching with Rayon
- 📏 **Human-readable** - File sizes in KB, MB, GB format
- 🔧 **Flexible sorting** - Sort by name, size, or modification date
- 🪶 **Lightweight** - Single binary with no external dependencies

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

## 🚀 Usage

### Basic Commands

List current directory:

```bash
bestls
```

List specific directory:

```bash
bestls -p /path/to/directory
```

### Output Formats

Pretty table (default):

```bash
bestls
```

Compact JSON:

```bash
bestls --json
```

Pretty formatted JSON:

```bash
bestls --json-pretty
```

### Sorting Options

Sort by file size:

```bash
bestls -s size
```

Sort by modification date:

```bash
bestls -s date
```

Sort by name (default):

```bash
bestls -s name
```

### Examples

```bash
# List home directory with size sorting
bestls -p ~ -s size

# Output current directory as pretty JSON
bestls --json-pretty

# List /etc directory sorted by modification date
bestls -p /etc -s date
```

### Help

View all available options:

```bash
bestls --help
```

## 🛠️ Command Line Options

| Option          | Short | Description                     |
| --------------- | ----- | ------------------------------- |
| `--path`        | `-p`  | Directory path to list          |
| `--sort`        | `-s`  | Sort by: `name`, `size`, `date` |
| `--json`        |       | Output compact JSON format      |
| `--json-pretty` |       | Output pretty formatted JSON    |
| `--help`        | `-h`  | Show help information           |
| `--version`     | `-V`  | Show version information        |

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

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under:

- **MIT License** - See [LICENSE](LICENSE) for details

## 👨‍💻 Author

**Murtaza Nazar**

- Email: [mkm9284@gmail.com](mailto:mkm9284@gmail.com)
- GitHub: [@MurtadaNazar](https://github.com/MurtadaNazar)

## 🔗 Links

- [Repository](https://github.com/MurtadaNazar/bestls)
- [Issues](https://github.com/MurtadaNazar/bestls/issues)
- [Releases](https://github.com/MurtadaNazar/bestls/releases)
