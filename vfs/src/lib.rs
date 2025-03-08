//! A library for reading files from different file systems.
//!

#![deny(clippy::module_name_repetitions)]
#![deny(clippy::similar_names)]
#![deny(clippy::doc_lazy_continuation)]
#![deny(clippy::empty_docs)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::unnecessary_safety_doc)]
#![deny(clippy::wildcard_imports)]
#![deny(clippy::suspicious_doc_comments)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unnecessary_to_owned)]

use std::fmt;
use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

mod embed;
mod local;

pub struct FileInformation {
    pub file: File,
    pub content: String,
}

pub struct DirectoryInformation<'a> {
    pub name: &'a str,
    pub files: Vec<PathBuf>,
}

impl fmt::Display for DirectoryInformation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut files = String::new();

        self.files.iter().for_each(|file| {
            files.push_str(
                format!(
                    "Path: {} Is Dir: {}\n",
                    file.to_str().unwrap(),
                    file.is_dir()
                )
                .as_str(),
            )
        });

        write!(f, "Path: {}\nFiles:\n{}\n", self.name, files)
    }
}

pub enum FS {
    Local,
}

impl FileInformation {
    pub fn new(file: File, content: String) -> Self {
        Self { file, content }
    }
}

/// Read the content of a file.
/// # Arguments
/// * `fs_type` - The type of file system to read from.
/// * `filename` - The name of the file to read.
/// # Returns
/// A `Result` with the file information.
/// # Errors
/// If the file cannot be read.
pub fn read_file(fs_type: FS, filename: &str) -> Result<FileInformation> {
    match fs_type {
        FS::Local => local::read_file(filename),
    }
}

/// List the files in a directory.
/// # Arguments
/// * `fs_type` - The type of file system to read from.
/// * `path` - The path to the directory to list.
/// # Returns
/// A `Result` with a vector of file names.
/// # Errors
/// If the directory cannot be read.
pub fn list_files(fs_type: FS, path: &str) -> Result<DirectoryInformation> {
    match fs_type {
        FS::Local => local::list_files(path),
    }
}

/// Get the app icon/logo
pub fn get_icon() -> Vec<u8> {
    embed::Assets::get_logo()
}
