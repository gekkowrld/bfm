use std::path::PathBuf;

pub mod embed;
pub mod file;
pub mod pagination;
pub mod xdg;

pub fn get_files(path: &PathBuf) -> std::io::Result<file::Directory> {
    file::directory_content(path)
}
