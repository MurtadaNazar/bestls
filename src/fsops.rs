// src/fsops.rs
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
