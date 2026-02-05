//! # Table Formatting and Display Module
//!
//! This module handles the formatting and colorized display of file listings in table format.
//! It uses the `tabled` crate to create well-formatted, colored tables that are easy to read
//! and visually appealing.
//!
//! ## Key Components
//!
//! - [`DisplayEntry`]: Internal struct for table formatting (derived from [`FileEntry`])
//! - [`format_table`]: Main function for rendering and formatting tables as strings
//!
//! ## Features
//!
//! * **Colorized Output**: Different colors for columns (names, sizes, dates, headers)
//! * **Rounded Borders**: Uses rounded table style for modern appearance
//! * **Custom Column Names**: User-friendly column headers
//! * **Flexible Formatting**: Automatically adjusts to terminal width
//!
//! ## Color Scheme
//!
//! The table uses a carefully chosen color scheme for optimal readability:
//!
//! - **Names (Column 1)**: Bright Cyan - Makes filenames stand out
//! - **Sizes (Column 3)**: Bright Magenta - Highlights file sizes
//! - **Dates (Column 4)**: Bright Yellow - Makes modification times visible
//! - **Headers (First Row)**: Bright Green - Clearly separates column headers
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! use bestls::table::format_table;
//! use bestls::fsops::{get_files, FileEntry};
//! use std::path::Path;
//!
//! let path = Path::new(".");
//! let files = get_files(&path, false)?;
//!
//! // Format and print table
//! let output = format_table(&files, None, false, true);
//! println!("{}", output);
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ### Sample Output
//!
//! ```text
//! ╭────────────┬───────────┬────────┬─────────────────────────┬─────────────┬───────┬───────╮
//! │ Name       │ Type      │ Size   │ Modified                │ Permissions │ Owner │ Group │
//! ├────────────┼───────────┼────────┼─────────────────────────┼─────────────┼───────┼───────┤
//! │ Cargo.toml │ File      │ 1.1 KB │ Thu 22 Aug 2024 17:44:23│ rw-r--r--   │ user  │ staff │
//! │ src        │ Directory │ 128 B  │ Thu 22 Aug 2024 17:44:23│ rwxr-xr-x   │ user  │ staff │
//! │ README.md  │ File      │ 4.8 KB │ Thu 22 Aug 2024 17:44:23│ rw-r--r--   │ user  │ staff │
//! ╰────────────┴───────────┴────────┴─────────────────────────┴─────────────┴───────┴───────╯
//! ```
//!
//! ## Design Choices
//!
//! ### Internal DisplayEntry Struct
//!
//! The module uses an internal [`DisplayEntry`] struct rather than implementing [`Tabled`]
//! directly on [`FileEntry`] for several reasons:
//!
//! 1. **Separation of Concerns**: Keeps formatting logic separate from data structures
//! 2. **Customization**: Allows custom column names and ordering without affecting core data
//! 3. **Future Flexibility**: Easy to modify display without changing serialization
//!
//! ### Color Selection
//!
//! Colors were chosen to:
//! - Provide good contrast on both light and dark terminals
//! - Help users quickly identify different types of information
//! - Maintain professional appearance while being visually helpful

use crate::color::Theme;
use crate::fsops::FileEntry;
use std::collections::HashSet;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::Style;
use tabled::{Table, Tabled};

/// Internal representation of a file entry optimized for table display.
///
/// This struct is derived from [`FileEntry`] and is specifically designed for use with
/// the `tabled` crate. It provides custom column names and ordering for optimal
/// table presentation.
///
/// # Fields
///
/// All fields are strings optimized for display:
///
/// * `name` - Filename (displayed as "Name")
/// * `e_type` - File type as string (displayed as "Type")
/// * `human_size` - Human-readable size (displayed as "Size")
/// * `modified` - Formatted modification time (displayed as "Modified")
/// * `permissions` - Permission string (displayed as "Permissions")
/// * `owner` - Owner name (displayed as "Owner")
/// * `group` - Group name (displayed as "Group")
///
/// # Design
///
/// This struct is intentionally kept private as it's an implementation detail.
/// External code should use [`print_table`] rather than creating [`DisplayEntry`] instances directly.
///
/// # Attributes
///
/// Uses `tabled(rename = "...")` attributes to provide user-friendly column headers
/// that are more readable than the field names.
#[derive(Tabled)]
struct DisplayEntry {
    /// Filename or directory name
    #[tabled(rename = "Name")]
    name: String,
    /// File type (File, Directory, or Symlink)
    #[tabled(rename = "Type")]
    e_type: String,
    /// Human-readable file size
    #[tabled(rename = "Size")]
    human_size: String,
    /// Formatted modification date and time
    #[tabled(rename = "Modified")]
    modified: String,
    /// File permissions string
    #[tabled(rename = "Permissions")]
    permissions: String,
    /// File owner name
    #[tabled(rename = "Owner")]
    owner: String,
    /// File group name
    #[tabled(rename = "Group")]
    group: String,
}

/// Display a collection of file entries as a colorized, formatted table.
///
/// This function takes a vector of [`FileEntry`] structs and renders them as a beautiful,
/// colorized table with rounded borders. It's the primary output method for bestls when
/// not using JSON format.
///
/// # Arguments
///
/// * `entries` - Vector of file entries to display
///
/// # Output Format
///
/// The table includes the following columns:
///
/// 1. **Name** (Bright Cyan) - Filename or directory name
/// 2. **Type** - File type (File, Directory, Symlink)
/// 3. **Size** (Bright Magenta) - Human-readable file size
/// 4. **Modified** (Bright Yellow) - Modification date and time
/// 5. **Permissions** - File permissions string
/// 6. **Owner** - File owner name
/// 7. **Group** - File group name
///
/// # Styling
///
/// * **Borders**: Rounded style with Unicode box-drawing characters
/// * **Headers**: Bright green for clear separation
/// * **Colors**: Carefully chosen for readability on various terminal themes
///
/// # Examples
///
/// ## Basic Table Display
///
/// ```rust
/// use bestls::table::print_table;
/// use bestls::fsops::{get_files, FileEntry, FileType};
/// use std::path::Path;
///
/// // Get files from current directory
/// let path = Path::new(".");
/// let files = get_files(&path, false)?;
///
/// // Display as formatted table
/// print_table(files);
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// ## With Custom File Entries
///
/// ```rust
/// use bestls::table::print_table;
/// use bestls::fsops::{FileEntry, FileType};
///
/// let entries = vec![
///     FileEntry {
///         name: "document.txt".to_string(),
///         e_type: FileType::File,
///         len_bytes: 1024,
///         human_size: "1.0 KB".to_string(),
///         modified: "Thu 22 Aug 2024 14:30:25".to_string(),
///         permissions: "rw-r--r--".to_string(),
///         owner: "user".to_string(),
///         group: "staff".to_string(),
///     }
/// ];
///
/// print_table(entries);
/// ```
///
/// # Performance
///
/// This function is designed for interactive use and prioritizes readability over performance.
/// For large numbers of files (thousands), consider using JSON output instead for better
/// performance and programmatic processing.
///
/// # Terminal Compatibility
///
/// The table uses Unicode box-drawing characters which are supported by most modern terminals.
/// Colors use ANSI escape codes that work with virtually all terminal emulators.
///
/// # Empty Input
///
/// If provided with an empty vector, the function will display an empty table with just headers:
///
/// ```text
/// ╭──────┬──────┬──────┬──────────┬─────────────┬───────┬───────╮
/// │ Name │ Type │ Size │ Modified │ Permissions │ Owner │ Group │
/// ╰──────┴──────┴──────┴──────────┴─────────────┴───────┴───────╯
/// ```
/// Parse column names from comma-separated string.
///
/// # Arguments
/// * `cols` - Comma-separated column specification (e.g., "name,size,date")
///
/// # Returns
/// A HashSet of valid column names
///
/// # Note
/// This function is reserved for future column filtering implementation.
#[allow(dead_code)]
pub fn parse_columns(cols: &str) -> HashSet<String> {
    cols.split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| {
            matches!(
                s.as_str(),
                "name" | "type" | "size" | "date" | "permissions" | "owner" | "group"
            )
        })
        .collect()
}

/// Format compact output as string (internal helper)
fn format_compact_inner(entries: &[FileEntry]) -> String {
    entries
        .iter()
        .map(|f| f.name.clone())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format compact output as string
///
/// Deprecated: Use `format_table(..., compact = true, ...)` instead
#[allow(dead_code)]
#[deprecated(
    since = "1.3.0",
    note = "use format_table(..., compact = true, ...) instead"
)]
pub fn format_compact(entries: &[FileEntry]) -> String {
    format_compact_inner(entries)
}

/// Format table output as a string
///
/// # Arguments
/// * `entries` - Vector of file entries to format
/// * `columns` - Optional column selection (reserved for future use)
/// * `compact` - If true, return single-column format
/// * `use_color` - If true, apply color styling
/// * `theme` - Optional theme for colors (uses default if None)
pub fn format_table(
    entries: &[FileEntry],
    columns: Option<String>,
    compact: bool,
    use_color: bool,
    theme: Option<&Theme>,
) -> String {
    if compact {
        return format_compact_inner(entries);
    }

    // Column selection reserved for future implementation; not currently wired up
    let _ = columns;

    let data: Vec<DisplayEntry> = entries
        .iter()
        .map(|e: &FileEntry| DisplayEntry {
            name: e.name.clone(),
            e_type: e.e_type.to_string(),
            human_size: e.human_size.clone(),
            modified: e.modified.clone(),
            permissions: e.permissions.clone(),
            owner: e.owner.clone(),
            group: e.group.clone(),
        })
        .collect();

    let mut table: Table = Table::new(data);
    table.with(Style::rounded());

    if use_color {
        // Use provided theme or create a default one with longer lifetime
        let default_theme;
        let active_theme = match theme {
            Some(t) => t,
            None => {
                default_theme = Theme::default();
                &default_theme
            }
        };

        table.modify(Columns::first(), active_theme.table.name.to_tabled_color());
        table.modify(Columns::one(2), active_theme.table.size.to_tabled_color());
        table.modify(Columns::one(3), active_theme.table.date.to_tabled_color());
        table.modify(Rows::first(), active_theme.table.header.to_tabled_color());
    }

    table.to_string()
}

/// Print table output directly to stdout
///
/// Deprecated: Use `format_table` and print the result for better testability and flexibility
#[allow(dead_code)]
#[deprecated(
    since = "1.3.0",
    note = "use format_table() to get the string and print it yourself"
)]
pub fn print_table(
    entries: Vec<FileEntry>,
    columns: Option<String>,
    compact: bool,
    use_color: bool,
) {
    println!(
        "{}",
        format_table(&entries, columns, compact, use_color, None)
    );
}
