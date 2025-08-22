# ✅ Roadmap for `bestls`

A structured and professional development plan for future enhancements to the `bestls` CLI tool.

---

## 🔁 v1.1 – Hidden Files & Permissions

- [x] Add `--all` / `-a` flag to display hidden files
- [x] Display Unix-style file permissions
- [x] Show file owner and group information (via `users` crate)
- [x] Refactor table output to include new columns

---

## 🌲 v1.2 – Tree View & Filtering

- [ ] Add `--tree` flag for recursive directory listing
- [ ] Implement depth limit with `--depth <n>`
- [ ] Add filtering options:
  - `--filter-ext <ext>`
  - `--filter-name <pattern>`
- [ ] Add size-based filters with human-readable units:
  - `--min-size <size>`
  - `--max-size <size>`

---

## 🧹 v1.3 – Output Customization

- [ ] Add `--compact` for minimal single-column output (like `ls -1`)
- [ ] Add `--columns name,size,date,...` to customize visible columns
- [ ] Add `--out <file>` option to export output
- [ ] Add `--format json|table` override for flexible output formats

---

## 🎨 v1.4 – Color & Theme

- [ ] Color code files by extension
- [ ] Theme support via `~/.config/bestls/config.toml`
- [ ] Add `--no-color` flag to disable colors

---

## ⚙️ v1.5 – Shell Completions & Config

- [ ] Add shell completions (via `clap_complete`)
- [ ] Support `--generate-completions` command
- [ ] Load default settings from config file (`TOML`/`JSON`)
- [ ] Support default sorting, columns, and color scheme via config

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
