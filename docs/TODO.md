# ✅ Roadmap for `bestls`

A structured and professional development plan for future enhancements to the `bestls` CLI tool.

---

## 🔁 v1.1 – Hidden Files & Permissions

- [x] Add `--all` / `-a` flag to display hidden files
- [x] Display Unix-style file permissions
- [x] Show file owner and group information (via `users` crate)
- [x] Refactor table output to include new columns

---

## ⚙️ v1.2 – Shell Completions ✅

- [x] Add shell completions (via `clap_complete`)
- [x] Support `completion` subcommand for generating completions
- [x] Integration with Bash, Zsh, and Fish shells
- [x] Added completion installation instructions

---

## ✅ v1.3 – Tree View & Filtering

- [x] Add `--tree` flag for recursive directory listing
- [x] Implement depth limit with `--depth <n>`
- [x] Add filtering options:
  - [x] `--filter-ext <ext>` (comma-separated extensions)
  - [x] `--filter-name <pattern>` (glob-style patterns)
- [x] Add size-based filters with human-readable units:
  - [x] `--min-size <size>` (e.g., 1KB, 1MB, 100B)
  - [x] `--max-size <size>` (e.g., 1KB, 1MB, 100B)

---

## ✅ v1.4 – Output Customization

- [x] Add `--compact` for minimal single-column output (like `ls -1`)
- [x] Add `--columns name,size,date,...` to customize visible columns
- [x] Add `--out <file>` option to export output
- [x] Add `--format json|table` override for flexible output formats
- [x] Add `--no-color` flag to disable colored output

---

## 🎨 v1.5 – Color & Theme

- [ ] Color code files by extension
- [ ] Theme support via `~/.config/bestls/config.toml`
- [ ] Add `--no-color` flag to disable colors

---

## 🔧 v1.6 – Advanced Configuration

- [ ] Load default settings from config file (`TOML`/`JSON`)
- [ ] Support default sorting, columns, and color scheme via config
- [ ] Add `--config <path>` to specify custom config file location
- [ ] Environment variable support for common options

---

## 🧪 Internal Improvements

- [ ] Add integration tests (`assert_cmd`, `tempdir`)
- [ ] Write unit tests for sorting and formatting logic
- [ ] Add benchmarks with `criterion`

---

## 🌐 Community & Ecosystem

- [ ] Add `CONTRIBUTING.md` guidelines
- [ ] Enable GitHub Discussions & issue templates
- [ ] Publish package to AUR (PKGBUILD)
- [ ] Improve `crates.io` metadata
- [ ] Maintain `CHANGELOG.md`

---

## 🧱 v2.0 – Plugin Architecture

- [ ] Design extensible plugin system
- [ ] Support column and filter plugins
- [ ] Enable dynamic `.so` / `.dll` modules or subcommand extensions

---

## 📦 Distribution

- [ ] Add precompiled binaries to GitHub releases
- [ ] Create a Homebrew formula for macOS users
- [ ] Provide Snap and AppImage support for Linux distributions
