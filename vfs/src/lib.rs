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
use std::io::Result;
use suppaftp::FtpStream;

mod embed;
mod ftp;
mod local;

pub type FTPStream = FtpStream;

pub struct FileInformation {
    pub file: FileInfo,
    pub content: String,
}

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub is_ftp: bool,
}

#[derive(Debug)]
pub struct DirectoryInformation {
    pub name: String,
    pub files: Vec<FileInfo>,
}

impl DirectoryInformation {
    pub fn new(name: String, files: Vec<FileInfo>) -> Self {
        Self { name, files }
    }
}

impl fmt::Display for DirectoryInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut files = String::new();

        self.files.iter().for_each(|file| {
            files.push_str(format!("Path: {} Is Dir: {}\n", file.name, file.is_dir).as_str())
        });

        write!(f, "Path: {}\nFiles:\n{}\n", self.name, files)
    }
}

pub enum FS<'a> {
    Local,
    FTP(&'a mut FTPStream),
}

impl FileInformation {
    pub fn new(file: FileInfo, content: String) -> Self {
        Self { file, content }
    }
}

pub fn connect(address: &str, username: &str, password: &str) -> Option<FtpStream> {
    ftp::connect(address, username, password)
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
        FS::FTP(stream) => ftp::read_file(stream, filename),
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
        FS::FTP(stream) => ftp::list_files(stream, path),
    }
}

/// Get the app icon/logo
pub fn get_icon() -> Vec<u8> {
    embed::Assets::get_logo()
}
