# Color & Theme Configuration Guide

`bestls` supports customizable color themes to match your terminal preferences and coding style.

## Quick Start

### 1. Generate a Configuration File

```bash
bestls theme init --show
```

This creates `~/.config/bestls/config.toml` with sensible defaults and displays the contents.

### 2. Customize Your Theme

Edit `~/.config/bestls/config.toml` to adjust colors:

```toml
[colors]
file = "bright_cyan"
directory = "bright_blue"
symlink = "bright_magenta"

[colors.table]
name = "bright_cyan"
size = "bright_magenta"
date = "bright_yellow"
header = "bright_green"

[colors.extensions]
rs = "yellow"
py = "blue"
md = "cyan"
```

### 3. Reset to Default

```bash
bestls theme reset
```

## Configuration File Location

The configuration file is located at:
- **Linux/macOS**: `~/.config/bestls/config.toml`
- **Windows**: `%APPDATA%\bestls\config.toml`

To find your config file path, run:

```bash
bestls theme path
```

## Supported Colors

### Basic Colors
- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`

### Bright Colors
- `bright_black`
- `bright_red`
- `bright_green`
- `bright_yellow`
- `bright_blue`
- `bright_magenta`
- `bright_cyan`
- `bright_white`

## Configuration Sections

### `[colors]` Section

File type colors:

```toml
[colors]
file = "bright_cyan"        # Regular file color
directory = "bright_blue"   # Directory color
symlink = "bright_magenta"  # Symbolic link color
```

### `[colors.table]` Section

Table output column colors:

```toml
[colors.table]
name = "bright_cyan"     # Filename column
size = "bright_magenta"  # Size column
date = "bright_yellow"   # Modified date column
header = "bright_green"  # Header row
```

### `[colors.extensions]` Section

Color specific file extensions:

```toml
[colors.extensions]
rs = "yellow"
py = "blue"
js = "yellow"
ts = "blue"
go = "bright_cyan"
md = "cyan"
json = "green"
toml = "red"
yaml = "magenta"
```

**Supported Extensions** (with defaults):

| Category | Extensions |
|----------|-----------|
| **Languages** | `rs` (yellow), `py` (blue), `js` (yellow), `ts` (blue), `go` (bright_cyan), `c` (bright_blue), `cpp` (bright_blue), `java` (red) |
| **Documents** | `md` (cyan), `txt` (white), `pdf` (red) |
| **Config** | `toml` (red), `json` (green), `yaml`/`yml` (magenta), `xml` (yellow) |
| **Archives** | `zip`, `tar`, `gz` (red) |
| **Images** | `png`, `jpg`, `jpeg`, `gif` (magenta), `svg` (yellow) |

## Examples

### Dark Terminal Theme

```toml
[colors]
file = "cyan"
directory = "bright_blue"
symlink = "bright_magenta"

[colors.table]
name = "cyan"
size = "bright_magenta"
date = "yellow"
header = "bright_green"

[colors.extensions]
rs = "bright_yellow"
py = "bright_blue"
md = "bright_cyan"
```

### Light Terminal Theme

```toml
[colors]
file = "blue"
directory = "bright_blue"
symlink = "magenta"

[colors.table]
name = "blue"
size = "magenta"
date = "yellow"
header = "green"

[colors.extensions]
rs = "yellow"
py = "blue"
md = "cyan"
```

### Monochrome Theme

```toml
[colors]
file = "white"
directory = "white"
symlink = "white"

[colors.table]
name = "white"
size = "white"
date = "white"
header = "white"

[colors.extensions]
# No extension-specific colors
```

## Command-Line Options

### Disable Colors Globally

```bash
bestls --no-color
```

This overrides the theme configuration and displays output without colors.

## Default Color Scheme

If no configuration file exists, `bestls` uses these defaults:

- **Files**: Bright Cyan
- **Directories**: Bright Blue
- **Symlinks**: Bright Magenta
- **Table Names**: Bright Cyan
- **Table Sizes**: Bright Magenta
- **Table Dates**: Bright Yellow
- **Table Headers**: Bright Green

## Troubleshooting

### Colors Look Wrong

1. Verify your terminal supports 256 colors or true color
2. Check that the config file syntax is valid TOML
3. Ensure color names are lowercase (e.g., `bright_cyan` not `BrightCyan`)

### Config Not Being Applied

1. Run `bestls theme path` to verify config location
2. Ensure the file is in the correct directory
3. Check file permissions: `ls -la ~/.config/bestls/config.toml`

### Reset to Defaults

```bash
bestls theme reset
```

This removes the config file and restores default colors.

## Performance

Color configuration has negligible performance impact. Themes are loaded once at startup.

## See Also

- [README.md](../README.md) - Overview of bestls
- [EXAMPLES.md](./EXAMPLES.md) - Usage examples
