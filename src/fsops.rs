//! # File System Operations Module
//!
//! This module handles all file system operations and data structures for **bestls**.
//! It provides comprehensive file metadata extraction with cross-platform support
//! and parallel processing capabilities using `rayon`.
//!
//! ## Key Components
//!
//! - [`FileEntry`]: Core data structure representing a file or directory with complete metadata
//! - [`FileType`]: Enumeration of supported file system entry types
//! - [`get_files`]: Main function for retrieving and processing directory contents
//!
//! ## Features
//!
//! * **Parallel Processing**: Uses `rayon` for concurrent metadata fetching
//! * **Cross-Platform Support**: Handles Unix permissions with graceful Windows/other platform fallbacks
//! * **Rich Metadata**: Extracts size, permissions, ownership, and modification times
//! * **Human-Readable Output**: Formats file sizes and dates for easy reading
//!
//! ## Platform-Specific Behavior
//!
//! ### Unix Systems (Linux, macOS, etc.)
//! - Full permission handling (rwxrwxrwx format)
//! - User and group name resolution via `nix` crate
//! - Complete file metadata extraction
//!
//! ### Windows Systems
//! - Basic readonly/readwrite permissions
//! - Placeholder "Owner"/"Group" values
//! - Standard file metadata
//!
//! ### Other Platforms
//! - "N/A" fallbacks for unsupported features
//! - Basic file information only
//!
//! ## Examples
//!
//! ### Basic Directory Listing
//!
//! ```rust
//! use std::path::Path;
//! use bestls::fsops::get_files;
//!
//! let path = Path::new(".");
//! let include_hidden = false;
//!
//! match get_files(&path, include_hidden) {
//!     Ok(files) => {
//!         for file in files {
//!             println!("{}: {} ({})", file.name, file.human_size, file.e_type);
//!         }
//!     }
//!     Err(e) => eprintln!("Error reading directory: {}", e),
//! }
//! ```
//!
//! ### Including Hidden Files
//!
//! ```rust
//! use std::path::Path;
//! use bestls::fsops::get_files;
//!
//! let path = Path::new("/home/user");
//! let include_hidden = true; // Include files starting with '.'
//!
//! let files = get_files(&path, include_hidden)?;
//! println!("Found {} files (including hidden)", files.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::Serialize;
use std::{fs, io, path::Path};
use strum::Display;

#[cfg(unix)]
use nix::unistd::{Group, User};
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};

/// Enumeration of file system entry types supported by bestls.
///
/// This enum represents the different types of file system entries that can be encountered
/// during directory traversal. It derives several traits for display and serialization.
///
/// # Variants
///
/// * `File` - Regular file
/// * `Directory` - Directory/folder
/// * `Symlink` - Symbolic link
///
/// # Traits
///
/// - `Debug`: For debugging output
/// - `Display`: For human-readable string representation via `strum`
/// - `Serialize`: For JSON output via `serde`
/// - `Clone`: For efficient copying
///
/// # Examples
///
/// ```rust
/// use bestls::fsops::FileType;
///
/// let file_type = FileType::File;
/// println!("Entry type: {}", file_type); // Prints: "Entry type: File"
///
/// // Use in match expressions
/// match file_type {
///     FileType::File => println!("This is a regular file"),
///     FileType::Directory => println!("This is a directory"),
///     FileType::Symlink => println!("This is a symbolic link"),
/// }
/// ```
///
/// # JSON Serialization
///
/// When serialized to JSON, the variants become strings:
///
/// ```json
/// {
///   "e_type": "File"
/// }
/// ```
#[derive(Debug, Display, Serialize, Clone)]
pub enum FileType {
    /// Regular file
    File,
    /// Directory or folder
    Directory,
    /// Symbolic link
    Symlink,
}

/// Comprehensive file system entry representation with rich metadata.
///
/// This struct contains all the metadata for a file system entry that bestls can extract.
/// It's designed to be serializable to JSON and provides human-readable formatting for
/// sizes and dates.
///
/// # Fields
///
/// * `name` - The filename or directory name (without path)
/// * `e_type` - The type of entry (File, Directory, or Symlink)
/// * `len_bytes` - Raw file size in bytes (for sorting and calculations)
/// * `human_size` - Human-readable size string (e.g., "1.5 KB", "2.1 MB")
/// * `modified` - Formatted modification date and time
/// * `permissions` - File permissions string (Unix: "rwxrwxrwx", Windows: "rw-" or "r--")
/// * `owner` - File owner name (Unix: resolved username, Windows: "Owner", other: "N/A")
/// * `group` - File group name (Unix: resolved group name, Windows: "Group", other: "N/A")
///
/// # Platform Differences
///
/// ## Unix Systems (Linux, macOS, etc.)
/// ```text
/// FileEntry {
///     name: "document.txt",
///     e_type: File,
///     len_bytes: 1024,
///     human_size: "1.0 KB",
///     modified: "Mon 15 Jan 2024 14:30:25",
///     permissions: "rw-r--r--",
///     owner: "username",
///     group: "users"
/// }
/// ```
///
/// ## Windows Systems
/// ```text
/// FileEntry {
///     name: "document.txt",
///     e_type: File,
///     len_bytes: 1024,
///     human_size: "1.0 KB",
///     modified: "Mon 15 Jan 2024 14:30:25",
///     permissions: "rw-",
///     owner: "Owner",
///     group: "Group"
/// }
/// ```
///
/// # Examples
///
/// ## Creating and Using FileEntry
///
/// ```rust
/// use bestls::fsops::{FileEntry, FileType};
/// use serde_json;
///
/// let entry = FileEntry {
///     name: "example.txt".to_string(),
///     e_type: FileType::File,
///     len_bytes: 2048,
///     human_size: "2.0 KB".to_string(),
///     modified: "Mon 15 Jan 2024 14:30:25".to_string(),
///     permissions: "rw-r--r--".to_string(),
///     owner: "user".to_string(),
///     group: "staff".to_string(),
/// };
///
/// // Serialize to JSON
/// let json = serde_json::to_string_pretty(&entry)?;
/// println!("{}", json);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Sorting FileEntry Collections
///
/// ```rust
/// use bestls::fsops::FileEntry;
///
/// fn sort_by_size(mut files: Vec<FileEntry>) -> Vec<FileEntry> {
///     files.sort_by(|a, b| a.len_bytes.cmp(&b.len_bytes));
///     files
/// }
///
/// fn sort_by_name(mut files: Vec<FileEntry>) -> Vec<FileEntry> {
///     files.sort_by(|a, b| a.name.cmp(&b.name));
///     files
/// }
/// ```
///
/// # JSON Output Format
///
/// When serialized to JSON, FileEntry produces well-structured output:
///
/// ```json
/// {
///   "name": "example.txt",
///   "e_type": "File",
///   "len_bytes": 2048,
///   "human_size": "2.0 KB",
///   "modified": "Mon 15 Jan 2024 14:30:25",
///   "permissions": "rw-r--r--",
///   "owner": "user",
///   "group": "staff"
/// }
/// ```
#[derive(Debug, Serialize, Clone)]
pub struct FileEntry {
    /// The filename or directory name (without path components)
    pub name: String,
    /// The type of file system entry (File, Directory, or Symlink)
    pub e_type: FileType,
    /// Raw file size in bytes (used for sorting and calculations)
    pub len_bytes: u64,
    /// Human-readable file size (e.g., "1.5 KB", "2.1 MB", "1.2 GB")
    pub human_size: String,
    /// Formatted modification date and time string
    pub modified: String,
    /// File permissions string (format varies by platform)
    pub permissions: String,
    /// File owner name (platform-dependent format)
    pub owner: String,
    /// File group name (platform-dependent format)
    pub group: String,
}

/// Retrieve and process all files in a directory with optional hidden file inclusion.
///
/// This is the main entry point for file system operations in bestls. It reads a directory,
/// optionally filters out hidden files, and processes all entries in parallel using `rayon`
/// to extract comprehensive metadata.
///
/// # Arguments
///
/// * `path` - The directory path to read
/// * `include_hidden` - Whether to include files starting with '.' (Unix hidden files)
///
/// # Returns
///
/// * `Ok(Vec<FileEntry>)` - Vector of file entries with complete metadata
/// * `Err(io::Error)` - I/O error if directory cannot be read
///
/// # Performance
///
/// This function uses parallel processing via `rayon` to extract metadata from multiple files
/// concurrently. This provides significant performance benefits for directories with many files,
/// especially when accessing network filesystems or slow storage devices.
///
/// # Examples
///
/// ## Basic Directory Listing
///
/// ```rust
/// use std::path::Path;
/// use bestls::fsops::get_files;
///
/// let current_dir = Path::new(".");
/// let files = get_files(&current_dir, false)?;
///
/// for file in files {
///     println!("{}: {}", file.name, file.human_size);
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// ## Including Hidden Files
///
/// ```rust
/// use std::path::Path;
/// use bestls::fsops::get_files;
///
/// let home_dir = Path::new("/home/user");
/// let all_files = get_files(&home_dir, true)?; // Include .bashrc, .profile, etc.
///
/// let hidden_count = all_files.iter()
///     .filter(|f| f.name.starts_with('.'))
///     .count();
/// 
/// println!("Found {} hidden files", hidden_count);
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// ## Error Handling
///
/// ```rust
/// use std::path::Path;
/// use bestls::fsops::get_files;
///
/// let restricted_dir = Path::new("/root");
/// match get_files(&restricted_dir, false) {
///     Ok(files) => println!("Found {} files", files.len()),
///     Err(e) => eprintln!("Cannot access directory: {}", e),
/// }
/// ```
///
/// # Error Conditions
///
/// This function can return errors in several scenarios:
/// - Directory does not exist
/// - Permission denied to read directory
/// - Path points to a file rather than directory
/// - I/O errors during filesystem access
///
/// Individual file metadata extraction errors are silently ignored to allow partial
/// directory listings even when some files cannot be accessed.
pub fn get_files(path: &Path, include_hidden: bool) -> Result<Vec<FileEntry>, io::Error> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry: &fs::DirEntry| {
            include_hidden || !entry.file_name().to_string_lossy().starts_with('.')
        })
        .collect();

    let files: Vec<FileEntry> = entries
        .par_iter()
        .map(map_data)
        .filter_map(Result::ok)
        .collect();

    Ok(files)
}

/// Internal function to extract comprehensive metadata from a file system entry.
///
/// This function takes a `DirEntry` and extracts all available metadata, including
/// file type, size, permissions, ownership, and modification time. It handles
/// platform-specific differences gracefully using conditional compilation.
///
/// # Arguments
///
/// * `entry` - A directory entry from `fs::read_dir()`
///
/// # Returns
///
/// * `Ok(FileEntry)` - Complete file entry with all metadata
/// * `Err(io::Error)` - I/O error during metadata extraction
///
/// # Platform-Specific Processing
///
/// ## Unix Systems
/// - Extracts full rwxrwxrwx permissions using bit manipulation
/// - Resolves user and group names via `nix` crate
/// - Provides complete file system metadata
///
/// ## Windows Systems
/// - Simplifies permissions to "rw-" or "r--" based on readonly flag
/// - Uses placeholder "Owner" and "Group" strings
/// - Extracts standard file metadata
///
/// ## Other Platforms
/// - Falls back to "N/A" for unsupported features
/// - Provides basic file information
///
/// # Examples
///
/// This function is called internally by [`get_files`] and typically not used directly:
///
/// ```rust
/// // Internal usage within get_files()
/// let files: Vec<FileEntry> = entries
///     .par_iter()
///     .map(map_data)  // <- This function
///     .filter_map(Result::ok)
///     .collect();
/// ```
///
/// # Error Handling
///
/// Errors can occur during:
/// - Metadata extraction (`entry.metadata()`)
/// - File system access issues
/// - Platform-specific permission/ownership resolution
///
/// These errors are typically handled by the calling [`get_files`] function,
/// which filters out failed entries to provide partial results.
fn map_data(entry: &fs::DirEntry) -> Result<FileEntry, io::Error> {
    let metadata: fs::Metadata = entry.metadata()?;
    let file_type: fs::FileType = metadata.file_type();

    let modified: String = metadata
        .modified()
        .map(|m: std::time::SystemTime| {
            let dt: DateTime<Utc> = m.into();
            dt.format("%a %d %b %Y %H:%M:%S").to_string()
        })
        .unwrap_or_default();

    // Permissions
    #[cfg(unix)]
    let permissions: String = {
        let mode: u32 = metadata.permissions().mode();
        format!(
            "{}{}{}{}{}{}{}{}{}",
            if mode & 0o400 != 0 { 'r' } else { '-' },
            if mode & 0o200 != 0 { 'w' } else { '-' },
            if mode & 0o100 != 0 { 'x' } else { '-' },
            if mode & 0o040 != 0 { 'r' } else { '-' },
            if mode & 0o020 != 0 { 'w' } else { '-' },
            if mode & 0o010 != 0 { 'x' } else { '-' },
            if mode & 0o004 != 0 { 'r' } else { '-' },
            if mode & 0o002 != 0 { 'w' } else { '-' },
            if mode & 0o001 != 0 { 'x' } else { '-' },
        )
    };

    #[cfg(windows)]
    let permissions = if metadata.permissions().readonly() {
        "r--".into()
    } else {
        "rw-".into()
    };

    #[cfg(not(any(unix, windows)))]
    let permissions = "N/A".to_string();

    // Owner / Group - Using nix crate instead of users
    #[cfg(unix)]
    let (owner_name, group_name) = get_owner_group(&metadata);

    #[cfg(windows)]
    let (owner_name, group_name) = ("Owner".into(), "Group".into());

    #[cfg(not(any(unix, windows)))]
    let (owner_name, group_name) = ("N/A".into(), "N/A".into());

    Ok(FileEntry {
        name: entry.file_name().to_string_lossy().to_string(),
        e_type: if file_type.is_file() {
            FileType::File
        } else if file_type.is_dir() {
            FileType::Directory
        } else if file_type.is_symlink() {
            FileType::Symlink
        } else {
            FileType::File
        },
        len_bytes: metadata.len(),
        human_size: ByteSize(metadata.len()).to_string(),
        modified,
        permissions,
        owner: owner_name,
        group: group_name,
    })
}

/// Extract user and group names from file metadata on Unix systems.
///
/// This function is only compiled on Unix-like systems (Linux, macOS, etc.) and uses
/// the `nix` crate to resolve numeric user and group IDs to their corresponding names.
/// If name resolution fails, it falls back to displaying the numeric IDs.
///
/// # Arguments
///
/// * `metadata` - File metadata from which to extract ownership information
///
/// # Returns
///
/// A tuple containing:
/// * `String` - Username (or UID if name resolution fails)
/// * `String` - Group name (or GID if name resolution fails)
///
/// # Platform Support
///
/// This function is only available on Unix-like systems. It's conditionally compiled
/// using `#[cfg(unix)]` and will not be included in builds for Windows or other platforms.
///
/// # Examples
///
/// This function is used internally by [`map_data`]:
///
/// ```rust
/// // Internal usage (Unix only)
/// #[cfg(unix)]
/// let (owner_name, group_name) = get_owner_group(&metadata);
/// // Result might be ("alice", "developers") or ("1001", "1002") if names can't be resolved
/// ```
///
/// # Error Handling
///
/// The function gracefully handles cases where:
/// - User ID cannot be resolved to a username (returns numeric UID)
/// - Group ID cannot be resolved to a group name (returns numeric GID)
/// - System calls fail during name resolution (returns numeric IDs)
///
/// This ensures the function always returns valid strings, even in edge cases
/// like deleted users or system inconsistencies.
///
/// # Dependencies
///
/// Requires the `nix` crate for Unix system calls and the following metadata traits:
/// - `std::os::unix::fs::MetadataExt` for accessing `uid()` and `gid()`
/// - `nix::unistd::{User, Group, Uid, Gid}` for name resolution
#[cfg(unix)]
fn get_owner_group(metadata: &fs::Metadata) -> (String, String) {
    use nix::unistd::{Gid, Uid};

    let uid: Uid = Uid::from(metadata.uid());
    let gid: Gid = Gid::from(metadata.gid());

    let user: String = User::from_uid(uid)
        .ok()
        .flatten()
        .map(|u: User| u.name)
        .unwrap_or_else(|| uid.to_string());

    let group: String = Group::from_gid(gid)
        .ok()
        .flatten()
        .map(|g: Group| g.name)
        .unwrap_or_else(|| gid.to_string());

    (user, group)
}

/// Parse human-readable size strings (e.g., "1KB", "1.5MB", "100B")
/// 
/// # Examples
/// - "1KB" → 1024
/// - "1.5MB" → 1572864
/// - "100" → 100 (defaults to bytes)
/// - "invalid" → None (logs warning)
pub fn parse_size(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    if size_str.is_empty() {
        eprintln!("Warning: empty size string provided");
        return None;
    }
    
    let (num_part, unit) = if let Some(pos) = size_str.find(|c: char| c.is_alphabetic()) {
        (&size_str[..pos], &size_str[pos..])
    } else {
        (&size_str[..], "B")
    };
    
    let num: f64 = match num_part.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Warning: invalid size value '{}' (expected a number)", num_part.trim());
            return None;
        }
    };
    
    let multiplier = match unit {
        "B" => 1u64,
        "KB" | "K" => 1024u64,
        "MB" | "M" => 1024u64 * 1024u64,
        "GB" | "G" => 1024u64 * 1024u64 * 1024u64,
        "TB" | "T" => 1024u64 * 1024u64 * 1024u64 * 1024u64,
        _ => {
            eprintln!("Warning: unknown size unit '{}' (valid units: B, KB, MB, GB, TB)", unit);
            return None;
        }
    };
    
    Some((num * multiplier as f64) as u64)
}

/// Check if filename matches extension filter
pub fn matches_extension(filename: &str, extensions: &[String]) -> bool {
    if extensions.is_empty() {
        return true;
    }
    
    extensions.iter().any(|ext| {
        let ext = ext.trim_start_matches('.');
        filename.ends_with(&format!(".{}", ext))
    })
}

/// Glob-style pattern matching using standard glob semantics
/// 
/// Supports:
/// - `*` - matches any sequence of characters except path separators
/// - `?` - matches a single character
/// - `[abc]` - character classes
/// - `[!abc]` - negated character classes
///
/// # Examples
/// - `*.rs` - matches all .rs files
/// - `test_*` - matches files starting with test_
/// - `*_test.rs` - matches files ending with _test.rs
pub fn matches_pattern(filename: &str, pattern: &str) -> bool {
    match glob::Pattern::new(pattern) {
        Ok(pat) => pat.matches(filename),
        Err(e) => {
            eprintln!("Warning: invalid glob pattern '{}': {}", pattern, e);
            false
        }
    }
}

/// Recursively get files with optional depth limit
pub fn get_files_recursive(
    path: &Path,
    include_hidden: bool,
    max_depth: Option<usize>,
) -> Result<Vec<FileEntry>, io::Error> {
    let mut files = Vec::new();
    collect_files_recursive(path, include_hidden, max_depth, 0, &mut files)?;
    Ok(files)
}

fn collect_files_recursive(
    path: &Path,
    include_hidden: bool,
    max_depth: Option<usize>,
    current_depth: usize,
    files: &mut Vec<FileEntry>,
) -> Result<(), io::Error> {
    // Check depth limit
    if let Some(max) = max_depth {
        if current_depth > max {
            return Ok(());
        }
    }
    
    let entries: Vec<fs::DirEntry> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry: &fs::DirEntry| {
            include_hidden || !entry.file_name().to_string_lossy().starts_with('.')
        })
        .collect();
    
    let mut file_entries: Vec<FileEntry> = entries
        .par_iter()
        .map(|entry| {
            let file_entry = map_data(entry);
            file_entry
        })
        .filter_map(Result::ok)
        .collect();
    
    files.append(&mut file_entries);
    
    // Recurse into directories
    if current_depth < max_depth.unwrap_or(usize::MAX) {
        for entry in entries {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let _ = collect_files_recursive(
                        &entry.path(),
                        include_hidden,
                        max_depth,
                        current_depth + 1,
                        files,
                    );
                }
            }
        }
    }
    
    Ok(())
}
