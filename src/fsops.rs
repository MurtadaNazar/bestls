// src/fsops.rs
use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::Serialize;
use std::{fs, io, path::Path};
use strum::Display;

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
}

pub fn get_files(path: &Path) -> Result<Vec<FileEntry>, io::Error> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(path)?
        .filter_map(Result::ok)
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
        .map(|m| {
            let dt: DateTime<Utc> = m.into();
            dt.format("%a %d %b %Y %H:%M:%S").to_string()
        })
        .unwrap_or_default();

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
    })
}
