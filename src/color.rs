//! # Color & Theme Module
//!
//! This module handles color theming for bestls, supporting:
//! - File extension-based coloring
//! - Customizable theme configuration via `~/.config/bestls/config.toml`
//! - Default theme with built-in color mappings
//!
//! ## Configuration
//!
//! Users can customize colors by creating `~/.config/bestls/config.toml`:
//!
//! ```toml
//! [colors]
//! # File type colors
//! file = "bright_cyan"
//! directory = "bright_blue"
//! symlink = "bright_magenta"
//!
//! # Extension-based colors (optional)
//! [colors.extensions]
//! rs = "yellow"
//! toml = "red"
//! json = "green"
//! md = "cyan"
//! ```
//!
//! ## Supported Colors
//!
//! - `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
//! - `bright_black`, `bright_red`, `bright_green`, `bright_yellow`
//! - `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`

use crate::fsops::FileType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tabled::settings::Color;

/// Represents ANSI color codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorValue {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl ColorValue {
    /// Convert to tabled::settings::Color
    pub fn to_tabled_color(self) -> Color {
        match self {
            ColorValue::Black => Color::FG_BLACK,
            ColorValue::Red => Color::FG_RED,
            ColorValue::Green => Color::FG_GREEN,
            ColorValue::Yellow => Color::FG_YELLOW,
            ColorValue::Blue => Color::FG_BLUE,
            ColorValue::Magenta => Color::FG_MAGENTA,
            ColorValue::Cyan => Color::FG_CYAN,
            ColorValue::White => Color::FG_WHITE,
            ColorValue::BrightBlack => Color::FG_BRIGHT_BLACK,
            ColorValue::BrightRed => Color::FG_BRIGHT_RED,
            ColorValue::BrightGreen => Color::FG_BRIGHT_GREEN,
            ColorValue::BrightYellow => Color::FG_BRIGHT_YELLOW,
            ColorValue::BrightBlue => Color::FG_BRIGHT_BLUE,
            ColorValue::BrightMagenta => Color::FG_BRIGHT_MAGENTA,
            ColorValue::BrightCyan => Color::FG_BRIGHT_CYAN,
            ColorValue::BrightWhite => Color::FG_BRIGHT_WHITE,
        }
    }

    /// Parse from string (e.g., "bright_cyan")
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "black" => Some(ColorValue::Black),
            "red" => Some(ColorValue::Red),
            "green" => Some(ColorValue::Green),
            "yellow" => Some(ColorValue::Yellow),
            "blue" => Some(ColorValue::Blue),
            "magenta" => Some(ColorValue::Magenta),
            "cyan" => Some(ColorValue::Cyan),
            "white" => Some(ColorValue::White),
            "bright_black" => Some(ColorValue::BrightBlack),
            "bright_red" => Some(ColorValue::BrightRed),
            "bright_green" => Some(ColorValue::BrightGreen),
            "bright_yellow" => Some(ColorValue::BrightYellow),
            "bright_blue" => Some(ColorValue::BrightBlue),
            "bright_magenta" => Some(ColorValue::BrightMagenta),
            "bright_cyan" => Some(ColorValue::BrightCyan),
            "bright_white" => Some(ColorValue::BrightWhite),
            _ => None,
        }
    }
}

impl std::fmt::Display for ColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ColorValue::Black => "black",
            ColorValue::Red => "red",
            ColorValue::Green => "green",
            ColorValue::Yellow => "yellow",
            ColorValue::Blue => "blue",
            ColorValue::Magenta => "magenta",
            ColorValue::Cyan => "cyan",
            ColorValue::White => "white",
            ColorValue::BrightBlack => "bright_black",
            ColorValue::BrightRed => "bright_red",
            ColorValue::BrightGreen => "bright_green",
            ColorValue::BrightYellow => "bright_yellow",
            ColorValue::BrightBlue => "bright_blue",
            ColorValue::BrightMagenta => "bright_magenta",
            ColorValue::BrightCyan => "bright_cyan",
            ColorValue::BrightWhite => "bright_white",
        };
        write!(f, "{}", s)
    }
}

/// File type color mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FileTypeColors {
    pub file: ColorValue,
    pub directory: ColorValue,
    pub symlink: ColorValue,
}

impl Default for FileTypeColors {
    fn default() -> Self {
        Self {
            file: ColorValue::BrightCyan,
            directory: ColorValue::BrightBlue,
            symlink: ColorValue::BrightMagenta,
        }
    }
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    /// File type colors
    pub file_types: FileTypeColors,
    /// Extension-based colors (e.g., "rs" -> "yellow")
    pub extensions: HashMap<String, ColorValue>,
    /// Table column colors
    pub table: TableColors,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            file_types: FileTypeColors::default(),
            extensions: default_extension_colors(),
            table: TableColors::default(),
        }
    }
}

/// Table column color settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TableColors {
    pub name: ColorValue,
    pub size: ColorValue,
    pub date: ColorValue,
    pub header: ColorValue,
}

impl Default for TableColors {
    fn default() -> Self {
        Self {
            name: ColorValue::BrightCyan,
            size: ColorValue::BrightMagenta,
            date: ColorValue::BrightYellow,
            header: ColorValue::BrightGreen,
        }
    }
}

/// Get default extension color mapping
fn default_extension_colors() -> HashMap<String, ColorValue> {
    [
        // Programming languages
        ("rs", ColorValue::Yellow),      // Rust
        ("py", ColorValue::Blue),        // Python
        ("js", ColorValue::Yellow),      // JavaScript
        ("ts", ColorValue::Blue),        // TypeScript
        ("go", ColorValue::BrightCyan),  // Go
        ("c", ColorValue::BrightBlue),   // C
        ("cpp", ColorValue::BrightBlue), // C++
        ("java", ColorValue::Red),       // Java
        // Documents
        ("md", ColorValue::Cyan),   // Markdown
        ("txt", ColorValue::White), // Text
        ("pdf", ColorValue::Red),   // PDF
        // Configuration
        ("toml", ColorValue::Red),     // TOML
        ("json", ColorValue::Green),   // JSON
        ("yaml", ColorValue::Magenta), // YAML
        ("yml", ColorValue::Magenta),  // YAML
        ("xml", ColorValue::Yellow),   // XML
        // Archives
        ("zip", ColorValue::Red), // ZIP
        ("tar", ColorValue::Red), // TAR
        ("gz", ColorValue::Red),  // GZIP
        // Images
        ("png", ColorValue::Magenta),  // PNG
        ("jpg", ColorValue::Magenta),  // JPEG
        ("jpeg", ColorValue::Magenta), // JPEG
        ("gif", ColorValue::Magenta),  // GIF
        ("svg", ColorValue::Yellow),   // SVG
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), *v))
    .collect()
}

/// Load theme from config file or use default
pub fn load_theme() -> Theme {
    if let Ok(theme) = load_theme_from_config() {
        return theme;
    }
    Theme::default()
}

/// Try to load theme from config file
fn load_theme_from_config() -> Result<Theme, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("bestls");

    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        return Err("Config file not found".into());
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: ThemeConfig = toml::from_str(&content)?;

    Ok(config.into_theme())
}

/// Configuration file structure
#[derive(Debug, Serialize, Deserialize)]
struct ThemeConfig {
    #[serde(default)]
    colors: ColorConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ColorConfig {
    #[serde(default)]
    file_types: Option<FileTypeColors>,
    #[serde(default)]
    extensions: Option<HashMap<String, String>>,
    #[serde(default)]
    table: Option<TableColors>,
}

impl ThemeConfig {
    fn into_theme(self) -> Theme {
        let mut theme = Theme::default();

        if let Some(ft) = self.colors.file_types {
            theme.file_types = ft;
        }

        if let Some(exts) = self.colors.extensions {
            for (ext, color_str) in exts {
                if let Some(color) = ColorValue::from_str(&color_str) {
                    theme.extensions.insert(ext, color);
                }
            }
        }

        if let Some(tc) = self.colors.table {
            theme.table = tc;
        }

        theme
    }
}

/// Get color for a file based on type and extension
#[allow(dead_code)]
pub fn get_file_color(file_type: &FileType, filename: &str, theme: &Theme) -> ColorValue {
    match file_type {
        FileType::File => {
            // Check extension-based coloring first
            if let Some(pos) = filename.rfind('.') {
                let ext = &filename[pos + 1..].to_lowercase();
                if let Some(color) = theme.extensions.get(ext) {
                    return *color;
                }
            }
            // Fall back to default file color
            theme.file_types.file
        }
        FileType::Directory => theme.file_types.directory,
        FileType::Symlink => theme.file_types.symlink,
    }
}

/// Create a sample config file for the user
#[allow(dead_code)]
pub fn create_sample_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("bestls");

    std::fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        let sample_config = r#"# bestls Configuration File
# Location: ~/.config/bestls/config.toml

[colors]
# File type colors
file = "bright_cyan"
directory = "bright_blue"
symlink = "bright_magenta"

[colors.table]
# Table column colors
name = "bright_cyan"
size = "bright_magenta"
date = "bright_yellow"
header = "bright_green"

[colors.extensions]
# Extension-based file colors (case-insensitive)
rs = "yellow"
py = "blue"
js = "yellow"
ts = "blue"
go = "bright_cyan"
md = "cyan"
json = "green"
toml = "red"
yaml = "magenta"
yml = "magenta"
"#;

        std::fs::write(&config_path, sample_config)?;
    }

    Ok(config_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_value_from_str() {
        assert_eq!(
            ColorValue::from_str("bright_cyan"),
            Some(ColorValue::BrightCyan)
        );
        assert_eq!(ColorValue::from_str("red"), Some(ColorValue::Red));
        assert_eq!(ColorValue::from_str("invalid"), None);
    }

    #[test]
    fn test_get_file_color() {
        let theme = Theme::default();

        // Test extension-based coloring
        let color = get_file_color(&FileType::File, "test.rs", &theme);
        assert_eq!(color, ColorValue::Yellow);

        // Test default file color
        let color = get_file_color(&FileType::File, "test.unknown", &theme);
        assert_eq!(color, theme.file_types.file);

        // Test directory color
        let color = get_file_color(&FileType::Directory, "src", &theme);
        assert_eq!(color, theme.file_types.directory);
    }

    #[test]
    fn test_default_extension_colors() {
        let colors = default_extension_colors();
        assert!(colors.contains_key("rs"));
        assert_eq!(colors.get("rs"), Some(&ColorValue::Yellow));
        assert!(colors.contains_key("py"));
    }
}
