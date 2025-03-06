use std::path::{Path, PathBuf};

pub mod embed;
pub mod file;
pub mod pagination;
pub mod xdg;

pub fn get_files(path: &PathBuf) -> std::io::Result<file::Directory> {
    file::directory_content(path)
}

pub fn get_file_content(file_path: &PathBuf) -> Result<String, std::io::Error> {
    file::file_content(file_path)
}

pub fn path_to_string(path: &Path) -> String {
    path.to_str().unwrap().to_string()
}
