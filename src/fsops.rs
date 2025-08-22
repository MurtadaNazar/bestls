// src/fsops.rs
use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::Serialize;
use std::{fs, io, path::Path};
use strum::Display;

#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};
#[cfg(unix)]
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Display, Serialize, Clone)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub e_type: FileType,
    pub len_bytes: u64,
    pub human_size: String,
    pub modified: String,
    pub permissions: String,
    pub owner: String,
    pub group: String,
}

pub fn get_files(path: &Path, include_hidden: bool) -> Result<Vec<FileEntry>, io::Error> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| include_hidden || !entry.file_name().to_string_lossy().starts_with('.'))
        .collect::<Vec<_>>();

    let files: Vec<FileEntry> = entries
        .par_iter()
        .map(map_data)
        .filter_map(Result::ok)
        .collect();

    Ok(files)
}

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
    let permissions: String = if cfg!(unix) {
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
    } else {
        "N/A".into()
    };

    // Owner / Group
    let (owner_name, group_name) = get_owner_group(&metadata);

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

#[cfg(unix)]
fn get_owner_group(metadata: &fs::Metadata) -> (String, String) {
    let uid: u32 = metadata.uid();
    let gid: u32 = metadata.gid();
    let user: String = get_user_by_uid(uid)
        .map(|u: users::User| u.name().to_string_lossy().into_owned())
        .unwrap_or(uid.to_string());
    let group: String = get_group_by_gid(gid)
        .map(|g: users::Group| g.name().to_string_lossy().into_owned())
        .unwrap_or(gid.to_string());
    (user, group)
}

#[cfg(not(unix))]
fn get_owner_group(_metadata: &fs::Metadata) -> (String, String) {
    ("N/A".into(), "N/A".into())
}
